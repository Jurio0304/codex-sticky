# Codex Sticky Patchset

Codex Sticky is an unofficial lightweight fork of `openai/codex`; it is not
maintained, sponsored, or endorsed by OpenAI. This release branch keeps the
patchset narrow so future stable upstream releases can be reviewed with low
maintenance cost.

## Release Baseline

- Official repository: `openai/codex`
- Sticky fork: `Jurio0304/codex-sticky`
- Release branch: `release/0.137.0-sticky.1`
- Upstream base tag: `rust-v0.137.0`
- Upstream base commit: `f221438b691b8f749d98f22077c93ebe01923fbe`
- Sticky release version: `0.137.0-sticky.1`

`0.137.0-sticky.1` means OpenAI Codex `rust-v0.137.0` plus the first Sticky
enhancement patchset for that upstream version. Formal Sticky release tags omit
the `v` prefix. Preserve the historical `v0.1.0` tag; never delete, overwrite,
or reuse it for the new scheme.

## Included Patch Areas

This release branch intentionally carries only:

- Sticky Transcript TUI behavior and tests,
- the minimal Linux x86_64 GNU release workflow,
- the `scripts/install.sh` installer for published release assets,
- release/user notes under `docs/codex-sticky/`, and
- README installation/disclaimer notes for this fork.

It does not merge Sticky `main`, `chore/sticky-maintenance-infra`, or unrelated
`upstream/main` development commits wholesale.

## Release Target

The release workflow builds `x86_64-unknown-linux-gnu`. Local validation of
`x86_64-unknown-linux-musl` failed because the Rusty V8 musl prebuilt archive
was unavailable or unstable during download, so musl is deferred until it can be
re-evaluated without adding maintenance complexity.

## Non-Goals

Do not replace official `codex`, copy the full official multi-platform release
pipeline, push tags automatically, upload hidden artifacts, or publish anything
without an explicit human release decision.
