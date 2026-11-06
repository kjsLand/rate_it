import "./BuildPlaylist.css";
import { useState } from "react";

import { useAction, useAsync } from "../components/async";
import { ActionStatus, LoadStatus } from "../components/Status";
import { buildPlaylist, libraryArtists } from "../ipc";

const RATINGS = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

/** Build Playlist: select artists and a minimum rating; the name is trimmed and required. */
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
      <p>
        Select artists and a minimum rating. The playlist gets your songs with that rating or
        higher.
      </p>
      <LoadStatus state={artists.state} />
      {artists.state.status === "ok" && (
        <ul className="select-list" aria-label="Artists">
          {artists.state.value.map((artist) => (
            <li key={artist.id}>
              <label>
                <input
                  type="checkbox"
                  checked={selected.has(artist.id)}
                  onChange={(e) => toggle(artist.id, e.target.checked)}
                />
                {artist.name}
              </label>
              <select
                aria-label={`Minimum rating for ${artist.name}`}
                disabled={!selected.has(artist.id)}
                value={selected.get(artist.id) ?? 8}
                onChange={(e) => setMin(artist.id, Number(e.target.value))}
              >
                {RATINGS.map((r) => (
                  <option key={r} value={r}>
                    {r}
                  </option>
                ))}
              </select>
            </li>
          ))}
        </ul>
      )}
      <form
        className="button-row"
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
        <label>
          Playlist name
          <input value={name} maxLength={100} required onChange={(e) => setName(e.target.value)} />
        </label>
        <button type="submit" disabled={!ready}>
          Make playlist
        </button>
      </form>
      <ActionStatus action={action} />
    </section>
  );
}
