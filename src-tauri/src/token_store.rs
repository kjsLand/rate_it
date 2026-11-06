//! Where the refresh token stays between app starts.
//!
//! Desktop: the system keychain (`keyring`). Mobile: a file in the app data folder, until a
//! keychain plugin is chosen (see `docs/PROGRESS.md`, "Mobile token storage").

use rate_it_spotify::{SpotifyError, SpotifyResult, TokenStore};

#[cfg(desktop)]
const SERVICE: &str = "com.kevinland.rateit";
#[cfg(desktop)]
const ACCOUNT: &str = "spotify-refresh-token";

/// The refresh token in Keychain, Credential Manager or Secret Service.
#[cfg(desktop)]
#[derive(Debug, Default)]
pub(crate) struct KeyringTokenStore;

#[cfg(desktop)]
impl KeyringTokenStore {
    fn entry() -> SpotifyResult<keyring::Entry> {
        keyring::Entry::new(SERVICE, ACCOUNT).map_err(|e| SpotifyError::TokenStore(e.to_string()))
    }
}

#[cfg(desktop)]
impl TokenStore for KeyringTokenStore {
    fn load(&self) -> SpotifyResult<Option<String>> {
        match Self::entry()?.get_password() {
            Ok(token) => Ok(Some(token)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(SpotifyError::TokenStore(e.to_string())),
        }
    }

    fn save(&self, refresh_token: &str) -> SpotifyResult<()> {
        Self::entry()?
            .set_password(refresh_token)
            .map_err(|e| SpotifyError::TokenStore(e.to_string()))
    }

    fn clear(&self) -> SpotifyResult<()> {
        match Self::entry()?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(SpotifyError::TokenStore(e.to_string())),
        }
    }
}

/// The refresh token in a file of the app data folder (mobile).
#[cfg(mobile)]
#[derive(Debug)]
pub(crate) struct FileTokenStore {
    path: std::path::PathBuf,
}

#[cfg(mobile)]
impl FileTokenStore {
    pub(crate) fn new(dir: &std::path::Path) -> Self {
        Self {
            path: dir.join("spotify-refresh-token"),
        }
    }
}

#[cfg(mobile)]
impl TokenStore for FileTokenStore {
    fn load(&self) -> SpotifyResult<Option<String>> {
        match std::fs::read_to_string(&self.path) {
            Ok(token) if !token.trim().is_empty() => Ok(Some(token.trim().to_owned())),
            Ok(_) => Ok(None),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(SpotifyError::TokenStore(e.to_string())),
        }
    }

    fn save(&self, refresh_token: &str) -> SpotifyResult<()> {
        std::fs::write(&self.path, refresh_token)
            .map_err(|e| SpotifyError::TokenStore(e.to_string()))
    }

    fn clear(&self) -> SpotifyResult<()> {
        match std::fs::remove_file(&self.path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(SpotifyError::TokenStore(e.to_string())),
        }
    }
}
