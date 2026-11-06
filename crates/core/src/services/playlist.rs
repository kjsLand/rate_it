//! Build Playlist: songs of the selected artists with a minimum rating.

use std::collections::HashSet;

use crate::App;
use crate::convert::track_uri;
use crate::dto::{PlaylistRequest, PlaylistResult};
use crate::error::{CoreError, CoreResult};
use crate::services::rating;

/// The longest playlist name that the app sends.
const MAX_NAME: usize = 100;

impl App {
    /// Makes a playlist from your ratings. The name is trimmed and required (F8, F14).
    pub async fn build_playlist(&self, request: &PlaylistRequest) -> CoreResult<PlaylistResult> {
        let name = request.name.trim();
        if name.is_empty() {
            return Err(CoreError::Invalid("Enter a playlist name".into()));
        }
        if name.chars().count() > MAX_NAME {
            return Err(CoreError::Invalid(format!(
                "The name is longer than {MAX_NAME} characters"
            )));
        }
        if request.selections.is_empty() {
            return Err(CoreError::Invalid("Select at least one artist".into()));
        }
        let mut seen = HashSet::new();
        let mut uris = Vec::new();
        for selection in &request.selections {
            let min = rating(selection.min_rating)?;
            for row in self.repo.artist_ratings(&selection.artist_id).await? {
                if row.rating >= min && seen.insert(row.track_id.clone()) {
                    uris.push(track_uri(&row.track_id));
                }
            }
        }
        if uris.is_empty() {
            return Err(CoreError::Invalid("No songs have that rating".into()));
        }
        self.make_playlist(name, &uris).await
    }
}

#[cfg(test)]
mod tests {
    use crate::ErrorKind;
    use crate::dto::{ArtistSelection, PlaylistRequest};
    use crate::fakes::{app, track};

    fn request(name: &str, selections: &[(&str, u8)]) -> PlaylistRequest {
        PlaylistRequest {
            name: name.into(),
            selections: selections
                .iter()
                .map(|(a, m)| ArtistSelection {
                    artist_id: (*a).into(),
                    min_rating: *m,
                })
                .collect(),
        }
    }

    #[tokio::test]
    async fn builds_playlist_with_min_rating() {
        let (app, spotify) = app().await;
        for (id, artist, value) in [("t1", "a1", 9), ("t2", "a1", 5), ("t3", "b1", 10)] {
            spotify.play(track(id, "al1", &[artist]));
            app.rate_current(value).await.unwrap();
        }
        let result = app
            .build_playlist(&request("  Mix  ", &[("a1", 8), ("b1", 1)]))
            .await
            .unwrap();
        assert_eq!((result.name.as_str(), result.track_count), ("Mix", 2));
        let playlists = spotify.playlists.lock().unwrap();
        assert_eq!(playlists[0].1, vec!["spotify:track:t1", "spotify:track:t3"]);
    }

    #[tokio::test]
    async fn refuses_bad_input() {
        let (app, _) = app().await;
        let kind = |r: crate::CoreResult<_>| {
            r.map(|_: crate::dto::PlaylistResult| ())
                .unwrap_err()
                .kind()
        };
        assert_eq!(
            kind(app.build_playlist(&request("   ", &[("a1", 5)])).await),
            ErrorKind::Invalid
        );
        assert_eq!(
            kind(app.build_playlist(&request("Mix", &[])).await),
            ErrorKind::Invalid
        );
        assert_eq!(
            kind(app.build_playlist(&request("Mix", &[("a1", 0)])).await),
            ErrorKind::Invalid
        );
        assert_eq!(
            kind(app.build_playlist(&request("Mix", &[("a1", 5)])).await),
            ErrorKind::Invalid
        );
    }
}
