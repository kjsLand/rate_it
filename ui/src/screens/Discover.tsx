import type { ReactNode } from "react";

import { useAction } from "../components/async";
import { ProgressBar } from "../components/ProgressBar";
import { ActionStatus } from "../components/Status";
import { useLongTask } from "../components/useLongTask";
import { discoverPlaylist, queueCurrentAlbum, queueMyTopTen } from "../ipc";
import "./Discover.css";

type Tone = "umber" | "rose" | "brick" | "night";

/** One record sleeve in the bin. The record slides out on hover and on keyboard focus. */
function Bin({
  tone,
  title,
  text,
  children,
}: {
  tone: Tone;
  title: string;
  text: string;
  children: ReactNode;
}) {
  return (
    <article className="bin" data-tone={tone}>
      <span className="bin-disc" aria-hidden="true" />
      <div className="bin-sleeve">
        <h2>{title}</h2>
        <p>{text}</p>
        {children}
      </div>
    </article>
  );
}

/** Discover ("The Record Bins"): discography playlists, the album queue and "My Top 10". */
export function Discover() {
  const task = useLongTask("discover");
  const queue = useAction();

  return (
    <section aria-labelledby="discover-title">
      <header className="screen-header">
        <h1 id="discover-title">Discover</h1>
      </header>
      <div className="bins">
        <Bin
          tone="umber"
          title="Full Discography"
          text="Makes a playlist of all songs of the artist that plays now. Find it under the artist's name."
        >
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
        </Bin>
        <Bin
          tone="rose"
          title="Unrated Songs"
          text='Makes a playlist of the songs of the artist that you have not rated. Its name ends with "(unrated)".'
        >
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
        </Bin>
        <Bin tone="brick" title="Album" text="Adds all songs of the current album to your queue.">
          <button
            type="button"
            onClick={() => void queue.run(queueCurrentAlbum, (r) => `Queued ${r.added} songs`)}
          >
            Album
          </button>
        </Bin>
        <Bin
          tone="night"
          title="My Top 10"
          text="Adds your 10 highest-rated songs of the current artist to your queue. (Spotify removed the artist top tracks.)"
        >
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
        </Bin>
      </div>
      {task.running && <ProgressBar progress={task.progress} onCancel={task.cancel} />}
      <ActionStatus action={task.action} />
      <ActionStatus action={queue.action} />
    </section>
  );
}
