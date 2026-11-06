//! Export of all rating rows, for the JSON file that moves ratings between devices.

use std::collections::HashMap;

use sqlx::SqlitePool;

use crate::StorageResult;
use crate::model::{ArtistRef, NewRating, Rating, RatingSource, TrackInfo};

/// Reads all rating rows, oldest first, each with its track and its artists in order.
pub(crate) async fn export(pool: &SqlitePool) -> StorageResult<Vec<NewRating>> {
    let links = sqlx::query!(
        r#"SELECT ta.track_id AS "track_id!", a.id AS "id!", a.name AS "name!"
           FROM track_artists ta JOIN artists a ON a.id = ta.artist_id
           ORDER BY ta.track_id, ta.position"#
    )
    .fetch_all(pool)
    .await?;
    let mut artists: HashMap<String, Vec<ArtistRef>> = HashMap::new();
    for link in links {
        artists.entry(link.track_id).or_default().push(ArtistRef {
            id: link.id,
            name: link.name,
        });
    }

    let rows = sqlx::query!(
        r#"SELECT t.id AS "track_id!", t.name AS "track_name!",
             t.album_id AS "album_id?", t.album_name AS "album_name?",
             r.rating AS "rating!", r.rated_at AS "rated_at!", r.source AS "source!"
           FROM ratings r JOIN tracks t ON t.id = r.track_id
           ORDER BY r.id"#
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|r| {
            Ok(NewRating {
                track: TrackInfo {
                    artists: artists.get(&r.track_id).cloned().unwrap_or_default(),
                    id: r.track_id,
                    name: r.track_name,
                    album_id: r.album_id,
                    album_name: r.album_name,
                },
                rating: Rating::new(r.rating)?,
                source: RatingSource::parse(&r.source)?,
                rated_at: Some(r.rated_at),
            })
        })
        .collect()
}
