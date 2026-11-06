//! The one error type of the Spotify client (F2).

use reqwest::StatusCode;

/// An error from a Spotify request. Each variant names the endpoint.
#[derive(Debug, thiserror::Error)]
pub enum SpotifyError {
    /// Spotify answered with a status that is not a success.
    #[error("Spotify returned {status} for {endpoint}: {message}")]
    Http {
        status: StatusCode,
        endpoint: String,
        message: String,
    },
    /// The request did not get to Spotify, or the connection failed.
    #[error("network error for {endpoint}: {source}")]
    Network {
        endpoint: String,
        #[source]
        source: reqwest::Error,
    },
    /// The request took more than the timeout.
    #[error("timeout for {endpoint}")]
    Timeout { endpoint: String },
    /// The response body did not have the expected shape.
    #[error("unexpected response from {endpoint}: {source}")]
    Decode {
        endpoint: String,
        #[source]
        source: serde_json::Error,
    },
    /// Spotify sent HTTP 429 more times than the retry limit.
    #[error("rate limit for {endpoint}: no more retries")]
    RateLimited { endpoint: String },
    /// No refresh token: the user must sign in.
    #[error("not signed in to Spotify")]
    NotSignedIn,
    /// The sign-in or the token refresh failed.
    #[error("Spotify sign-in failed: {0}")]
    Auth(String),
    /// The token store (keychain or file) failed.
    #[error("token storage failed: {0}")]
    TokenStore(String),
}

impl SpotifyError {
    /// The HTTP status, if Spotify sent one.
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Self::Http { status, .. } => Some(*status),
            _ => None,
        }
    }

    pub(crate) fn from_reqwest(endpoint: &str, source: reqwest::Error) -> Self {
        if source.is_timeout() {
            Self::Timeout {
                endpoint: endpoint.to_owned(),
            }
        } else {
            Self::Network {
                endpoint: endpoint.to_owned(),
                source,
            }
        }
    }
}

pub type SpotifyResult<T> = Result<T, SpotifyError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_is_given_for_http_errors_only() {
        let e = SpotifyError::Http {
            status: StatusCode::NOT_FOUND,
            endpoint: "GET /x".into(),
            message: "no".into(),
        };
        assert_eq!(e.status(), Some(StatusCode::NOT_FOUND));
        assert!(e.to_string().contains("GET /x"));
        assert_eq!(SpotifyError::NotSignedIn.status(), None);
    }
}
