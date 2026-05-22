# Release process

End-to-end recipe for cutting a Vinaria release on Intel Mac.

## Prereqs

- Working Wine build at `~/.vinaria/wine/` (see `quickstart.md`).
- Node 20+, npm.
- Rust 1.85+ (current stable at the time of writing).
- Optional: `cargo install tauri-cli --locked` for full `.dmg` bundling.
  Without it, `bundle-app.sh` builds just the binary.

## Build the app bundle

```bash
./scripts/bundle-app.sh
```

This will:

1. `cd ui && npm install && npm run build` → produces `ui/dist/`
2. `cargo build --release -p vinaria` → produces `target/release/vinaria`
3. If `cargo-tauri` is installed: `cargo tauri build` produces
   `target/release/bundle/macos/Vinaria.app` and a `.dmg`.

## Sign for local trust

We don't pay Apple Developer fees, so we ad-hoc sign:

```bash
./scripts/sign-adhoc.sh target/release/bundle/macos/Vinaria.app
```

After this, the user can right-click → Open the first time to bypass
Gatekeeper. Once they've done that, normal double-clicks work.

## Verify

```bash
open target/release/bundle/macos/Vinaria.app
```

The Vinaria window should come up with the bottles screen.

## Distribute

Two options:

1. **GitHub Releases** — drop the `.dmg` as a release asset, write release
   notes summarizing the new recipes and any phase changes. Users `xattr -cr`
   the app after download.
2. **Homebrew Cask** (future) — once we have a stable cadence, ship as
   `brew install --cask vinaria`. Requires the cask manifest in
   `homebrew-cask` upstream.

## Versioning

Phase-xp uses 0.x.y. The major bumps to 1.0 when:

- The recipe schema is frozen.
- BottleManager has a migration story.
- We have at least 10 recipes in the catalog.
- A first non-author has shipped a recipe via PR.

Until then, every release is allowed to break previous recipes.

## Out of scope for phase-xp

- Notarization (requires paid Apple Developer ID — $99/year).
- Auto-update via Tauri's updater (would also need notarization).
- App Store distribution.
