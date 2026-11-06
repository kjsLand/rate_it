//! Share: the song card and the album card.

use crate::App;
use crate::convert::best_image;
use crate::dto::{AlbumCard, AlbumCardTrack, SongCard};
use crate::error::CoreResult;

impl App {
    /// The card of the song that plays now.
    pub async fn song_card(&self) -> CoreResult<SongCard> {
        let track = self.playing_track().await?;
        let rating = self
            .repo
            .current_rating(&track.id)
            .await?
            .map(|r| r.rating.value());
        Ok(SongCard {
            artist_name: track
                .artists
                .first()
                .map(|a| a.name.clone())
                .unwrap_or_default(),
            album_name: track.album.name.clone(),
            image_url: best_image(&track.album.images),
            name: track.name,
            rating,
        })
    }

    /// The card of the album that plays now: each track with its rating, and the average.
    pub async fn album_card(&self) -> CoreResult<AlbumCard> {
        let track = self.playing_track().await?;
        let album_tracks = self.album_tracks(&track.album.id).await?;
        let mut tracks = Vec::with_capacity(album_tracks.len());
        let (mut sum, mut count) = (0_u32, 0_u32);
        for t in album_tracks {
            let rating = self
                .repo
                .current_rating(&t.id)
                .await?
                .map(|r| r.rating.value());
            if let Some(r) = rating {
                sum += u32::from(r);
                count += 1;
            }
            tracks.push(AlbumCardTrack {
                track_id: t.id,
                name: t.name,
                rating,
            });
        }
        let average =
            (count > 0).then(|| (f64::from(sum) / f64::from(count) * 10.0).round() / 10.0);
        Ok(AlbumCard {
            name: track.album.name.clone(),
            artist_name: track
                .artists
                .first()
                .map(|a| a.name.clone())
                .unwrap_or_default(),
            image_url: best_image(&track.album.images),
            average,
            tracks,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::fakes::{album, app, track};

    #[tokio::test]
    async fn cards_show_ratings() {
        let (app, spotify) = app().await;
        spotify.add_album("a1", album("al1", &[("t1", &["a1"]), ("t2", &["a1"])]));
        spotify.play(track("t1", "al1", &["a1"]));
        assert_eq!(app.song_card().await.unwrap().rating, None);
        app.rate_current(8).await.unwrap();
        let song = app.song_card().await.unwrap();
        assert_eq!(
            (song.rating, song.artist_name.as_str()),
            (Some(8), "Name a1")
        );
        let card = app.album_card().await.unwrap();
        assert_eq!(card.average, Some(8.0));
        assert_eq!(
            card.tracks.iter().map(|t| t.rating).collect::<Vec<_>>(),
            vec![Some(8), None]
        );
    }
}
