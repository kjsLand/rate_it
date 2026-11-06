-- Initial schema. See docs/SPEC.md, "Data".
CREATE TABLE artists (
  id   TEXT PRIMARY KEY,          -- Spotify artist ID
  name TEXT NOT NULL
);

CREATE TABLE tracks (
  id         TEXT PRIMARY KEY,    -- Spotify track ID
  name       TEXT NOT NULL,
  album_id   TEXT,
  album_name TEXT
);

CREATE TABLE track_artists (
  track_id  TEXT NOT NULL REFERENCES tracks(id),
  artist_id TEXT NOT NULL REFERENCES artists(id),
  position  INTEGER NOT NULL,     -- 0 = main artist
  PRIMARY KEY (track_id, artist_id)
);

CREATE TABLE ratings (
  id       INTEGER PRIMARY KEY,
  track_id TEXT NOT NULL REFERENCES tracks(id),
  rating   INTEGER NOT NULL CHECK (rating BETWEEN 1 AND 10),
  rated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  source   TEXT NOT NULL CHECK (source IN ('app', 'import'))
);
CREATE INDEX ratings_track ON ratings(track_id, id);

-- The last rating of each track
CREATE VIEW current_ratings AS
SELECT r.* FROM ratings r
WHERE r.id = (SELECT MAX(id) FROM ratings WHERE track_id = r.track_id);

CREATE TABLE schema_migrations (
  version    INTEGER PRIMARY KEY,
  applied_at TEXT NOT NULL
);
INSERT INTO schema_migrations (version, applied_at)
VALUES (1, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'));
