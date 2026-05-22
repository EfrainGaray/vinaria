#!/usr/bin/env bash
# Sign a Vinaria .app with an ad-hoc signature (no Apple Developer ID).
# Result: the app runs locally without Gatekeeper warnings if launched via
# right-click → Open at least once. Not suitable for App Store distribution.
set -euo pipefail

APP="${1:-target/release/bundle/macos/Vinaria.app}"

if [ ! -d "$APP" ]; then
  echo "error: app bundle not found: $APP" >&2
  echo "       run ./scripts/bundle-app.sh first" >&2
  exit 1
fi

echo "==> Stripping quarantine attributes"
xattr -cr "$APP" || true

echo "==> Ad-hoc signing"
codesign --force --deep --sign - "$APP"

echo "==> Verifying signature"
codesign -dv --verbose=2 "$APP" 2>&1 | head -8

echo ""
echo "==> Done. First launch: right-click $APP → Open."
echo "    For distribution to others, they will need to do the same."
