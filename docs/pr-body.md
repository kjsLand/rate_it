## What

Implements `docs/SPEC.md`: the Tauri 2 rewrite of rate_it, next to the Python app (which stays until Phase 5).

- `crates/storage`: SQLite through `sqlx` (offline data in `.sqlx/`), the spec schema plus `0002` for settings and the album cache. Ratings are append-only; `current_ratings` view gives the last one.
- `crates/spotify`: own `reqwest` client (rustls, 10 s timeout, refresh once on 401, `Retry-After` on 429 with 3 retries, 4 parallel requests, one `SpotifyError`), PKCE with `state` check. Only post-February-2026 endpoints.
- `crates/core`: services behind `SpotifyApi` and `RatingsRepository` traits; `rate.txt` import; JSON export/import; "My Top 10" replaces Top 10.
- `src-tauri`: thin typed commands (tauri-specta), keyring token store, desktop loopback sign-in, mobile deep-link path, progress channels and cancel. Capability gives the WebView `core:default` only; CSP with `script-src 'self'`.
- `ui/`: React screens for every row of the Feature parity table. Only `ui/src/ipc/` imports `@tauri-apps/api` or `bindings.ts`.
- Lint gates wired into `pnpm check`; anti-slop vendored in `tools/anti-slop/`; CI in `.github/workflows/ci.yml`.

## Why

The spec's migration plan: one Rust backend and one React UI for desktop and mobile, with strict lint and tests.

## Notes for the reviewer

- Decisions for the spec's open questions are in `docs/PROGRESS.md` (the spec's recommendations were taken). `docs/SPEC.md` is unchanged.
- Spotify now allows at most 10 albums per page for `GET /artists/{id}/albums`; the client pages through them.
- `cargo-deny`: allowed `CDLA-Permissive-2.0`, `BSL-1.0`, `CC0-1.0` (Tauri / rustls transitive crates), and `unmaintained = "workspace"`.
- Node 24 is required (Oxlint loads the TypeScript anti-slop plugin).
- After a command change run `pnpm bindings`; the test `bindings_are_current` fails otherwise.

## Testing

- `pnpm check` passes locally: 68 Rust tests (wiremock for 204/401/429/timeout/bad body; in-memory SQLite) and 14 Vitest tests with `mockIPC`.
- A `vi.mock()` probe fails the TypeScript gate; an `unwrap()` probe fails the Rust gate.
- `pnpm tauri dev` starts on macOS and creates the database with both migrations.
- Not tested: a real Spotify sign-in, WebDriver end-to-end tests, mobile builds. CI has not run yet.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
