//! Now Playing: the current song, the stars and the log.

use crate::App;
use crate::convert::{now_playing, rated_track, track_info};
use crate::dto::{NowPlaying, RatedTrackDto};
use crate::error::{CoreError, CoreResult};
use crate::services::rating;
use rate_it_spotify::Track;
use rate_it_storage::{NewRating, RatingSource};

/// The number of rows in the Now Playing log.
const LOG_SIZE: u32 = 10;

impl App {
    /// The song that plays now, or `None` (HTTP 204: "Nothing playing").
    pub async fn now_playing(&self) -> CoreResult<Option<NowPlaying>> {
        let Some(track) = self.spotify.currently_playing().await? else {
            return Ok(None);
        };
        let current = self.repo.current_rating(&track.id).await?;
        Ok(Some(now_playing(&track, current.map(|r| r.rating.value()))))
    }

    /// The song that plays now. An error if nothing plays.
    pub(crate) async fn playing_track(&self) -> CoreResult<Track> {
        self.spotify
            .currently_playing()
            .await?
            .ok_or(CoreError::NothingPlaying)
    }

    /// Rates the song that plays now. A second rating adds a row; the history stays.
    pub async fn rate_current(&self, value: u8) -> CoreResult<NowPlaying> {
        let value = rating(value)?;
        let track = self.playing_track().await?;
        let new = NewRating {
            track: track_info(&track),
            rating: value,
            source: RatingSource::App,
            rated_at: None,
        };
        let row = self.repo.add_rating(&new).await?;
        Ok(now_playing(&track, Some(row.rating.value())))
    }

    /// The last 10 ratings, newest first. They come from the database, so they stay after a restart.
    pub async fn recent_ratings(&self) -> CoreResult<Vec<RatedTrackDto>> {
        let rows = self.repo.recent_ratings(LOG_SIZE).await?;
        Ok(rows.into_iter().map(rated_track).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::ErrorKind;
    use crate::fakes::{app, track};

    #[tokio::test]
    async fn nothing_playing_gives_none_and_rate_fails() {
        let (app, _) = app().await;
        assert_eq!(app.now_playing().await.unwrap(), None);
        assert_eq!(
            app.rate_current(5).await.unwrap_err().kind(),
            ErrorKind::NothingPlaying
        );
    }

    #[tokio::test]
    async fn rate_current_twice_keeps_both_in_log() {
        let (app, spotify) = app().await;
        spotify.play(track("t1", "al1", &["a1", "a2"]));
        assert_eq!(app.rate_current(7).await.unwrap().rating, Some(7));
        assert_eq!(app.rate_current(9).await.unwrap().rating, Some(9));
        assert_eq!(app.now_playing().await.unwrap().unwrap().rating, Some(9));
        let log = app.recent_ratings().await.unwrap();
        assert_eq!(log.iter().map(|r| r.rating).collect::<Vec<_>>(), vec![9, 7]);
        assert_eq!(log[0].artist_id, "a1");
    }

    #[tokio::test]
    async fn rate_current_refuses_out_of_range() {
        let (app, spotify) = app().await;
        spotify.play(track("t1", "al1", &["a1"]));
        assert_eq!(
            app.rate_current(11).await.unwrap_err().kind(),
            ErrorKind::Invalid
        );
        assert_eq!(
            app.rate_current(0).await.unwrap_err().kind(),
            ErrorKind::Invalid
        );
    }
}
