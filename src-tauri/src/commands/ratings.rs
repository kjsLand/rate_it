//! Now Playing and Library commands. Each one checks input and calls `core`.

use rate_it_core::dto::{ArtistSummaryDto, HistoryEntry, NowPlaying, RatedTrackDto};
use tauri::State;

use crate::error::CommandResult;
use crate::state::AppState;

/// The song that plays now, or `null` ("Nothing playing").
#[tauri::command]
#[specta::specta]
pub(crate) async fn now_playing(state: State<'_, AppState>) -> CommandResult<Option<NowPlaying>> {
    Ok(state.app.now_playing().await?)
}

/// Rates the song that plays now (1-10).
#[tauri::command]
#[specta::specta]
pub(crate) async fn rate_current(
    state: State<'_, AppState>,
    rating: u8,
) -> CommandResult<NowPlaying> {
    Ok(state.app.rate_current(rating).await?)
}

/// The last 10 ratings.
#[tauri::command]
#[specta::specta]
pub(crate) async fn recent_ratings(
    state: State<'_, AppState>,
) -> CommandResult<Vec<RatedTrackDto>> {
    Ok(state.app.recent_ratings().await?)
}

/// All artists with ratings.
#[tauri::command]
#[specta::specta]
pub(crate) async fn library_artists(
    state: State<'_, AppState>,
) -> CommandResult<Vec<ArtistSummaryDto>> {
    Ok(state.app.library_artists().await?)
}

/// The rated songs of one artist.
#[tauri::command]
#[specta::specta]
pub(crate) async fn library_tracks(
    state: State<'_, AppState>,
    artist_id: String,
) -> CommandResult<Vec<RatedTrackDto>> {
    Ok(state.app.library_tracks(&artist_id).await?)
}

/// Changes the rating of a song. The old rating stays in the history.
#[tauri::command]
#[specta::specta]
pub(crate) async fn rerate_track(
    state: State<'_, AppState>,
    track_id: String,
    rating: u8,
) -> CommandResult<RatedTrackDto> {
    Ok(state.app.rerate_track(&track_id, rating).await?)
}

/// All ratings of one song.
#[tauri::command]
#[specta::specta]
pub(crate) async fn track_history(
    state: State<'_, AppState>,
    track_id: String,
) -> CommandResult<Vec<HistoryEntry>> {
    Ok(state.app.track_history(&track_id).await?)
}
