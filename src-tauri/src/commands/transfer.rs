//! Settings > Import and Export. Rust opens the file dialogs and reads and writes the files,
//! so the `WebView` gets no file system permission.

use std::path::PathBuf;

use rate_it_core::dto::ImportReportDto;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

use crate::error::{CommandError, CommandResult};
use crate::state::AppState;

/// The largest file that an import reads.
const MAX_IMPORT_BYTES: u64 = 20 * 1024 * 1024;

fn pick_file(
    handle: &AppHandle,
    title: &str,
    extensions: &[&str],
) -> CommandResult<Option<PathBuf>> {
    let picked = handle
        .dialog()
        .file()
        .set_title(title)
        .add_filter("File", extensions)
        .blocking_pick_file();
    picked
        .map(|p| {
            p.into_path()
                .map_err(|e| CommandError::invalid(format!("bad file: {e}")))
        })
        .transpose()
}

fn read_text(path: &PathBuf) -> CommandResult<String> {
    let size = std::fs::metadata(path)
        .map_err(|e| CommandError::invalid(e.to_string()))?
        .len();
    if size > MAX_IMPORT_BYTES {
        return Err(CommandError::invalid("the file is too large"));
    }
    std::fs::read_to_string(path)
        .map_err(|e| CommandError::invalid(format!("cannot read the file: {e}")))
}

/// The one-time import of `rate.txt`. `null` if the user closes the dialog.
#[tauri::command]
#[specta::specta]
pub(crate) async fn import_legacy_file(
    handle: AppHandle,
    state: State<'_, AppState>,
) -> CommandResult<Option<ImportReportDto>> {
    let Some(path) = pick_file(&handle, "Select rate.txt", &["txt", "json"])? else {
        return Ok(None);
    };
    let text = read_text(&path)?;
    Ok(Some(state.app.import_legacy(&text).await?))
}

/// Imports a JSON export file from another device. `null` if the user closes the dialog.
#[tauri::command]
#[specta::specta]
pub(crate) async fn import_ratings_file(
    handle: AppHandle,
    state: State<'_, AppState>,
) -> CommandResult<Option<ImportReportDto>> {
    let Some(path) = pick_file(&handle, "Select a rate_it export", &["json"])? else {
        return Ok(None);
    };
    let text = read_text(&path)?;
    Ok(Some(state.app.import_json(&text).await?))
}

/// Writes all ratings to a JSON file. Returns the file name, or `null` if the user cancels.
#[tauri::command]
#[specta::specta]
pub(crate) async fn export_ratings_file(
    handle: AppHandle,
    state: State<'_, AppState>,
) -> CommandResult<Option<String>> {
    let picked = handle
        .dialog()
        .file()
        .set_title("Export ratings")
        .set_file_name("rate_it-export.json")
        .add_filter("JSON", &["json"])
        .blocking_save_file();
    let Some(file) = picked else { return Ok(None) };
    let path = file
        .into_path()
        .map_err(|e| CommandError::invalid(format!("bad file: {e}")))?;
    let text = state.app.export_json().await?;
    std::fs::write(&path, text).map_err(|e| CommandError::invalid(format!("cannot write: {e}")))?;
    Ok(Some(path.display().to_string()))
}
