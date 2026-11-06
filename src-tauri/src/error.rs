//! The error that commands return to the UI.

use rate_it_core::{CoreError, ErrorKind};
use serde::Serialize;

/// A command error: a class and a message to show.
#[derive(Debug, Clone, Serialize, specta::Type, thiserror::Error)]
#[serde(rename_all = "camelCase")]
#[error("{message}")]
pub(crate) struct CommandError {
    pub(crate) kind: ErrorKind,
    pub(crate) message: String,
}

impl CommandError {
    pub(crate) fn invalid(message: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::Invalid,
            message: message.into(),
        }
    }
}

impl From<CoreError> for CommandError {
    fn from(e: CoreError) -> Self {
        Self {
            kind: e.kind(),
            message: e.to_string(),
        }
    }
}

impl From<rate_it_spotify::SpotifyError> for CommandError {
    fn from(e: rate_it_spotify::SpotifyError) -> Self {
        CoreError::from(e).into()
    }
}

pub(crate) type CommandResult<T> = Result<T, CommandError>;
