# GitHub Settings For Codex Sticky

Codex Sticky is an unofficial lightweight fork of `openai/codex`; it is not
maintained, sponsored, or endorsed by OpenAI. Repository settings should keep
this fork reviewable and avoid automation that changes code without a human.

## Identity

- Describe the repo as a community fork, not an OpenAI project.
- Keep the fork binary named `codex-sticky`; do not replace official `codex`.
- Point maintainers to `docs/codex-sticky/` for fork-specific operations.

## Branches, Tags, Actions

Recommended `main` protections:

- require review for non-emergency infrastructure work,
- require Sticky CI when code or maintenance config changes,
- disallow force-pushes and branch deletion,
- apply protections to administrators when practical.

Use minimal workflow permissions:

- CI: `contents: read`.
- Upstream watch: `contents: read`, `issues: write` only for reminder issues.

The former GitHub Actions release workflow has been removed. Release assets are
uploaded manually through the GitHub Release draft UI.

Preserve historical tags, especially `v0.1.0`. New release tags use
`<upstream-version>-sticky.<revision>` and must not overwrite or delete old
Sticky tags. Prepare releases from official stable tags, for example
`0.138.0-sticky.1` from `rust-v0.138.0` on a dedicated release branch, then tag
only after review.

## Automation Boundaries

Workflows and scripts must not push branches, force-push, merge upstream, create
PRs, create tags, upload hidden artifacts, modify GitHub settings, or publish a
release. Sticky Releases are drafted and published manually from existing tags.
