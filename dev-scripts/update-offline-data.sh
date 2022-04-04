#! /bin/bash -eu
export DATABASE_URL=sqlite:update-offline.db

# Make sure cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "cargo is not installed, or is not in the PATH"
    echo "If cargo is not installed, set it up using rustup: https://rustup.rs"
    exit 1
fi

# Make sure sqlx is installed
if ! command -v sqlx &> /dev/null; then
    echo "command not found: sqlx"
    echo "Install sqlx with this command: cargo install sqlx-cli"
    echo "If sqlx-cli is installed, make sure the installation directory is in your PATH."
    exit 1
fi

echo Removing pre-existing temp db
rm -f update-offline.db

echo Re-creating temp db
sqlx database create

echo Running migrations on temp db
sqlx migrate run

echo Creating offline schema
cargo sqlx prepare

echo Cleaning up temp db
rm -f update-offline.db

echo "Finished!"