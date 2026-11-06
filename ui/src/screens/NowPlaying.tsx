import { useState } from "react";

import { errorText, useAsync } from "../components/async";
import { Stars } from "../components/Stars";
import { LoadStatus } from "../components/Status";
import { nowPlaying, rateCurrent, recentRatings } from "../ipc";
import type { NowPlaying as Playing, RatedTrackDto } from "../ipc";

/** Now Playing: stars 1-10 for the current song, and the last 10 ratings. */
export function NowPlaying() {
  const song = useAsync(nowPlaying, undefined);
  const log = useAsync(recentRatings, undefined);
  const [rating, setRating] = useState(false);
  const [message, setMessage] = useState<string | null>(null);

  async function rate(value: number) {
    setRating(true);
    setMessage(null);

    try {
      const rated = await rateCurrent(value);

      setMessage(`${rated.name}: ${value}`);
      song.reload();
      log.reload();
    } catch (error) {
      // SAFETY: commands reject with an Error (IpcError or a JavaScript failure).
      setMessage(errorText(error as Error));
    } finally {
      setRating(false);
    }
  }

  return (
    <section aria-labelledby="now-playing-title">
      <header className="screen-header">
        <h1 id="now-playing-title">Now Playing</h1>
        <button type="button" onClick={song.reload}>
          Refresh
        </button>
      </header>
      <LoadStatus state={song.state} />
      {song.state.status === "ok" && (
        <CurrentSong song={song.state.value} rating={rating} onRate={(v) => void rate(v)} />
      )}
      {message !== null && (
        <p className="status" role="status">
          {message}
        </p>
      )}
      <h2>Last 10 ratings</h2>
      <LoadStatus state={log.state} />
      {log.state.status === "ok" && <RatingLog rows={log.state.value} />}
    </section>
  );
}

function CurrentSong({
  song,
  rating,
  onRate,
}: {
  song: Playing | null;
  rating: boolean;
  onRate: (value: number) => void;
}) {
  if (song === null) {
    return <p className="empty">Nothing playing</p>;
  }

  return (
    <div className="current-song">
      {song.imageUrl !== null && <img src={song.imageUrl} alt="" className="cover" />}
      <div>
        <p className="song-name">{song.name}</p>
        <p className="song-artists">{song.artists.map((a) => a.name).join(", ")}</p>
        <p className="song-album">{song.albumName}</p>
        <Stars value={song.rating} disabled={rating} onRate={onRate} />
        <p className="song-rating">
          {song.rating === null ? "Not rated" : `Rating: ${song.rating}`}
        </p>
      </div>
    </div>
  );
}

function RatingLog({ rows }: { rows: RatedTrackDto[] }) {
  if (rows.length === 0) {
    return <p className="empty">No ratings yet</p>;
  }

  return (
    <ol className="log" aria-label="Last 10 ratings">
      {rows.map((row) => (
        <li key={`${row.trackId}-${row.ratedAt}`}>
          {row.trackName} by {row.artistName} — {row.rating}
        </li>
      ))}
    </ol>
  );
}
