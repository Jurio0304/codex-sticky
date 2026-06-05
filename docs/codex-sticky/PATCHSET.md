# Codex Sticky Patchset

Codex Sticky is an unofficial lightweight fork of `openai/codex`; it is not
maintained, sponsored, or endorsed by OpenAI. The goal is a thin, low-cost
patchset that can periodically sync with official stable Codex releases.

## Baseline

- Official repository: `openai/codex`
- Sticky fork: `Jurio0304/codex-sticky`
- `upstream/main`: `55aa071b17c825bdb66fac99cde2e7a7acfbdee7`
- `origin/main` / Sticky main: `25ac28fa5ef0052d8c82aa410606abc532a38a2c`
- Latest known stable tag from local history:
  `codex-rs-6a8a936f75ea44faf05ff4fab0c6a36fc970428d-1-rust-v0.0.2506261603`

The stable tag was selected from local tags merged into `upstream/main`,
excluding prerelease markers such as `alpha`, `beta`, `rc`, and `pre`. Confirm
with GitHub before making release decisions if the local tag list may be stale.

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
