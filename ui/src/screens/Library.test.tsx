import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { expect, test } from "vitest";

import { artists, ratedTrack } from "../ipc/fixtures";
import { calls, fakeCommands } from "../ipc/testing";
import { Library, parseRating } from "./Library";

test("parseRating accepts whole numbers 1-10 only", () => {
  expect(parseRating("7")).toBe(7);
  expect(parseRating(" 10 ")).toBe(10);
  expect(parseRating("0")).toBeNull();
  expect(parseRating("11")).toBeNull();
  expect(parseRating("7.5")).toBeNull();
  expect(parseRating("")).toBeNull();
});

test("changes a rating in place and shows the history", async () => {
  let current = 9;

  fakeCommands({
    library_artists: () => artists,
    library_tracks: () => [ratedTrack("t1", "Human Nature", current)],
    rerate_track: (args) => {
      current = Number(args["rating"]);

      return ratedTrack("t1", "Human Nature", current);
    },
    track_history: () => [
      { rating: 9, ratedAt: "2026-11-01T00:00:00Z", source: "app" },
      { rating: 6, ratedAt: "2026-11-05T00:00:00Z", source: "app" },
    ],
  });
  render(<Library />);
  await userEvent.click(await screen.findByRole("button", { name: /Yot Club/ }));
  const input = await screen.findByLabelText("New rating");
  const save = screen.getByRole("button", { name: "Save" });

  await userEvent.clear(input);
  await userEvent.type(input, "11");
  expect(save.hasAttribute("disabled")).toBe(true);

  await userEvent.clear(input);
  await userEvent.type(input, "6");
  await userEvent.click(save);
  expect(await screen.findByText("Saved: 6")).toBeTruthy();
  expect(calls).toContain("rerate_track");

  await userEvent.click(screen.getByRole("button", { name: "History" }));
  const history = await screen.findByRole("list", { name: "Rating history" });

  expect(history.textContent).toContain("9 on 2026-11-01");
  expect(history.textContent).toContain("6 on 2026-11-05");
});
