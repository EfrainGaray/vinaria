# Contributing to Vinaria

Vinaria is in **phase xp** (experimental). Things move fast and the public API
is allowed to break. That said, here's how to land changes that we'll merge.

## Ground rules

- **English** in code, commits, branches, PRs, and docs.
- **Conventional commits**: `feat:`, `fix:`, `chore:`, `docs:`, `refactor:`,
  `test:`, `perf:`. Scope when it helps (`feat(core): …`, `fix(scripts): …`).
- **Atomic commits**: each commit should build by itself and represent one
  logical change. We rebase, not squash-on-merge.
- **No vendored Wine binaries.** Wine is downloaded and compiled by
  `./scripts/build-wine.sh`. Do not commit prebuilt `.dll` / `.drv` files.
- **No proprietary code.** Studying CrossOver locally is fine — copying any of
  its non-LGPL code into the repo is not.

## Workflow

1. Fork on GitHub, branch from `main`.
2. Make the change. If it touches the build pipeline, run the affected
   `./scripts/*.sh` end-to-end on your machine before pushing.
3. Open a PR. Describe *why*, not just *what*.
4. Expect review comments — phase xp is opinionated about clarity even when
   the code is throwaway.

## Phase awareness

The repo's `CLAUDE.md` declares the current phase. As of phase xp, expect:

- Single-file modules.
- No exhaustive error handling on internal paths.
- Tests where they pay off, not everywhere.
- Public API may change between commits without deprecation.

When we promote to phase `solid` (Tauri shell stable, recipe schema frozen),
the bar tightens: tests required for new behaviour, no breaking changes
without a migration note, lints clean.

## Adding a recipe

For now, drop a TOML file in `recipes/`. Phase 6 splits the catalog into a
separate community repo so recipe contributions can land independently of
Vinaria releases.

## Reporting bugs

Open a GitHub issue with: macOS version, CPU (Intel/Silicon), the recipe or
app you were trying to run, the command you executed, and the last 50 lines
of the relevant log from `~/.vinaria/logs/`.
