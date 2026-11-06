//! Plain data types of the storage layer.

use serde::{Deserialize, Serialize};

use crate::StorageError;

/// A rating from 1 to 10. The constructor refuses other values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "i64", into = "i64")]
pub struct Rating(u8);

impl Rating {
    /// Makes a rating. Returns an error if `value` is not between 1 and 10.
    pub fn new(value: i64) -> Result<Self, StorageError> {
        match u8::try_from(value) {
            Ok(v @ 1..=10) => Ok(Self(v)),
            _ => Err(StorageError::InvalidRating(value)),
        }
    }

    /// The value, from 1 to 10.
    pub fn value(self) -> u8 {
        self.0
    }
}

impl TryFrom<i64> for Rating {
    type Error = StorageError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Rating> for i64 {
    fn from(r: Rating) -> Self {
        i64::from(r.0)
    }
}

/// Where a rating row came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RatingSource {
    App,
    Import,
}

impl RatingSource {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::App => "app",
            Self::Import => "import",
        }
    }

    pub(crate) fn parse(value: &str) -> Result<Self, StorageError> {
        match value {
            "app" => Ok(Self::App),
            "import" => Ok(Self::Import),
            other => Err(StorageError::InvalidData(format!(
                "unknown rating source {other}"
            ))),
        }
    }
}

/// An artist, by Spotify ID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtistRef {
    pub id: String,
    pub name: String,
}

/// A track and its artists. The first artist is the main artist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackInfo {
    pub id: String,
    pub name: String,
    pub album_id: Option<String>,
    pub album_name: Option<String>,
    pub artists: Vec<ArtistRef>,
}

/// A rating to write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewRating {
    pub track: TrackInfo,
    pub rating: Rating,
    pub source: RatingSource,
    /// The time of the rating (ISO 8601). `None` means "now".
    pub rated_at: Option<String>,
}

/// A rating row joined with its track and main artist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RatedTrack {
    pub rating_id: i64,
    pub track_id: String,
    pub track_name: String,
    pub album_id: Option<String>,
    pub album_name: Option<String>,
    pub artist_id: String,
    pub artist_name: String,
    pub rating: Rating,
    pub rated_at: String,
}

/// The ratings of one main artist.
#[derive(Debug, Clone, PartialEq)]
pub struct ArtistSummary {
    pub id: String,
    pub name: String,
    pub rated_count: u32,
    pub average: f64,
}

/// One row of the history of a track.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RatingEntry {
    pub id: i64,
    pub rating: Rating,
    pub rated_at: String,
    pub source: RatingSource,
}

/// Which rows an import skips.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportMode {
    /// Skip a track that has a rating already (the one-time `rate.txt` import).
    SkipRatedTracks,
    /// Skip a row with the same track and time as a row in the database (JSON import).
    SkipDuplicateRows,
}

/// The result of an import.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ImportReport {
    pub imported: u32,
    pub skipped: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating_accepts_one_to_ten_only() {
        assert!(Rating::new(0).is_err());
        assert!(Rating::new(11).is_err());
        assert!(Rating::new(-1).is_err());
        assert_eq!(Rating::new(1).unwrap().value(), 1);
        assert_eq!(Rating::new(10).unwrap().value(), 10);
        assert_eq!(i64::from(Rating::try_from(7).unwrap()), 7);
    }

    #[test]
    fn rating_source_round_trips() {
        for s in [RatingSource::App, RatingSource::Import] {
            assert_eq!(RatingSource::parse(s.as_str()).unwrap(), s);
        }
        assert!(RatingSource::parse("other").is_err());
    }

    #[test]
    fn rating_deserialize_checks_range() {
        assert!(serde_json::from_str::<Rating>("11").is_err());
        assert_eq!(serde_json::from_str::<Rating>("4").unwrap().value(), 4);
    }
}
