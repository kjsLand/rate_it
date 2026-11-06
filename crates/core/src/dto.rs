//! The data that the commands send to the UI. Field names are camelCase in TypeScript.
//!
//! No 64-bit integers here: TypeScript `number` cannot hold all of them.

use serde::{Deserialize, Serialize};
use specta::Type;

/// An artist of a track.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ArtistDto {
    pub id: String,
    pub name: String,
}

/// The song that plays now, with its current rating.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct NowPlaying {
    pub track_id: String,
    pub name: String,
    pub artists: Vec<ArtistDto>,
    pub album_id: String,
    pub album_name: String,
    pub image_url: Option<String>,
    pub rating: Option<u8>,
}

/// One rating row in the Now Playing log or a list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RatedTrackDto {
    pub track_id: String,
    pub track_name: String,
    pub album_name: Option<String>,
    pub artist_id: String,
    pub artist_name: String,
    pub rating: u8,
    pub rated_at: String,
}

/// One artist in the Library.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ArtistSummaryDto {
    pub id: String,
    pub name: String,
    pub rated_count: u32,
    pub average: f64,
}

/// One row of the history of a track.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntry {
    pub rating: u8,
    pub rated_at: String,
    pub source: String,
}

/// The Artist screen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ArtistDetails {
    pub id: String,
    pub name: String,
    pub image_url: Option<String>,
    pub average: Option<f64>,
    pub rated_count: u32,
    pub total_count: u32,
    pub top: Vec<RatedTrackDto>,
}

/// The song card of the Share screen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SongCard {
    pub name: String,
    pub artist_name: String,
    pub album_name: String,
    pub image_url: Option<String>,
    pub rating: Option<u8>,
}

/// One track of the album card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AlbumCardTrack {
    pub track_id: String,
    pub name: String,
    pub rating: Option<u8>,
}

/// The album card of the Share screen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AlbumCard {
    pub name: String,
    pub artist_name: String,
    pub image_url: Option<String>,
    pub average: Option<f64>,
    pub tracks: Vec<AlbumCardTrack>,
}

/// One artist and the minimum rating for the Build Playlist screen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ArtistSelection {
    pub artist_id: String,
    pub min_rating: u8,
}

/// The input of the Build Playlist screen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistRequest {
    pub name: String,
    pub selections: Vec<ArtistSelection>,
}

/// A playlist that the app made.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistResult {
    pub playlist_id: String,
    pub name: String,
    pub track_count: u32,
}

/// An artist who made tracks with the current artist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Collaborator {
    pub id: String,
    pub name: String,
    pub track_count: u32,
}

/// Which songs a discography playlist gets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum DiscoverMode {
    /// All songs of the artist's albums and singles.
    Full,
    /// Only the songs that have no rating.
    Unrated,
}

/// The number of songs that a command added to the queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct QueueResult {
    pub added: u32,
}

/// The report of an import.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportReportDto {
    pub imported: u32,
    pub skipped: u32,
    pub failed: u32,
}

/// The progress of a long command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub task: String,
    pub done: u32,
    pub total: u32,
}

/// The sign-in state for the Settings screen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AccountStatus {
    pub signed_in: bool,
    pub client_id: Option<String>,
    pub display_name: Option<String>,
}
