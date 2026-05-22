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

# CodeWeavers source tarball bundles many components; Wine lives in `wine/`.
# Use the fixed path so we don't accidentally pick another project's configure
# (e.g. ghostscript) when the parent dir name contains "wine".
WINE_SRC="$SRC_DIR/wine"
if [ ! -f "$WINE_SRC/configure" ]; then
  echo "error: $WINE_SRC/configure missing. Did the tarball layout change?" >&2
  exit 1
fi
echo "==> Wine source at: $WINE_SRC"

mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# Homebrew prefix detection — Intel default is /usr/local
BREW_PREFIX="$(brew --prefix)"

# Prepend Homebrew keg-only tools that macOS either ships too old (bison) or
# whose default /usr/bin/* are stubs that fail under partial Xcode CLT (m4).
export PATH="$BREW_PREFIX/opt/bison/bin:$BREW_PREFIX/opt/flex/bin:$BREW_PREFIX/opt/m4/bin:$PATH"
echo "==> Using bison: $(which bison) ($(bison --version | head -1))"
echo "==> Using flex:  $(which flex) ($(flex --version | head -1))"
echo "==> Using m4:    $(which m4) ($(m4 --version | head -1))"

# Configure if not already done
if [ ! -f Makefile ]; then
  echo "==> Configuring (this writes config.log to $BUILD_DIR/)"
  # OPENGL_LIBS override: Wine's configure looks for libGL.dylib on disk,
  # but macOS Sonoma (Darwin 23+) hides it in the dyld shared cache. Force
  # the framework link directly so configure detects OpenGL support.
  PKG_CONFIG_PATH="$BREW_PREFIX/opt/openssl@3/lib/pkgconfig:$BREW_PREFIX/lib/pkgconfig" \
  CFLAGS="-I$BREW_PREFIX/include -O2 -g" \
  LDFLAGS="-L$BREW_PREFIX/lib" \
  OPENGL_LIBS="-framework OpenGL" \
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
