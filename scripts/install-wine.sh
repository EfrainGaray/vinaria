#!/usr/bin/env bash
# Install the built Wine into ~/.vinaria/wine/ so it can be invoked by the launcher.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="$REPO_ROOT/build/wine-build"
STAGE_DIR="$REPO_ROOT/build/wine-install"
INSTALL_DIR="$HOME/.vinaria/wine"

if [ ! -f "$BUILD_DIR/Makefile" ]; then
  echo "error: nothing built yet. Run ./scripts/build-wine.sh first." >&2
  exit 1
fi

# First run `make install` into the in-repo staging dir (configure --prefix
# points there). Then copy the staging dir into the canonical user location.
# This two-step keeps the staging dir for inspection and a clean atomic swap
# at the destination.
cd "$BUILD_DIR"
echo "==> Installing to staging: $STAGE_DIR"
make install >/dev/null

mkdir -p "$(dirname "$INSTALL_DIR")"
rm -rf "$INSTALL_DIR"
cp -R "$STAGE_DIR" "$INSTALL_DIR"
echo "==> Copied to canonical: $INSTALL_DIR"

echo ""
echo "==> Verify"
"$INSTALL_DIR/bin/wine" --version
echo ""
echo "==> Installed. The launcher can now use: WINELOADER=$INSTALL_DIR/bin/wine"
