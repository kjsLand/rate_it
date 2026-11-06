import { useAction, useAsync } from "../components/async";
import { Icon } from "../components/Icon";
import { RatingChip } from "../components/RatingChip";
import { ActionStatus, LoadStatus } from "../components/Status";
import { artistDetails, playTrack } from "../ipc";
import "./Artist.css";

/** Artist ("The Gig Poster"): the current artist, with the average, the rated count and the top 5. */
export function Artist() {
  const details = useAsync(artistDetails, null);
  const { action, run } = useAction();

  return (
    <section aria-labelledby="artist-title">
      <header className="screen-header">
        <h1 id="artist-title">Artist</h1>
        <button type="button" className="quiet" onClick={details.reload}>
          Refresh
        </button>
      </header>
      <LoadStatus state={details.state} />
      {details.state.status === "ok" && (
        <Poster
          artist={details.state.value}
          onPlay={(trackId, name) =>
            void run(
              () => playTrack(trackId),
              () => `Queued ${name}`,
            )
          }
        />
      )}
      <ActionStatus action={action} />
    </section>
  );
}

type Details = Awaited<ReturnType<typeof artistDetails>>;

function Poster({
  artist,
  onPlay,
}: {
  artist: Details;
  onPlay: (trackId: string, name: string) => void;
}) {
  const average = artist.average === null ? "none" : artist.average.toFixed(1);
  const share = artist.totalCount > 0 ? (artist.ratedCount / artist.totalCount) * 100 : 0;

  return (
    <article className="poster">
      <div className="poster-hero">
        {artist.imageUrl === null ? (
          <span className="poster-blank" aria-hidden="true">
            <Icon name="mic" />
          </span>
        ) : (
          <img src={artist.imageUrl} alt="" className="poster-image" />
        )}
      </div>
      <h2 className="poster-name">{artist.name}</h2>
      <div className="bill">
        <div className="stat">
          <p className="eyebrow">Average</p>
          <p className="visually-hidden">Average rating: {average}</p>
          <p className="stat-num" aria-hidden="true">
            {artist.average === null ? "–" : average}
          </p>
        </div>
        <div className="stat">
          <p className="eyebrow">Rated</p>
          <p className="visually-hidden">
            Rated: {artist.ratedCount} / {artist.totalCount}
          </p>
          <div
            className="ring"
            aria-hidden="true"
            style={{
              background: `conic-gradient(var(--accent) ${share}%, var(--surface-raised) 0)`,
            }}
          >
            <span>
              {artist.ratedCount} / {artist.totalCount}
            </span>
          </div>
        </div>
      </div>
      <h3>Top 5</h3>
      <ol className="top5" aria-label="Top 5 songs">
        {artist.top.map((track) => (
          <li key={track.trackId}>
            <span className="top5-song">{track.trackName}</span>
            <RatingChip rating={track.rating} />
            <button
              type="button"
              className="play"
              aria-label={`Play ${track.trackName}`}
              onClick={() => onPlay(track.trackId, track.trackName)}
            >
              <Icon name="play" />
            </button>
          </li>
        ))}
      </ol>
    </article>
  );
}
