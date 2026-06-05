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
- Release: `contents: write` only to create a release for an existing Sticky tag.
- Upstream watch: `contents: read`, `issues: write` only for reminder issues.

Preserve historical tags, especially `v0.1.0`. New release tags use
`v<upstream-version>-sticky.<revision>` and must not overwrite or delete old
Sticky tags.

## Automation Boundaries

Workflows and scripts must not push branches, force-push, merge upstream, create
PRs, create tags, upload hidden artifacts, modify GitHub settings, or publish a
release except for the explicit tag-triggered Sticky release workflow.
