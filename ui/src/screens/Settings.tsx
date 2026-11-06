import { useState } from "react";

import { useAction, useAsync } from "../components/async";
import { Icon } from "../components/Icon";
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
import "./Settings.css";

function reportText(report: ImportReportDto | null): string {
  if (report === null) return "No file selected";

  return `Imported ${report.imported}, skipped ${report.skipped}, failed ${report.failed}`;
}

/** Sign-in and Settings ("The Fine Print"): Client ID, sign in and out, import, export, theme. */
export function Settings() {
  const status = useAsync(accountStatus, undefined);
  const { action, run } = useAction();

  return (
    <section aria-labelledby="settings-title" className="fine-print">
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
      <div className="panel">
        <h2>Ratings</h2>
        <div className="button-row">
          <button type="button" onClick={() => void run(importLegacyFile, reportText)}>
            <Icon name="fileIn" />
            Import rate.txt
          </button>
          <button type="button" onClick={() => void run(importRatingsFile, reportText)}>
            <Icon name="fileIn" />
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
            <Icon name="fileOut" />
            Export
          </button>
        </div>
        <ActionStatus action={action} />
      </div>
      <div className="panel">
        <h2>Appearance</h2>
        <ThemeSwitch />
      </div>
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
    <div className="panel account">
      <h2>Spotify account</h2>
      <p className="pill" data-on={account.signedIn}>
        {account.signedIn && <Icon name="check" />}
        {account.signedIn
          ? `Signed in${account.displayName === null ? "" : ` as ${account.displayName}`}`
          : "Not signed in"}
      </p>
      <form
        className="client-id"
        onSubmit={(e) => {
          e.preventDefault();
          onAction(
            () => saveClientId(clientId),
            () => "Client ID saved",
          );
        }}
      >
        <label>
          <span className="field-label">Spotify Client ID</span>
          <input value={clientId} required onChange={(e) => setClientId(e.target.value.trim())} />
        </label>
        <button type="submit" disabled={busy || clientId.length === 0}>
          Save
        </button>
      </form>
      <p className="hint">
        Make an app in the Spotify developer dashboard and add the redirect URI{" "}
        <code>http://127.0.0.1/callback</code> (desktop) or <code>rateit://callback</code> (mobile).
      </p>
      {account.signedIn ? (
        <button type="button" disabled={busy} onClick={() => onAction(signOut, () => "Signed out")}>
          Sign out
        </button>
      ) : (
        <button
          type="button"
          className="primary"
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
