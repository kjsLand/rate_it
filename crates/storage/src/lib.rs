//! SQLite storage for `rate_it`. See `docs/SPEC.md`, "Data".
//!
//! The rest of the code uses only the [`RatingsRepository`] trait. It never uses SQL directly.

mod model;
mod sqlite;
mod sqlite_export;

pub use model::{
    ArtistRef, ArtistSummary, ImportMode, ImportReport, NewRating, RatedTrack, Rating, RatingEntry,
    RatingSource, TrackInfo,
};
pub use sqlite::SqliteRepository;

use async_trait::async_trait;

/// Errors from the storage layer.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("migration error: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error("rating {0} is not between 1 and 10")]
    InvalidRating(i64),
    #[error("invalid data: {0}")]
    InvalidData(String),
}

pub type StorageResult<T> = Result<T, StorageError>;

/// All reads and writes of ratings, settings and the album cache.
#[async_trait]
pub trait RatingsRepository: Send + Sync + std::fmt::Debug {
    /// Adds a new rating row. Old rows stay, so the history stays.
    async fn add_rating(&self, new: &NewRating) -> StorageResult<RatedTrack>;

    /// The last rating of a track, if it has one.
    async fn current_rating(&self, track_id: &str) -> StorageResult<Option<RatedTrack>>;

    /// The last `limit` rating rows, newest first (the Now Playing log).
    async fn recent_ratings(&self, limit: u32) -> StorageResult<Vec<RatedTrack>>;

    /// One row for each main artist that has rated tracks.
    async fn artist_summaries(&self) -> StorageResult<Vec<ArtistSummary>>;

    /// The current ratings of the tracks of a main artist, highest rating first.
    async fn artist_ratings(&self, artist_id: &str) -> StorageResult<Vec<RatedTrack>>;

    /// All rating rows of one track, oldest first.
    async fn rating_history(&self, track_id: &str) -> StorageResult<Vec<RatingEntry>>;

    /// Writes many ratings in one transaction. See [`ImportMode`] for the skip rule.
    async fn import_ratings(
        &self,
        rows: &[NewRating],
        mode: ImportMode,
    ) -> StorageResult<ImportReport>;

    /// All rating rows (not only the current ones), with track and artist data.
    async fn export_ratings(&self) -> StorageResult<Vec<NewRating>>;

    /// Reads one value from `app_settings`.
    async fn setting(&self, key: &str) -> StorageResult<Option<String>>;

    /// Writes one value to `app_settings`.
    async fn set_setting(&self, key: &str, value: &str) -> StorageResult<()>;

    /// Removes one value from `app_settings`.
    async fn delete_setting(&self, key: &str) -> StorageResult<()>;

    /// Reads the cached JSON body of an album.
    async fn cached_album(&self, album_id: &str) -> StorageResult<Option<String>>;

    /// Writes the JSON body of an album to the cache.
    async fn cache_album(&self, album_id: &str, body: &str) -> StorageResult<()>;
}
