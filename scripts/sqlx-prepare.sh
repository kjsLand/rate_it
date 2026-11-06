#!/usr/bin/env sh
# Makes a development database from the migrations and writes the sqlx offline data (.sqlx/).
# Run this after each change to a query! macro or a migration, and commit .sqlx/.
set -eu
cd "$(dirname "$0")/.."
mkdir -p target
DB="$(pwd)/target/sqlx-dev.db"
rm -f "$DB" "$DB-wal" "$DB-shm"
export DATABASE_URL="sqlite:$DB"
sqlx database create
sqlx migrate run --source crates/storage/migrations
cargo sqlx prepare --workspace -- --all-targets --all-features
