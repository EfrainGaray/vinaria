# Vinaria

Free and open-source manager to run Windows applications on **Intel Macs**,
where modern alternatives (Whisky, Game Porting Toolkit) don't reach. A
community challenger to CrossOver, with the same engine inside — just without
the paywall.

> *Vinaria* — Latin for "wine cellar". The place where bottles are kept.

## Status

**Phase: xp (experimental)** — works end-to-end on Intel Mac Sonoma. Tested
with Norland; the catalog has starter recipes for Stardew Valley, Hollow
Knight, and Celeste.

| Phase | What | State |
|------:|------|-------|
| 1–2 | Reverse-engineer what CrossOver patches | ✅ |
| 3 | Compile Wine 11.0 from CodeWeavers source | ✅ |
| 4 | Rust core (BottleManager, RecipeRunner, ProcessSpawner) | ✅ |
| 5 | Astro UI with Tauri IPC | ✅ |
| 6 | Recipe catalog + validator | ✅ |
| 7 | Distribution scripts + docs | ✅ |

## Quick start

```bash
git clone https://github.com/EfrainGaray/vinaria.git
cd vinaria

./scripts/setup-deps.sh        # Homebrew dependencies
./scripts/fetch-wine.sh        # CodeWeavers source (~142 MB)
./scripts/build-wine.sh        # ~45 min on Intel
./scripts/install-wine.sh      # → ~/.vinaria/wine/
./scripts/bundle-app.sh        # produces Vinaria.app + .dmg
```

Full walkthrough including the smoke-test path: **[docs/end-to-end.md](docs/end-to-end.md)**.

## Architecture

```
┌──────────────────────────────────────┐
│   UI (Astro 5 + TypeScript)          │  bottles, recipes, wine, logs, settings
└────────────┬─────────────────────────┘
             │ Tauri IPC
┌────────────▼─────────────────────────┐
│   src-tauri (Rust binary, Tauri 2)   │  thin shell: IPC → vinaria-core
└────────────┬─────────────────────────┘
             │ Rust function calls
┌────────────▼─────────────────────────┐
│   vinaria-core (pure Rust library)   │  BottleManager, RecipeRunner, Spawner
└────────────┬─────────────────────────┘
             │ std::process::Command
┌────────────▼─────────────────────────┐
│   winecx (CodeWeavers source build)  │  built from CodeWeavers' LGPL source
│   at ~/.vinaria/wine/                 │  with macOS Sonoma OpenGL patch
└──────────────────────────────────────┘
```

Detailed design: **[docs/architecture.md](docs/architecture.md)** ·
**[docs/design-language.md](docs/design-language.md)**.

## Why Vinaria exists

CrossOver is the only reliable way to run many Windows apps on Intel Mac in
2026 — but it's a $74 one-time purchase that not everyone can afford. Whisky,
the obvious open-source alternative, dropped Intel support. Wine vanilla on
Intel Mac has known regressions in the Mac driver and D3D11 path that block
GameMaker, Steam's CEF, and other common runtimes.

Vinaria compiles Wine from the same LGPL source CodeWeavers publishes,
applies the macOS Sonoma OpenGL detection patch we need on top, wraps it
with a bottle manager and recipe catalog, and ships the whole thing as a
free `.app`.

## Adding a recipe

Drop a TOML file in `recipes/`, validate it:

```bash
./scripts/validate-recipes.sh
```

Open a PR. See **[recipes/README.md](recipes/README.md)** for the schema.

## License

LGPL-2.1-or-later (matches Wine's license). See `LICENSE`.

## Credits

- **Wine** — https://www.winehq.org
- **CodeWeavers** — https://www.codeweavers.com (LGPL source we build from)
- **DXMT** — https://github.com/3Shain/dxmt (D3D11 → Metal backend reference)
- **Whisky** — https://github.com/Whisky-App/Whisky (UX inspiration)

Vinaria's UI design language is **inspired by CleanMyMac** — see
`docs/design-language.md` for the visual posture we're targeting. No
proprietary assets are vendored.

## Contributing

See **[CONTRIBUTING.md](CONTRIBUTING.md)**. Ground rules: English in code,
conventional commits, atomic commits, no vendored Wine binaries, no
proprietary code.
