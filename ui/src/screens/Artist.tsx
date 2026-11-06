import { useAction, useAsync } from "../components/async";
import { ActionStatus, LoadStatus } from "../components/Status";
import { artistDetails, playTrack } from "../ipc";

/** Artist: the current artist, with the average, the rated count and the top 5 songs. */
export function Artist() {
  const details = useAsync(artistDetails, null);
  const { action, run } = useAction();

  return (
    <section aria-labelledby="artist-title">
      <header className="screen-header">
        <h1 id="artist-title">Artist</h1>
        <button type="button" onClick={details.reload}>
          Refresh
        </button>
      </header>
      <LoadStatus state={details.state} />
      {details.state.status === "ok" && (
        <div className="artist">
          <h2>{details.state.value.name}</h2>
          {details.state.value.imageUrl !== null && (
            <img src={details.state.value.imageUrl} alt="" className="cover" />
          )}
          <p>
            Average rating:{" "}
            {details.state.value.average === null ? "none" : details.state.value.average.toFixed(1)}
          </p>
          <p>
            Rated: {details.state.value.ratedCount} / {details.state.value.totalCount}
          </p>
          <h3>Top 5</h3>
          <ol aria-label="Top 5 songs">
            {details.state.value.top.map((track) => (
              <li key={track.trackId}>
                {track.trackName} — {track.rating}{" "}
                <button
                  type="button"
                  aria-label={`Play ${track.trackName}`}
                  onClick={() =>
                    void run(
                      () => playTrack(track.trackId),
                      () => `Queued ${track.trackName}`,
                    )
                  }
                >
                  Play
                </button>
              </li>
            ))}
          </ol>
          <ActionStatus action={action} />
        </div>
      )}
    </section>
  );
}
