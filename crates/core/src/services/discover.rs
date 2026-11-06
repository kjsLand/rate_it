//! Discover: discography playlists, the album queue and "My Top 10".
//! Also the album loader that Collaborators and Share use.

use std::collections::HashSet;
use std::sync::Arc;

use rate_it_spotify::{SimplifiedArtist, SimplifiedTrack};
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::App;
use crate::convert::track_uri;
use crate::dto::{DiscoverMode, PlaylistResult, Progress, QueueResult};
use crate::error::{CoreError, CoreResult};
use crate::progress::{CancellationToken, ProgressSink};

/// The most album requests at the same time.
const PARALLEL_ALBUMS: usize = 4;
/// The number of songs in "My Top 10".
const TOP_COUNT: usize = 10;
/// The description of the playlists that the app makes.
pub(crate) const PLAYLIST_DESCRIPTION: &str = "Made by rate_it.";

impl App {
    /// The tracks of an album, from the cache table or from `GET /albums/{id}`.
    pub(crate) async fn album_tracks(&self, album_id: &str) -> CoreResult<Vec<SimplifiedTrack>> {
        if let Some(body) = self.repo.cached_album(album_id).await?
            && let Ok(tracks) = serde_json::from_str::<Vec<SimplifiedTrack>>(&body)
        {
            return Ok(tracks);
        }
        let album = self.spotify.album(album_id).await?;
        let body = serde_json::to_string(&album.tracks.items)
            .map_err(|e| CoreError::Invalid(format!("album cache: {e}")))?;
        self.repo.cache_album(album_id, &body).await?;
        Ok(album.tracks.items)
    }

    /// The main artist of the song that plays now, or the given artist.
    pub(crate) async fn target_artist(
        &self,
        artist_id: Option<&str>,
    ) -> CoreResult<SimplifiedArtist> {
        if let Some(id) = artist_id {
            let artist = self
                .spotify
                .artist(crate::services::spotify_id(id)?)
                .await?;
            return Ok(SimplifiedArtist {
                id: artist.id,
                name: artist.name,
            });
        }
        let track = self.playing_track().await?;
        track
            .artists
            .into_iter()
            .next()
            .ok_or(CoreError::NothingPlaying)
    }

    /// All tracks of the albums and singles of an artist, in album order.
    /// One `GET /albums/{id}` per album, no more than 4 at the same time.
    pub(crate) async fn discography_tracks(
        &self,
        artist_id: &str,
        task: &str,
        progress: &dyn ProgressSink,
        cancel: &CancellationToken,
    ) -> CoreResult<Vec<SimplifiedTrack>> {
        let albums = self.spotify.artist_albums(artist_id).await?;
        let total = u32::try_from(albums.len()).unwrap_or(u32::MAX);
        progress.report(Progress {
            task: task.into(),
            done: 0,
            total,
        });
        let limiter = Arc::new(Semaphore::new(PARALLEL_ALBUMS));
        let mut set = JoinSet::new();
        for (index, album) in albums.into_iter().enumerate() {
            let (app, limiter) = (self.clone(), limiter.clone());
            set.spawn(async move {
                let _permit = limiter.acquire_owned().await;
                (index, app.album_tracks(&album.id).await)
            });
        }
        let mut results: Vec<Option<Vec<SimplifiedTrack>>> = vec![None; set.len()];
        let mut done = 0;
        loop {
            let next = tokio::select! {
                () = cancel.cancelled() => {
                    set.abort_all();
                    return Err(CoreError::Cancelled);
                }
                next = set.join_next() => next,
            };
            let Some(joined) = next else { break };
            let (index, tracks) =
                joined.map_err(|e| CoreError::Invalid(format!("album task failed: {e}")))?;
            if let Some(slot) = results.get_mut(index) {
                *slot = Some(tracks?);
            }
            done += 1;
            progress.report(Progress {
                task: task.into(),
                done,
                total,
            });
        }
        Ok(results.into_iter().flatten().flatten().collect())
    }

    /// Makes a playlist of the discography of the current artist (or `artist_id`).
    /// Tracks are keyed by ID. Unrated mode removes rated tracks by ID.
    pub async fn discover_playlist(
        &self,
        artist_id: Option<&str>,
        mode: DiscoverMode,
        progress: &dyn ProgressSink,
        cancel: &CancellationToken,
    ) -> CoreResult<PlaylistResult> {
        let artist = self.target_artist(artist_id).await?;
        let tracks = self
            .discography_tracks(&artist.id, "discover", progress, cancel)
            .await?;
        let mut seen = HashSet::new();
        let mut uris = Vec::new();
        for track in tracks {
            if !seen.insert(track.id.clone()) {
                continue;
            }
            if mode == DiscoverMode::Unrated && self.repo.current_rating(&track.id).await?.is_some()
            {
                continue;
            }
            uris.push(track_uri(&track.id));
        }
        if uris.is_empty() {
            return Err(CoreError::Invalid("No songs to add".into()));
        }
        let name = match mode {
            DiscoverMode::Full => artist.name,
            DiscoverMode::Unrated => format!("{} (unrated)", artist.name),
        };
        if cancel.is_cancelled() {
            return Err(CoreError::Cancelled);
        }
        self.make_playlist(&name, &uris).await
    }

    /// Makes a playlist and adds the songs (in groups of 100, in the client).
    pub(crate) async fn make_playlist(
        &self,
        name: &str,
        uris: &[String],
    ) -> CoreResult<PlaylistResult> {
        let playlist = self
            .spotify
            .create_playlist(name, PLAYLIST_DESCRIPTION)
            .await?;
        self.spotify.add_playlist_items(&playlist.id, uris).await?;
        Ok(PlaylistResult {
            playlist_id: playlist.id,
            name: playlist.name,
            track_count: u32::try_from(uris.len()).unwrap_or(u32::MAX),
        })
    }

    /// Adds all songs of the current album to the queue.
    pub async fn queue_current_album(&self) -> CoreResult<QueueResult> {
        let track = self.playing_track().await?;
        let tracks = self.album_tracks(&track.album.id).await?;
        self.queue_uris(tracks.iter().map(|t| track_uri(&t.id)))
            .await
    }

    /// "My Top 10": adds your 10 highest-rated songs of the artist to the queue.
    /// It replaces Top 10, because Spotify removed `GET /artists/{id}/top-tracks`.
    pub async fn queue_my_top_ten(&self, artist_id: Option<&str>) -> CoreResult<QueueResult> {
        let artist = self.target_artist(artist_id).await?;
        let rows = self.repo.artist_ratings(&artist.id).await?;
        if rows.is_empty() {
            return Err(CoreError::NotFound(format!(
                "You have no ratings of {}",
                artist.name
            )));
        }
        self.queue_uris(rows.iter().take(TOP_COUNT).map(|r| track_uri(&r.track_id)))
            .await
    }

    async fn queue_uris(&self, uris: impl Iterator<Item = String>) -> CoreResult<QueueResult> {
        let mut added = 0;
        for uri in uris {
            self.spotify.add_to_queue(&uri).await?;
            added += 1;
        }
        Ok(QueueResult { added })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;

    use crate::ErrorKind;
    use crate::dto::DiscoverMode;
    use crate::fakes::{album, app, track};
    use crate::progress::{CancellationToken, NoProgress};

    #[tokio::test]
    async fn full_and_unrated_playlists_key_by_track_id() {
        let (app, spotify) = app().await;
        spotify.add_album("a1", album("al1", &[("t1", &["a1"]), ("t2", &["a1"])]));
        // The same track ID on a single: it is added one time only.
        spotify.add_album(
            "a1",
            album("al2", &[("t2", &["a1"]), ("t3", &["a1", "b1"])]),
        );
        spotify.play(track("t1", "al1", &["a1"]));
        app.rate_current(9).await.unwrap();

        let cancel = CancellationToken::new();
        let full = app
            .discover_playlist(None, DiscoverMode::Full, &NoProgress, &cancel)
            .await
            .unwrap();
        assert_eq!((full.name.as_str(), full.track_count), ("Name a1", 3));
        let unrated = app
            .discover_playlist(None, DiscoverMode::Unrated, &NoProgress, &cancel)
            .await
            .unwrap();
        assert_eq!(
            (unrated.name.as_str(), unrated.track_count),
            ("Name a1 (unrated)", 2)
        );
        let playlists = spotify.playlists.lock().unwrap();
        assert_eq!(playlists[1].1, vec!["spotify:track:t2", "spotify:track:t3"]);
        assert_eq!(
            spotify.album_calls.load(Ordering::SeqCst),
            2,
            "the second run uses the cache"
        );
    }

    #[tokio::test]
    async fn cancelled_discover_stops() {
        let (app, spotify) = app().await;
        spotify.add_album("a1", album("al1", &[("t1", &["a1"])]));
        let cancel = CancellationToken::new();
        cancel.cancel();
        let err = app
            .discover_playlist(Some("a1"), DiscoverMode::Full, &NoProgress, &cancel)
            .await;
        assert_eq!(err.unwrap_err().kind(), ErrorKind::Cancelled);
        assert!(spotify.playlists.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn queue_album_and_my_top_ten() {
        let (app, spotify) = app().await;
        spotify.add_album("a1", album("al1", &[("t1", &["a1"]), ("t2", &["a1"])]));
        spotify.play(track("t1", "al1", &["a1"]));
        assert_eq!(
            app.queue_my_top_ten(None).await.unwrap_err().kind(),
            ErrorKind::NotFound
        );
        assert_eq!(app.queue_current_album().await.unwrap().added, 2);
        app.rate_current(6).await.unwrap();
        spotify.play(track("t2", "al1", &["a1"]));
        app.rate_current(10).await.unwrap();
        assert_eq!(app.queue_my_top_ten(None).await.unwrap().added, 2);
        let queue = spotify.queue.lock().unwrap();
        assert_eq!(&queue[2..], ["spotify:track:t2", "spotify:track:t1"]);
    }
}
