import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { expect, test } from "vitest";

import { App } from "./App";
import { account } from "./ipc/fixtures";
import { fakeCommands } from "./ipc/testing";

test("navigates between screens", async () => {
  fakeCommands({
    now_playing: () => null,
    recent_ratings: () => [],
    account_status: () => account,
  });
  render(<App />);
  expect(await screen.findByRole("heading", { name: "Now Playing" })).toBeTruthy();
  await userEvent.click(screen.getByRole("button", { name: "Settings" }));
  expect(await screen.findByRole("heading", { name: "Settings" })).toBeTruthy();
  expect(screen.getByRole("button", { name: "Settings" }).getAttribute("aria-current")).toBe(
    "page",
  );
});
