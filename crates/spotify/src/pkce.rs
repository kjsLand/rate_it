//! PKCE values and the sign-in URL (Authorization Code flow with PKCE).

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use sha2::{Digest, Sha256};
use url::Url;

use crate::error::{SpotifyError, SpotifyResult};

/// The scopes of the Python app, less `user-read-email` and `user-read-recently-played`.
pub const SCOPES: &[&str] = &[
    "user-read-playback-state",
    "user-modify-playback-state",
    "user-read-currently-playing",
    "user-read-private",
    "playlist-modify-public",
    "playlist-modify-private",
];

/// The fixed Spotify sign-in URL. The app opens no other URL for sign-in.
pub const AUTHORIZE_URL: &str = "https://accounts.spotify.com/authorize";

/// The values of one sign-in attempt. Keep them in Rust memory only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PkceSession {
    pub verifier: String,
    pub challenge: String,
    pub state: String,
}

impl PkceSession {
    /// Makes a new random verifier (64 characters), its S256 challenge and a random state.
    pub fn generate() -> Self {
        let verifier = random_token::<48>();
        let challenge = challenge_for(&verifier);
        Self {
            verifier,
            challenge,
            state: random_token::<24>(),
        }
    }

    /// The URL of the Spotify sign-in page for this session.
    pub fn authorize_url(&self, client_id: &str, redirect_uri: &str) -> SpotifyResult<Url> {
        let scope = SCOPES.join(" ");
        let params = [
            ("response_type", "code"),
            ("client_id", client_id),
            ("redirect_uri", redirect_uri),
            ("scope", scope.as_str()),
            ("code_challenge_method", "S256"),
            ("code_challenge", self.challenge.as_str()),
            ("state", self.state.as_str()),
        ];
        Url::parse_with_params(AUTHORIZE_URL, params)
            .map_err(|e| SpotifyError::Auth(format!("bad sign-in URL: {e}")))
    }

    /// Checks the redirect URL and returns the authorization code (F11).
    ///
    /// The URL must start with `expected_redirect`, carry the same `state`, and carry a `code`.
    pub fn code_from_redirect(
        &self,
        redirect: &str,
        expected_redirect: &str,
    ) -> Result<String, String> {
        let url = Url::parse(redirect).map_err(|e| format!("bad redirect URL: {e}"))?;
        let expected =
            Url::parse(expected_redirect).map_err(|e| format!("bad redirect URI: {e}"))?;
        let same_target = url.scheme() == expected.scheme()
            && url.host_str() == expected.host_str()
            && url.port_or_known_default() == expected.port_or_known_default()
            && url.path().trim_end_matches('/') == expected.path().trim_end_matches('/');
        if !same_target {
            return Err("the redirect URL is not the expected redirect URI".into());
        }
        let param = |name: &str| {
            url.query_pairs()
                .find(|(k, _)| k == name)
                .map(|(_, v)| v.into_owned())
        };
        if let Some(error) = param("error") {
            return Err(format!("Spotify refused the sign-in: {error}"));
        }
        if param("state").as_deref() != Some(self.state.as_str()) {
            return Err("the state value does not agree".into());
        }
        param("code")
            .filter(|c| !c.is_empty())
            .ok_or_else(|| "no code in the redirect".into())
    }
}

/// The S256 code challenge of a verifier.
pub fn challenge_for(verifier: &str) -> String {
    URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()))
}

fn random_token<const N: usize>() -> String {
    let mut bytes = [0_u8; N];
    rand::fill(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_matches_rfc_7636_example() {
        // RFC 7636, Appendix B.
        let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        assert_eq!(
            challenge_for(verifier),
            "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"
        );
    }

    #[test]
    fn generate_makes_valid_unique_values() {
        let a = PkceSession::generate();
        let b = PkceSession::generate();
        assert_eq!(a.verifier.len(), 64);
        assert!((43..=128).contains(&a.verifier.len()));
        assert_ne!(a.verifier, b.verifier);
        assert_ne!(a.state, b.state);
        assert_eq!(a.challenge, challenge_for(&a.verifier));
    }

    #[test]
    fn authorize_url_has_all_parameters() {
        let s = PkceSession::generate();
        let url = s
            .authorize_url("cid", "http://127.0.0.1:5000/callback")
            .unwrap();
        assert!(url.as_str().starts_with(AUTHORIZE_URL));
        let pairs: Vec<(String, String)> = url.query_pairs().into_owned().collect();
        let get = |k: &str| pairs.iter().find(|(n, _)| n == k).map(|(_, v)| v.clone());
        assert_eq!(get("client_id").as_deref(), Some("cid"));
        assert_eq!(get("code_challenge_method").as_deref(), Some("S256"));
        assert_eq!(get("state"), Some(s.state.clone()));
        assert!(
            get("scope")
                .unwrap_or_default()
                .contains("playlist-modify-private")
        );
        assert!(!get("scope").unwrap_or_default().contains("user-read-email"));
    }

    #[test]
    fn code_from_redirect_checks_state_and_target() {
        let s = PkceSession::generate();
        let expected = "http://127.0.0.1:5000/callback";
        let ok = format!("{expected}?code=abc&state={}", s.state);
        assert_eq!(s.code_from_redirect(&ok, expected), Ok("abc".into()));
        let bad_state = format!("{expected}?code=abc&state=other");
        assert!(s.code_from_redirect(&bad_state, expected).is_err());
        let other_host = format!("http://evil.example/callback?code=abc&state={}", s.state);
        assert!(s.code_from_redirect(&other_host, expected).is_err());
        let refused = format!("{expected}?error=access_denied&state={}", s.state);
        assert!(
            s.code_from_redirect(&refused, expected)
                .unwrap_err()
                .contains("access_denied")
        );
        let custom = format!("rateit://callback?code=xyz&state={}", s.state);
        assert_eq!(
            s.code_from_redirect(&custom, "rateit://callback"),
            Ok("xyz".into())
        );
    }
}
