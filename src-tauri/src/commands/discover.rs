//! Artist, Build Playlist, Collaborators and Discover commands.

use rate_it_core::dto::{
    ArtistDetails, Collaborator, DiscoverMode, PlaylistRequest, PlaylistResult, Progress,
    QueueResult,
};
use tauri::State;
use tauri::ipc::Channel;

use crate::error::{CommandError, CommandResult};
use crate::progress::ChannelProgress;
use crate::state::AppState;

/// The names of the tasks that `cancel_task` accepts.
const TASKS: [&str; 2] = ["collaborators", "discover"];

/// The Artist screen of the current artist, or of `artist_id`.
#[tauri::command]
#[specta::specta]
pub(crate) async fn artist_details(
    state: State<'_, AppState>,
    artist_id: Option<String>,
) -> CommandResult<ArtistDetails> {
    Ok(state.app.artist_details(artist_id.as_deref()).await?)
}

/// Adds one song to the Spotify queue.
#[tauri::command]
#[specta::specta]
pub(crate) async fn play_track(state: State<'_, AppState>, track_id: String) -> CommandResult<()> {
    Ok(state.app.play_track(&track_id).await?)
}

/// Makes a playlist from your ratings.
#[tauri::command]
#[specta::specta]
pub(crate) async fn build_playlist(
    state: State<'_, AppState>,
    request: PlaylistRequest,
) -> CommandResult<PlaylistResult> {
    Ok(state.app.build_playlist(&request).await?)
}

/// The collaborators of the current artist. Sends progress; `cancel_task("collaborators")` stops it.
#[tauri::command]
#[specta::specta]
pub(crate) async fn find_collaborators(
    state: State<'_, AppState>,
    artist_id: Option<String>,
    on_progress: Channel<Progress>,
) -> CommandResult<Vec<Collaborator>> {
    let cancel = state.start_task("collaborators").await;
    let sink = ChannelProgress(on_progress);
    Ok(state
        .app
        .collaborators(artist_id.as_deref(), &sink, &cancel)
        .await?)
}

/// A discography playlist. Sends progress; `cancel_task("discover")` stops it.
#[tauri::command]
#[specta::specta]
pub(crate) async fn discover_playlist(
    state: State<'_, AppState>,
    artist_id: Option<String>,
    mode: DiscoverMode,
    on_progress: Channel<Progress>,
) -> CommandResult<PlaylistResult> {
    let cancel = state.start_task("discover").await;
    let sink = ChannelProgress(on_progress);
    Ok(state
        .app
        .discover_playlist(artist_id.as_deref(), mode, &sink, &cancel)
        .await?)
}

/// Adds the current album to the queue.
#[tauri::command]
#[specta::specta]
pub(crate) async fn queue_current_album(state: State<'_, AppState>) -> CommandResult<QueueResult> {
    Ok(state.app.queue_current_album().await?)
}

/// "My Top 10": your 10 highest-rated songs of the artist, to the queue.
#[tauri::command]
#[specta::specta]
pub(crate) async fn queue_my_top_ten(
    state: State<'_, AppState>,
    artist_id: Option<String>,
) -> CommandResult<QueueResult> {
    Ok(state.app.queue_my_top_ten(artist_id.as_deref()).await?)
}

/// Cancels a long command ("collaborators" or "discover").
#[tauri::command]
#[specta::specta]
pub(crate) async fn cancel_task(state: State<'_, AppState>, task: String) -> CommandResult<()> {
    if !TASKS.contains(&task.as_str()) {
        return Err(CommandError::invalid(format!("unknown task {task}")));
    }
    state.cancel_task(&task).await;
    Ok(())
}
