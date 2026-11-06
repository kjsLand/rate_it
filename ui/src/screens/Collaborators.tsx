import { useState } from "react";

import { useAction } from "../components/async";
import { Icon } from "../components/Icon";
import { ProgressBar } from "../components/ProgressBar";
import { ActionStatus } from "../components/Status";
import { useLongTask } from "../components/useLongTask";
import { findCollaborators, queueMyTopTen } from "../ipc";
import type { Collaborator } from "../ipc";
import "./Collaborators.css";

const songs = (count: number) => `${count} ${count === 1 ? "song" : "songs"}`;

/** Collaborators ("The Credits"): the artists on the songs of the current artist. */
export function Collaborators() {
  const [people, setPeople] = useState<Collaborator[]>([]);
  const task = useLongTask("collaborators");
  const queue = useAction();

  function find() {
    void task.start(
      (onProgress) => findCollaborators(null, onProgress),
      (found) => {
        setPeople(found);

        return `${found.length} artists found`;
      },
    );
  }

  return (
    <section aria-labelledby="collaborators-title">
      <header className="screen-header">
        <h1 id="collaborators-title">Collaborators</h1>
      </header>
      <p className="credits-lead">
        Find new artists through the features on the songs of the artist that plays now.
      </p>
      <div className="button-row">
        <button type="button" className="primary" disabled={task.running} onClick={find}>
          Find collaborators
        </button>
        <button type="button" className="quiet" onClick={() => setPeople([])}>
          Clear
        </button>
      </div>
      {task.running && <ProgressBar progress={task.progress} onCancel={task.cancel} />}
      <ActionStatus action={task.action} />
      {people.length === 0 && !task.running && (
        <div className="credits-empty" aria-hidden="true">
          <Icon name="users" />
          <span>The credits show here.</span>
        </div>
      )}
      <ul className="credits" aria-label="Collaborators">
        {people.map((person) => (
          <li key={person.id}>
            <span className="visually-hidden">
              {person.name} ({songs(person.trackCount)})
            </span>
            <span className="credit-name" aria-hidden="true">
              {person.name}
            </span>
            <span className="leader" aria-hidden="true" />
            <span className="credit-count" aria-hidden="true">
              {songs(person.trackCount)}
            </span>
            <button
              type="button"
              className="quiet"
              onClick={() =>
                void queue.run(
                  () => queueMyTopTen(person.id),
                  (r) => `Queued ${r.added} songs of ${person.name}`,
                )
              }
            >
              My Top 10
            </button>
          </li>
        ))}
      </ul>
      <ActionStatus action={queue.action} />
    </section>
  );
}
