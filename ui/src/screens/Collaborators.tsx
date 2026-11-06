import "./Collaborators.css";
import { useState } from "react";

import { useAction } from "../components/async";
import { ProgressBar } from "../components/ProgressBar";
import { ActionStatus } from "../components/Status";
import { useLongTask } from "../components/useLongTask";
import { findCollaborators, queueMyTopTen } from "../ipc";
import type { Collaborator } from "../ipc";

/** Collaborators: the artists on the songs of the current artist. */
export function Collaborators() {
  const [people, setPeople] = useState<Collaborator[]>([]);
  const task = useLongTask("collaborators");
  const queue = useAction();

  return (
    <section aria-labelledby="collaborators-title">
      <header className="screen-header">
        <h1 id="collaborators-title">Collaborators</h1>
      </header>
      <p>Find new artists through the features on the songs of the artist that plays now.</p>
      <div className="button-row">
        <button
          type="button"
          disabled={task.running}
          onClick={() =>
            void task.start(
              (onProgress) => findCollaborators(null, onProgress),
              (found) => {
                setPeople(found);

                return `${found.length} artists found`;
              },
            )
          }
        >
          Find collaborators
        </button>
        <button type="button" onClick={() => setPeople([])}>
          Clear
        </button>
      </div>
      {task.running && <ProgressBar progress={task.progress} onCancel={task.cancel} />}
      <ActionStatus action={task.action} />
      <ul className="collaborators" aria-label="Collaborators">
        {people.map((person) => (
          <li key={person.id}>
            {person.name} ({person.trackCount} {person.trackCount === 1 ? "song" : "songs"}){" "}
            <button
              type="button"
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
