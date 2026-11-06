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

test("the More button opens the sheet and a choice closes it", async () => {
  fakeCommands({
    now_playing: () => null,
    recent_ratings: () => [],
    account_status: () => account,
  });
  render(<App />);
  const more = screen.getByRole("button", { name: "More" });

  expect(more.getAttribute("aria-expanded")).toBe("false");
  await userEvent.click(more);
  expect(more.getAttribute("aria-expanded")).toBe("true");
  await userEvent.click(screen.getByRole("button", { name: "Settings" }));
  expect(more.getAttribute("aria-expanded")).toBe("false");
  expect(more.dataset["active"]).toBe("true");
  expect(screen.getByRole("main").dataset["screen"]).toBe("settings");
});
