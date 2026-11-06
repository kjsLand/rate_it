import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { expect, test } from "vitest";

import { FakeCommandError, fakeCommands } from "../ipc/testing";
import { Collaborators } from "./Collaborators";
import { Discover } from "./Discover";

test("discover makes an unrated playlist and queues My Top 10", async () => {
  fakeCommands({
    discover_playlist: (args) => ({
      playlistId: "p1",
      name: args["mode"] === "unrated" ? "Joji (unrated)" : "Joji",
      trackCount: 12,
    }),
    queue_my_top_ten: () => ({ added: 10 }),
  });
  render(<Discover />);
  await userEvent.click(screen.getByRole("button", { name: "Unrated Songs" }));
  expect(await screen.findByText('Made "Joji (unrated)" with 12 songs')).toBeTruthy();
  await userEvent.click(screen.getByRole("button", { name: "My Top 10" }));
  expect(await screen.findByText("Queued 10 songs")).toBeTruthy();
});

test("discover shows an error from the album queue", async () => {
  fakeCommands({
    queue_current_album: () => {
      throw new FakeCommandError("nothingPlaying", "Nothing playing");
    },
  });
  render(<Discover />);
  await userEvent.click(screen.getByRole("button", { name: "Album" }));
  expect((await screen.findByRole("alert")).textContent).toBe("Nothing playing");
});

test("collaborators lists the artists found", async () => {
  fakeCommands({
    find_collaborators: () => [
      { id: "b1", name: "Clairo", trackCount: 2 },
      { id: "c1", name: "Rex", trackCount: 1 },
    ],
  });
  render(<Collaborators />);
  await userEvent.click(screen.getByRole("button", { name: "Find collaborators" }));
  expect(await screen.findByText("2 artists found")).toBeTruthy();
  expect(screen.getByText(/Clairo \(2 songs\)/)).toBeTruthy();
  expect(screen.getByText(/Rex \(1 song\)/)).toBeTruthy();
});
