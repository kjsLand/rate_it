# Implementation progress

This file records the state of the Tauri 2 rewrite (see `SPEC.md`), so that work can continue
after an interruption. Update it in the same commit as the work it describes.

Branch: `tauri-rewrite`.

## Decisions for the open questions

The spec asked for decisions. These were made without a human in the loop, from the spec's own
recommendations. Change them here if the owner decides differently.

| Question | Decision |
| --- | --- |
| Spotify client | Own Rust client (`crates/spotify`), as recommended. |
| Mobile token storage | Desktop: `keyring`. Mobile: a file in the app data folder (`TokenStore` trait), until a keychain plugin is chosen. |
| tauri-specta | Accept the release candidate. |
| Top 10 | Replace with "My Top 10" (your 10 highest-rated songs of the artist). |
| Mobile redirect URI | Try `rateit://callback` first. |
| Users | 5 Development Mode users is enough for version 1. |
| Sync | Export and import of a JSON file. |
| Spotify Client ID | Not a secret with PKCE. Read from the `app_settings` table (Settings screen), else from the build-time variable `RATE_IT_SPOTIFY_CLIENT_ID`. |

## Tooling notes

- Node 24 or later (`.nvmrc`). Oxlint loads the TypeScript anti-slop plugin through Node.
- pnpm through corepack.
- Rust through rustup (`rust-toolchain.toml`).

## Status

Legend: `[x]` done, `[~]` partly done, `[ ]` not started.

- [ ] Phase 0: workspace, lint gates, CI
- [~] Phase 1: `storage` crate and migrations done, with tests. `rate.txt` parse is in `core`.
- [~] Phase 2: `spotify` crate done (client, PKCE, refresh, 204/401/429/timeout tests). Keyring store goes in `src-tauri`.
- [ ] Phase 3: `core` services, Tauri commands, React screens
- [ ] Phase 4: mobile (not testable on this machine without devices)
- [ ] Phase 5: retire Python app (owner action)

## Next steps

- Write the `core` crate (services with fake traits, `rate.txt` parse).

## How to build (for a resumed session)

```bash
export PATH=$HOME/.cargo/bin:$HOME/.local/bin:$PATH
source ~/.nvm/nvm.sh && nvm use 24
./scripts/sqlx-prepare.sh   # after a change to a query or a migration
cargo nextest run --workspace
```
