import type { Progress } from "../ipc";
import "./ProgressBar.css";

/** The progress of a long command, with a Cancel button. */
export function ProgressBar({
  progress,
  onCancel,
}: {
  progress: Progress | null;
  onCancel: () => void;
}) {
  const done = progress?.done ?? 0;
  const total = progress?.total ?? 0;

  return (
    <div className="progress">
      <span className="progress-disc" aria-hidden="true" />
      <progress max={total || 1} value={done} aria-label="Progress" />
      <span className="progress-label">
        {total > 0 ? `${done} / ${total} albums` : "Starting…"}
      </span>
      <button type="button" className="quiet" onClick={onCancel}>
        Cancel
      </button>
    </div>
  );
}
