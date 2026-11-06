//! Parse of the legacy `rate.txt` file of the Python app.
//!
//! The file is one JSON object: `{ artist name: { track ID: { "rating": n, "name": s } } }`.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::error::{CoreError, CoreResult};

/// One rating of `rate.txt`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyEntry {
    pub artist_name: String,
    pub track_id: String,
    pub track_name: String,
    /// The rating as written in the file. It can be out of range.
    pub rating: i64,
}

/// The Python app wrote numbers, but text such as `"9"` is accepted too.
#[derive(Deserialize)]
#[serde(untagged)]
enum LegacyRating {
    Number(i64),
    Text(String),
}

#[derive(Deserialize)]
struct LegacyTrack {
    rating: LegacyRating,
    name: String,
}

/// Parses the text of `rate.txt`. An empty file gives no entries.
pub fn parse_legacy(text: &str) -> CoreResult<Vec<LegacyEntry>> {
    let first_line = text.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
    if first_line.trim().is_empty() {
        return Ok(Vec::new());
    }
    let file: BTreeMap<String, BTreeMap<String, LegacyTrack>> = serde_json::from_str(first_line)
        .map_err(|e| CoreError::Invalid(format!("rate.txt is not valid: {e}")))?;
    let mut entries = Vec::new();
    for (artist_name, tracks) in file {
        for (track_id, track) in tracks {
            let rating = match track.rating {
                LegacyRating::Number(n) => n,
                LegacyRating::Text(t) => t.trim().parse().map_err(|_| {
                    CoreError::Invalid(format!(
                        "rate.txt: rating {t:?} of {track_id} is not a number"
                    ))
                })?,
            };
            entries.push(LegacyEntry {
                artist_name: artist_name.clone(),
                track_id,
                track_name: track.name,
                rating,
            });
        }
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CURRENT_FILE: &str = r#"{"24kGoldn": {"7bDLEI7aaqmbASyo6zAhfH": {"rating": 9, "name": "I Remember"}, "3ZZ3WWjNIdg2jDUmbJf5o4": {"rating": 10, "name": "I'm Here"}}, "Yot Club": {"7q2euBzunDGE9082q4eRV0": {"rating": 10, "name": "Human Nature"}}, "Still Woozy": {"6RmWk7p9QY8VMGHdgq4Ulh": {"rating": 8, "name": "Shotput"}}, "Joji": {"2OZVskV28xxJjjhQqKTLSg": {"rating": 8, "name": "PIXELATED KISSES"}}}"#;

    #[test]
    fn parses_current_file_into_five_entries_of_four_artists() {
        let entries = parse_legacy(CURRENT_FILE).unwrap();
        assert_eq!(entries.len(), 5);
        let mut artists: Vec<&str> = entries.iter().map(|e| e.artist_name.as_str()).collect();
        artists.dedup();
        assert_eq!(artists.len(), 4);
    }

    #[test]
    fn accepts_text_ratings_and_empty_file() {
        let entries = parse_legacy(r#"{"A": {"t1": {"rating": "7", "name": "S"}}}"#).unwrap();
        assert_eq!(entries[0].rating, 7);
        assert_eq!(parse_legacy("\n").unwrap(), Vec::new());
    }

    #[test]
    fn refuses_bad_files() {
        assert!(parse_legacy("not json").is_err());
        assert!(parse_legacy(r#"{"A": {"t1": {"rating": "x", "name": "S"}}}"#).is_err());
        assert!(parse_legacy(r#"{"A": {"t1": {"name": "S"}}}"#).is_err());
    }
}
