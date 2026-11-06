import { useState } from "react";

import { useAction, useAsync } from "../components/async";
import { ActionStatus, LoadStatus } from "../components/Status";
import { libraryArtists, libraryTracks, rerateTrack, trackHistory } from "../ipc";
import type { ArtistSummaryDto, RatedTrackDto } from "../ipc";

/** Library: all ratings by artist, sorted by rating. Change a rating in place. */
export function Library() {
  const artists = useAsync(libraryArtists, undefined);
  const [selected, setSelected] = useState<ArtistSummaryDto | null>(null);

  return (
    <section aria-labelledby="library-title">
      <header className="screen-header">
        <h1 id="library-title">Library</h1>
      </header>
      <LoadStatus state={artists.state} />
      {artists.state.status === "ok" && artists.state.value.length === 0 && (
        <p className="empty">No ratings yet</p>
      )}
      <div className="columns">
        {artists.state.status === "ok" && (
          <ul className="artist-list" aria-label="Artists">
            {artists.state.value.map((artist) => (
              <li key={artist.id}>
                <button
                  type="button"
                  aria-pressed={selected?.id === artist.id}
                  onClick={() => setSelected(artist)}
                >
                  {artist.name} ({artist.ratedCount}, average {(artist.average ?? 0).toFixed(1)})
                </button>
              </li>
            ))}
          </ul>
        )}
        {selected !== null && (
          <ArtistTracks key={selected.id} artist={selected} onChanged={artists.reload} />
        )}
      </div>
    </section>
  );
}

function ArtistTracks({ artist, onChanged }: { artist: ArtistSummaryDto; onChanged: () => void }) {
  const tracks = useAsync(libraryTracks, artist.id);

  return (
    <div className="artist-tracks">
      <h2>{artist.name}</h2>
      <LoadStatus state={tracks.state} />
      {tracks.state.status === "ok" && (
        <ul aria-label={`Songs of ${artist.name}`}>
          {tracks.state.value.map((track) => (
            <TrackRow
              key={track.trackId}
              track={track}
              onChanged={() => {
                tracks.reload();
                onChanged();
              }}
            />
          ))}
        </ul>
      )}
    </div>
  );
}

/** Parses the rating input. Only whole numbers 1-10 are accepted. */
export function parseRating(text: string): number | null {
  const trimmed = text.trim();

  if (!/^\d{1,2}$/.test(trimmed)) {
    return null;
  }

  const value = Number(trimmed);

  return value >= 1 && value <= 10 ? value : null;
}

function TrackRow({ track, onChanged }: { track: RatedTrackDto; onChanged: () => void }) {
  const [text, setText] = useState(String(track.rating));
  const [showHistory, setShowHistory] = useState(false);
  const { action, run } = useAction();
  const value = parseRating(text);

  return (
    <li className="track-row">
      <span className="track-name">{track.trackName}</span>
      <span className="track-rating">{track.rating}</span>
      <label>
        New rating
        <input
          type="number"
          min={1}
          max={10}
          step={1}
          value={text}
          aria-invalid={value === null}
          onChange={(e) => setText(e.target.value)}
        />
      </label>
      <button
        type="button"
        disabled={value === null || value === track.rating || action.status === "running"}
        onClick={() => {
          if (value !== null) {
            void run(
              () => rerateTrack(track.trackId, value),
              (row) => `Saved: ${row.rating}`,
            ).then(onChanged);
          }
        }}
      >
        Save
      </button>
      <button type="button" onClick={() => setShowHistory((s) => !s)}>
        {showHistory ? "Hide history" : "History"}
      </button>
      <ActionStatus action={action} />
      {showHistory && <History trackId={track.trackId} />}
    </li>
  );
}

function History({ trackId }: { trackId: string }) {
  const history = useAsync(trackHistory, trackId);

  return (
    <div className="history">
      <LoadStatus state={history.state} />
      {history.state.status === "ok" && (
        <ol aria-label="Rating history">
          {history.state.value.map((entry) => (
            <li key={entry.ratedAt}>
              {entry.rating} on {entry.ratedAt.slice(0, 10)} ({entry.source})
            </li>
          ))}
        </ol>
      )}
    </div>
  );
}
