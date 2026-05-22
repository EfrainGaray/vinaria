# Vinaria

Free and open-source manager to run Windows applications on **Intel Macs**, where
modern alternatives (Whisky, Game Porting Toolkit) don't reach. A community
challenger to CrossOver, with the same engine inside — just without the paywall.

> *Vinaria* — Latin for "wine cellar". The place where bottles are kept.

## Why

CrossOver is the only reliable way to run many Windows apps on Intel Macs in
2026 — but it's a $74 one-time purchase that not everyone can afford. Whisky,
the obvious open-source alternative, dropped Intel support. Wine vanilla on
Intel Mac has known regressions in the Mac driver and D3D11 path that block
GameMaker, Steam's CEF, and other common runtimes.

Vinaria wraps a CodeWeavers-patched Wine build (the same patches CrossOver
ships, sourced from their LGPL distribution) with a bottle manager, recipe
catalog, and a clean web UI. Result: anyone with an Intel Mac can run their
Windows games and apps for free.

## Status

**Phase: xp (experimental).** Working proof of concept for Norland confirmed.
Wine compilation from CodeWeavers source in progress. UI not started yet.

## Architecture

```
┌──────────────────────────────────────┐
│   UI (Astro + TypeScript)            │  bottles, install wizard, launch
└────────────┬─────────────────────────┘
             │ Tauri IPC
┌────────────▼─────────────────────────┐
│   Core Backend (Rust + Tauri)        │  bottle manager, recipe runner
└────────────┬─────────────────────────┘
             │ exec
┌────────────▼─────────────────────────┐
│   winecx (CodeWeavers source build)  │  built from upstream LGPL source
│   + DXMT / DXVK / Goldberg modules   │
└──────────────────────────────────────┘
```

## Roadmap

- [x] Phase 1-2: Identify what CrossOver patches (DLLs + binary + winemac.drv)
- [ ] Phase 3: Compile Wine winecx from CodeWeavers official source
- [ ] Phase 4: Rust + Tauri core (BottleManager, RecipeRunner, ProcessSpawner)
- [ ] Phase 5: Astro UI
- [ ] Phase 6: Recipe catalog (community repo)
- [ ] Phase 7: Distribution (DMG, signing, docs)

## License

LGPL-2.1-or-later (matches Wine's license). See `LICENSE`.

## Credits

- Wine — https://www.winehq.org
- CodeWeavers — https://www.codeweavers.com (source we build from)
- DXMT — https://github.com/3Shain/dxmt (D3D11→Metal backend option)
- Whisky — https://github.com/Whisky-App/Whisky (inspiration for wrapper UX)
