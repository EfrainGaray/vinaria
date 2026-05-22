#!/usr/bin/env bash
# Download and extract the CodeWeavers CrossOver Wine source tarball.
# Source is LGPL — published by CodeWeavers at media.codeweavers.com.
set -euo pipefail

CX_VERSION="${CX_VERSION:-26.1.0}"
TARBALL="crossover-sources-${CX_VERSION}.tar.gz"
URL="https://media.codeweavers.com/pub/crossover/source/${TARBALL}"
BUILD_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/build"

mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

if [ -f "$TARBALL" ]; then
  echo "==> Tarball already present: $TARBALL ($(du -h "$TARBALL" | cut -f1))"
else
  echo "==> Downloading $URL"
  echo "    (~142 MB)"
  curl -L --fail --progress-bar -o "$TARBALL" "$URL"
fi

if [ ! -d "wine-source-${CX_VERSION}" ]; then
  echo "==> Extracting"
  mkdir -p "wine-source-${CX_VERSION}"
  tar xzf "$TARBALL" -C "wine-source-${CX_VERSION}" --strip-components=1
  echo "    extracted to build/wine-source-${CX_VERSION}/"
else
  echo "==> Source already extracted at build/wine-source-${CX_VERSION}/"
fi

echo ""
echo "==> Source ready."
echo "    Next: ./scripts/build-wine.sh"
