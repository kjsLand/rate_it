import { useState } from "react";

import { useAction, useAsync } from "../components/async";
import { Icon } from "../components/Icon";
import { RatingChip } from "../components/RatingChip";
import { ActionStatus, LoadStatus } from "../components/Status";
import { libraryArtists, libraryTracks, rerateTrack, trackHistory } from "../ipc";
import type { ArtistSummaryDto, RatedTrackDto } from "../ipc";
import "./Library.css";
import "./LibraryHistory.css";

/** Library ("The Crate"): all ratings by artist, sorted by rating. Change a rating in place. */
export function Library() {
  const artists = useAsync(libraryArtists, undefined);
  const [selected, setSelected] = useState<ArtistSummaryDto | null>(null);

  return (
    <section aria-labelledby="library-title" className="crate" data-open={selected !== null}>
      <header className="screen-header">
        <h1 id="library-title">Library</h1>
      </header>
      <LoadStatus state={artists.state} />
      {artists.state.status === "ok" && artists.state.value.length === 0 && (
        <div className="crate-empty">
          <Icon name="crate" />
          <p className="empty">No ratings yet</p>
        </div>
      )}
      <div className="columns">
        {artists.state.status === "ok" && artists.state.value.length > 0 && (
          <ul className="dividers" aria-label="Artists">
            {artists.state.value.map((artist) => (
              <li key={artist.id}>
                <Divider
                  artist={artist}
                  pressed={selected?.id === artist.id}
                  onSelect={() => setSelected(artist)}
                />
              </li>
            ))}
          </ul>
        )}
        {selected === null && artists.state.status === "ok" && artists.state.value.length > 0 && (
          <p className="crate-hint">Pick an artist from the crate to see your ratings.</p>
        )}
        {selected !== null && (
          <ArtistTracks
            key={selected.id}
            artist={selected}
            onBack={() => setSelected(null)}
            onChanged={artists.reload}
          />
        )}
      </div>
    </section>
  );
}

/** One crate divider: the initial, the name, the count and an average bar. */
function Divider({
  artist,
  pressed,
  onSelect,
}: {
  artist: ArtistSummaryDto;
  pressed: boolean;
  onSelect: () => void;
}) {
  const average = (artist.average ?? 0).toFixed(1);

  return (
    <button type="button" className="divider" aria-pressed={pressed} onClick={onSelect}>
      <span className="visually-hidden">
        {artist.name} ({artist.ratedCount}, average {average})
      </span>
      <span className="divider-tab" aria-hidden="true">
        {artist.name.slice(0, 1)}
      </span>
      <span className="divider-text" aria-hidden="true">
        <span className="divider-name">{artist.name}</span>
        <span className="divider-meta">
          {artist.ratedCount} rated · avg {average}
        </span>
        <span className="divider-bar">
          <span style={{ width: `${(artist.average ?? 0) * 10}%` }} />
        </span>
      </span>
    </button>
  );
}

function ArtistTracks({
  artist,
  onBack,
  onChanged,
}: {
  artist: ArtistSummaryDto;
  onBack: () => void;
  onChanged: () => void;
}) {
  const tracks = useAsync(libraryTracks, artist.id);

  return (
    <div className="artist-tracks">
      <button type="button" className="quiet crate-back" onClick={onBack}>
        ← Artists
      </button>
      <p className="eyebrow">{artist.ratedCount} rated songs</p>
      <h2>{artist.name}</h2>
      <LoadStatus state={tracks.state} />
      {tracks.state.status === "ok" && (
        <ul className="track-list" aria-label={`Songs of ${artist.name}`}>
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

function step(text: string, by: number): string {
  const value = parseRating(text) ?? 5;

  return String(Math.min(10, Math.max(1, value + by)));
}

function TrackRow({ track, onChanged }: { track: RatedTrackDto; onChanged: () => void }) {
  const [text, setText] = useState(String(track.rating));
  const [showHistory, setShowHistory] = useState(false);
  const { action, run } = useAction();
  const value = parseRating(text);

  return (
    <li className="track-row">
      <span className="track-name">{track.trackName}</span>
      <RatingChip rating={track.rating} label={`Current rating ${track.rating}`} />
      <span className="stepper">
        <button
          type="button"
          className="step"
          aria-label="Lower the new rating"
          onClick={() => setText(step(text, -1))}
        >
          <Icon name="minus" />
        </button>
        <label>
          <span className="visually-hidden">New rating</span>
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
          className="step"
          aria-label="Raise the new rating"
          onClick={() => setText(step(text, 1))}
        >
          <Icon name="plus" />
        </button>
      </span>
      <button
        type="button"
        className="primary small"
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
      <button type="button" className="quiet" onClick={() => setShowHistory((s) => !s)}>
        {showHistory ? "Hide history" : "History"}
      </button>
      {value === null && <span className="hint track-hint">1–10</span>}
      <ActionStatus action={action} />
      {showHistory && <History trackId={track.trackId} />}
    </li>
  );
}

/** The rating history as a timeline ("the run-out groove"). The newest rating is the current one. */
function History({ trackId }: { trackId: string }) {
  const history = useAsync(trackHistory, trackId);

  if (history.state.status !== "ok") {
    return <LoadStatus state={history.state} />;
  }

  const entries = history.state.value;
  const newest = entries.reduce((max, e) => (e.ratedAt > max ? e.ratedAt : max), "");

  return (
    <ol className="history" aria-label="Rating history">
      {entries.map((entry) => (
        <li key={entry.ratedAt} data-current={entry.ratedAt === newest}>
          <span className="history-rating">{entry.rating}</span> on{" "}
          <time dateTime={entry.ratedAt}>{entry.ratedAt.slice(0, 10)}</time>{" "}
          <span className="tag">{entry.source}</span>
        </li>
      ))}
    </ol>
  );
}
