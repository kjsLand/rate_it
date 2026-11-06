import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { expect, test } from "vitest";

import { account } from "../ipc/fixtures";
import { fakeCommands } from "../ipc/testing";
import { Settings } from "./Settings";

test("saves the Client ID and shows the import report", async () => {
  const saved: string[] = [];

  fakeCommands({
    account_status: () => account,
    save_client_id: (args) => {
      saved.push(String(args["clientId"]));

      return { ...account, clientId: String(args["clientId"]) };
    },
    import_legacy_file: () => ({ imported: 5, skipped: 0, failed: 0 }),
  });
  render(<Settings />);
  const input = await screen.findByLabelText("Spotify Client ID");

  await userEvent.clear(input);
  await userEvent.type(input, "newid42");
  await userEvent.click(screen.getByRole("button", { name: "Save" }));
  expect(await screen.findByText("Client ID saved")).toBeTruthy();
  expect(saved).toEqual(["newid42"]);

  await userEvent.click(screen.getByRole("button", { name: "Import rate.txt" }));
  expect(await screen.findByText("Imported 5, skipped 0, failed 0")).toBeTruthy();
});

test("signed-in account shows the name and a Sign out button", async () => {
  fakeCommands({
    account_status: () => ({ ...account, signedIn: true, displayName: "Kevin" }),
    sign_out: () => account,
  });
  render(<Settings />);
  expect(await screen.findByText("Signed in as Kevin")).toBeTruthy();
  await userEvent.click(screen.getByRole("button", { name: "Sign out" }));
  expect(await screen.findByText("Signed out")).toBeTruthy();
});
