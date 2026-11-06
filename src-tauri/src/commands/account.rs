//! Sign-in and Settings commands: Client ID, sign in, sign out.

use rate_it_core::dto::AccountStatus;
use rate_it_spotify::PkceSession;
use tauri::{AppHandle, State};
use tauri_plugin_opener::OpenerExt;

use crate::error::{CommandError, CommandResult};
use crate::state::AppState;

/// The sign-in state, the Client ID and the Spotify name.
#[tauri::command]
#[specta::specta]
pub(crate) async fn account_status(state: State<'_, AppState>) -> CommandResult<AccountStatus> {
    status(&state).await
}

pub(crate) async fn status(state: &AppState) -> CommandResult<AccountStatus> {
    let signed_in = state.client.is_signed_in().await;
    let display_name = if signed_in {
        state.app.display_name().await.ok().flatten()
    } else {
        None
    };
    Ok(AccountStatus {
        signed_in,
        client_id: state.client.client_id().await,
        display_name,
    })
}

/// Saves the Spotify Client ID of your Spotify app (Spotify developer dashboard).
#[tauri::command]
#[specta::specta]
pub(crate) async fn save_client_id(
    state: State<'_, AppState>,
    client_id: String,
) -> CommandResult<AccountStatus> {
    let id = state.app.save_client_id(&client_id).await?;
    state.client.set_client_id(Some(id)).await;
    status(&state).await
}

/// Forgets the tokens. The ratings stay.
#[tauri::command]
#[specta::specta]
pub(crate) async fn sign_out(state: State<'_, AppState>) -> CommandResult<AccountStatus> {
    state.client.sign_out().await?;
    status(&state).await
}

/// Opens the fixed Spotify sign-in page in the system browser.
fn open_sign_in(
    handle: &AppHandle,
    session: &PkceSession,
    client_id: &str,
    redirect_uri: &str,
) -> CommandResult<()> {
    let url = session.authorize_url(client_id, redirect_uri)?;
    handle
        .opener()
        .open_url(url.as_str(), None::<&str>)
        .map_err(|e| CommandError::invalid(format!("cannot open the browser: {e}")))
}

async fn required_client_id(state: &AppState) -> CommandResult<String> {
    state.client.client_id().await.ok_or_else(|| {
        CommandError::invalid("Enter the Client ID of your Spotify app in Settings first")
    })
}

/// Desktop sign-in: PKCE with a temporary loopback server on 127.0.0.1.
#[cfg(desktop)]
#[tauri::command]
#[specta::specta]
pub(crate) async fn sign_in(
    handle: AppHandle,
    state: State<'_, AppState>,
) -> CommandResult<AccountStatus> {
    use std::time::Duration;

    let client_id = required_client_id(&state).await?;
    let session = PkceSession::generate();
    let (tx, rx) = tokio::sync::oneshot::channel::<String>();
    let mut tx = Some(tx);
    let config = tauri_plugin_oauth::OauthConfig {
        ports: None,
        response: Some("<html><body>Signed in. Return to rate_it.</body></html>".into()),
        redirect_uri: None,
    };
    let port = tauri_plugin_oauth::start_with_config(config, move |url| {
        if let Some(sender) = tx.take() {
            // The command can have stopped waiting; then the URL is not needed.
            sender.send(url).ok();
        }
    })
    .map_err(|e| CommandError::invalid(format!("cannot start the sign-in server: {e}")))?;
    let redirect_uri = format!("http://127.0.0.1:{port}/callback");
    open_sign_in(&handle, &session, &client_id, &redirect_uri)?;
    let received = tokio::time::timeout(Duration::from_secs(300), rx).await;
    // The server stops after one request; cancel it in case it did not get one.
    // An error means that the server is already stopped.
    tauri_plugin_oauth::cancel(port).ok();
    let Ok(Ok(url)) = received else {
        return Err(CommandError::invalid(
            "The sign-in did not finish in 5 minutes",
        ));
    };
    let code = session
        .code_from_redirect(&url, &redirect_uri)
        .map_err(CommandError::invalid)?;
    state
        .client
        .complete_sign_in(&code, &session.verifier, &redirect_uri)
        .await?;
    status(&state).await
}

/// The redirect URI of the mobile sign-in (a custom scheme, given to Rust by the deep link).
#[cfg(mobile)]
pub(crate) const MOBILE_REDIRECT: &str = "rateit://callback";

/// Mobile sign-in: opens the browser. The deep link handler finishes the sign-in.
#[cfg(mobile)]
#[tauri::command]
#[specta::specta]
pub(crate) async fn sign_in(
    handle: AppHandle,
    state: State<'_, AppState>,
) -> CommandResult<AccountStatus> {
    let client_id = required_client_id(&state).await?;
    let session = PkceSession::generate();
    open_sign_in(&handle, &session, &client_id, MOBILE_REDIRECT)?;
    *state.pending.lock().await = Some(crate::state::PendingSignIn {
        session,
        redirect_uri: MOBILE_REDIRECT.into(),
    });
    status(&state).await
}

/// Finishes a mobile sign-in from a deep link. The URL is untrusted: another app can send it.
#[cfg(mobile)]
pub(crate) async fn finish_mobile_sign_in(state: &AppState, url: &str) -> CommandResult<()> {
    let Some(pending) = state.pending.lock().await.take() else {
        return Err(CommandError::invalid("no sign-in is waiting"));
    };
    let code = pending
        .session
        .code_from_redirect(url, &pending.redirect_uri)
        .map_err(CommandError::invalid)?;
    state
        .client
        .complete_sign_in(&code, &pending.session.verifier, &pending.redirect_uri)
        .await?;
    Ok(())
}
