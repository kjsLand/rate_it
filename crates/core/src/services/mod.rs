//! The services, as methods of [`crate::App`]. One module for each screen group.

mod artist;
mod collaborators;
mod discover;
mod library;
mod playlist;
mod rating;
mod settings;
mod share;
mod transfer;

use rate_it_storage::Rating;

use crate::error::{CoreError, CoreResult};

/// Checks a rating from the UI (1-10).
pub(crate) fn rating(value: u8) -> CoreResult<Rating> {
    Rating::new(i64::from(value))
        .map_err(|_| CoreError::Invalid(format!("rating {value} is not 1-10")))
}

/// Checks a Spotify ID from the UI.
pub(crate) fn spotify_id(id: &str) -> CoreResult<&str> {
    if !id.is_empty() && id.len() <= 64 && id.chars().all(|c| c.is_ascii_alphanumeric()) {
        Ok(id)
    } else {
        Err(CoreError::Invalid(format!("invalid Spotify ID {id:?}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating_accepts_one_to_ten() {
        assert!(rating(0).is_err());
        assert!(rating(11).is_err());
        assert_eq!(rating(10).unwrap().value(), 10);
    }

    #[test]
    fn spotify_id_refuses_bad_text() {
        assert!(spotify_id("abc123").is_ok());
        assert!(spotify_id("").is_err());
        assert!(spotify_id("a:b").is_err());
    }
}
