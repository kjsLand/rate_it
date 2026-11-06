import { useState } from "react";

import { errorText, useAsync } from "../components/async";
import { Icon } from "../components/Icon";
import { RatingChip } from "../components/RatingChip";
import { RatingMeter } from "../components/RatingMeter";
import { LoadStatus } from "../components/Status";
import { nowPlaying, rateCurrent, recentRatings } from "../ipc";
import type { NowPlaying as Playing, RatedTrackDto } from "../ipc";
import "./NowPlaying.css";

/** Now Playing ("The Stage"): stars 1-10 for the current song, and the last 10 ratings. */
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
        <button type="button" className="quiet" onClick={song.reload}>
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
    return (
      <div className="stage stage-empty">
        <span className="disc disc-still" aria-hidden="true" />
        <p className="empty">Nothing playing</p>
      </div>
    );
  }

  return (
    <div className="stage">
      {song.imageUrl !== null && (
        <img src={song.imageUrl} alt="" className="stage-backdrop" aria-hidden="true" />
      )}
      <div className="stage-body">
        <div className="sleeve">
          <span className="disc" aria-hidden="true" />
          {song.imageUrl === null ? (
            <span className="cover cover-blank" aria-hidden="true">
              <Icon name="disc" />
            </span>
          ) : (
            <img src={song.imageUrl} alt="" className="cover" />
          )}
        </div>
        <div className="song">
          <p className="eyebrow">Now spinning</p>
          <p className="song-name">{song.name}</p>
          <p className="song-artists">{song.artists.map((a) => a.name).join(", ")}</p>
          <p className="song-album">{song.albumName}</p>
          <div className="rate">
            <RatingMeter value={song.rating} disabled={rating} onRate={onRate} />
            <Score rating={song.rating} />
          </div>
        </div>
      </div>
    </div>
  );
}

function Score({ rating }: { rating: number | null }) {
  if (rating === null) {
    return (
      <p className="score score-none">
        <span className="score-box" aria-hidden="true" />
        Not rated
      </p>
    );
  }

  return (
    <p className="score">
      <span className="visually-hidden">Rating: {rating}</span>
      <span className="score-num" aria-hidden="true">
        {rating}
      </span>
      <span className="score-max" aria-hidden="true">
        /10
      </span>
    </p>
  );
}

function RatingLog({ rows }: { rows: RatedTrackDto[] }) {
  if (rows.length === 0) {
    return <p className="empty">No ratings yet</p>;
  }

  return (
    <ol className="setlist" aria-label="Last 10 ratings">
      {rows.map((row) => (
        <li key={`${row.trackId}-${row.ratedAt}`}>
          <span className="visually-hidden">
            {row.trackName} by {row.artistName} — {row.rating}
          </span>
          <span className="setlist-song" aria-hidden="true">
            {row.trackName}
          </span>
          <span className="setlist-artist" aria-hidden="true">
            {row.artistName}
          </span>
          <span aria-hidden="true">
            <RatingChip rating={row.rating} />
          </span>
        </li>
      ))}
    </ol>
  );
}
