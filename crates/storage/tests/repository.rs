//! Tests of the SQLite repository against an in-memory database.
#![expect(
    clippy::unwrap_used,
    reason = "test helpers: a panic is a test failure"
)]

use rate_it_storage::{
    ArtistRef, ImportMode, NewRating, Rating, RatingSource, RatingsRepository, SqliteRepository,
    TrackInfo,
};

fn track(id: &str, name: &str, artists: &[(&str, &str)]) -> TrackInfo {
    TrackInfo {
        id: id.into(),
        name: name.into(),
        album_id: Some(format!("album-{id}")),
        album_name: Some(format!("Album {id}")),
        artists: artists
            .iter()
            .map(|(i, n)| ArtistRef {
                id: (*i).into(),
                name: (*n).into(),
            })
            .collect(),
    }
}

fn rating(t: TrackInfo, value: i64, source: RatingSource) -> NewRating {
    NewRating {
        track: t,
        rating: Rating::new(value).unwrap(),
        source,
        rated_at: None,
    }
}

/// The five ratings of the current `rate.txt`: 4 artists, no artist IDs.
fn legacy_rows() -> Vec<NewRating> {
    let rows = [
        ("7bDLEI7aaqmbASyo6zAhfH", "I Remember", "24kGoldn", 9),
        ("3ZZ3WWjNIdg2jDUmbJf5o4", "I'm Here", "24kGoldn", 10),
        ("7q2euBzunDGE9082q4eRV0", "Human Nature", "Yot Club", 10),
        ("6RmWk7p9QY8VMGHdgq4Ulh", "Shotput", "Still Woozy", 8),
        ("2OZVskV28xxJjjhQqKTLSg", "PIXELATED KISSES", "Joji", 8),
    ];
    rows.iter()
        .map(|(id, name, artist, value)| {
            let artist_id = format!("legacy:{artist}");
            rating(
                track(id, name, &[(&artist_id, artist)]),
                *value,
                RatingSource::Import,
            )
        })
        .collect()
}

#[tokio::test]
async fn migrations_run_and_schema_versions_are_recorded() {
    let repo = SqliteRepository::in_memory().await.unwrap();
    assert_eq!(repo.recent_ratings(10).await.unwrap().len(), 0);
    assert_eq!(repo.artist_summaries().await.unwrap().len(), 0);
}

#[tokio::test]
async fn rerating_keeps_history_and_current_view_shows_last() {
    let repo = SqliteRepository::in_memory().await.unwrap();
    let t = track("t1", "Song", &[("a1", "Artist"), ("a2", "Feature")]);
    repo.add_rating(&rating(t.clone(), 6, RatingSource::App))
        .await
        .unwrap();
    let second = repo
        .add_rating(&rating(t, 9, RatingSource::App))
        .await
        .unwrap();
    assert_eq!(second.rating.value(), 9);
    assert_eq!(second.artist_id, "a1");

    let current = repo.current_rating("t1").await.unwrap().unwrap();
    assert_eq!(current.rating.value(), 9);
    let history = repo.rating_history("t1").await.unwrap();
    assert_eq!(
        history.iter().map(|h| h.rating.value()).collect::<Vec<_>>(),
        vec![6, 9]
    );

    let summaries = repo.artist_summaries().await.unwrap();
    assert_eq!(summaries.len(), 1, "only the main artist is listed");
    assert_eq!(
        summaries[0].rated_count, 1,
        "the view counts the last rating only"
    );
    assert!((summaries[0].average - 9.0).abs() < f64::EPSILON);
    assert_eq!(
        repo.recent_ratings(10).await.unwrap().len(),
        2,
        "the log shows all rows"
    );
}

#[tokio::test]
async fn artist_ratings_are_sorted_by_rating() {
    let repo = SqliteRepository::in_memory().await.unwrap();
    for (id, value) in [("t1", 5), ("t2", 10), ("t3", 7)] {
        let t = track(id, id, &[("a1", "Artist")]);
        repo.add_rating(&rating(t, value, RatingSource::App))
            .await
            .unwrap();
    }
    let rows = repo.artist_ratings("a1").await.unwrap();
    let values: Vec<u8> = rows.iter().map(|r| r.rating.value()).collect();
    assert_eq!(values, vec![10, 7, 5]);
    assert_eq!(repo.artist_ratings("other").await.unwrap().len(), 0);
}

#[tokio::test]
async fn recent_ratings_are_newest_first_and_limited() {
    let repo = SqliteRepository::in_memory().await.unwrap();
    for i in 0..12 {
        let t = track(&format!("t{i}"), "Song", &[("a1", "Artist")]);
        repo.add_rating(&rating(t, 5, RatingSource::App))
            .await
            .unwrap();
    }
    let log = repo.recent_ratings(10).await.unwrap();
    assert_eq!(log.len(), 10);
    assert_eq!(log[0].track_id, "t11");
}

#[tokio::test]
async fn legacy_import_gives_five_ratings_for_four_artists_and_is_idempotent() {
    let repo = SqliteRepository::in_memory().await.unwrap();
    let first = repo
        .import_ratings(&legacy_rows(), ImportMode::SkipRatedTracks)
        .await
        .unwrap();
    assert_eq!((first.imported, first.skipped), (5, 0));
    let summaries = repo.artist_summaries().await.unwrap();
    assert_eq!(summaries.len(), 4);
    let total: u32 = summaries.iter().map(|s| s.rated_count).sum();
    assert_eq!(total, 5);

    let second = repo
        .import_ratings(&legacy_rows(), ImportMode::SkipRatedTracks)
        .await
        .unwrap();
    assert_eq!((second.imported, second.skipped), (0, 5));
    assert_eq!(repo.export_ratings().await.unwrap().len(), 5);
}

#[tokio::test]
async fn failed_import_writes_nothing() {
    let repo = SqliteRepository::in_memory().await.unwrap();
    let mut rows = legacy_rows();
    rows[3].track.artists.clear();
    assert!(
        repo.import_ratings(&rows, ImportMode::SkipRatedTracks)
            .await
            .is_err()
    );
    assert!(
        repo.export_ratings().await.unwrap().is_empty(),
        "the transaction rolled back"
    );
}

#[tokio::test]
async fn export_then_import_moves_all_rows_once() {
    let source = SqliteRepository::in_memory().await.unwrap();
    let t = track("t1", "Song", &[("a1", "Artist"), ("a2", "Feature")]);
    source
        .add_rating(&rating(t.clone(), 4, RatingSource::App))
        .await
        .unwrap();
    source
        .add_rating(&rating(t, 8, RatingSource::App))
        .await
        .unwrap();
    let exported = source.export_ratings().await.unwrap();
    assert_eq!(exported.len(), 2);
    assert_eq!(exported[0].track.artists.len(), 2);

    let target = SqliteRepository::in_memory().await.unwrap();
    let report = target
        .import_ratings(&exported, ImportMode::SkipDuplicateRows)
        .await
        .unwrap();
    assert_eq!(report.imported, 2);
    let again = target
        .import_ratings(&exported, ImportMode::SkipDuplicateRows)
        .await
        .unwrap();
    assert_eq!((again.imported, again.skipped), (0, 2));
    assert_eq!(
        target
            .current_rating("t1")
            .await
            .unwrap()
            .unwrap()
            .rating
            .value(),
        8
    );
}

/// Two ratings of one song in the same millisecond are two rows, not a duplicate.
/// (This made `export_then_import_moves_all_rows_once` fail now and then.)
#[tokio::test]
async fn import_keeps_two_ratings_with_the_same_time() {
    let at = "2026-11-05T10:00:00.000Z";
    let rows: Vec<NewRating> = [4, 8]
        .into_iter()
        .map(|value| NewRating {
            rated_at: Some(at.into()),
            ..rating(
                track("t1", "Song", &[("a1", "Artist")]),
                value,
                RatingSource::App,
            )
        })
        .collect();

    let target = SqliteRepository::in_memory().await.unwrap();
    let report = target
        .import_ratings(&rows, ImportMode::SkipDuplicateRows)
        .await
        .unwrap();
    assert_eq!((report.imported, report.skipped), (2, 0));
    let again = target
        .import_ratings(&rows, ImportMode::SkipDuplicateRows)
        .await
        .unwrap();
    assert_eq!((again.imported, again.skipped), (0, 2));
    assert_eq!(
        target
            .current_rating("t1")
            .await
            .unwrap()
            .unwrap()
            .rating
            .value(),
        8
    );
}

#[tokio::test]
async fn settings_and_album_cache_round_trip() {
    let repo = SqliteRepository::in_memory().await.unwrap();
    assert_eq!(repo.setting("client_id").await.unwrap(), None);
    repo.set_setting("client_id", "abc").await.unwrap();
    repo.set_setting("client_id", "def").await.unwrap();
    assert_eq!(
        repo.setting("client_id").await.unwrap().as_deref(),
        Some("def")
    );
    repo.delete_setting("client_id").await.unwrap();
    assert_eq!(repo.setting("client_id").await.unwrap(), None);

    assert_eq!(repo.cached_album("al1").await.unwrap(), None);
    repo.cache_album("al1", "[1]").await.unwrap();
    repo.cache_album("al1", "[2]").await.unwrap();
    assert_eq!(
        repo.cached_album("al1").await.unwrap().as_deref(),
        Some("[2]")
    );
}

#[tokio::test]
async fn open_creates_file_database_with_wal() {
    let dir = std::env::temp_dir().join(format!("rate-it-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("ratings.db");
    let repo = SqliteRepository::open(&path).await.unwrap();
    let t = track("t1", "Song", &[("a1", "Artist")]);
    repo.add_rating(&rating(t, 3, RatingSource::App))
        .await
        .unwrap();
    drop(repo);
    let reopened = SqliteRepository::open(&path).await.unwrap();
    assert_eq!(
        reopened
            .current_rating("t1")
            .await
            .unwrap()
            .unwrap()
            .rating
            .value(),
        3
    );
    std::fs::remove_dir_all(&dir).unwrap();
}

#[tokio::test]
async fn rerate_adds_row_for_known_track_and_keeps_features() {
    let repo = SqliteRepository::in_memory().await.unwrap();
    assert_eq!(
        repo.rerate("t1", Rating::new(5).unwrap()).await.unwrap(),
        None
    );
    let t = track("t1", "Song", &[("a1", "Artist"), ("a2", "Feature")]);
    repo.add_rating(&rating(t, 3, RatingSource::App))
        .await
        .unwrap();
    let after = repo
        .rerate("t1", Rating::new(8).unwrap())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(after.rating.value(), 8);
    assert_eq!(repo.rating_history("t1").await.unwrap().len(), 2);
    let exported = repo.export_ratings().await.unwrap();
    assert_eq!(
        exported[1].track.artists.len(),
        2,
        "the feature artist stays linked"
    );
}
