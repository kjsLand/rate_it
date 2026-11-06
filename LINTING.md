# Linting

Two lint gates. `pnpm check` runs both, then the tests. See `docs/SPEC.md` ("Code quality") for the reasons.

## TypeScript UI (`ui/`)

| File | Tool |
| --- | --- |
| `ui/.oxlintrc.json` | Oxlint + anti-slop rules |
| `ui/tsconfig.json` | `tsc --noEmit`, strict |
| `.dependency-cruiser.cjs` | Layer rules (only `ui/src/ipc/` calls Rust) |
| `knip.json` | Unused files, exports, dependencies |
| `.prettierrc.json`, `.prettierignore` | Format |

Set up (once the `ui/` app exists):

```bash
pnpm add -D -w oxlint @oxlint/plugins typescript dependency-cruiser knip prettier vitest
```

Then vendor anti-slop into `tools/anti-slop/` (see `tools/anti-slop/README.md`).
`@oxlint/plugins` must match the installed Oxlint version.

## Rust backend (`src-tauri/`, `crates/`)

| File | Tool |
| --- | --- |
| `Cargo.toml` (`[workspace.lints]`) | rustc + Clippy lint levels |
| `clippy.toml` | Clippy thresholds |
| `rustfmt.toml` | Format |
| `deny.toml` | cargo-deny: advisories, licenses, sources |
| `rust-toolchain.toml` | Same Rust, Clippy and rustfmt for everyone |

Each crate's `Cargo.toml` must contain:

```toml
[lints]
workspace = true
```

Set up:

```bash
cargo install --locked cargo-deny cargo-machete cargo-nextest
```

## Commands

```bash
pnpm lint:ts
pnpm lint:rust
pnpm check
```
