//! Spotify Web API client and PKCE sign-in for `rate_it`. See `docs/SPEC.md`, "Spotify client".

mod api;
mod auth;
mod client;
mod error;
mod model;
mod pkce;

pub use api::{MAX_PLAYLIST_ITEMS, SpotifyApi};
pub use auth::{MemoryTokenStore, TokenResponse, TokenStore};
pub use client::{ClientConfig, SpotifyClient};
pub use error::{SpotifyError, SpotifyResult};
pub use model::{
    Album, Artist, Image, Page, Playlist, SimplifiedAlbum, SimplifiedArtist, SimplifiedTrack,
    Track, UserProfile,
};
pub use pkce::{AUTHORIZE_URL, PkceSession, SCOPES, challenge_for};
