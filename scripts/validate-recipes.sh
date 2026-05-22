#!/usr/bin/env bash
# Validate every recipe TOML by parsing it through vinaria-core.
# Run before committing recipe changes.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

failed=0
for recipe in recipes/*.toml; do
  [ -f "$recipe" ] || continue
  if cargo run -q -p vinaria-core --example validate -- "$recipe" 2>&1; then
    echo "✓ $recipe"
  else
    echo "✗ $recipe"
    failed=$((failed + 1))
  fi
done

if [ "$failed" -gt 0 ]; then
  echo ""
  echo "$failed recipe(s) failed to parse."
  exit 1
fi
echo ""
echo "All recipes parse OK."
