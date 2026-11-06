//! Token requests to the Spotify accounts service, and the refresh-token store.

use std::sync::Mutex;

use serde::Deserialize;
use url::Url;

use crate::error::{SpotifyError, SpotifyResult};

/// The answer of `POST /api/token`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    /// Spotify can send a new refresh token. If it does not, keep the old one.
    pub refresh_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TokenError {
    error: String,
    error_description: Option<String>,
}

/// Sends a form to `{accounts_base}api/token`. PKCE: no client secret.
pub(crate) async fn request_token(
    http: &reqwest::Client,
    accounts_base: &Url,
    form: &[(&str, &str)],
) -> SpotifyResult<TokenResponse> {
    let endpoint = "POST /api/token";
    let url = accounts_base
        .join("api/token")
        .map_err(|e| SpotifyError::Auth(format!("bad accounts URL: {e}")))?;
    let response = http
        .post(url)
        .form(form)
        .send()
        .await
        .map_err(|e| SpotifyError::from_reqwest(endpoint, e))?;
    let status = response.status();
    let bytes = response
        .bytes()
        .await
        .map_err(|e| SpotifyError::from_reqwest(endpoint, e))?;
    if !status.is_success() {
        let message = serde_json::from_slice::<TokenError>(&bytes).map_or_else(
            |_| String::from_utf8_lossy(&bytes).into_owned(),
            |e| format!("{}: {}", e.error, e.error_description.unwrap_or_default()),
        );
        return Err(SpotifyError::Auth(format!("{status}: {message}")));
    }
    serde_json::from_slice(&bytes).map_err(|source| SpotifyError::Decode {
        endpoint: endpoint.into(),
        source,
    })
}

/// Keeps the refresh token between app starts (keychain on desktop).
pub trait TokenStore: Send + Sync + std::fmt::Debug {
    fn load(&self) -> SpotifyResult<Option<String>>;
    fn save(&self, refresh_token: &str) -> SpotifyResult<()>;
    fn clear(&self) -> SpotifyResult<()>;
}

/// A token store in memory only (tests, and a fallback that forgets at exit).
#[derive(Debug, Default)]
pub struct MemoryTokenStore {
    token: Mutex<Option<String>>,
}

impl MemoryTokenStore {
    /// A store that already holds `token`.
    pub fn with_token(token: &str) -> Self {
        Self {
            token: Mutex::new(Some(token.to_owned())),
        }
    }
}

fn poisoned<T>(_: T) -> SpotifyError {
    SpotifyError::TokenStore("lock poisoned".into())
}

impl TokenStore for MemoryTokenStore {
    fn load(&self) -> SpotifyResult<Option<String>> {
        Ok(self.token.lock().map_err(poisoned)?.clone())
    }

    fn save(&self, refresh_token: &str) -> SpotifyResult<()> {
        *self.token.lock().map_err(poisoned)? = Some(refresh_token.to_owned());
        Ok(())
    }

    fn clear(&self) -> SpotifyResult<()> {
        *self.token.lock().map_err(poisoned)? = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_store_saves_loads_and_clears() {
        let store = MemoryTokenStore::default();
        assert_eq!(store.load().unwrap(), None);
        store.save("r1").unwrap();
        assert_eq!(store.load().unwrap().as_deref(), Some("r1"));
        store.clear().unwrap();
        assert_eq!(store.load().unwrap(), None);
        assert_eq!(
            MemoryTokenStore::with_token("x").load().unwrap().as_deref(),
            Some("x")
        );
    }

    #[test]
    fn token_response_accepts_missing_refresh_token() {
        let t: TokenResponse =
            serde_json::from_str(r#"{"access_token":"a","token_type":"Bearer","expires_in":3600}"#)
                .unwrap();
        assert_eq!(t.refresh_token, None);
    }
}
