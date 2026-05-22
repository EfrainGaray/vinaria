# Architecture

Vinaria is a thin, opinionated wrapper around Wine. The goal is to make running
a Windows app on an Intel Mac as easy as opening a `.app`, while keeping every
moving part inspectable and replaceable.

## Layers

```
┌──────────────────────────────────────────────────────────┐
│   UI (Astro 5 + TypeScript)                              │
│   - Pages: bottle list, install wizard, settings, logs   │
│   - Renders inside Tauri webview                         │
└────────────────────────────┬─────────────────────────────┘
                             │ Tauri IPC (invoke / events)
┌────────────────────────────▼─────────────────────────────┐
│   src-tauri (Rust binary, Tauri 2)                       │
│   - Thin shell: handlers map IPC -> vinaria-core calls   │
│   - Owns the OS window, menu, file dialogs               │
└────────────────────────────┬─────────────────────────────┘
                             │ Rust function calls
┌────────────────────────────▼─────────────────────────────┐
│   vinaria-core (Rust library, no Tauri deps)             │
│   - BottleManager: CRUD over ~/.vinaria/bottles/         │
│   - RecipeRunner: parse TOML, apply to a bottle          │
│   - ProcessSpawner: launch wine with proper env          │
│   - Reusable from a future CLI or alt frontend           │
└────────────────────────────┬─────────────────────────────┘
                             │ std::process::Command
┌────────────────────────────▼─────────────────────────────┐
│   Wine (built from CodeWeavers source)                   │
│   - Lives at ~/.vinaria/wine/                            │
│   - One installation, shared across all bottles          │
│   - Bottles are wine prefixes elsewhere                  │
└──────────────────────────────────────────────────────────┘
```

## Filesystem layout (user side)

```
~/.vinaria/
├── wine/                       # the Wine install (binaries + libs)
│   ├── bin/wine
│   ├── bin/wineserver
│   └── lib/wine/x86_64-windows/{*.dll, winemac.drv, ...}
├── bottles/
│   ├── <bottle-uuid>/
│   │   ├── meta.toml           # bottle metadata (name, recipe, etc.)
│   │   └── prefix/             # the wine prefix itself
│   │       └── drive_c/...
│   └── ...
├── recipes/                    # user-local recipes (override repo ones)
└── logs/                       # rotating logs per bottle
```

## Why a separate `vinaria-core` crate?

Two reasons:

1. **Testability.** Bottle logic and recipe parsing have no business knowing
   about webviews or JavaScript bridges. Keeping them in a pure-Rust lib means
   we can unit-test them without Tauri's test scaffolding.
2. **Future frontends.** A CLI like `vinaria run norland` should be trivial to
   add — it just imports `vinaria_core` and skips the Tauri layer.

## Why Tauri 2 instead of Electron?

- Bundle size: ~10 MB Tauri vs ~150 MB Electron.
- Memory: Tauri shares the system WebKit; Electron ships its own Chromium.
- Rust IPC instead of Node — no second runtime to manage.

The trade-off: Tauri's webview is WebKit, not Chromium, so any HTML/CSS the UI
uses must work in Safari/WebKit. We treat that as a feature (forces lean
markup) not a bug.
