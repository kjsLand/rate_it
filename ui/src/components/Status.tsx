import type { ActionState, AsyncState } from "./async";
import { Icon } from "./Icon";

function Working({ text }: { text: string }) {
  return (
    <p className="status">
      <span className="spinner" aria-hidden="true" />
      {text}
    </p>
  );
}

function Failed({ message }: { message: string }) {
  return (
    <p className="status error" role="alert">
      <Icon name="alert" />
      {message}
    </p>
  );
}

/** Shows "Loading" or the error of an `AsyncState`. Shows nothing when the data is there. */
export function LoadStatus<T>({ state }: { state: AsyncState<T> }) {
  if (state.status === "loading") {
    return <Working text="Loading…" />;
  }

  if (state.status === "error") {
    return <Failed message={state.message} />;
  }

  return null;
}

/** Shows the progress or the result of a button action. */
export function ActionStatus({ action }: { action: ActionState }) {
  switch (action.status) {
    case "idle":
      return null;
    case "running":
      return <Working text="Working…" />;
    case "done":
      return (
        <p className="status ok" role="status">
          <Icon name="check" />
          {action.message}
        </p>
      );
    case "error":
      return <Failed message={action.message} />;
  }
}
