# End-to-end build guide

What it takes to go from a fresh Intel Mac to running Norland (or any other
recipe-supported Windows app) under Vinaria, with no proprietary binaries
at runtime.

## What you need

- Intel Mac, macOS 12.5+
- ~7 GB free disk (Wine source + build artifacts + installed Wine + game)
- Homebrew (https://brew.sh)
- Xcode Command Line Tools (`xcode-select --install`)
- Rust toolchain (https://rustup.rs)
- Node 20+

## The seven scripts

The whole pipeline is automated. Each script is idempotent and prints a
clear next-step at the end.

```bash
git clone https://github.com/EfrainGaray/vinaria.git
cd vinaria

# Phase 3: build Wine from CodeWeavers source
./scripts/setup-deps.sh        # Homebrew packages
./scripts/fetch-wine.sh        # download CodeWeavers source (~142 MB)
./scripts/build-wine.sh        # configure + make (~45 min on Intel)
./scripts/install-wine.sh      # copy to ~/.vinaria/wine/

# Phase 6: validate recipes
./scripts/validate-recipes.sh

# Phase 7: bundle the desktop app
./scripts/bundle-app.sh
./scripts/sign-adhoc.sh target/release/bundle/macos/Vinaria.app
```

## What happens behind each script

| script                  | what it does                                            |
| ----------------------- | ------------------------------------------------------- |
| `setup-deps.sh`         | brews bison/flex/m4/mingw-w64/freetype/MoltenVK/etc.    |
| `fetch-wine.sh`         | curl + tar of `crossover-sources-26.1.0.tar.gz`         |
| `patch-wine-sonoma.sh`  | (auto-called) rewrites configure for libGL.dylib hack   |
| `build-wine.sh`         | sets SDKROOT + OPENGL_LIBS, runs configure + make -j    |
| `install-wine.sh`       | `make install` to staging then `cp -R` to `~/.vinaria/` |
| `validate-recipes.sh`   | parses every `recipes/*.toml` through vinaria-core      |
| `bundle-app.sh`         | astro build + cargo build --release + tauri bundle      |
| `sign-adhoc.sh`         | `codesign --force --deep --sign -` for local trust      |

## Smoke test (without the Tauri shell)

Once Wine is installed (`./scripts/install-wine.sh` done), you can sanity-check
without launching the desktop app at all:

```bash
# Download a game with SteamCMD (Windows build) — you need your Steam login
~/.local/steamcmd/steamcmd.sh \
  +@sSteamCmdForcePlatformType windows \
  +force_install_dir "$HOME/Games/Norland" \
  +login YOUR_USERNAME \
  +app_update 1857090 validate \
  +quit

echo "1857090" > ~/Games/Norland/steam_appid.txt

WINEPREFIX=~/.wine-norland \
WINESERVER=~/.vinaria/wine/bin/wineserver \
WINELOADER=~/.vinaria/wine/bin/wine \
DYLD_FALLBACK_LIBRARY_PATH=/usr/local/lib:/usr/lib \
WINEMSYNC=1 \
~/.vinaria/wine/bin/wine ~/Games/Norland/Norland.exe
```

If you see Norland's menu, the Wine build is healthy.

## Common gotchas

- **bison too old.** macOS ships bison 2.3. `setup-deps.sh` brews 3.x, and
  `build-wine.sh` prepends `/usr/local/opt/bison/bin` to PATH. If you run
  `make` by hand, do the same.
- **xcrun: SDK 'macosx' cannot be located.** Happens on some CommandLineTools
  installs. `build-wine.sh` pins `SDKROOT` explicitly to work around it.
- **No OpenGL library found.** Sonoma removed libGL.dylib from disk (it lives
  in the dyld shared cache). `patch-wine-sonoma.sh` rewrites Wine's configure
  to use `-framework OpenGL` instead.
- **Make fails with truncated archive .o.** Race condition in `make -j` when
  multiple ranlib processes touch the same `.a`. Just re-run `make`. Make
  is idempotent and will regenerate what it broke.
- **Wine cannot find the FreeType font library.** Wine dlopen's
  `libfreetype.6.dylib` at runtime. Either run inside Tauri (which sets
  `DYLD_FALLBACK_LIBRARY_PATH` for you) or export it yourself before launch.

## Disk footprint after a full build

```
~/.vinaria/                    ~700 MB  (installed Wine + bottles + recipes)
<repo>/build/wine-source-X.Y/   ~830 MB  (extracted CodeWeavers source)
<repo>/build/wine-build/        ~3.0 GB  (object files + intermediate artifacts)
<repo>/build/wine-install/      ~700 MB  (staging copy of installed Wine)
<repo>/target/                  ~1.5 GB  (Rust build dir)
<repo>/ui/node_modules/         ~400 MB
```

You can delete `build/` after `install-wine.sh` if you don't plan to rebuild
soon. ~700 MB in `~/.vinaria/` is the irreducible runtime footprint.
