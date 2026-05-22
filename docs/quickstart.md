# Quickstart (phase xp)

Building and running Vinaria from source today. This is for contributors —
end users will get a `.app` once phase 7 lands.

## Requirements

- Intel Mac (x86_64). Apple Silicon support is out of scope for now.
- macOS 12.5+ (Monterey or later).
- Xcode Command Line Tools (`xcode-select --install`).
- Homebrew (https://brew.sh).
- ~5 GB of free disk space for source + build output.
- Patience: first build takes 30–90 minutes.

## Build Wine (phase 3)

```bash
git clone https://github.com/EfrainGaray/vinaria.git
cd vinaria

./scripts/setup-deps.sh    # Homebrew packages
./scripts/fetch-wine.sh    # download CodeWeavers source (~142 MB)
./scripts/build-wine.sh    # configure + make
./scripts/install-wine.sh  # install to ~/.vinaria/wine/
```

After `install-wine.sh`, you should have a working `wine` binary at
`~/.vinaria/wine/bin/wine`. Sanity check:

```bash
~/.vinaria/wine/bin/wine --version
```

## What's not here yet

- **The Tauri app** (phase 4-5). For now you launch Windows apps through a
  wineprefix and the binary directly, like you would with vanilla Wine.
- **Recipes** (phase 6). No catalog yet — bring your own bottle and your own
  `steam_appid.txt` if needed.
- **DMG / install bundle** (phase 7).

## Sanity test with a known-good app

If you have Norland on Steam:

```bash
# Create a prefix
WINEPREFIX=~/.wine-test ~/.vinaria/wine/bin/wineboot --init

# Download with SteamCMD (forcing Windows platform)
~/.local/steamcmd/steamcmd.sh \
  +@sSteamCmdForcePlatformType windows \
  +force_install_dir "$HOME/Games/Norland" \
  +login YOUR_STEAM_USER \
  +app_update 1857090 validate \
  +quit

# Run the game
cd ~/Games/Norland
echo "1857090" > steam_appid.txt
WINEPREFIX=~/.wine-test ~/.vinaria/wine/bin/wine Norland.exe
```
