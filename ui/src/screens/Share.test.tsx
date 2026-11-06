import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { expect, test } from "vitest";

import { fakeCommands } from "../ipc/testing";
import { Share } from "./Share";

test("shows the song card and the album card", async () => {
  fakeCommands({
    song_card: () => ({
      name: "Shotput",
      artistName: "Still Woozy",
      albumName: "If This Isn't Nice",
      imageUrl: null,
      rating: 8,
    }),
    album_card: () => ({
      name: "If This Isn't Nice",
      artistName: "Still Woozy",
      imageUrl: null,
      average: 8,
      tracks: [
        { trackId: "t1", name: "Shotput", rating: 8 },
        { trackId: "t2", name: "Window", rating: null },
      ],
    }),
  });
  render(<Share />);
  await userEvent.click(screen.getByRole("button", { name: "Song card" }));
  const card = await screen.findByRole("figure", { name: "Rating card" });

  expect(card.textContent).toContain("Rating: 8 / 10");

  await userEvent.click(screen.getByRole("button", { name: "Album card" }));
  expect(await screen.findByText("Overall rating: 8.0")).toBeTruthy();
  expect(screen.getByText("Window: Not rated")).toBeTruthy();
});
