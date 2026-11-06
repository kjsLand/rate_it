//! Response shapes of the Spotify endpoints that the app uses.

use serde::{Deserialize, Serialize};

/// An image of an album or an artist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// An artist in a track or an album.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimplifiedArtist {
    pub id: String,
    pub name: String,
}

/// The full artist object (`GET /artists/{id}`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub images: Vec<Image>,
}

/// An album in a track or in the albums of an artist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimplifiedAlbum {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub total_tracks: u32,
    #[serde(default)]
    pub images: Vec<Image>,
    #[serde(default)]
    pub artists: Vec<SimplifiedArtist>,
}

/// A track without its album (in an album's track list).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimplifiedTrack {
    pub id: String,
    pub name: String,
    pub uri: String,
    #[serde(default)]
    pub artists: Vec<SimplifiedArtist>,
}

/// The full track object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub uri: String,
    #[serde(default)]
    pub explicit: bool,
    pub album: SimplifiedAlbum,
    pub artists: Vec<SimplifiedArtist>,
}

/// One page of a list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    #[serde(default)]
    pub total: u32,
    #[serde(default)]
    pub next: Option<String>,
}

/// The full album object, with the first page of its tracks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Album {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub images: Vec<Image>,
    #[serde(default)]
    pub artists: Vec<SimplifiedArtist>,
    pub tracks: Page<SimplifiedTrack>,
}

/// The response of `GET /me/player/currently-playing`. `item` is null for an ad.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct CurrentlyPlaying {
    pub(crate) item: Option<Track>,
}

/// The current user (`GET /me`). `country` was removed in February 2026.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub display_name: Option<String>,
}

/// A playlist that the app made.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub uri: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn track_deserializes_without_removed_fields() {
        let json = r#"{"id":"t","name":"N","uri":"spotify:track:t","explicit":false,
            "album":{"id":"a","name":"A","images":[{"url":"u","width":64,"height":64}]},
            "artists":[{"id":"x","name":"X"}]}"#;
        let t: Track = serde_json::from_str(json).unwrap();
        assert_eq!(t.album.images.len(), 1);
        assert_eq!(t.album.total_tracks, 0);
    }

    #[test]
    fn page_defaults_next_and_total() {
        let p: Page<SimplifiedArtist> = serde_json::from_str(r#"{"items":[]}"#).unwrap();
        assert_eq!((p.total, p.next), (0, None));
    }
}
