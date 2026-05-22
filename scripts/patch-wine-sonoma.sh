#!/usr/bin/env bash
# Apply our Sonoma-compat patches to the unpacked Wine source.
# These are idempotent: re-running on already-patched source is safe.
set -euo pipefail

CX_VERSION="${CX_VERSION:-26.1.0}"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC_DIR="$REPO_ROOT/build/wine-source-${CX_VERSION}/wine"

if [ ! -f "$SRC_DIR/configure" ]; then
  echo "error: $SRC_DIR/configure missing. Run ./scripts/fetch-wine.sh first." >&2
  exit 1
fi

echo "==> Patching Wine configure for macOS Sonoma OpenGL detection"

# In macOS Sonoma+, libGL.dylib is no longer on disk; it's in the dyld shared
# cache. Wine's configure auto-detection links against a hardcoded path and
# fails, then declares OpenGL unsupported, which disables Direct3D entirely.
#
# Replace the dylib_file hack with a plain -framework OpenGL link line. The
# test then succeeds against macOS's actual OpenGL framework.
if grep -q 'dylib_file /System/Library/Frameworks/OpenGL.framework' "$SRC_DIR/configure"; then
  # Two replacements: the detection LIBS line and the resulting OPENGL_LIBS line.
  sed -i.sonoma-bak '
    s|-dylib_file /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGL.dylib:/System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGL.dylib -lGL|-framework OpenGL|g
    s|"-Xlinker -dylib_file -Xlinker /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGL.dylib:/System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGL.dylib -lGL"|"-framework OpenGL"|g
  ' "$SRC_DIR/configure"
  echo "    applied: OpenGL framework link patch (both detection and resolved LIBS)"
else
  echo "    skipped: OpenGL framework patch already applied or upstream changed"
fi

echo "==> Done. Now re-run ./scripts/build-wine.sh"
