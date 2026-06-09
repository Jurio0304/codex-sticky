# Codex Sticky Patchset

Codex Sticky is an unofficial lightweight fork of `openai/codex`; it is not
maintained, sponsored, or endorsed by OpenAI. The goal is a thin, low-cost
patchset that can periodically sync with official stable Codex releases.

## Release Baseline

- Official repository: `openai/codex`
- Sticky fork: `Jurio0304/codex-sticky`
- Release branch: `release/0.138.0-sticky.1`
- Upstream base tag: `rust-v0.138.0`
- Upstream base commit: `c18e9f478bc940ef1ef8e1c426364c0fe3d86b73`
- Sticky release version: `0.138.0-sticky.1`

`0.138.0-sticky.1` means OpenAI Codex `rust-v0.138.0` plus the first Sticky
enhancement patchset for that upstream version. Formal Sticky release tags omit
a `v` prefix. Preserve the historical `v0.1.0` tag; never delete, overwrite, or
reuse it for the current scheme.

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
