import { useState } from "react";

import { cancelTask } from "../ipc";
import type { Progress } from "../ipc";
import { useAction } from "./async";

/** A long command with progress and a Cancel button (Collaborators, Discover). */
export function useLongTask(task: "collaborators" | "discover") {
  const [progress, setProgress] = useState<Progress | null>(null);
  const { action, run } = useAction();

  async function start<T>(
    command: (onProgress: (p: Progress) => void) => Promise<T>,
    done: (value: T) => string,
  ) {
    setProgress(null);
    await run(() => command(setProgress), done);
  }

  function cancel() {
    void cancelTask(task);
  }

  return { progress, action, start, cancel, running: action.status === "running" };
}
