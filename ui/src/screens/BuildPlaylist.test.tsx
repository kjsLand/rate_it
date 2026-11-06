import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { expect, test } from "vitest";

import { artists } from "../ipc/fixtures";
import { fakeCommands } from "../ipc/testing";
import type { FakeResult } from "../ipc/testing";
import { BuildPlaylist } from "./BuildPlaylist";

test("name is trimmed and required, and the selections are sent", async () => {
  let request: FakeResult = null;

  fakeCommands({
    library_artists: () => artists,
    build_playlist: (args) => {
      request = args["request"] ?? null;

      return { playlistId: "p1", name: "Best", trackCount: 3 };
    },
  });
  render(<BuildPlaylist />);
  const make = screen.getByRole("button", { name: "Make playlist" });

  await userEvent.click(await screen.findByRole("checkbox", { name: "Joji" }));
  await userEvent.type(screen.getByLabelText("Playlist name"), "   ");
  expect(make.hasAttribute("disabled")).toBe(true);

  await userEvent.type(screen.getByLabelText("Playlist name"), "Best  ");
  await userEvent.selectOptions(screen.getByLabelText("Minimum rating for Joji"), "9");
  await userEvent.click(make);

  expect(await screen.findByText('Made "Best" with 3 songs')).toBeTruthy();
  expect(request).toEqual({ name: "Best", selections: [{ artistId: "a2", minRating: 9 }] });
});
