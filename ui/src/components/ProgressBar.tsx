import type { Progress } from "../ipc";

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
      <progress max={total || 1} value={done} aria-label="Progress" />
      <span>{total > 0 ? `${done} / ${total} albums` : "Starting…"}</span>
      <button type="button" onClick={onCancel}>
        Cancel
      </button>
    </div>
  );
}
