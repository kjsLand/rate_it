//! Artist: average rating, rated count / total count, top 5 songs, Play buttons.

use crate::App;
use crate::convert::{best_image, rated_track, track_uri};
use crate::dto::ArtistDetails;
use crate::error::CoreResult;
use crate::services::spotify_id;

/// The number of songs in the top list of the Artist screen.
const TOP_SONGS: usize = 5;

impl App {
    /// The Artist screen for the current artist (or `artist_id`).
    pub async fn artist_details(&self, artist_id: Option<&str>) -> CoreResult<ArtistDetails> {
        let target = self.target_artist(artist_id).await?;
        let artist = self.spotify.artist(&target.id).await?;
        let albums = self.spotify.artist_albums(&target.id).await?;
        let total_count = albums.iter().map(|a| a.total_tracks).sum();
        let rows = self.repo.artist_ratings(&target.id).await?;
        let rated_count = u32::try_from(rows.len()).unwrap_or(u32::MAX);
        let average = if rows.is_empty() {
            None
        } else {
            let sum: f64 = rows.iter().map(|r| f64::from(r.rating.value())).sum();
            Some((sum / f64::from(rated_count) * 10.0).round() / 10.0)
        };
        Ok(ArtistDetails {
            id: artist.id,
            name: artist.name,
            image_url: best_image(&artist.images),
            average,
            rated_count,
            total_count,
            top: rows.into_iter().take(TOP_SONGS).map(rated_track).collect(),
        })
    }

    /// Adds one song to the queue. Each Play button sends `spotify:track:<id>` of its own song (F6).
    pub async fn play_track(&self, track_id: &str) -> CoreResult<()> {
        let uri = track_uri(spotify_id(track_id)?);
        Ok(self.spotify.add_to_queue(&uri).await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::ErrorKind;
    use crate::fakes::{album, app, track};

    #[tokio::test]
    async fn artist_details_count_and_average() {
        let (app, spotify) = app().await;
        spotify.add_album(
            "a1",
            album("al1", &[("t1", &["a1"]), ("t2", &["a1"]), ("t3", &["a1"])]),
        );
        spotify.play(track("t1", "al1", &["a1"]));
        app.rate_current(7).await.unwrap();
        spotify.play(track("t2", "al1", &["a1"]));
        app.rate_current(8).await.unwrap();
        let details = app.artist_details(None).await.unwrap();
        assert_eq!((details.rated_count, details.total_count), (2, 3));
        assert_eq!(details.average, Some(7.5));
        assert_eq!(details.top[0].track_id, "t2");
    }

    #[tokio::test]
    async fn play_track_sends_its_own_uri() {
        let (app, spotify) = app().await;
        app.play_track("t1").await.unwrap();
        app.play_track("t2").await.unwrap();
        assert_eq!(
            *spotify.queue.lock().unwrap(),
            ["spotify:track:t1", "spotify:track:t2"]
        );
        assert_eq!(
            app.play_track("bad:id").await.unwrap_err().kind(),
            ErrorKind::Invalid
        );
    }
}
