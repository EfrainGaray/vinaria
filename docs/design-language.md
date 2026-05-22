# Design Language

Vinaria's UI is **inspired by CleanMyMac** — the visual posture of a friendly
macOS utility that does serious work behind the scenes. Big bold titles,
a moody dark theme that changes color per module, a single floating action
button at the bottom, calm glassmorphic 3D art.

> We **do not** vendor or copy any assets from CleanMyMac (or any other
> proprietary app). This document captures the *spirit* of the design,
> distilled from public screenshots, and reimplements it from scratch.

## Core posture

1. **Always dark.** No light mode — that's CleanMyMac's signature. Each module
   has its own color identity that bathes the whole screen.
2. **Sidebar is icon-only.** Just colored badges, no labels. The active item
   gets a thick colored ring + tinted background.
3. **One floating action button.** A big circular button at the bottom-center
   ("Analyze" in CleanMyMac, "Install" / "Launch" for us). It glows in the
   module's color and is *the* primary action.
4. **3D glassmorphic mascot.** Every module has a translucent, slightly tilted
   3D object as its visual anchor on the left side of the main pane.
5. **Generous padding.** ~64px from the edges to content. Cards breathe.

## Per-module color signature

CleanMyMac assigns one color per top-level concept. The whole screen takes that
color: radial gradient from the center outward, vignetting to near-black at
the corners. We adopt the same pattern.

```
Bottles      → wine red    (#8b3a3a base, radial → #1a0a0a)
Recipes      → gold        (#c2a04e base, radial → #1f1607)
Wine install → slate blue  (#5d6b85 base, radial → #0d1018)
Settings     → graphite    (#6f6c66 base, radial → #14130f)
Logs         → emerald     (#3a8b5a base, radial → #07140d)
```

The base color is the saturated hero hue. The vignette is the deep variant.
A subtle inner glow (radial-gradient at 30% opacity) softens the center.

## Palette (phase-xp)

```
--bg-app-base:    #0a0a0d   /* deep neutral when no module focus    */
--bg-card:        rgba(255, 255, 255, 0.06)  /* glass cards on the gradient */
--bg-card-strong: rgba(255, 255, 255, 0.10)
--border-glass:   rgba(255, 255, 255, 0.10)
--fg-primary:     #ffffff
--fg-muted:       rgba(255, 255, 255, 0.60)
--fg-faint:       rgba(255, 255, 255, 0.35)

/* module colors (hero / vignette) */
--mod-bottles-hero:    #8b3a3a
--mod-bottles-deep:    #1a0a0a
--mod-recipes-hero:    #c2a04e
--mod-recipes-deep:    #1f1607
--mod-wine-hero:       #5d6b85
--mod-wine-deep:       #0d1018
--mod-settings-hero:   #6f6c66
--mod-settings-deep:   #14130f

/* states */
--ok:       #4cd178
--warn:     #f0b84a
--danger:   #ff5c4d
--ring:     rgba(255, 255, 255, 0.20)

/* premium CTA (top-right amber pill) */
--cta-amber:    #f5c845
--cta-amber-fg: #1a1402
```

## Sidebar

- Width 76px, full height, dark vertical strip on a slight gradient that
  matches the active module color at 30% opacity.
- Vertical stack of 40×40 icon badges with 10px radius.
- Active item: 2px outer ring in the module color + module-tinted background.
- Hover: 8% white background.
- Tiny status dot in the bottom-left of a badge if that module has alerts.

## Hero area

```
┌──┬──────────────────────────────────────────────────────────────┐
│  │   [3D mascot, big, glass effect]      Module Title (44px)    │
│  │                                       One sentence subtitle. │
│  │                                                              │
│  │                                       ● Feature 1            │
│sb│                                       ● Feature 2            │
│  │                                       ● Feature 3            │
│  │                                                              │
│  │                                                              │
│  │                          (   Action   ) ← floating CTA       │
└──┴──────────────────────────────────────────────────────────────┘
```

- Two-column layout: 3D mascot left (~38%), text right (~62%).
- Title 44px / 600, subtitle 16px / muted.
- Feature list: 24px circle icon (module color) + label, 12px gap between.
- Floating action button: 84px circle, position fixed at bottom-center 32px
  above bottom edge, big shadow glowing in the module color.

## Cards / lists

When there's actual data (bottles, recipes, log entries), it lives in glass
cards that float on top of the gradient.

- Background: `--bg-card` (6% white) with `backdrop-filter: blur(20px)`.
- Border: 1px `--border-glass`.
- Radius: 16px.
- Padding: 20-24px.
- Hover: lift to `--bg-card-strong` and ring `--ring`.

## Floating action button

The single most important UI element on each module screen.

```css
.fab {
  position: fixed;
  bottom: 32px; left: 50%;
  transform: translateX(-50%);
  width: 84px; height: 84px;
  border-radius: 50%;
  background: var(--mod-hero);
  box-shadow:
    0 0 0 4px rgba(255,255,255,0.06),
    0 8px 32px var(--mod-hero),
    0 16px 64px var(--mod-hero);
  color: white;
  font-weight: 600;
}
```

## Top bar

Minimal. Window controls on the left (handled by macOS), module name
centered in the title bar area, and the amber **"Unlock full version"** pill
at the right when relevant. For Vinaria the pill is "Star on GitHub" instead
of an unlock — same prominence, different goal.

## Typography

- **System UI stack** (`-apple-system`, `SF Pro Display`, fallback). Display
  weight for titles.
- Sizes:
  - Hero 44px / 1.05 / 600
  - Section 28px / 1.1 / 600
  - Body 16px / 1.55 / 400 (a touch bigger than the previous spec; CleanMyMac
    leans roomy)
  - Caption 13px / 1.4 / 500
- `font-variant-numeric: tabular-nums` everywhere.

## Motion

- **Module transitions:** 240ms cross-fade of the background gradient color +
  3D mascot. Bottom CTA recolors in the same beat.
- **Sidebar select:** 120ms tinted ring grows from the badge.
- **Cards entering a list:** 180ms ease-out, 8px translate-y + fade.
- **No bouncy springs.** CleanMyMac's motion is calm, not playful.

## 3D mascots

CleanMyMac has bespoke 3D illustrations per module. We won't reproduce those.
For phase-xp we use **simple SVG + CSS** vignette mascots:

- A wine bottle (Bottles)
- A scroll (Recipes)
- A barrel (Wine install)
- A gear (Settings)

These render as flat-but-glossy SVGs with a `backdrop-filter` halo to fake
the glass effect. Phase-7 may upgrade to actual 3D via three.js if it's
worth the bundle size.

## Onboarding

The welcome modal pattern works for us too:

- Full-width modal centered, 920×540, rounded 24px corners.
- Two columns: 3D art (~45%), copy + CTA (~55%).
- Top progress stepper showing tour length.
- Primary CTA in the module color.

For Vinaria the tour is 3 steps: build Wine → create your first bottle →
install your first app via a recipe.

## What we deliberately don't do

- **No 3D models in phase-xp.** Use SVG mascots — much faster, no bundle hit.
- **No glassmorphism on text.** It hurts legibility. Cards only.
- **No marketing-style hero on the home screen.** The bottle list IS the
  home screen.
- **No emojis except 🍷 in the brand mark.**
