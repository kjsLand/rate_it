//! The HTTP layer: one `reqwest::Client`, token refresh, retries and the request limit.

use std::sync::Arc;
use std::time::{Duration, Instant};

use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
use tokio::sync::{Mutex, Semaphore};
use url::Url;

use crate::auth::{TokenResponse, TokenStore, request_token};
use crate::error::{SpotifyError, SpotifyResult};

/// Settings of the client. Tests point the base URLs to a mock server.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub api_base: Url,
    pub accounts_base: Url,
    pub timeout: Duration,
    pub max_retries: u32,
    pub max_parallel: usize,
}

impl ClientConfig {
    /// The real Spotify URLs, a 10 second timeout, 3 retries for HTTP 429 and 4 parallel requests.
    pub fn spotify() -> SpotifyResult<Self> {
        let parse = |s: &str| Url::parse(s).map_err(|e| SpotifyError::Auth(e.to_string()));
        Ok(Self {
            api_base: parse("https://api.spotify.com/v1/")?,
            accounts_base: parse("https://accounts.spotify.com/")?,
            timeout: Duration::from_secs(10),
            max_retries: 3,
            max_parallel: 4,
        })
    }
}

#[derive(Debug, Default)]
struct TokenState {
    client_id: Option<String>,
    access: Option<(String, Instant)>,
    refresh: Option<String>,
}

/// The Spotify Web API client. The access token stays in this struct only.
#[derive(Debug)]
pub struct SpotifyClient {
    http: reqwest::Client,
    config: ClientConfig,
    tokens: Mutex<TokenState>,
    store: Arc<dyn TokenStore>,
    limiter: Semaphore,
}

/// The longest wait for one `Retry-After` value.
const MAX_RETRY_WAIT: Duration = Duration::from_secs(60);

impl SpotifyClient {
    /// Makes the client and loads the refresh token from `store`.
    pub fn new(config: ClientConfig, store: Arc<dyn TokenStore>) -> SpotifyResult<Self> {
        let http = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|e| SpotifyError::from_reqwest("client setup", e))?;
        let refresh = store.load()?;
        let limiter = Semaphore::new(config.max_parallel.max(1));
        let tokens = Mutex::new(TokenState {
            refresh,
            ..TokenState::default()
        });
        Ok(Self {
            http,
            config,
            tokens,
            store,
            limiter,
        })
    }

    /// Sets the Spotify Client ID. It is not a secret with PKCE.
    pub async fn set_client_id(&self, client_id: Option<String>) {
        self.tokens.lock().await.client_id = client_id;
    }

    /// The Spotify Client ID, if set.
    pub async fn client_id(&self) -> Option<String> {
        self.tokens.lock().await.client_id.clone()
    }

    /// True if the client has a refresh token.
    pub async fn is_signed_in(&self) -> bool {
        self.tokens.lock().await.refresh.is_some()
    }

    /// Exchanges the authorization code for tokens and keeps the refresh token.
    pub async fn complete_sign_in(
        &self,
        code: &str,
        verifier: &str,
        redirect_uri: &str,
    ) -> SpotifyResult<()> {
        let mut state = self.tokens.lock().await;
        let client_id = state.client_id.clone().ok_or(SpotifyError::NotSignedIn)?;
        let form = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", redirect_uri),
            ("client_id", client_id.as_str()),
            ("code_verifier", verifier),
        ];
        let response = request_token(&self.http, &self.config.accounts_base, &form).await?;
        if response.refresh_token.is_none() {
            return Err(SpotifyError::Auth("Spotify sent no refresh token".into()));
        }
        self.keep_tokens(&mut state, response)
    }

    /// Forgets all tokens, also in the store.
    pub async fn sign_out(&self) -> SpotifyResult<()> {
        let mut state = self.tokens.lock().await;
        state.access = None;
        state.refresh = None;
        self.store.clear()
    }

    fn keep_tokens(&self, state: &mut TokenState, response: TokenResponse) -> SpotifyResult<()> {
        let lifetime = Duration::from_secs(response.expires_in);
        state.access = Some((response.access_token, Instant::now() + lifetime));
        if let Some(refresh) = response.refresh_token {
            self.store.save(&refresh)?;
            state.refresh = Some(refresh);
        }
        Ok(())
    }

    /// Returns a valid access token. Refreshes it when it is old, or when `force` is true.
    async fn access_token(&self, force: bool) -> SpotifyResult<String> {
        let mut state = self.tokens.lock().await;
        let margin = Duration::from_secs(30);
        if !force
            && let Some((token, expires)) = &state.access
            && *expires > Instant::now() + margin
        {
            return Ok(token.clone());
        }
        let refresh = state.refresh.clone().ok_or(SpotifyError::NotSignedIn)?;
        let client_id = state.client_id.clone().ok_or(SpotifyError::NotSignedIn)?;
        let form = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh.as_str()),
            ("client_id", client_id.as_str()),
        ];
        let response = request_token(&self.http, &self.config.accounts_base, &form).await?;
        let token = response.access_token.clone();
        self.keep_tokens(&mut state, response)?;
        Ok(token)
    }

    /// Sends one API request. Returns `Ok(None)` for HTTP 204 or an empty body.
    pub(crate) async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
        body: Option<&serde_json::Value>,
    ) -> SpotifyResult<Option<T>> {
        let endpoint = format!("{method} /{path}");
        let response = self.send(method, path, query, body, &endpoint).await?;
        decode(response, &endpoint).await
    }

    /// Sends one API request and ignores the body of a success.
    pub(crate) async fn request_unit(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
        body: Option<&serde_json::Value>,
    ) -> SpotifyResult<()> {
        let endpoint = format!("{method} /{path}");
        let response = self.send(method, path, query, body, &endpoint).await?;
        if response.status().is_success() {
            return Ok(());
        }
        decode::<serde::de::IgnoredAny>(response, &endpoint)
            .await
            .map(|_| ())
    }

    /// The retry loop: refresh one time on 401, wait and retry on 429.
    async fn send(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
        body: Option<&serde_json::Value>,
        endpoint: &str,
    ) -> SpotifyResult<reqwest::Response> {
        let url = self
            .config
            .api_base
            .join(path)
            .map_err(|e| SpotifyError::Auth(format!("bad API path {path}: {e}")))?;
        let _permit = self
            .limiter
            .acquire()
            .await
            .map_err(|_| SpotifyError::Auth("request limiter closed".into()))?;
        let mut refreshed = false;
        let mut retries = 0;
        loop {
            let token = self.access_token(false).await?;
            let mut builder = self
                .http
                .request(method.clone(), url.clone())
                .bearer_auth(token);
            if !query.is_empty() {
                builder = builder.query(query);
            }
            if let Some(json) = body {
                builder = builder.json(json);
            }
            let response = builder
                .send()
                .await
                .map_err(|e| SpotifyError::from_reqwest(endpoint, e))?;
            match response.status() {
                StatusCode::UNAUTHORIZED if !refreshed => {
                    refreshed = true;
                    self.access_token(true).await?;
                }
                StatusCode::TOO_MANY_REQUESTS if retries < self.config.max_retries => {
                    retries += 1;
                    tokio::time::sleep(retry_after(&response)).await;
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    return Err(SpotifyError::RateLimited {
                        endpoint: endpoint.to_owned(),
                    });
                }
                _ => return Ok(response),
            }
        }
    }
}

/// The wait that the `Retry-After` header asks for (seconds). One second if it is missing.
fn retry_after(response: &reqwest::Response) -> Duration {
    response
        .headers()
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.trim().parse::<u64>().ok())
        .map_or(Duration::from_secs(1), |s| {
            Duration::from_secs(s).min(MAX_RETRY_WAIT)
        })
}

#[derive(serde::Deserialize)]
struct ApiErrorBody {
    error: ApiErrorDetail,
}

#[derive(serde::Deserialize)]
struct ApiErrorDetail {
    message: String,
}

async fn decode<T: DeserializeOwned>(
    response: reqwest::Response,
    endpoint: &str,
) -> SpotifyResult<Option<T>> {
    let status = response.status();
    let bytes = response
        .bytes()
        .await
        .map_err(|e| SpotifyError::from_reqwest(endpoint, e))?;
    if !status.is_success() {
        let message = serde_json::from_slice::<ApiErrorBody>(&bytes).map_or_else(
            |_| String::from_utf8_lossy(&bytes).into_owned(),
            |b| b.error.message,
        );
        return Err(SpotifyError::Http {
            status,
            endpoint: endpoint.to_owned(),
            message,
        });
    }
    if status == StatusCode::NO_CONTENT || bytes.iter().all(u8::is_ascii_whitespace) {
        return Ok(None);
    }
    serde_json::from_slice(&bytes)
        .map(Some)
        .map_err(|source| SpotifyError::Decode {
            endpoint: endpoint.to_owned(),
            source,
        })
}
