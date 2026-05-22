#!/usr/bin/env bash
# Bundle Vinaria into a macOS .app + .dmg using Tauri's bundler.
# Self-signed with an ad-hoc identity so it runs locally without paying
# Apple Developer fees. Distributed users will see "unidentified developer"
# on first launch and need to right-click → Open once.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

# Build the frontend first so Tauri can embed it.
echo "==> Building Astro frontend"
(cd ui && npm install --silent && npm run build)

echo "==> Bundling .app and .dmg with tauri bundle"
# Use cargo's bundle target; Tauri 2 ships a `cargo tauri` subcommand but
# we don't require it as a dep — instead invoke the underlying build.
cargo build --release -p vinaria
mkdir -p target/release/bundle

# Tauri's bundle command (if cargo-tauri is installed)
if command -v cargo-tauri >/dev/null 2>&1; then
  cargo tauri build
else
  echo "    cargo-tauri not installed — building binary only."
  echo "    Install with: cargo install tauri-cli --locked"
  echo "    Binary at: target/release/vinaria"
  ls -lh target/release/vinaria 2>/dev/null || true
  exit 0
fi

echo ""
echo "==> Built artifacts:"
find target/release/bundle -type f \( -name "*.app" -o -name "*.dmg" \) | head -5
