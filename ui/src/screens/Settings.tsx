import { useState } from "react";

import { useAction, useAsync } from "../components/async";
import { ActionStatus, LoadStatus } from "../components/Status";
import { ThemeSwitch } from "../components/ThemeSwitch";
import {
  accountStatus,
  exportRatingsFile,
  importLegacyFile,
  importRatingsFile,
  saveClientId,
  signIn,
  signOut,
} from "../ipc";
import type { AccountStatus, ImportReportDto } from "../ipc";

function reportText(report: ImportReportDto | null): string {
  if (report === null) return "No file selected";

  return `Imported ${report.imported}, skipped ${report.skipped}, failed ${report.failed}`;
}

/** Sign-in and Settings: Client ID, sign in and out, import and export. */
export function Settings() {
  const status = useAsync(accountStatus, undefined);
  const { action, run } = useAction();

  return (
    <section aria-labelledby="settings-title">
      <header className="screen-header">
        <h1 id="settings-title">Settings</h1>
      </header>
      <LoadStatus state={status.state} />
      {status.state.status === "ok" && (
        <Account
          account={status.state.value}
          busy={action.status === "running"}
          onAction={(task, done) => void run(task, done).then(status.reload)}
        />
      )}
      <h2>Ratings</h2>
      <div className="button-row">
        <button type="button" onClick={() => void run(importLegacyFile, reportText)}>
          Import rate.txt
        </button>
        <button type="button" onClick={() => void run(importRatingsFile, reportText)}>
          Import
        </button>
        <button
          type="button"
          onClick={() =>
            void run(exportRatingsFile, (path) =>
              path === null ? "Export cancelled" : `Exported to ${path}`,
            )
          }
        >
          Export
        </button>
      </div>
      <ActionStatus action={action} />
      <h2>Appearance</h2>
      <ThemeSwitch />
    </section>
  );
}

function Account({
  account,
  busy,
  onAction,
}: {
  account: AccountStatus;
  busy: boolean;
  onAction: (task: () => Promise<AccountStatus>, done: (value: AccountStatus) => string) => void;
}) {
  const [clientId, setClientId] = useState(account.clientId ?? "");

  return (
    <div className="account">
      <h2>Spotify account</h2>
      <p>
        {account.signedIn
          ? `Signed in${account.displayName === null ? "" : ` as ${account.displayName}`}`
          : "Not signed in"}
      </p>
      <form
        className="button-row"
        onSubmit={(e) => {
          e.preventDefault();
          onAction(
            () => saveClientId(clientId),
            () => "Client ID saved",
          );
        }}
      >
        <label>
          Spotify Client ID
          <input value={clientId} required onChange={(e) => setClientId(e.target.value.trim())} />
        </label>
        <button type="submit" disabled={busy || clientId.length === 0}>
          Save
        </button>
      </form>
      <p className="hint">
        Make an app in the Spotify developer dashboard and add the redirect URI
        http://127.0.0.1/callback (desktop) or rateit://callback (mobile).
      </p>
      {account.signedIn ? (
        <button type="button" disabled={busy} onClick={() => onAction(signOut, () => "Signed out")}>
          Sign out
        </button>
      ) : (
        <button
          type="button"
          disabled={busy || account.clientId === null}
          onClick={() =>
            onAction(signIn, (s) =>
              s.signedIn ? "Signed in" : "Finish the sign-in in the browser",
            )
          }
        >
          Sign in with Spotify
        </button>
      )}
    </div>
  );
}
