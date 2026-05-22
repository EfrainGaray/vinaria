#!/usr/bin/env bash
# Install the built Wine into ~/.vinaria/wine/ so it can be invoked by the launcher.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="$REPO_ROOT/build/wine-build"
INSTALL_DIR="$HOME/.vinaria/wine"

if [ ! -f "$BUILD_DIR/Makefile" ]; then
  echo "error: nothing built yet. Run ./scripts/build-wine.sh first." >&2
  exit 1
fi

mkdir -p "$INSTALL_DIR"
cd "$BUILD_DIR"
echo "==> Installing to $INSTALL_DIR"
make install DESTDIR=

echo ""
echo "==> Verify"
"$INSTALL_DIR/bin/wine" --version
echo ""
echo "==> Installed. The launcher can now use: WINELOADER=$INSTALL_DIR/bin/wine"
