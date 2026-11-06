import { useState } from "react";

import { useAction, useAsync } from "../components/async";
import { ActionStatus, LoadStatus } from "../components/Status";
import { buildPlaylist, libraryArtists } from "../ipc";
import type { ArtistSummaryDto } from "../ipc";
import "./BuildPlaylist.css";

const RATINGS = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

/** Build Playlist ("The Mixtape"): select artists and a minimum rating; the name is required. */
export function BuildPlaylist() {
  const artists = useAsync(libraryArtists, undefined);
  const [name, setName] = useState("");
  const [selected, setSelected] = useState<ReadonlyMap<string, number>>(new Map());
  const { action, run } = useAction();
  const trimmed = name.trim();
  const ready = trimmed.length > 0 && selected.size > 0 && action.status !== "running";

  function toggle(artistId: string, on: boolean) {
    const next = new Map(selected);

    if (on) next.set(artistId, selected.get(artistId) ?? 8);
    else next.delete(artistId);
    setSelected(next);
  }

  function setMin(artistId: string, min: number) {
    setSelected(new Map(selected).set(artistId, min));
  }

  return (
    <section aria-labelledby="playlist-title">
      <header className="screen-header">
        <h1 id="playlist-title">Build Playlist</h1>
      </header>
      <form
        className="mixtape"
        onSubmit={(e) => {
          e.preventDefault();

          if (!ready) return;

          const selections = [...selected].map(([artistId, minRating]) => ({
            artistId,
            minRating,
          }));

          void run(
            () => buildPlaylist({ name: trimmed, selections }),
            (result) => `Made "${result.name}" with ${result.trackCount} songs`,
          );
        }}
      >
        <label className="jcard">
          <span className="jcard-reels" aria-hidden="true">
            <span />
            <span />
          </span>
          <span className="eyebrow">Playlist name</span>
          <input
            value={name}
            maxLength={100}
            required
            placeholder="Side A — …"
            onChange={(e) => setName(e.target.value)}
          />
        </label>
        <p className="mix-intro">
          Select artists and a minimum rating. The playlist gets your songs with that rating or
          higher.
        </p>
        <LoadStatus state={artists.state} />
        {artists.state.status === "ok" && (
          <ul className="picks" aria-label="Artists">
            {artists.state.value.map((artist) => (
              <Pick
                key={artist.id}
                artist={artist}
                min={selected.get(artist.id)}
                onToggle={(on) => toggle(artist.id, on)}
                onMin={(min) => setMin(artist.id, min)}
              />
            ))}
          </ul>
        )}
        <div className="mix-bar">
          <span className="mix-count">
            {selected.size} {selected.size === 1 ? "artist" : "artists"} selected
          </span>
          <button type="submit" disabled={!ready}>
            Make playlist
          </button>
        </div>
      </form>
      <ActionStatus action={action} />
    </section>
  );
}

/** One artist row: a checkbox and the minimum rating. `min` is undefined when not selected. */
function Pick({
  artist,
  min,
  onToggle,
  onMin,
}: {
  artist: ArtistSummaryDto;
  min: number | undefined;
  onToggle: (on: boolean) => void;
  onMin: (min: number) => void;
}) {
  const on = min !== undefined;

  return (
    <li className="pick" data-on={on}>
      <label className="pick-label">
        <input type="checkbox" checked={on} onChange={(e) => onToggle(e.target.checked)} />
        {artist.name}
      </label>
      <span className="pick-meta">{artist.ratedCount} rated</span>
      <select
        className="pick-min"
        aria-label={`Minimum rating for ${artist.name}`}
        disabled={!on}
        value={min ?? 8}
        onChange={(e) => onMin(Number(e.target.value))}
      >
        {RATINGS.map((r) => (
          <option key={r} value={r}>
            ≥ {r}
          </option>
        ))}
      </select>
    </li>
  );
}
