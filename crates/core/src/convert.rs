//! Conversions between Spotify, storage and UI types.

use rate_it_spotify::{Image, Track};
use rate_it_storage::{ArtistRef, RatedTrack, TrackInfo};

use crate::dto::{ArtistDto, NowPlaying, RatedTrackDto};

/// The storage form of a Spotify track.
pub(crate) fn track_info(track: &Track) -> TrackInfo {
    TrackInfo {
        id: track.id.clone(),
        name: track.name.clone(),
        album_id: Some(track.album.id.clone()),
        album_name: Some(track.album.name.clone()),
        artists: track
            .artists
            .iter()
            .map(|a| ArtistRef {
                id: a.id.clone(),
                name: a.name.clone(),
            })
            .collect(),
    }
}

/// The URL of the largest image, if any.
pub(crate) fn best_image(images: &[Image]) -> Option<String> {
    images
        .iter()
        .max_by_key(|i| i.width.unwrap_or(0))
        .map(|i| i.url.clone())
}

/// The Now Playing form of a track.
pub(crate) fn now_playing(track: &Track, rating: Option<u8>) -> NowPlaying {
    NowPlaying {
        track_id: track.id.clone(),
        name: track.name.clone(),
        artists: track
            .artists
            .iter()
            .map(|a| ArtistDto {
                id: a.id.clone(),
                name: a.name.clone(),
            })
            .collect(),
        album_id: track.album.id.clone(),
        album_name: track.album.name.clone(),
        image_url: best_image(&track.album.images),
        rating,
    }
}

/// The UI form of a rating row.
pub(crate) fn rated_track(row: RatedTrack) -> RatedTrackDto {
    RatedTrackDto {
        track_id: row.track_id,
        track_name: row.track_name,
        album_name: row.album_name,
        artist_id: row.artist_id,
        artist_name: row.artist_name,
        rating: row.rating.value(),
        rated_at: row.rated_at,
    }
}

/// The Spotify URI of a track ID.
pub(crate) fn track_uri(track_id: &str) -> String {
    format!("spotify:track:{track_id}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_image_picks_the_widest() {
        let img = |w: u32| Image {
            url: format!("u{w}"),
            width: Some(w),
            height: Some(w),
        };
        assert_eq!(
            best_image(&[img(64), img(640), img(300)]).as_deref(),
            Some("u640")
        );
        assert_eq!(best_image(&[]), None);
    }

    #[test]
    fn track_uri_has_spotify_prefix() {
        assert_eq!(track_uri("abc"), "spotify:track:abc");
    }

    #[test]
    fn track_info_and_now_playing_keep_artist_order() {
        let track = crate::fakes::track("t1", "al1", &["a1", "a2"]);
        let info = track_info(&track);
        assert_eq!(info.artists[0].id, "a1");
        assert_eq!(info.album_id.as_deref(), Some("al1"));
        let np = now_playing(&track, Some(7));
        assert_eq!((np.artists.len(), np.rating), (2, Some(7)));
    }
}
