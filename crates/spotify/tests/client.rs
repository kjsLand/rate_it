//! Tests of the Spotify client against a `wiremock` server.
#![expect(
    clippy::unwrap_used,
    reason = "test helpers: a panic is a test failure"
)]

use std::sync::Arc;
use std::time::Duration;

use rate_it_spotify::{
    ClientConfig, MemoryTokenStore, SpotifyApi, SpotifyClient, SpotifyError, TokenStore,
};
use serde_json::json;
use url::Url;
use wiremock::matchers::{body_string_contains, header, method, path, query_param};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

fn config(server: &MockServer) -> ClientConfig {
    let base = Url::parse(&format!("{}/", server.uri())).unwrap();
    ClientConfig {
        api_base: base.join("v1/").unwrap(),
        accounts_base: base,
        timeout: Duration::from_millis(500),
        max_retries: 3,
        max_parallel: 4,
    }
}

async fn signed_in_client(server: &MockServer, store: Arc<MemoryTokenStore>) -> SpotifyClient {
    let client = SpotifyClient::new(config(server), store).unwrap();
    client.set_client_id(Some("cid".into())).await;
    client
}

fn token_body(access: &str, refresh: Option<&str>) -> serde_json::Value {
    json!({
        "access_token": access, "token_type": "Bearer", "expires_in": 3600,
        "refresh_token": refresh,
    })
}

async fn mount_refresh(server: &MockServer, access: &str) {
    Mock::given(method("POST"))
        .and(path("/api/token"))
        .and(body_string_contains("grant_type=refresh_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(token_body(access, None)))
        .mount(server)
        .await;
}

fn track_json(id: &str) -> serde_json::Value {
    json!({
        "id": id, "name": format!("Track {id}"), "uri": format!("spotify:track:{id}"),
        "explicit": false,
        "album": { "id": "al1", "name": "Album", "images": [] },
        "artists": [{ "id": "ar1", "name": "Artist" }]
    })
}

#[tokio::test]
async fn no_refresh_token_means_not_signed_in() {
    let server = MockServer::start().await;
    let client = signed_in_client(&server, Arc::new(MemoryTokenStore::default())).await;
    assert!(!client.is_signed_in().await);
    assert!(matches!(client.me().await, Err(SpotifyError::NotSignedIn)));
}

#[tokio::test]
async fn http_204_gives_none() {
    let server = MockServer::start().await;
    mount_refresh(&server, "a1").await;
    Mock::given(method("GET"))
        .and(path("/v1/me/player/currently-playing"))
        .and(query_param("market", "from_token"))
        .and(header("authorization", "Bearer a1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;
    let client = signed_in_client(&server, Arc::new(MemoryTokenStore::with_token("r1"))).await;
    assert_eq!(client.currently_playing().await.unwrap(), None);
}

#[tokio::test]
async fn http_401_refreshes_one_time_and_sends_again() {
    let server = MockServer::start().await;
    // The first refresh gives a token that the API refuses; the second gives a good one.
    Mock::given(method("POST"))
        .and(path("/api/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(token_body("old", None)))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/api/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(token_body("new", Some("r2"))))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/me"))
        .and(header("authorization", "Bearer old"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/me"))
        .and(header("authorization", "Bearer new"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"id": "u1", "display_name": "U"})),
        )
        .mount(&server)
        .await;
    let store = Arc::new(MemoryTokenStore::with_token("r1"));
    let client = signed_in_client(&server, store.clone()).await;
    assert_eq!(client.me().await.unwrap().id, "u1");
    assert_eq!(
        store.load().unwrap().as_deref(),
        Some("r2"),
        "a new refresh token is kept"
    );
}

#[tokio::test]
async fn second_401_is_an_error() {
    let server = MockServer::start().await;
    mount_refresh(&server, "a1").await;
    Mock::given(method("GET"))
        .and(path("/v1/me"))
        .respond_with(
            ResponseTemplate::new(401)
                .set_body_json(json!({"error": {"status": 401, "message": "expired"}})),
        )
        .expect(2)
        .mount(&server)
        .await;
    let client = signed_in_client(&server, Arc::new(MemoryTokenStore::with_token("r1"))).await;
    let err = client.me().await.unwrap_err();
    assert_eq!(err.status().map(|s| s.as_u16()), Some(401));
    assert!(err.to_string().contains("expired"));
    assert!(err.to_string().contains("GET /me"));
}

#[tokio::test]
async fn http_429_waits_for_retry_after_then_succeeds() {
    let server = MockServer::start().await;
    mount_refresh(&server, "a1").await;
    Mock::given(method("GET"))
        .and(path("/v1/artists/ar1"))
        .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "1"))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/artists/ar1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "ar1", "name": "A"})))
        .mount(&server)
        .await;
    let client = signed_in_client(&server, Arc::new(MemoryTokenStore::with_token("r1"))).await;
    let start = std::time::Instant::now();
    assert_eq!(client.artist("ar1").await.unwrap().name, "A");
    assert!(
        start.elapsed() >= Duration::from_secs(1),
        "the client waited for Retry-After"
    );
}

#[tokio::test]
async fn http_429_stops_after_three_retries() {
    let server = MockServer::start().await;
    mount_refresh(&server, "a1").await;
    Mock::given(method("GET"))
        .and(path("/v1/artists/ar1"))
        .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "0"))
        .expect(4)
        .mount(&server)
        .await;
    let client = signed_in_client(&server, Arc::new(MemoryTokenStore::with_token("r1"))).await;
    assert!(matches!(
        client.artist("ar1").await,
        Err(SpotifyError::RateLimited { .. })
    ));
}

#[tokio::test]
async fn slow_response_is_a_timeout() {
    let server = MockServer::start().await;
    mount_refresh(&server, "a1").await;
    Mock::given(method("GET"))
        .and(path("/v1/me"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(2)))
        .mount(&server)
        .await;
    let client = signed_in_client(&server, Arc::new(MemoryTokenStore::with_token("r1"))).await;
    assert!(matches!(
        client.me().await,
        Err(SpotifyError::Timeout { .. })
    ));
}

#[tokio::test]
async fn incorrect_response_shape_is_a_decode_error() {
    let server = MockServer::start().await;
    mount_refresh(&server, "a1").await;
    Mock::given(method("GET"))
        .and(path("/v1/tracks/t1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": 5})))
        .mount(&server)
        .await;
    let client = signed_in_client(&server, Arc::new(MemoryTokenStore::with_token("r1"))).await;
    let err = client.track("t1").await.unwrap_err();
    assert!(matches!(err, SpotifyError::Decode { .. }), "{err}");
}

#[tokio::test]
async fn track_404_gives_none() {
    let server = MockServer::start().await;
    mount_refresh(&server, "a1").await;
    Mock::given(method("GET"))
        .and(path("/v1/tracks/gone"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/tracks/t1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(track_json("t1")))
        .mount(&server)
        .await;
    let client = signed_in_client(&server, Arc::new(MemoryTokenStore::with_token("r1"))).await;
    assert_eq!(client.track("gone").await.unwrap(), None);
    assert_eq!(
        client.track("t1").await.unwrap().unwrap().artists[0].id,
        "ar1"
    );
}

#[tokio::test]
async fn artist_albums_reads_all_pages_with_include_groups() {
    let server = MockServer::start().await;
    mount_refresh(&server, "a1").await;
    let album = |id: &str| json!({"id": id, "name": id, "total_tracks": 3});
    Mock::given(method("GET"))
        .and(path("/v1/artists/ar1/albums"))
        .and(query_param("include_groups", "album,single"))
        .and(query_param("offset", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"items": [album("x1"), album("x2")], "total": 3, "next": "more"}),
        ))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/artists/ar1/albums"))
        .and(query_param("offset", "10"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"items": [album("x3")], "total": 3})),
        )
        .mount(&server)
        .await;
    let client = signed_in_client(&server, Arc::new(MemoryTokenStore::with_token("r1"))).await;
    let albums = client.artist_albums("ar1").await.unwrap();
    assert_eq!(
        albums.iter().map(|a| a.id.as_str()).collect::<Vec<_>>(),
        ["x1", "x2", "x3"]
    );
}

#[tokio::test]
async fn album_reads_remaining_track_pages() {
    let server = MockServer::start().await;
    mount_refresh(&server, "a1").await;
    let t = |id: &str| json!({"id": id, "name": id, "uri": format!("spotify:track:{id}"), "artists": []});
    Mock::given(method("GET"))
        .and(path("/v1/albums/al1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "al1", "name": "Album",
            "tracks": {"items": [t("t1")], "total": 2, "next": "more"}
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/albums/al1/tracks"))
        .and(query_param("offset", "1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"items": [t("t2")], "total": 2})),
        )
        .mount(&server)
        .await;
    let client = signed_in_client(&server, Arc::new(MemoryTokenStore::with_token("r1"))).await;
    let album = client.album("al1").await.unwrap();
    assert_eq!(album.tracks.items.len(), 2);
}

#[tokio::test]
async fn playlist_uses_me_playlists_and_items_in_groups_of_100() {
    let server = MockServer::start().await;
    mount_refresh(&server, "a1").await;
    Mock::given(method("POST"))
        .and(path("/v1/me/playlists"))
        .and(body_string_contains("\"name\":\"Mix\""))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({"id": "p1", "name": "Mix"})))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/playlists/p1/items"))
        .respond_with(|req: &Request| {
            let body: serde_json::Value = serde_json::from_slice(&req.body).unwrap();
            let count = body["uris"].as_array().map_or(0, Vec::len);
            if count <= 100 {
                ResponseTemplate::new(201).set_body_json(json!({"snapshot_id": "s"}))
            } else {
                ResponseTemplate::new(400)
            }
        })
        .expect(3)
        .mount(&server)
        .await;
    let client = signed_in_client(&server, Arc::new(MemoryTokenStore::with_token("r1"))).await;
    let playlist = client
        .create_playlist("Mix", "made by rate_it")
        .await
        .unwrap();
    let uris: Vec<String> = (0..250).map(|i| format!("spotify:track:t{i}")).collect();
    client
        .add_playlist_items(&playlist.id, &uris)
        .await
        .unwrap();
}

#[tokio::test]
async fn add_to_queue_accepts_empty_success_body() {
    let server = MockServer::start().await;
    mount_refresh(&server, "a1").await;
    Mock::given(method("POST"))
        .and(path("/v1/me/player/queue"))
        .and(query_param("uri", "spotify:track:t1"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .expect(1)
        .mount(&server)
        .await;
    let client = signed_in_client(&server, Arc::new(MemoryTokenStore::with_token("r1"))).await;
    client.add_to_queue("spotify:track:t1").await.unwrap();
}

#[tokio::test]
async fn sign_in_exchanges_code_with_verifier_and_sign_out_clears() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/token"))
        .and(body_string_contains("grant_type=authorization_code"))
        .and(body_string_contains("code_verifier=ver"))
        .and(body_string_contains("client_id=cid"))
        .respond_with(ResponseTemplate::new(200).set_body_json(token_body("a1", Some("r1"))))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/me"))
        .and(header("authorization", "Bearer a1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "u1"})))
        .mount(&server)
        .await;
    let store = Arc::new(MemoryTokenStore::default());
    let client = signed_in_client(&server, store.clone()).await;
    client
        .complete_sign_in("code", "ver", "http://127.0.0.1:1/callback")
        .await
        .unwrap();
    assert!(client.is_signed_in().await);
    assert_eq!(store.load().unwrap().as_deref(), Some("r1"));
    assert_eq!(
        client.me().await.unwrap().id,
        "u1",
        "no refresh: the access token is kept"
    );
    client.sign_out().await.unwrap();
    assert!(!client.is_signed_in().await);
    assert_eq!(store.load().unwrap(), None);
}

#[tokio::test]
async fn refused_code_exchange_is_an_auth_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/token"))
        .respond_with(
            ResponseTemplate::new(400)
                .set_body_json(json!({"error": "invalid_grant", "error_description": "bad code"})),
        )
        .mount(&server)
        .await;
    let client = signed_in_client(&server, Arc::new(MemoryTokenStore::default())).await;
    let err = client.complete_sign_in("x", "y", "z").await.unwrap_err();
    assert!(
        matches!(err, SpotifyError::Auth(ref m) if m.contains("invalid_grant")),
        "{err}"
    );
}
