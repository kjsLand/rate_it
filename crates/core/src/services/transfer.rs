//! Import of `rate.txt`, and export and import of the JSON file between devices.

use rate_it_storage::{ArtistRef, ImportMode, NewRating, Rating, RatingSource, TrackInfo};
use serde::{Deserialize, Serialize};

use crate::App;
use crate::convert::track_info;
use crate::dto::ImportReportDto;
use crate::error::{CoreError, CoreResult};
use crate::legacy::{LegacyEntry, parse_legacy};
use crate::services::spotify_id;

/// The marker of an export file.
const EXPORT_FORMAT: &str = "rate_it-export";
const EXPORT_VERSION: u32 = 1;

/// The JSON export file: all rows of the `ratings` table, with track and artist data.
#[derive(Debug, Serialize, Deserialize)]
struct ExportFile {
    format: String,
    version: u32,
    ratings: Vec<NewRating>,
}

impl App {
    /// Imports `rate.txt` one time. A parse failure changes nothing.
    /// A track that Spotify does not know gets the artist ID `legacy:<artist name>`.
    /// A track that has a rating already is skipped, so a second import adds nothing.
    pub async fn import_legacy(&self, text: &str) -> CoreResult<ImportReportDto> {
        let entries = parse_legacy(text)?;
        let mut report = ImportReportDto::default();
        let mut rows = Vec::with_capacity(entries.len());
        for entry in entries {
            match self.legacy_row(&entry).await {
                Ok(Some(row)) => rows.push(row),
                Ok(None) => report.failed += 1,
                Err(e) => return Err(e),
            }
        }
        let result = self
            .repo
            .import_ratings(&rows, ImportMode::SkipRatedTracks)
            .await?;
        report.imported = result.imported;
        report.skipped = result.skipped;
        Ok(report)
    }

    /// One `rate.txt` entry as a row. `None` if the entry cannot be imported.
    async fn legacy_row(&self, entry: &LegacyEntry) -> CoreResult<Option<NewRating>> {
        let (Ok(rating), Ok(id)) = (Rating::new(entry.rating), spotify_id(&entry.track_id)) else {
            return Ok(None);
        };
        if self.repo.current_rating(id).await?.is_some() {
            // Skipped by the import: do not ask Spotify.
            return Ok(Some(legacy_fallback(entry, rating)));
        }
        let track = match self.spotify.track(id).await {
            Ok(Some(track)) => track_info(&track),
            Ok(None) => legacy_fallback(entry, rating).track,
            Err(rate_it_spotify::SpotifyError::NotSignedIn) => {
                return Err(CoreError::Spotify(
                    rate_it_spotify::SpotifyError::NotSignedIn,
                ));
            }
            Err(_) => return Ok(None),
        };
        Ok(Some(NewRating {
            track,
            rating,
            source: RatingSource::Import,
            rated_at: None,
        }))
    }

    /// The JSON text of all rating rows.
    pub async fn export_json(&self) -> CoreResult<String> {
        let file = ExportFile {
            format: EXPORT_FORMAT.into(),
            version: EXPORT_VERSION,
            ratings: self.repo.export_ratings().await?,
        };
        serde_json::to_string_pretty(&file).map_err(|e| CoreError::Invalid(e.to_string()))
    }

    /// Imports an export file. A row with the same track and time as a stored row is skipped.
    pub async fn import_json(&self, text: &str) -> CoreResult<ImportReportDto> {
        let file: ExportFile = serde_json::from_str(text)
            .map_err(|e| CoreError::Invalid(format!("The file is not a rate_it export: {e}")))?;
        if file.format != EXPORT_FORMAT || file.version != EXPORT_VERSION {
            return Err(CoreError::Invalid(
                "The file is not a rate_it export (version 1)".into(),
            ));
        }
        let mut failed = 0;
        let rows: Vec<NewRating> = file
            .ratings
            .into_iter()
            .filter(|r| {
                let ok = spotify_id(&r.track.id).is_ok() && !r.track.artists.is_empty();
                failed += u32::from(!ok);
                ok
            })
            .map(|r| NewRating {
                source: RatingSource::Import,
                ..r
            })
            .collect();
        let result = self
            .repo
            .import_ratings(&rows, ImportMode::SkipDuplicateRows)
            .await?;
        Ok(ImportReportDto {
            imported: result.imported,
            skipped: result.skipped,
            failed,
        })
    }
}

fn legacy_fallback(entry: &LegacyEntry, rating: Rating) -> NewRating {
    NewRating {
        track: TrackInfo {
            id: entry.track_id.clone(),
            name: entry.track_name.clone(),
            album_id: None,
            album_name: None,
            artists: vec![ArtistRef {
                id: format!("legacy:{}", entry.artist_name),
                name: entry.artist_name.clone(),
            }],
        },
        rating,
        source: RatingSource::Import,
        rated_at: None,
    }
}

#[cfg(test)]
mod tests {
    use crate::ErrorKind;
    use crate::fakes::{app, track};

    const FILE: &str = r#"{"24kGoldn": {"7bDLEI7aaqmbASyo6zAhfH": {"rating": 9, "name": "I Remember"}, "3ZZ3WWjNIdg2jDUmbJf5o4": {"rating": 10, "name": "I'm Here"}}, "Yot Club": {"7q2euBzunDGE9082q4eRV0": {"rating": 10, "name": "Human Nature"}}, "Still Woozy": {"6RmWk7p9QY8VMGHdgq4Ulh": {"rating": 8, "name": "Shotput"}}, "Joji": {"2OZVskV28xxJjjhQqKTLSg": {"rating": 8, "name": "PIXELATED KISSES"}}}"#;

    #[tokio::test]
    async fn legacy_import_gives_five_ratings_then_nothing() {
        let (app, spotify) = app().await;
        // Spotify knows one track; the others fall back to legacy artist IDs.
        let known = track("2OZVskV28xxJjjhQqKTLSg", "al1", &["jojiId"]);
        spotify
            .tracks
            .lock()
            .unwrap()
            .insert(known.id.clone(), known);
        let first = app.import_legacy(FILE).await.unwrap();
        assert_eq!((first.imported, first.skipped, first.failed), (5, 0, 0));
        let artists = app.library_artists().await.unwrap();
        assert_eq!(artists.len(), 4);
        assert!(artists.iter().any(|a| a.id == "jojiId"));
        assert!(artists.iter().any(|a| a.id == "legacy:Yot Club"));
        let second = app.import_legacy(FILE).await.unwrap();
        assert_eq!((second.imported, second.skipped), (0, 5));
    }

    #[tokio::test]
    async fn legacy_import_counts_failures_and_refuses_bad_file() {
        let (app, spotify) = app().await;
        spotify.failing_tracks.lock().unwrap().insert("t2".into());
        let text = r#"{"A": {"t1": {"rating": 11, "name": "S"}, "t2": {"rating": 5, "name": "T"}, "t3": {"rating": 5, "name": "U"}}}"#;
        let report = app.import_legacy(text).await.unwrap();
        assert_eq!((report.imported, report.failed), (1, 2));
        assert_eq!(
            app.import_legacy("{bad").await.unwrap_err().kind(),
            ErrorKind::Invalid
        );
    }

    #[tokio::test]
    async fn export_then_import_moves_ratings() {
        let (source, spotify) = app().await;
        spotify.play(track("t1", "al1", &["a1", "b1"]));
        source.rate_current(5).await.unwrap();
        source.rate_current(9).await.unwrap();
        let text = source.export_json().await.unwrap();

        let (target, _) = app().await;
        let report = target.import_json(&text).await.unwrap();
        assert_eq!((report.imported, report.skipped), (2, 0));
        assert_eq!(target.import_json(&text).await.unwrap().skipped, 2);
        assert_eq!(target.library_tracks("a1").await.unwrap()[0].rating, 9);
        assert_eq!(
            target.import_json("{}").await.unwrap_err().kind(),
            ErrorKind::Invalid
        );
    }
}
