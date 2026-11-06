//! The state that all commands share.

use std::collections::HashMap;
use std::sync::Arc;

use rate_it_core::{App, CancellationToken};
use rate_it_spotify::{ClientConfig, SpotifyClient, TokenStore};
use rate_it_storage::SqliteRepository;
use tauri::Manager;
use tokio::sync::Mutex;

use crate::error::{CommandError, CommandResult};

/// The Client ID given at build time, if any (`RATE_IT_SPOTIFY_CLIENT_ID`).
const BUILD_CLIENT_ID: Option<&str> = option_env!("RATE_IT_SPOTIFY_CLIENT_ID");

/// A sign-in that waits for its redirect (mobile deep link).
#[cfg(mobile)]
#[derive(Debug)]
pub(crate) struct PendingSignIn {
    pub(crate) session: rate_it_spotify::PkceSession,
    pub(crate) redirect_uri: String,
}

/// The managed state of the Tauri app.
#[derive(Debug)]
pub(crate) struct AppState {
    pub(crate) app: App,
    pub(crate) client: Arc<SpotifyClient>,
    tasks: Mutex<HashMap<String, CancellationToken>>,
    #[cfg(mobile)]
    pub(crate) pending: Mutex<Option<PendingSignIn>>,
}

impl AppState {
    /// Opens the database in the app data folder and makes the Spotify client.
    pub(crate) async fn open(handle: &tauri::AppHandle) -> CommandResult<Self> {
        let dir = handle
            .path()
            .app_data_dir()
            .map_err(|e| CommandError::invalid(format!("no app data folder: {e}")))?;
        std::fs::create_dir_all(&dir)
            .map_err(|e| CommandError::invalid(format!("cannot make {}: {e}", dir.display())))?;
        let repo = SqliteRepository::open(&dir.join("ratings.db"))
            .await
            .map_err(|e| CommandError::from(rate_it_core::CoreError::from(e)))?;
        let client = Arc::new(SpotifyClient::new(
            ClientConfig::spotify()?,
            token_store(&dir),
        )?);
        let app = App::new(client.clone(), Arc::new(repo));
        let client_id = app
            .saved_client_id()
            .await?
            .or(BUILD_CLIENT_ID.map(str::to_owned));
        client.set_client_id(client_id).await;
        Ok(Self {
            app,
            client,
            tasks: Mutex::new(HashMap::new()),
            #[cfg(mobile)]
            pending: Mutex::new(None),
        })
    }

    /// Starts a cancellable task. A running task with the same name is cancelled first.
    pub(crate) async fn start_task(&self, name: &str) -> CancellationToken {
        let token = CancellationToken::new();
        if let Some(old) = self
            .tasks
            .lock()
            .await
            .insert(name.to_owned(), token.clone())
        {
            old.cancel();
        }
        token
    }

    /// Cancels the task with this name, if it runs.
    pub(crate) async fn cancel_task(&self, name: &str) {
        if let Some(token) = self.tasks.lock().await.remove(name) {
            token.cancel();
        }
    }
}

#[cfg(desktop)]
fn token_store(_dir: &std::path::Path) -> Arc<dyn TokenStore> {
    Arc::new(crate::token_store::KeyringTokenStore)
}

#[cfg(mobile)]
fn token_store(dir: &std::path::Path) -> Arc<dyn TokenStore> {
    Arc::new(crate::token_store::FileTokenStore::new(dir))
}
