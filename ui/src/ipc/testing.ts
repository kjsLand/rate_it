/**
 * Test helpers: fake command results with `mockIPC` (no module mocking).
 * Only tests import this file.
 */
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";

import type { CommandError } from "../bindings";

/** A fake command: it gets the command arguments and returns the result. */
export type FakeCommand = (args: FakeArgs) => FakeResult | Promise<FakeResult>;

/** The arguments of a command, by name. */
type FakeArgs = Readonly<Record<string, FakeResult>>;

/** A JSON value that a fake command returns. */
export type FakeResult =
  string | number | boolean | null | readonly FakeResult[] | { readonly [key: string]: FakeResult };

/** Thrown by a fake command to make the command fail. */
export class FakeCommandError {
  readonly kind: CommandError["kind"];
  readonly message: string;

  constructor(kind: CommandError["kind"], message: string) {
    this.kind = kind;
    this.message = message;
  }
}

/** The names of the commands that the fakes received, in order. */
export const calls: string[] = [];

/** Installs fake commands. A command without a fake fails the test. */
export function fakeCommands(fakes: Readonly<Record<string, FakeCommand>>): void {
  calls.length = 0;
  mockIPC(async (cmd, args) => {
    calls.push(cmd);
    const fake = fakes[cmd];

    if (fake === undefined) {
      throw new Error(`no fake for command ${cmd}`);
    }

    try {
      // SAFETY: the Tauri mock gives the JSON arguments of the command.
      return await fake((args ?? {}) as FakeArgs);
    } catch (error) {
      if (error instanceof FakeCommandError) {
        throw { kind: error.kind, message: error.message };
      }

      throw error;
    }
  });
}

/** Removes the fakes after each test. */
export function resetFakes(): void {
  clearMocks();
}
