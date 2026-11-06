# Implementation progress

This file records the state of the Tauri 2 rewrite (see `SPEC.md`), so that work can continue
after an interruption. Update it in the same commit as the work it describes.

Branch: `tauri-rewrite`.

## Decisions for the open questions

The spec asked for decisions. These were made without a human in the loop, from the spec's own
recommendations. Change them here if the owner decides differently.

| Question             | Decision                                                                                                                                     |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| Spotify client       | Own Rust client (`crates/spotify`), as recommended.                                                                                          |
| Mobile token storage | Desktop: `keyring`. Mobile: a file in the app data folder (`TokenStore` trait), until a keychain plugin is chosen.                           |
| tauri-specta         | Accept the release candidate.                                                                                                                |
| Top 10               | Replace with "My Top 10" (your 10 highest-rated songs of the artist).                                                                        |
| Mobile redirect URI  | Try `rateit://callback` first.                                                                                                               |
| Users                | 5 Development Mode users is enough for version 1.                                                                                            |
| Sync                 | Export and import of a JSON file.                                                                                                            |
| Spotify Client ID    | Not a secret with PKCE. Read from the `app_settings` table (Settings screen), else from the build-time variable `RATE_IT_SPOTIFY_CLIENT_ID`. |

## Tooling notes

- Node 24 or later (`.nvmrc`). Oxlint loads the TypeScript anti-slop plugin through Node.
- pnpm through corepack.
- Rust through rustup (`rust-toolchain.toml`).

## Status

Legend: `[x]` done, `[~]` partly done, `[ ]` not started.

- [~] Phase 0: workspace, both lint gates and `.github/workflows/ci.yml` done. Checked: a `vi.mock()` file fails the TypeScript gate; an `unwrap()` fails the Rust gate. Owner actions: register the redirect URIs `http://127.0.0.1/callback` and `rateit://callback` in the Spotify dashboard; the CI has not run on GitHub yet.
- [x] Phase 1: `storage` crate, migrations, `rate.txt` import (`core::legacy` + `App::import_legacy`), with tests.
- [~] Phase 2: `spotify` crate (client, PKCE, refresh, 204/401/429/timeout tests) and keyring store (`src-tauri/src/token_store.rs`) done. Not verified: a real sign-in, and "more than 1 hour without a new sign-in" (needs a Spotify Client ID).
- [~] Phase 3: all screens of the Feature parity table are built (`ui/src/screens/`), with Vitest + mockIPC tests. `pnpm check` passes. `pnpm tauri dev` starts the app on macOS and makes the database (both migrations applied). Not done: WebDriver end-to-end tests (`tauri-driver` runs on Linux and Windows only; they also need a setting to point the client at a fake Spotify server); a manual run against real Spotify.
- [~] Phase 4: code paths exist: `rateit://callback` deep link sign-in (`cfg(mobile)`), file token store, JSON export/import. Not done: `tauri android init` / `tauri ios init`, native share sheet (`tauri-plugin-sharesheet`), device tests.
- [~] UI overhaul (`docs/UI-SPEC.md`): step 1 done: palette (`ui/src/theme/palette.ts`), contrast test, `ui/src/styles/` tokens and base, self-hosted fonts, old colors removed. Next: step 2 (shell and shared components).
- [ ] Phase 5: retire Python app (owner action)

## Next steps

1. Owner: enter the Spotify Client ID in Settings (or build with `RATE_IT_SPOTIFY_CLIENT_ID`), sign in, and try each screen.
2. Add WebDriver end-to-end tests on Linux: add a debug-only setting for the Spotify base URLs, run a fake Spotify server, drive the app with `tauri-driver`.
3. Phase 4: `pnpm tauri android init` and `pnpm tauri ios init`, the share sheet, a device test of the custom scheme.

## How to build (for a resumed session)

```bash
export PATH=$HOME/.cargo/bin:$HOME/.local/bin:$PATH
source ~/.nvm/nvm.sh && nvm use 24
./scripts/sqlx-prepare.sh   # after a change to a query or a migration
cargo nextest run --workspace
```
