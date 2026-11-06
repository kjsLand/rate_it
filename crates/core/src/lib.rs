//! Rating, playlist and discover services of `rate_it`. See `docs/SPEC.md`.
//!
//! [`App`] gets the Spotify client and the repository as trait objects, so tests can give fakes.

mod convert;
pub mod dto;
mod error;
pub mod legacy;
mod progress;
mod services;

#[cfg(test)]
mod fakes;

use std::sync::Arc;

use rate_it_spotify::SpotifyApi;
use rate_it_storage::RatingsRepository;

pub use error::{CoreError, CoreResult, ErrorKind};
pub use progress::{CancellationToken, NoProgress, ProgressSink};

/// The services of the app.
#[derive(Debug, Clone)]
pub struct App {
    spotify: Arc<dyn SpotifyApi>,
    repo: Arc<dyn RatingsRepository>,
}

impl App {
    /// Makes the services from a Spotify client and a repository.
    pub fn new(spotify: Arc<dyn SpotifyApi>, repo: Arc<dyn RatingsRepository>) -> Self {
        Self { spotify, repo }
    }
}

/// The key of the Spotify Client ID in `app_settings`.
pub const CLIENT_ID_SETTING: &str = "spotify_client_id";
