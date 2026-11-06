import { useCallback, useEffect, useState } from "react";

import { IpcError } from "../ipc";
import type { ErrorKind } from "../ipc";

/** The state of a command that loads screen data. */
export type AsyncState<T> =
  | { status: "loading" }
  | { status: "ok"; value: T }
  | { status: "error"; message: string; kind: ErrorKind | null };

/** The text to show for an error, with a hint for the sign-in errors. */
export function errorText(error: Error | IpcError | null | undefined | string): string {
  if (error instanceof IpcError) {
    return error.kind === "notSignedIn"
      ? "Not signed in. Open Settings and sign in to Spotify."
      : error.message;
  }

  if (error instanceof Error) {
    return error.message;
  }

  return error ?? "Unknown error";
}

type ErrorState = Extract<AsyncState<never>, { status: "error" }>;

function errorState(error: Error | string): ErrorState {
  return {
    status: "error",
    message: errorText(error),
    kind: error instanceof IpcError ? error.kind : null,
  };
}

/** A load request: the argument, and a new object for each reload. */
type LoadRequest<A> = { readonly arg: A };

/** The result of `useAsync`. */
export type AsyncLoad<T> = { state: AsyncState<T>; reload: () => void };

/**
 * Runs `load(arg)` when the screen opens, when `arg` changes and when `reload` is called.
 * A reload keeps the old data on screen until the new data is there.
 * `load` must be a stable function (a module function such as `nowPlaying`).
 */
export function useAsync<T, A = undefined>(load: (arg: A) => Promise<T>, arg: A): AsyncLoad<T> {
  const [state, setState] = useState<AsyncState<T>>({ status: "loading" });
  const [request, setRequest] = useState<LoadRequest<A>>({ arg });

  if (request.arg !== arg) {
    setRequest({ arg });
    setState({ status: "loading" });
  }

  useEffect(() => {
    const abort = new AbortController();

    load(request.arg).then(
      (value) => {
        if (!abort.signal.aborted) setState({ status: "ok", value });
      },
      (error: Error) => {
        if (!abort.signal.aborted) setState(errorState(error));
      },
    );

    return () => abort.abort();
  }, [load, request]);

  const reload = useCallback(() => setRequest((current) => ({ arg: current.arg })), []);

  return { state, reload };
}

/** The state of a button action. */
export type ActionState =
  | { status: "idle" }
  | { status: "running" }
  | { status: "done"; message: string }
  | { status: "error"; message: string };

export type ActionRunner = {
  action: ActionState;
  run: <T>(task: () => Promise<T>, done: (value: T) => string) => Promise<void>;
};

/** Runs one action at a time and keeps its result message. */
export function useAction(): ActionRunner {
  const [action, setAction] = useState<ActionState>({ status: "idle" });

  const run = useCallback(async <T>(task: () => Promise<T>, done: (value: T) => string) => {
    setAction({ status: "running" });

    try {
      const value = await task();

      setAction({ status: "done", message: done(value) });
    } catch (error) {
      // SAFETY: commands reject with an Error (IpcError or a JavaScript failure).
      setAction({ status: "error", message: errorText(error as Error) });
    }
  }, []);

  return { action, run };
}
