#!/usr/bin/env bash
# Configure and build Wine from CodeWeavers source.
# Builds 64-bit Wine for macOS Intel. Output goes to build/wine-build/.
# Expect ~30-90 minutes on first build.
set -euo pipefail

CX_VERSION="${CX_VERSION:-26.1.0}"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC_DIR="$REPO_ROOT/build/wine-source-${CX_VERSION}"
BUILD_DIR="$REPO_ROOT/build/wine-build"
JOBS="${JOBS:-$(sysctl -n hw.ncpu)}"

if [ ! -d "$SRC_DIR" ]; then
  echo "error: source not found at $SRC_DIR. Run ./scripts/fetch-wine.sh first." >&2
  exit 1
fi

# Tarball typically contains multiple components; Wine itself is in a sub-directory.
# Locate the actual Wine source root (the one with configure script).
WINE_SRC="$(find "$SRC_DIR" -name "configure" -maxdepth 4 -path "*/wine*" | head -1 | xargs dirname)"
if [ -z "$WINE_SRC" ] || [ ! -f "$WINE_SRC/configure" ]; then
  echo "error: could not locate Wine source dir with configure script under $SRC_DIR" >&2
  exit 1
fi
echo "==> Wine source at: $WINE_SRC"

mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# Homebrew prefix detection — Intel default is /usr/local
BREW_PREFIX="$(brew --prefix)"

# Configure if not already done
if [ ! -f Makefile ]; then
  echo "==> Configuring (this writes config.log to $BUILD_DIR/)"
  PKG_CONFIG_PATH="$BREW_PREFIX/opt/openssl@3/lib/pkgconfig:$BREW_PREFIX/lib/pkgconfig" \
  CFLAGS="-I$BREW_PREFIX/include -O2 -g" \
  LDFLAGS="-L$BREW_PREFIX/lib" \
  "$WINE_SRC/configure" \
    --prefix="$REPO_ROOT/build/wine-install" \
    --enable-win64 \
    --disable-tests \
    --with-png \
    --with-jpeg \
    --with-freetype \
    --with-gnutls 2>&1 | tee configure.log
else
  echo "==> Already configured. Delete $BUILD_DIR/Makefile to reconfigure."
fi

echo "==> Building with -j${JOBS}. This takes 30-90 minutes on Intel."
make -j"$JOBS" 2>&1 | tee build.log

echo ""
echo "==> Build done. Install with: ./scripts/install-wine.sh"
