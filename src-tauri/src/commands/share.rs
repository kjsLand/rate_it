//! Share commands: the cards, the cover image and the clipboard.

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use rate_it_core::dto::{AlbumCard, SongCard};
use tauri::{AppHandle, State};
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::error::{CommandError, CommandResult};
use crate::state::AppState;

/// The only host that cover images come from.
const COVER_HOST: &str = "i.scdn.co";
/// The largest cover image that the app reads.
const MAX_COVER_BYTES: usize = 5 * 1024 * 1024;

/// The card of the song that plays now.
#[tauri::command]
#[specta::specta]
pub(crate) async fn song_card(state: State<'_, AppState>) -> CommandResult<SongCard> {
    Ok(state.app.song_card().await?)
}

/// The card of the album that plays now.
#[tauri::command]
#[specta::specta]
pub(crate) async fn album_card(state: State<'_, AppState>) -> CommandResult<AlbumCard> {
    Ok(state.app.album_card().await?)
}

/// Reads a Spotify cover image and returns it as a `data:` URL, so the card canvas can use it.
#[tauri::command]
#[specta::specta]
pub(crate) async fn cover_data_url(url: String) -> CommandResult<String> {
    let parsed = url::Url::parse(&url).map_err(|_| CommandError::invalid("bad image URL"))?;
    if parsed.scheme() != "https" || parsed.host_str() != Some(COVER_HOST) {
        return Err(CommandError::invalid(
            "only Spotify cover images are permitted",
        ));
    }
    let fail = |e: reqwest::Error| CommandError::invalid(format!("cannot read the cover: {e}"));
    let response = reqwest::get(parsed)
        .await
        .map_err(fail)?
        .error_for_status()
        .map_err(fail)?;
    let mime = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .filter(|v| v.starts_with("image/"))
        .unwrap_or("image/jpeg")
        .to_owned();
    let bytes = response.bytes().await.map_err(fail)?;
    if bytes.len() > MAX_COVER_BYTES {
        return Err(CommandError::invalid("the cover image is too large"));
    }
    Ok(format!("data:{mime};base64,{}", STANDARD.encode(&bytes)))
}

/// Copies a PNG image (base64) of a card to the clipboard (desktop).
#[tauri::command]
#[specta::specta]
pub(crate) async fn copy_card_image(handle: AppHandle, png_base64: String) -> CommandResult<()> {
    let bytes = STANDARD
        .decode(png_base64.trim_start_matches("data:image/png;base64,"))
        .map_err(|_| CommandError::invalid("the card image is not base64"))?;
    let image = tauri::image::Image::from_bytes(&bytes)
        .map_err(|e| CommandError::invalid(format!("the card image is not a PNG: {e}")))?;
    handle
        .clipboard()
        .write_image(&image)
        .map_err(|e| CommandError::invalid(format!("cannot copy the image: {e}")))
}
