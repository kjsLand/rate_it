//! The `sqlx` implementation of [`RatingsRepository`].

use std::path::Path;
use std::str::FromStr;

use async_trait::async_trait;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions};
use sqlx::{Sqlite, Transaction};

use crate::model::{
    ArtistSummary, ImportMode, ImportReport, NewRating, RatedTrack, Rating, RatingEntry,
    RatingSource, TrackInfo,
};
use crate::{RatingsRepository, StorageError, StorageResult};

/// The SQLite repository. Clone it freely: the pool is shared.
#[derive(Debug, Clone)]
pub struct SqliteRepository {
    pool: SqlitePool,
}

impl SqliteRepository {
    /// Opens (or makes) the database file at `path` and runs all migrations.
    pub async fn open(path: &Path) -> StorageResult<Self> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .foreign_keys(true)
            .journal_mode(SqliteJournalMode::Wal);
        let pool = SqlitePoolOptions::new()
            .max_connections(4)
            .connect_with(options)
            .await?;
        Self::migrate(pool).await
    }

    /// Opens a new in-memory database (for tests) and runs all migrations.
    pub async fn in_memory() -> StorageResult<Self> {
        let options = SqliteConnectOptions::from_str("sqlite::memory:")?.foreign_keys(true);
        // One connection: each in-memory connection is a different database.
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;
        Self::migrate(pool).await
    }

    async fn migrate(pool: SqlitePool) -> StorageResult<Self> {
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }
}

/// Inserts or updates the track and its artists. The old artist links are replaced.
async fn upsert_track(tx: &mut Transaction<'_, Sqlite>, track: &TrackInfo) -> StorageResult<()> {
    if track.artists.is_empty() {
        return Err(StorageError::InvalidData(format!(
            "track {} has no artists",
            track.id
        )));
    }
    sqlx::query!(
        "INSERT INTO tracks (id, name, album_id, album_name) VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(id) DO UPDATE SET name = excluded.name,
           album_id = COALESCE(excluded.album_id, tracks.album_id),
           album_name = COALESCE(excluded.album_name, tracks.album_name)",
        track.id,
        track.name,
        track.album_id,
        track.album_name,
    )
    .execute(&mut **tx)
    .await?;
    sqlx::query!("DELETE FROM track_artists WHERE track_id = ?1", track.id)
        .execute(&mut **tx)
        .await?;
    for (position, artist) in (0_i64..).zip(&track.artists) {
        sqlx::query!(
            "INSERT INTO artists (id, name) VALUES (?1, ?2)
             ON CONFLICT(id) DO UPDATE SET name = excluded.name",
            artist.id,
            artist.name,
        )
        .execute(&mut **tx)
        .await?;
        sqlx::query!(
            "INSERT OR IGNORE INTO track_artists (track_id, artist_id, position) VALUES (?1, ?2, ?3)",
            track.id,
            artist.id,
            position,
        )
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

/// Inserts one rating row and returns its ID.
async fn insert_rating(tx: &mut Transaction<'_, Sqlite>, new: &NewRating) -> StorageResult<i64> {
    upsert_track(tx, &new.track).await?;
    let rating = i64::from(new.rating);
    let source = new.source.as_str();
    let id = match &new.rated_at {
        Some(at) => {
            check_timestamp(at)?;
            sqlx::query!(
                "INSERT INTO ratings (track_id, rating, rated_at, source) VALUES (?1, ?2, ?3, ?4)",
                new.track.id,
                rating,
                at,
                source,
            )
            .execute(&mut **tx)
            .await?
            .last_insert_rowid()
        }
        None => sqlx::query!(
            "INSERT INTO ratings (track_id, rating, source) VALUES (?1, ?2, ?3)",
            new.track.id,
            rating,
            source,
        )
        .execute(&mut **tx)
        .await?
        .last_insert_rowid(),
    };
    Ok(id)
}

/// Refuses a time that is not a short ISO 8601 text such as `2026-01-02T03:04:05.000Z`.
fn check_timestamp(at: &str) -> StorageResult<()> {
    let ok = (10..=40).contains(&at.len())
        && at
            .chars()
            .all(|c| c.is_ascii_digit() || "-:T.Z+".contains(c));
    if ok {
        Ok(())
    } else {
        Err(StorageError::InvalidData(format!("bad time {at}")))
    }
}

/// Tells if an import must skip `row`.
/// A duplicate row has the same track, time and rating: two ratings of one song can share a
/// millisecond, so the time alone does not identify a row.
async fn must_skip(
    tx: &mut Transaction<'_, Sqlite>,
    row: &NewRating,
    mode: ImportMode,
) -> StorageResult<bool> {
    let rating = i64::from(row.rating);
    let count = match (mode, &row.rated_at) {
        (ImportMode::SkipRatedTracks, _) | (ImportMode::SkipDuplicateRows, None) => {
            sqlx::query_scalar!(
                r#"SELECT COUNT(*) AS "n!: i64" FROM ratings WHERE track_id = ?1"#,
                row.track.id
            )
            .fetch_one(&mut **tx)
            .await?
        }
        (ImportMode::SkipDuplicateRows, Some(at)) => {
            sqlx::query_scalar!(
                r#"SELECT COUNT(*) AS "n!: i64" FROM ratings
               WHERE track_id = ?1 AND rated_at = ?2 AND rating = ?3"#,
                row.track.id,
                at,
                rating
            )
            .fetch_one(&mut **tx)
            .await?
        }
    };
    Ok(count > 0)
}

/// Builds a [`RatedTrack`] from query columns.
#[expect(
    clippy::too_many_arguments,
    reason = "one argument for each column of the row"
)]
fn rated_track(
    rating_id: i64,
    track_id: String,
    track_name: String,
    album_id: Option<String>,
    album_name: Option<String>,
    artist_id: String,
    artist_name: String,
    rating: i64,
    rated_at: String,
) -> StorageResult<RatedTrack> {
    Ok(RatedTrack {
        rating_id,
        track_id,
        track_name,
        album_id,
        album_name,
        artist_id,
        artist_name,
        rating: Rating::new(rating)?,
        rated_at,
    })
}

macro_rules! to_rated_track {
    ($r:expr) => {
        rated_track(
            $r.rating_id,
            $r.track_id,
            $r.track_name,
            $r.album_id,
            $r.album_name,
            $r.artist_id,
            $r.artist_name,
            $r.rating,
            $r.rated_at,
        )
    };
}

#[async_trait]
impl RatingsRepository for SqliteRepository {
    async fn add_rating(&self, new: &NewRating) -> StorageResult<RatedTrack> {
        let mut tx = self.pool.begin().await?;
        let id = insert_rating(&mut tx, new).await?;
        tx.commit().await?;
        let row = sqlx::query!(
            r#"SELECT r.id AS "rating_id!", t.id AS "track_id!", t.name AS "track_name!",
                 t.album_id AS "album_id?", t.album_name AS "album_name?",
                 a.id AS "artist_id!", a.name AS "artist_name!",
                 r.rating AS "rating!", r.rated_at AS "rated_at!"
               FROM ratings r JOIN tracks t ON t.id = r.track_id
               JOIN track_artists ta ON ta.track_id = t.id AND ta.position = 0
               JOIN artists a ON a.id = ta.artist_id
               WHERE r.id = ?1"#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        to_rated_track!(row)
    }

    async fn rerate(&self, track_id: &str, rating: Rating) -> StorageResult<Option<RatedTrack>> {
        let value = i64::from(rating);
        let inserted = sqlx::query!(
            "INSERT INTO ratings (track_id, rating, source)
             SELECT id, ?2, 'app' FROM tracks WHERE id = ?1",
            track_id,
            value
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        if inserted == 0 {
            return Ok(None);
        }
        self.current_rating(track_id).await
    }

    async fn current_rating(&self, track_id: &str) -> StorageResult<Option<RatedTrack>> {
        let row = sqlx::query!(
            r#"SELECT r.id AS "rating_id!", t.id AS "track_id!", t.name AS "track_name!",
                 t.album_id AS "album_id?", t.album_name AS "album_name?",
                 a.id AS "artist_id!", a.name AS "artist_name!",
                 r.rating AS "rating!", r.rated_at AS "rated_at!"
               FROM ratings r JOIN tracks t ON t.id = r.track_id
               JOIN track_artists ta ON ta.track_id = t.id AND ta.position = 0
               JOIN artists a ON a.id = ta.artist_id
               WHERE r.track_id = ?1
               ORDER BY r.id DESC LIMIT 1"#,
            track_id
        )
        .fetch_optional(&self.pool)
        .await?;
        row.map(|r| to_rated_track!(r)).transpose()
    }

    async fn recent_ratings(&self, limit: u32) -> StorageResult<Vec<RatedTrack>> {
        let limit = i64::from(limit);
        let rows = sqlx::query!(
            r#"SELECT r.id AS "rating_id!", t.id AS "track_id!", t.name AS "track_name!",
                 t.album_id AS "album_id?", t.album_name AS "album_name?",
                 a.id AS "artist_id!", a.name AS "artist_name!",
                 r.rating AS "rating!", r.rated_at AS "rated_at!"
               FROM ratings r JOIN tracks t ON t.id = r.track_id
               JOIN track_artists ta ON ta.track_id = t.id AND ta.position = 0
               JOIN artists a ON a.id = ta.artist_id
               ORDER BY r.id DESC LIMIT ?1"#,
            limit
        )
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter().map(|r| to_rated_track!(r)).collect()
    }

    async fn artist_summaries(&self) -> StorageResult<Vec<ArtistSummary>> {
        let rows = sqlx::query!(
            r#"SELECT a.id AS "id!", a.name AS "name!",
                 COUNT(*) AS "rated_count!: i64", AVG(c.rating) AS "average!: f64"
               FROM current_ratings c
               JOIN track_artists ta ON ta.track_id = c.track_id AND ta.position = 0
               JOIN artists a ON a.id = ta.artist_id
               GROUP BY a.id, a.name
               ORDER BY a.name COLLATE NOCASE"#
        )
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|r| {
                let rated_count = u32::try_from(r.rated_count)
                    .map_err(|_| StorageError::InvalidData("count out of range".into()))?;
                Ok(ArtistSummary {
                    id: r.id,
                    name: r.name,
                    rated_count,
                    average: r.average,
                })
            })
            .collect()
    }

    async fn artist_ratings(&self, artist_id: &str) -> StorageResult<Vec<RatedTrack>> {
        let rows = sqlx::query!(
            r#"SELECT c.id AS "rating_id!", t.id AS "track_id!", t.name AS "track_name!",
                 t.album_id AS "album_id?", t.album_name AS "album_name?",
                 a.id AS "artist_id!", a.name AS "artist_name!",
                 c.rating AS "rating!", c.rated_at AS "rated_at!"
               FROM current_ratings c JOIN tracks t ON t.id = c.track_id
               JOIN track_artists ta ON ta.track_id = t.id AND ta.position = 0
               JOIN artists a ON a.id = ta.artist_id
               WHERE a.id = ?1
               ORDER BY c.rating DESC, t.name COLLATE NOCASE"#,
            artist_id
        )
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter().map(|r| to_rated_track!(r)).collect()
    }

    async fn rating_history(&self, track_id: &str) -> StorageResult<Vec<RatingEntry>> {
        let rows = sqlx::query!(
            r#"SELECT id AS "id!", rating AS "rating!", rated_at AS "rated_at!", source AS "source!"
               FROM ratings WHERE track_id = ?1 ORDER BY id"#,
            track_id
        )
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|r| {
                Ok(RatingEntry {
                    id: r.id,
                    rating: Rating::new(r.rating)?,
                    rated_at: r.rated_at,
                    source: RatingSource::parse(&r.source)?,
                })
            })
            .collect()
    }

    async fn import_ratings(
        &self,
        rows: &[NewRating],
        mode: ImportMode,
    ) -> StorageResult<ImportReport> {
        let mut tx = self.pool.begin().await?;
        let mut report = ImportReport::default();
        for row in rows {
            if must_skip(&mut tx, row, mode).await? {
                report.skipped += 1;
            } else {
                insert_rating(&mut tx, row).await?;
                report.imported += 1;
            }
        }
        tx.commit().await?;
        Ok(report)
    }

    async fn export_ratings(&self) -> StorageResult<Vec<NewRating>> {
        crate::sqlite_export::export(&self.pool).await
    }

    async fn setting(&self, key: &str) -> StorageResult<Option<String>> {
        Ok(
            sqlx::query_scalar!("SELECT value FROM app_settings WHERE key = ?1", key)
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    async fn set_setting(&self, key: &str, value: &str) -> StorageResult<()> {
        sqlx::query!(
            "INSERT INTO app_settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            key,
            value
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_setting(&self, key: &str) -> StorageResult<()> {
        sqlx::query!("DELETE FROM app_settings WHERE key = ?1", key)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn cached_album(&self, album_id: &str) -> StorageResult<Option<String>> {
        Ok(
            sqlx::query_scalar!("SELECT body FROM album_cache WHERE album_id = ?1", album_id)
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    async fn cache_album(&self, album_id: &str, body: &str) -> StorageResult<()> {
        sqlx::query!(
            "INSERT INTO album_cache (album_id, body) VALUES (?1, ?2)
             ON CONFLICT(album_id) DO UPDATE SET body = excluded.body,
               fetched_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')",
            album_id,
            body
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::check_timestamp;

    #[test]
    fn check_timestamp_accepts_iso_only() {
        assert!(check_timestamp("2026-11-05T10:00:00.000Z").is_ok());
        assert!(check_timestamp("2026-11-05").is_ok());
        assert!(check_timestamp("yesterday").is_err());
        assert!(check_timestamp("2026-11-05'; DROP TABLE x").is_err());
    }
}
