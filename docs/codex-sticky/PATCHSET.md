# Codex Sticky Patchset

Codex Sticky is an unofficial lightweight fork of `openai/codex`; it is not
maintained, sponsored, or endorsed by OpenAI. The goal is a thin, low-cost
patchset that can periodically sync with official stable Codex releases.

## Baseline

- Official repository: `openai/codex`
- Sticky fork: `Jurio0304/codex-sticky`
- Synced ref: `upstream/main`
- Synced commit: `55aa071b17c825bdb66fac99cde2e7a7acfbdee7`
- `origin/main` / Sticky main: `25ac28fa5ef0052d8c82aa410606abc532a38a2c`
- GitHub latest stable release: `rust-v0.137.0` (`0.137.0`)

The current Sticky code comes from an initial `upstream/main` migration, not a
standard sync from `rust-v0.137.0`. The latest stable release value is recorded
from the GitHub Releases API for future sync planning and must not be described
as the exact base tag of the current code.

Formal Sticky release tags omit a `v` prefix. For example, `0.137.0-sticky.1`
means OpenAI Codex `rust-v0.137.0` plus the first Sticky enhancement patchset.
That release is prepared from a release branch based on `rust-v0.137.0`, not by
tagging the current initial-migration `main`.

## Release Target

The current minimal release workflow builds `x86_64-unknown-linux-gnu`. A local
`x86_64-unknown-linux-musl` validation failed because the Rusty V8 musl prebuilt
archive was unavailable or unstable during download, so musl is deferred until it
can be re-evaluated without adding maintenance complexity.

## Patch Areas

Keep fork changes concentrated in:

- Sticky Transcript TUI behavior and tests,
- the `codex-sticky` entry point or install wrapper,
- minimal maintenance scripts and GitHub Actions,
- fork-specific notes under `docs/codex-sticky/`.

## Non-Goals

Do not replace official `codex`, chase every `upstream/main` commit, copy the
full official multi-platform release pipeline without need, add broad product
docs to shared `docs/`, or delete/overwrite the historical `v0.1.0` tag.

## Review Checklist

Before merging, confirm the diff is thin, official Codex remains available as
`codex`, automation does not push/merge/tag unexpectedly, and any public surface
has a clear maintenance reason.
