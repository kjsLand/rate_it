//! A fake Spotify client for the service tests. The repository is an in-memory SQLite database.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use rate_it_spotify::{
    Album, Artist, Page, Playlist, SimplifiedAlbum, SimplifiedArtist, SimplifiedTrack, SpotifyApi,
    SpotifyError, SpotifyResult, Track, UserProfile,
};
use rate_it_storage::SqliteRepository;

use crate::App;

#[derive(Debug, Default)]
pub(crate) struct FakeSpotify {
    pub(crate) playing: Mutex<Option<Track>>,
    pub(crate) tracks: Mutex<HashMap<String, Track>>,
    pub(crate) failing_tracks: Mutex<HashSet<String>>,
    pub(crate) artists: Mutex<HashMap<String, Artist>>,
    pub(crate) artist_albums: Mutex<HashMap<String, Vec<SimplifiedAlbum>>>,
    pub(crate) albums: Mutex<HashMap<String, Album>>,
    pub(crate) album_calls: AtomicU32,
    pub(crate) queue: Mutex<Vec<String>>,
    pub(crate) playlists: Mutex<Vec<(String, Vec<String>)>>,
}

pub(crate) fn artist_ref(id: &str) -> SimplifiedArtist {
    SimplifiedArtist {
        id: id.into(),
        name: format!("Name {id}"),
    }
}

pub(crate) fn track(id: &str, album_id: &str, artists: &[&str]) -> Track {
    Track {
        id: id.into(),
        name: format!("Song {id}"),
        uri: format!("spotify:track:{id}"),
        explicit: false,
        album: SimplifiedAlbum {
            id: album_id.into(),
            name: format!("Album {album_id}"),
            total_tracks: 2,
            images: vec![],
            artists: vec![],
        },
        artists: artists.iter().map(|a| artist_ref(a)).collect(),
    }
}

pub(crate) fn album(id: &str, tracks: &[(&str, &[&str])]) -> Album {
    Album {
        id: id.into(),
        name: format!("Album {id}"),
        images: vec![],
        artists: vec![],
        tracks: Page {
            items: tracks
                .iter()
                .map(|(t, artists)| SimplifiedTrack {
                    id: (*t).into(),
                    name: format!("Song {t}"),
                    uri: format!("spotify:track:{t}"),
                    artists: artists.iter().map(|a| artist_ref(a)).collect(),
                })
                .collect(),
            total: 0,
            next: None,
        },
    }
}

impl FakeSpotify {
    pub(crate) fn play(&self, track: Track) {
        self.tracks
            .lock()
            .unwrap()
            .insert(track.id.clone(), track.clone());
        *self.playing.lock().unwrap() = Some(track);
    }

    pub(crate) fn add_album(&self, artist_id: &str, album: Album) {
        let simple = SimplifiedAlbum {
            id: album.id.clone(),
            name: album.name.clone(),
            total_tracks: u32::try_from(album.tracks.items.len()).unwrap(),
            images: vec![],
            artists: vec![],
        };
        self.artist_albums
            .lock()
            .unwrap()
            .entry(artist_id.into())
            .or_default()
            .push(simple);
        self.albums.lock().unwrap().insert(album.id.clone(), album);
    }
}

fn fake_error(what: &str) -> SpotifyError {
    SpotifyError::Auth(format!("fake failure: {what}"))
}

#[async_trait]
impl SpotifyApi for FakeSpotify {
    async fn me(&self) -> SpotifyResult<UserProfile> {
        Ok(UserProfile {
            id: "me".into(),
            display_name: Some("Me".into()),
        })
    }

    async fn currently_playing(&self) -> SpotifyResult<Option<Track>> {
        Ok(self.playing.lock().unwrap().clone())
    }

    async fn track(&self, id: &str) -> SpotifyResult<Option<Track>> {
        if self.failing_tracks.lock().unwrap().contains(id) {
            return Err(fake_error(id));
        }
        Ok(self.tracks.lock().unwrap().get(id).cloned())
    }

    async fn artist(&self, id: &str) -> SpotifyResult<Artist> {
        let found = self.artists.lock().unwrap().get(id).cloned();
        Ok(found.unwrap_or_else(|| Artist {
            id: id.into(),
            name: format!("Name {id}"),
            images: vec![],
        }))
    }

    async fn artist_albums(&self, id: &str) -> SpotifyResult<Vec<SimplifiedAlbum>> {
        Ok(self
            .artist_albums
            .lock()
            .unwrap()
            .get(id)
            .cloned()
            .unwrap_or_default())
    }

    async fn album(&self, id: &str) -> SpotifyResult<Album> {
        self.album_calls.fetch_add(1, Ordering::SeqCst);
        self.albums
            .lock()
            .unwrap()
            .get(id)
            .cloned()
            .ok_or_else(|| fake_error(id))
    }

    async fn add_to_queue(&self, uri: &str) -> SpotifyResult<()> {
        self.queue.lock().unwrap().push(uri.into());
        Ok(())
    }

    async fn create_playlist(&self, name: &str, _description: &str) -> SpotifyResult<Playlist> {
        let mut playlists = self.playlists.lock().unwrap();
        playlists.push((name.into(), vec![]));
        Ok(Playlist {
            id: format!("p{}", playlists.len()),
            name: name.into(),
            uri: String::new(),
        })
    }

    async fn add_playlist_items(&self, playlist_id: &str, uris: &[String]) -> SpotifyResult<()> {
        let index: usize = playlist_id.trim_start_matches('p').parse().unwrap();
        self.playlists.lock().unwrap()[index - 1]
            .1
            .extend_from_slice(uris);
        Ok(())
    }
}

/// An app with a fake Spotify client and an in-memory database.
pub(crate) async fn app() -> (App, Arc<FakeSpotify>) {
    let spotify = Arc::new(FakeSpotify::default());
    let repo = Arc::new(SqliteRepository::in_memory().await.unwrap());
    (App::new(spotify.clone(), repo), spotify)
}
