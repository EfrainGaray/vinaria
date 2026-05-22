# Design Language

Vinaria's UI is **inspired by CleanMyMac** — the visual posture of a friendly
macOS utility that does serious work behind the scenes. Big legible state,
calm animations, a sidebar that always knows where you are.

> We **do not** vendor or copy any assets from CleanMyMac (or any other
> proprietary app). This document captures the *spirit* of the design and
> reimplements it from scratch in CSS.

## Voice

- **Friendly but precise.** "12 bottles ready" beats "Bottles: 12".
- **Action-oriented.** Primary buttons name what they do, not what they are
  ("Install Norland", not "Submit").
- **Honest about state.** "Building Wine (4 of 7 phases)" beats a spinner.

## Layout primitives

```
┌─────────┬──────────────────────────────────────────────┐
│         │                                              │
│ Sidebar │           Main content area                  │
│  dark   │              (light or dark)                 │
│  navy   │                                              │
│         │   ┌────────────┐  ┌────────────┐             │
│  nav    │   │   Card     │  │   Card     │             │
│  items  │   │            │  │            │             │
│         │   └────────────┘  └────────────┘             │
│         │                                              │
└─────────┴──────────────────────────────────────────────┘
```

- **Sidebar** ~ 220px wide, dark navy (#161e2d), white labels, colored icon
  squares (8-10px radius) to the left of each label.
- **Main area** light by default (#f4f3ee — warm off-white, our "vellum"),
  dark mode auto-respected.
- **Cards** with 12-16px corner radius, faint 1-pixel border + subtle shadow
  (0 1px 3px rgba(0,0,0,0.04)). No heavy drop shadows.

## Palette (phase-xp)

```
--bg-app:      #f4f3ee   /* vellum — warm off-white */
--bg-sidebar:  #161e2d   /* navy ink */
--bg-card:     #ffffff
--fg-primary:  #1a1a1f
--fg-muted:    #6f6c66
--fg-on-dark:  #f3e9d2
--accent:      #8b3a3a   /* wine red — primary actions */
--accent-soft: #c97b7b
--success:     #4a7c59
--warning:     #c2a04e
--danger:      #a13b3b
--ring:        rgba(139, 58, 58, 0.25)
```

Dark mode swaps `--bg-app` to `#1a1110` and `--bg-card` to `#221615`. The
sidebar stays dark in both modes — that's the whole point of the look.

## Typography

- **System UI stack first** (`-apple-system`, then SF Pro fallback, then
  generic sans-serif). Keeps weight under 1KB before any custom font loads.
- **Sizes:**
  - Display 28px / 1.1 / 600 (page titles, bottle hero numbers)
  - Title 18px / 1.3 / 600 (card headers)
  - Body 14px / 1.5 / 400
  - Small 12px / 1.4 / 500 (badges, metadata)
- **Numerals:** `font-variant-numeric: tabular-nums` on stats so columns line
  up. CleanMyMac does this on every counter.

## Iconography

- 20×20 icons with 8px-radius colored backgrounds (badge style), used in the
  sidebar and on card headers.
- One color per top-level concept: bottles (wine red), recipes (gold), Wine
  install (slate), settings (gray). Helps with at-a-glance scanning.

## Motion

- **Page transitions:** 180ms ease-out fade + 6px translate-y.
- **List inserts/deletes:** 220ms cubic-bezier(0.2, 0.8, 0.2, 1) height +
  opacity. No bouncy springs.
- **Progress:** real progress bars, not indeterminate spinners. If we don't
  know the percentage, show the current step name instead.
- **Hover states:** 80ms ease. Subtle — no large transforms.

## Status badges

```
●  ready          (success green)
○  building       (neutral, animated dot)
!  error          (danger red, bold)
…  not configured (muted gray)
```

Bottles always carry a badge. Recipes carry one too while they're applying.

## Empty states

Three things, always: a one-line title, a one-sentence rationale, a primary
action that creates the missing thing. Never a full-page illustration that
takes attention away from the action.

## Things we deliberately don't do

- **No glassmorphism / heavy blur.** Performance + readability cost.
- **No huge hero sections** on the home screen. Vinaria isn't a marketing
  site; the home screen is the bottle list.
- **No emojis as UI elements** except in the brand mark (🍷).
- **No mystery-meat icons.** Every icon has a text label next to it.

## Open design questions (phase-xp)

- Sidebar collapsible? Probably not in v1 — desktop window is wide.
- Multi-window? Definitely not. One Vinaria window.
- Tray icon? Maybe v2.
