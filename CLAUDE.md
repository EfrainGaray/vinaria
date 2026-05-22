# Vinaria — Claude Code Project Config

**Phase:** xp (experimental, throwaway-grade code OK)
**Hardware target:** Intel Mac (x86_64), macOS 12.5+

## What this is

A free, open-source alternative to CrossOver for Intel Macs. We compile Wine
from CodeWeavers' LGPL source distribution, wrap it with a Rust/Tauri bottle
manager and an Astro UI, and ship a recipe catalog so users can install Windows
apps in a couple of clicks.

## Goals (phase xp)

1. Get one game (Norland) running with our own Wine build — no CrossOver binary
   in the loop at runtime.
2. Wrap launching/managing bottles in a CLI first, then minimal UI.
3. Open-source from day one.

## Non-goals (for now)

- Apple Silicon support — Whisky already does that well. Vinaria focuses on the
  gap Whisky left on Intel.
- Performance tuning. Get correctness first.
- Anti-cheat / DRM workarounds. We run what works under unmodified Wine.

## Conventions

- **Conventional commits**, English in code/commits/PRs, Spanish in chat with
  Efra (per global config).
- **Atomic commits** — every commit must build and represent one logical change.
- **No vendored binaries in repo** — Wine source is downloaded at build time
  from CodeWeavers, not committed.
- **No CrossOver-distributed binaries in the repo** — we link to the source
  tarball and build locally. Licensing-clean.

## Stack

- **Wine source**: CodeWeavers `crossover-sources-26.1.0.tar.gz` (LGPL)
- **Build tools**: Xcode CLT, Homebrew (bison/flex/mingw-w64/freetype/gstreamer/sdl2)
- **Core**: Rust + Tauri
- **UI**: Astro + TypeScript
- **Recipes**: TOML (one file per app)

## Layout

```
vinaria/
├── core/           # Rust crate — bottle manager, IPC, process spawner
├── ui/             # Astro app — frontend
├── src/            # Tauri main (Rust)
├── recipes/        # *.toml — app recipes (Norland, Steam, etc.)
├── scripts/        # build/install/dev helpers
├── docs/           # design notes, findings from the reverse-engineering lab
└── build/          # gitignored — Wine source tarball + build output
```

## Build commands (Phase 3)

```bash
./scripts/fetch-wine.sh      # download + extract CrossOver source
./scripts/build-wine.sh      # configure + make (~45 min on Intel)
./scripts/install-wine.sh    # install to ~/.vinaria/wine/
./scripts/test-norland.sh    # smoke test
```

## References

- CodeWeavers Wine source mirror: https://media.codeweavers.com/pub/crossover/source/
- Wine docs: https://wiki.winehq.org/Building_Wine_on_macOS
- Tauri docs: https://tauri.app
