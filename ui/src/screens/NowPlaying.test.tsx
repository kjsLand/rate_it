import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { expect, test } from "vitest";

import { playing, ratedTrack } from "../ipc/fixtures";
import { FakeCommandError, calls, fakeCommands } from "../ipc/testing";
import { NowPlaying } from "./NowPlaying";

test("shows Nothing playing for HTTP 204", async () => {
  fakeCommands({ now_playing: () => null, recent_ratings: () => [] });
  render(<NowPlaying />);

  expect(await screen.findByText("Nothing playing")).toBeTruthy();
  expect(await screen.findByText("No ratings yet")).toBeTruthy();
});

test("rates the current song and reloads the log", async () => {
  let rated: number | null = null;

  fakeCommands({
    now_playing: () => ({ ...playing, rating: rated }),
    recent_ratings: () => (rated === null ? [] : [ratedTrack("t1", "Human Nature", rated)]),
    rate_current: (args) => {
      rated = Number(args["rating"]);

      return { ...playing, rating: rated };
    },
  });
  render(<NowPlaying />);
  await screen.findByText("Human Nature");
  await userEvent.click(screen.getByRole("button", { name: "Rate 8" }));

  expect(await screen.findByText("Human Nature by Yot Club — 8")).toBeTruthy();
  expect(calls).toContain("rate_current");
  expect(await screen.findByText("Rating: 8")).toBeTruthy();
});

test("shows a sign-in hint when not signed in", async () => {
  fakeCommands({
    now_playing: () => {
      throw new FakeCommandError("notSignedIn", "not signed in to Spotify");
    },
    recent_ratings: () => [],
  });
  render(<NowPlaying />);

  expect((await screen.findByRole("alert")).textContent).toContain("Open Settings");
});
