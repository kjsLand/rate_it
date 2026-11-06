//! The error type of the services.

use rate_it_spotify::SpotifyError;
use rate_it_storage::StorageError;

/// An error from a service. The UI shows `to_string()` and uses [`CoreError::kind`].
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error(transparent)]
    Spotify(#[from] SpotifyError),
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error("Nothing playing")]
    NothingPlaying,
    #[error("{0}")]
    Invalid(String),
    #[error("{0}")]
    NotFound(String),
    #[error("Cancelled")]
    Cancelled,
}

/// A short name of the error class, for the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ErrorKind {
    NotSignedIn,
    Spotify,
    Network,
    Storage,
    NothingPlaying,
    Invalid,
    NotFound,
    Cancelled,
}

impl CoreError {
    /// The class of the error.
    pub fn kind(&self) -> ErrorKind {
        match self {
            Self::Spotify(SpotifyError::NotSignedIn) => ErrorKind::NotSignedIn,
            Self::Spotify(SpotifyError::Network { .. } | SpotifyError::Timeout { .. }) => {
                ErrorKind::Network
            }
            Self::Spotify(_) => ErrorKind::Spotify,
            Self::Storage(_) => ErrorKind::Storage,
            Self::NothingPlaying => ErrorKind::NothingPlaying,
            Self::Invalid(_) => ErrorKind::Invalid,
            Self::NotFound(_) => ErrorKind::NotFound,
            Self::Cancelled => ErrorKind::Cancelled,
        }
    }
}

pub type CoreResult<T> = Result<T, CoreError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_classifies_errors() {
        assert_eq!(
            CoreError::Spotify(SpotifyError::NotSignedIn).kind(),
            ErrorKind::NotSignedIn
        );
        let timeout = SpotifyError::Timeout {
            endpoint: "GET /me".into(),
        };
        assert_eq!(CoreError::Spotify(timeout).kind(), ErrorKind::Network);
        assert_eq!(CoreError::NothingPlaying.kind(), ErrorKind::NothingPlaying);
        assert_eq!(CoreError::Invalid("x".into()).to_string(), "x");
        assert_eq!(CoreError::Cancelled.kind(), ErrorKind::Cancelled);
    }
}
