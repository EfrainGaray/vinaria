# Findings — Wine Wrapper Lab

## 2026-05-22 — Phase 1+2 BREAKTHROUGH

**Result:** Norland boots and runs tutorial on our Wine 11.9 prefix with CrossOver's builtin `d3d11.dll` + `dxgi.dll` swapped in. ~1 hour from "decided to lab" to "playing in tutorial".

### What CrossOver does

- Wine fork: `wine-11.0-8720-g4351038808c` (Wine 11.0 base + their patches)
- `bin/wine` is a Perl wrapper that sets `WINEDLLPATH`, `WINELOADER`, `CX_APPLEGPTK_LIBD3DSHARED_PATH`, `GST_PLUGIN_SYSTEM_PATH`, `DOTNET_EnableWriteXorExecute=0`, etc.
- Ships THREE D3D backends:
  - `lib/dxvk/` — DXVK (D3D11→Vulkan, what we tried, crashed)
  - `lib/dxmt/` — DXMT (D3D11→Metal direct, no Vulkan)
  - `lib64/apple_gptk/` — Apple Game Porting Toolkit (proprietary)
- **But the Norland bottle uses NONE of those.** It uses Wine **builtin** d3d11+dxgi from `lib/wine/x86_64-windows/`.

### Critical insight

CrossOver's builtin `d3d11.dll` (425KB) and `dxgi.dll` (213KB) are patched. The patches fix the GameMaker enum-as-pointer crash that vanilla Wine 11.9 + DXVK both hit at `dxgi+0xd5f78 cmpq $0x14, 0x18(%rsi)` with `rsi=0x8`.

### Fix (working today)

```bash
CO="/Applications/CrossOver.app/Contents/SharedSupport/CrossOver/lib/wine/x86_64-windows"
PREFIX="$HOME/.wine-norland"
for dll in d3d11.dll dxgi.dll; do
  cp "$PREFIX/drive_c/windows/system32/$dll" "$PREFIX/drive_c/windows/system32/$dll.bak"
  cp "$CO/$dll" "$PREFIX/drive_c/windows/system32/$dll"
done
WINE="/Applications/Wine Staging.app/Contents/Resources/wine/bin/wine"
WINEPREFIX="$PREFIX" "$WINE" reg delete "HKCU\\Software\\Wine\\DllOverrides" /v "d3d11" /f 2>/dev/null
WINEPREFIX="$PREFIX" "$WINE" reg delete "HKCU\\Software\\Wine\\DllOverrides" /v "dxgi" /f 2>/dev/null
```

Then `norland` launcher works.

### Verified hashes (CrossOver 26.1.0 build 20260325T164441Z)

- `d3d11.dll`: sha256 `6d6eda305d29a94203e1c4c250c48b4263d6b83734ebe0a3a6fb6a9360aa84d5` (425552 bytes)
- `dxgi.dll`: sha256 `b874344f327493c28e291b3b9e11e03d377101b6e4f94a26148066d5facf56f5` (218176 bytes)

### Legal note

Wine is LGPL. CrossOver must publish source for LGPL-licensed modifications — their public fork on GitHub (`CodeWeavers/wine`) has the patches. Using compiled DLLs from a paid CrossOver install personally is fine. Redistribution requires also providing matching source — which CodeWeavers does publish, but worth tracking down for any future open-source wrapper distribution.

## Phase 3 (post-trial, optional)

To make this truly independent of CrossOver:

1. Identify which commits in CodeWeavers' Wine fork carry the d3d11/dxgi patches vs upstream Wine
2. Cherry-pick onto upstream Wine 11.9 source
3. Build Wine locally with the patches → produce our own `d3d11.dll`/`dxgi.dll`
4. Optionally: package as Whisky-style wrapper

CodeWeavers Wine source: https://github.com/CodeWeavers/wine — look for branches matching `cxoffice-26.1.0rc1`.

## Open questions

- Patches in `wined3d.dll` (core) or in d3d11/dxgi PE shims? Need to hash `wined3d.dll` next.
- Does the fix survive `wineboot --update`?
- Does this fix help other GameMaker titles? Other D3D11 indie games?
