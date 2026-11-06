-- Settings (for example the Spotify Client ID) and the Collaborators album cache.
CREATE TABLE app_settings (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

-- One row for each album that the Collaborators and Discover screens fetched.
-- body is the JSON of the album tracks, written and read by the core crate.
CREATE TABLE album_cache (
  album_id   TEXT PRIMARY KEY,
  body       TEXT NOT NULL,
  fetched_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

INSERT INTO schema_migrations (version, applied_at)
VALUES (2, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'));
