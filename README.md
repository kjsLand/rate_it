# rate_it
This is an application that uses Spotify's API to allow users to store ratings of songs and discover new songs.

## Tauri 2 version (in progress)

The new app is in `src-tauri/`, `crates/` and `ui/`. See `docs/SPEC.md` and `docs/PROGRESS.md`.
The Python files at the root are the legacy app; you do not need the Python `.venv` for the new one.

### 1. Install the tools (once)

The steps are for macOS.

**Xcode Command Line Tools** (C compiler and linker for Rust):

```bash
xcode-select --install
```

**Rust** through rustup. The exact version (1.99.1) is pinned in `rust-toolchain.toml`;
rustup downloads it the first time you run `cargo` in this folder.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

The installer adds `~/.cargo/bin` to your PATH in your shell startup files, but only for
**new** terminals. Open a new terminal, or run this in the current one:

```bash
source "$HOME/.cargo/env"
```

Check it with `cargo --version`. If you see `failed to run 'cargo metadata' ... No such file or directory`
from `pnpm tauri dev`, this step is the problem.

**Node 24 or later** (`.nvmrc` says 24). With nvm:

```bash
nvm install 24
```

```bash
nvm alias default 24
```

Open a new terminal (or run `nvm use` in this folder) and check it with `node --version`.

**pnpm** through corepack, which comes with Node. It uses the version in `package.json`:

```bash
corepack enable
```

### 2. Install the project dependencies

```bash
pnpm install
```

### 3. Set up a Spotify app (once)

1. Go to the [Spotify Developer Dashboard](https://developer.spotify.com/dashboard) and create an app (select "Web API").
2. In the app settings, add these Redirect URIs:
   - `http://127.0.0.1/callback` (desktop sign-in)
   - `rateit://callback` (mobile sign-in)
3. In "User Management", add the Spotify account email of each person who will use the app
   (Development Mode allows 5 users).
4. Copy the **Client ID**. It is not a secret (the app uses PKCE, so there is no client secret).

Give the Client ID to the app in one of two ways:

- Start the app, open **Settings**, paste the Client ID and save. It is kept in the app's database.
- Or set it at build time:

  ```bash
  RATE_IT_SPOTIFY_CLIENT_ID=your_client_id pnpm tauri dev
  ```

### 4. Run the app

```bash
pnpm tauri dev
```

The first run compiles all Rust dependencies and takes several minutes. Later runs are fast.

### Development tools (only for `pnpm check`)

`pnpm check` runs both lint gates and all tests. It needs some extra Cargo tools:

```bash
cargo install cargo-nextest cargo-deny cargo-machete --locked
```

```bash
pnpm check
```

If you change a SQL query or a migration, you also need `sqlx-cli`, then run the prepare script
and commit the `.sqlx/` folder:

```bash
cargo install sqlx-cli --no-default-features --features sqlite --locked
```

```bash
./scripts/sqlx-prepare.sh
```

If you change a Tauri command, generate `ui/src/bindings.ts` again with `pnpm bindings`.
