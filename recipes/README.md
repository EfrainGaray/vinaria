# Recipes

A recipe is a single TOML file that tells Vinaria how to set up a bottle for
one specific Windows application: which Wine version to use, which DLL
overrides apply, which registry tweaks to make, which env vars to set, and
which executable to launch.

## Schema (phase xp — will stabilize at v1.0)

```toml
id              = "norland"           # required, slug, must be unique
name            = "Norland"           # required, display name
steam_app_id    = 1857090             # optional, Steam app id
windows_version = "win10"             # optional: win7 / win10 / win11

[[registry]]                          # 0+ registry entries
key   = 'HKCU\Software\Wine\Direct3D'
name  = 'cb_access_map_w'
kind  = 'dword'                       # dword | string | binary | qword
value = 1

[launch]
executable = 'C:\Path\To\Game.exe'    # required, Windows-style path inside the prefix
args       = []                       # optional, list of args
[launch.env]                          # optional env vars
WINEMSYNC = "1"
```

## Contributing

For phase xp this directory is the "starter set". When phase 6 lands we'll
move the catalog to a dedicated community repo so people can PR recipes
without touching the main Vinaria tree.

## What recipes do NOT contain

- **Wine binaries** — Vinaria ships its own Wine build from CodeWeavers
  source. Recipes never include or reference proprietary Wine forks.
- **Cracked or pirated game files** — recipes describe configuration for
  *legally owned* software.
- **CodeWeavers / CrossOver code** — recipes can mention behaviors observed
  while studying CrossOver, but never copy code from it.
