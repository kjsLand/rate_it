import { useAction } from "../components/async";
import { ProgressBar } from "../components/ProgressBar";
import { ActionStatus } from "../components/Status";
import { useLongTask } from "../components/useLongTask";
import { discoverPlaylist, queueCurrentAlbum, queueMyTopTen } from "../ipc";

/** Discover: discography playlists, the album queue and "My Top 10". */
export function Discover() {
  const task = useLongTask("discover");
  const queue = useAction();

  return (
    <section aria-labelledby="discover-title">
      <header className="screen-header">
        <h1 id="discover-title">Discover</h1>
      </header>
      <div className="cards">
        <article className="discover-card">
          <h2>Full Discography</h2>
          <p>
            Makes a playlist of all songs of the artist that plays now. Find it under the artist's
            name.
          </p>
          <button
            type="button"
            disabled={task.running}
            onClick={() =>
              void task.start(
                (onProgress) => discoverPlaylist("full", onProgress),
                (r) => `Made "${r.name}" with ${r.trackCount} songs`,
              )
            }
          >
            Full Discography
          </button>
        </article>
        <article className="discover-card">
          <h2>Unrated Songs</h2>
          <p>
            Makes a playlist of the songs of the artist that you have not rated. Its name ends with
            "(unrated)".
          </p>
          <button
            type="button"
            disabled={task.running}
            onClick={() =>
              void task.start(
                (onProgress) => discoverPlaylist("unrated", onProgress),
                (r) => `Made "${r.name}" with ${r.trackCount} songs`,
              )
            }
          >
            Unrated Songs
          </button>
        </article>
        <article className="discover-card">
          <h2>Album</h2>
          <p>Adds all songs of the current album to your queue.</p>
          <button
            type="button"
            onClick={() => void queue.run(queueCurrentAlbum, (r) => `Queued ${r.added} songs`)}
          >
            Album
          </button>
        </article>
        <article className="discover-card">
          <h2>My Top 10</h2>
          <p>
            Adds your 10 highest-rated songs of the current artist to your queue. (Spotify removed
            the artist top tracks.)
          </p>
          <button
            type="button"
            onClick={() =>
              void queue.run(
                () => queueMyTopTen(null),
                (r) => `Queued ${r.added} songs`,
              )
            }
          >
            My Top 10
          </button>
        </article>
      </div>
      {task.running && <ProgressBar progress={task.progress} onCancel={task.cancel} />}
      <ActionStatus action={task.action} />
      <ActionStatus action={queue.action} />
    </section>
  );
}
