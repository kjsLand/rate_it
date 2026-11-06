# Rules for AI coding agents

This repository is moving from a Python/Tkinter app to Tauri 2 (React + TypeScript UI, Rust backend).
Read `docs/SPEC.md` before you change code. The Python files at the root are the legacy app.

## Layers

- `ui/` shows screens only. No business logic, no SQL, no tokens.
- Only `ui/src/ipc/` may import `@tauri-apps/api` or `ui/src/bindings.ts`.
- Do not edit `ui/src/bindings.ts`. Change the Rust command and generate the file again.
- Rust dependency direction: `src-tauri` -> `core` -> (`spotify`, `storage`).
  Commands in `src-tauri` are thin: check input, call `core`.
- `core` receives the Spotify client and the repository as traits, so tests can give fakes.

## Lint and tests

- Run `pnpm check` before each commit. It runs both lint gates and all tests.
- Do not turn off a lint without a reason in the code:
  - Rust: `#[expect(clippy::lint_name, reason = "...")]`. A bare `#[allow]` fails the build.
  - TypeScript: an `oxlint-disable-next-line` comment with the reason on the same line.
- No `unwrap()`, `expect()`, `panic!`, `todo!` or `println!` outside tests.
- No `any`, no non-null assertions, no `vi.mock()` (use injected fakes, MSW or `mockIPC`).
- Each new function in `core`, `spotify` or `storage` must have a test.
- Keep TypeScript files under 300 lines and Rust functions under 80 lines.

## Spotify API

Use only the endpoints in the "Spotify client" section of `docs/SPEC.md`.
Spotify removed several endpoints in February 2026; do not use batch album/track endpoints,
`/artists/{id}/top-tracks`, `/users/{id}/playlists` or `/playlists/{id}/tracks`.
