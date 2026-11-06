//! Library: all ratings by artist, rating changes in place, history.

use crate::App;
use crate::convert::rated_track;
use crate::dto::{ArtistSummaryDto, HistoryEntry, RatedTrackDto};
use crate::error::{CoreError, CoreResult};
use crate::services::rating;

impl App {
    /// All artists that have ratings, by name.
    pub async fn library_artists(&self) -> CoreResult<Vec<ArtistSummaryDto>> {
        let rows = self.repo.artist_summaries().await?;
        Ok(rows
            .into_iter()
            .map(|s| ArtistSummaryDto {
                id: s.id,
                name: s.name,
                rated_count: s.rated_count,
                average: s.average,
            })
            .collect())
    }

    /// The rated songs of an artist, highest rating first.
    pub async fn library_tracks(&self, artist_id: &str) -> CoreResult<Vec<RatedTrackDto>> {
        let rows = self.repo.artist_ratings(artist_id).await?;
        Ok(rows.into_iter().map(rated_track).collect())
    }

    /// Changes the rating of a song in the Library. The old rating stays in the history.
    pub async fn rerate_track(&self, track_id: &str, value: u8) -> CoreResult<RatedTrackDto> {
        let value = rating(value)?;
        let row = self
            .repo
            .rerate(track_id, value)
            .await?
            .ok_or_else(|| CoreError::NotFound(format!("track {track_id} has no rating")))?;
        Ok(rated_track(row))
    }

    /// All ratings of a song, oldest first.
    pub async fn track_history(&self, track_id: &str) -> CoreResult<Vec<HistoryEntry>> {
        let rows = self.repo.rating_history(track_id).await?;
        Ok(rows
            .into_iter()
            .map(|h| HistoryEntry {
                rating: h.rating.value(),
                rated_at: h.rated_at,
                source: match h.source {
                    rate_it_storage::RatingSource::App => "app".into(),
                    rate_it_storage::RatingSource::Import => "import".into(),
                },
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::ErrorKind;
    use crate::fakes::{app, track};

    #[tokio::test]
    async fn library_lists_artists_and_rerates_in_place() {
        let (app, spotify) = app().await;
        spotify.play(track("t1", "al1", &["a1"]));
        app.rate_current(4).await.unwrap();
        spotify.play(track("t2", "al1", &["a1"]));
        app.rate_current(8).await.unwrap();

        let artists = app.library_artists().await.unwrap();
        assert_eq!(artists.len(), 1);
        assert_eq!(artists[0].rated_count, 2);
        assert!((artists[0].average - 6.0).abs() < f64::EPSILON);

        let changed = app.rerate_track("t1", 10).await.unwrap();
        assert_eq!(changed.rating, 10);
        let tracks = app.library_tracks("a1").await.unwrap();
        assert_eq!(
            tracks.iter().map(|t| t.rating).collect::<Vec<_>>(),
            vec![10, 8]
        );
        let history = app.track_history("t1").await.unwrap();
        assert_eq!(
            history.iter().map(|h| h.rating).collect::<Vec<_>>(),
            vec![4, 10]
        );
        assert_eq!(history[0].source, "app");
    }

    #[tokio::test]
    async fn rerate_checks_range_and_track() {
        let (app, _) = app().await;
        assert_eq!(
            app.rerate_track("t1", 12).await.unwrap_err().kind(),
            ErrorKind::Invalid
        );
        assert_eq!(
            app.rerate_track("t1", 5).await.unwrap_err().kind(),
            ErrorKind::NotFound
        );
    }
}
