//! Settings: the Spotify Client ID and the account name.

use crate::error::{CoreError, CoreResult};
use crate::{App, CLIENT_ID_SETTING};

impl App {
    /// The saved Spotify Client ID, if any.
    pub async fn saved_client_id(&self) -> CoreResult<Option<String>> {
        Ok(self.repo.setting(CLIENT_ID_SETTING).await?)
    }

    /// Saves the Spotify Client ID. It must be letters and digits only.
    pub async fn save_client_id(&self, client_id: &str) -> CoreResult<String> {
        let id = client_id.trim();
        if id.is_empty() || id.len() > 64 || !id.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(CoreError::Invalid(
                "The Client ID must be letters and digits".into(),
            ));
        }
        self.repo.set_setting(CLIENT_ID_SETTING, id).await?;
        Ok(id.to_owned())
    }

    /// The Spotify display name of the user.
    pub async fn display_name(&self) -> CoreResult<Option<String>> {
        let me = self.spotify.me().await?;
        Ok(me.display_name.or(Some(me.id)))
    }
}

#[cfg(test)]
mod tests {
    use crate::fakes::app;

    #[tokio::test]
    async fn client_id_is_checked_and_saved() {
        let (app, _) = app().await;
        assert_eq!(app.saved_client_id().await.unwrap(), None);
        assert!(app.save_client_id("bad id!").await.is_err());
        assert_eq!(app.save_client_id(" abc123 ").await.unwrap(), "abc123");
        assert_eq!(
            app.saved_client_id().await.unwrap().as_deref(),
            Some("abc123")
        );
        assert_eq!(app.display_name().await.unwrap().as_deref(), Some("Me"));
    }
}
