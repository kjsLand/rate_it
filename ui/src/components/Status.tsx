import type { ActionState, AsyncState } from "./async";

/** Shows "Loading" or the error of an `AsyncState`. Shows nothing when the data is there. */
export function LoadStatus<T>({ state }: { state: AsyncState<T> }) {
  if (state.status === "loading") {
    return <p className="status">Loading…</p>;
  }

  if (state.status === "error") {
    return (
      <p className="status error" role="alert">
        {state.message}
      </p>
    );
  }

  return null;
}

/** Shows the progress or the result of a button action. */
export function ActionStatus({ action }: { action: ActionState }) {
  switch (action.status) {
    case "idle":
      return null;
    case "running":
      return <p className="status">Working…</p>;
    case "done":
      return (
        <p className="status ok" role="status">
          {action.message}
        </p>
      );
    case "error":
      return (
        <p className="status error" role="alert">
          {action.message}
        </p>
      );
  }
}
