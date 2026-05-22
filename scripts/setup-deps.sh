#!/usr/bin/env bash
# Install Homebrew dependencies needed to build Wine from CodeWeavers source on macOS Intel.
# Idempotent — re-running is safe and skips already-installed packages.
set -euo pipefail

PACKAGES=(
  bison
  flex
  m4
  gettext
  mingw-w64
  freetype
  gstreamer
  sdl2
  mpg123
  libpng
  libjpeg
  libtiff
  openssl@3
  pcre2
  pkg-config
  cmake
)

OPTIONAL_CASKS=(
  gstreamer-runtime
)

if ! command -v brew >/dev/null 2>&1; then
  echo "error: Homebrew not installed. See https://brew.sh" >&2
  exit 1
fi

if ! xcode-select -p >/dev/null 2>&1; then
  echo "error: Xcode Command Line Tools missing. Run: xcode-select --install" >&2
  exit 1
fi

echo "==> Installing Homebrew packages for Wine build"
brew install "${PACKAGES[@]}"

echo "==> Optional casks (pkg installers — may prompt for sudo)"
for cask in "${OPTIONAL_CASKS[@]}"; do
  if ! brew list --cask "$cask" >/dev/null 2>&1; then
    echo "    $cask not installed. Run manually if needed:"
    echo "    brew install --cask $cask"
  fi
done

echo ""
echo "==> Build deps ready."
echo "    Next: ./scripts/fetch-wine.sh"
