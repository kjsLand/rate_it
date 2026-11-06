//! The endpoints that the app uses, as the [`SpotifyApi`] trait.
//!
//! Only the endpoints of `docs/SPEC.md`, "Spotify client". No batch endpoints, no
//! `/artists/{id}/top-tracks`, no `/users/{id}/playlists`, no `/playlists/{id}/tracks`.

use async_trait::async_trait;
use reqwest::{Method, StatusCode};
use serde_json::json;

use crate::client::SpotifyClient;
use crate::error::{SpotifyError, SpotifyResult};
use crate::model::{
    Album, Artist, CurrentlyPlaying, Page, Playlist, SimplifiedAlbum, SimplifiedTrack, Track,
    UserProfile,
};

/// The most items in one `POST /playlists/{id}/items` request.
pub const MAX_PLAYLIST_ITEMS: usize = 100;
/// The most albums in one page of `GET /artists/{id}/albums` (10 since February 2026).
const ARTIST_ALBUMS_PAGE: u32 = 10;
/// The most tracks in one page of `GET /albums/{id}/tracks`.
const ALBUM_TRACKS_PAGE: u32 = 50;
/// A safety limit for paging loops.
const MAX_PAGES: u32 = 200;

/// The Spotify calls of the app. `core` uses this trait, so tests can give a fake.
#[async_trait]
pub trait SpotifyApi: Send + Sync + std::fmt::Debug {
    /// `GET /me`.
    async fn me(&self) -> SpotifyResult<UserProfile>;
    /// `GET /me/player/currently-playing`. `None` for HTTP 204 or when no track plays.
    async fn currently_playing(&self) -> SpotifyResult<Option<Track>>;
    /// `GET /tracks/{id}`. `None` for HTTP 404.
    async fn track(&self, id: &str) -> SpotifyResult<Option<Track>>;
    /// `GET /artists/{id}`.
    async fn artist(&self, id: &str) -> SpotifyResult<Artist>;
    /// All pages of `GET /artists/{id}/albums?include_groups=album,single`.
    async fn artist_albums(&self, id: &str) -> SpotifyResult<Vec<SimplifiedAlbum>>;
    /// `GET /albums/{id}`, with all pages of its tracks.
    async fn album(&self, id: &str) -> SpotifyResult<Album>;
    /// `POST /me/player/queue`.
    async fn add_to_queue(&self, uri: &str) -> SpotifyResult<()>;
    /// `POST /me/playlists`.
    async fn create_playlist(&self, name: &str, description: &str) -> SpotifyResult<Playlist>;
    /// `POST /playlists/{id}/items`, in groups of 100.
    async fn add_playlist_items(&self, playlist_id: &str, uris: &[String]) -> SpotifyResult<()>;
}

/// Refuses an ID that could change the request path.
fn check_id(id: &str) -> SpotifyResult<&str> {
    if !id.is_empty() && id.len() <= 64 && id.chars().all(|c| c.is_ascii_alphanumeric()) {
        Ok(id)
    } else {
        Err(SpotifyError::Auth(format!("invalid Spotify ID {id:?}")))
    }
}

fn missing(endpoint: &str) -> SpotifyError {
    SpotifyError::Http {
        status: StatusCode::NO_CONTENT,
        endpoint: endpoint.to_owned(),
        message: "empty response".into(),
    }
}

#[async_trait]
impl SpotifyApi for SpotifyClient {
    async fn me(&self) -> SpotifyResult<UserProfile> {
        self.request(Method::GET, "me", &[], None)
            .await?
            .ok_or_else(|| missing("GET /me"))
    }

    async fn currently_playing(&self) -> SpotifyResult<Option<Track>> {
        let query = [("market", "from_token"), ("additional_types", "track")];
        let playing: Option<CurrentlyPlaying> = self
            .request(Method::GET, "me/player/currently-playing", &query, None)
            .await?;
        Ok(playing.and_then(|p| p.item))
    }

    async fn track(&self, id: &str) -> SpotifyResult<Option<Track>> {
        let path = format!("tracks/{}", check_id(id)?);
        match self
            .request(Method::GET, &path, &[("market", "from_token")], None)
            .await
        {
            Err(e) if e.status() == Some(StatusCode::NOT_FOUND) => Ok(None),
            other => other,
        }
    }

    async fn artist(&self, id: &str) -> SpotifyResult<Artist> {
        let path = format!("artists/{}", check_id(id)?);
        self.request(Method::GET, &path, &[], None)
            .await?
            .ok_or_else(|| missing(&path))
    }

    async fn artist_albums(&self, id: &str) -> SpotifyResult<Vec<SimplifiedAlbum>> {
        let path = format!("artists/{}/albums", check_id(id)?);
        let limit = ARTIST_ALBUMS_PAGE.to_string();
        let mut albums = Vec::new();
        for page_index in 0..MAX_PAGES {
            let offset = (page_index * ARTIST_ALBUMS_PAGE).to_string();
            let query = [
                ("include_groups", "album,single"),
                ("market", "from_token"),
                ("limit", limit.as_str()),
                ("offset", offset.as_str()),
            ];
            let page: Page<SimplifiedAlbum> = self
                .request(Method::GET, &path, &query, None)
                .await?
                .ok_or_else(|| missing(&path))?;
            let done = page.next.is_none() || page.items.is_empty();
            albums.extend(page.items);
            if done {
                break;
            }
        }
        Ok(albums)
    }

    async fn album(&self, id: &str) -> SpotifyResult<Album> {
        let id = check_id(id)?;
        let path = format!("albums/{id}");
        let mut album: Album = self
            .request(Method::GET, &path, &[("market", "from_token")], None)
            .await?
            .ok_or_else(|| missing(&path))?;
        let tracks_path = format!("albums/{id}/tracks");
        let limit = ALBUM_TRACKS_PAGE.to_string();
        let mut next = album.tracks.next.take();
        while next.is_some() {
            let offset = album.tracks.items.len().to_string();
            let query = [
                ("market", "from_token"),
                ("limit", &limit),
                ("offset", &offset),
            ];
            let page: Page<SimplifiedTrack> = self
                .request(Method::GET, &tracks_path, &query, None)
                .await?
                .ok_or_else(|| missing(&tracks_path))?;
            next = if page.items.is_empty() {
                None
            } else {
                page.next
            };
            album.tracks.items.extend(page.items);
        }
        Ok(album)
    }

    async fn add_to_queue(&self, uri: &str) -> SpotifyResult<()> {
        self.request_unit(Method::POST, "me/player/queue", &[("uri", uri)], None)
            .await
    }

    async fn create_playlist(&self, name: &str, description: &str) -> SpotifyResult<Playlist> {
        let body = json!({ "name": name, "public": false, "description": description });
        self.request(Method::POST, "me/playlists", &[], Some(&body))
            .await?
            .ok_or_else(|| missing("POST /me/playlists"))
    }

    async fn add_playlist_items(&self, playlist_id: &str, uris: &[String]) -> SpotifyResult<()> {
        let path = format!("playlists/{}/items", check_id(playlist_id)?);
        for chunk in uris.chunks(MAX_PLAYLIST_ITEMS) {
            let body = json!({ "uris": chunk });
            self.request_unit(Method::POST, &path, &[], Some(&body))
                .await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::check_id;

    #[test]
    fn check_id_refuses_path_characters() {
        assert!(check_id("7bDLEI7aaqmbASyo6zAhfH").is_ok());
        assert!(check_id("").is_err());
        assert!(check_id("../me").is_err());
        assert!(check_id("a/b").is_err());
        assert!(check_id("legacy:Joji").is_err());
    }
}
