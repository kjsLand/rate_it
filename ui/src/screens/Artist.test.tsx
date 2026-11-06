import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { expect, test } from "vitest";

import { ratedTrack } from "../ipc/fixtures";
import { fakeCommands } from "../ipc/testing";
import { Artist } from "./Artist";

test("each Play button sends its own track ID", async () => {
  const played: string[] = [];

  fakeCommands({
    artist_details: () => ({
      id: "a1",
      name: "Yot Club",
      imageUrl: null,
      average: 9.5,
      ratedCount: 2,
      totalCount: 30,
      top: [ratedTrack("t1", "Human Nature", 10), ratedTrack("t2", "Rare Pleasure", 9)],
    }),
    play_track: (args) => {
      played.push(String(args["trackId"]));

      return null;
    },
  });
  render(<Artist />);

  expect(await screen.findByText("Rated: 2 / 30")).toBeTruthy();
  expect(screen.getByText("Average rating: 9.5")).toBeTruthy();
  await userEvent.click(screen.getByRole("button", { name: "Play Rare Pleasure" }));
  await userEvent.click(screen.getByRole("button", { name: "Play Human Nature" }));
  expect(played).toEqual(["t2", "t1"]);
});
