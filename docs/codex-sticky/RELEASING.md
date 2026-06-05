# Codex Sticky Releasing

Codex Sticky releases are unofficial fork releases; they are not OpenAI releases
and are not maintained, sponsored, or endorsed by OpenAI.

## Version Names

Use:

```text
v<upstream-version>-sticky.<revision>
```

Example: `v0.0.2506261603-sticky.1`. The upstream version comes from the synced
official stable tag; the Sticky revision counts rebuilds or fork fixes for that
same upstream version. Preserve historical `v0.1.0`; never delete, overwrite, or
reuse it for the new scheme.

## Preflight

Before creating a release tag:

```bash
bash scripts/sticky/check-release.sh v<upstream-version>-sticky.<revision>
```

The script is only a guardrail. It must not push tags, create GitHub Releases,
upload binaries, build artifacts, or modify GitHub settings.

## Artifacts

Start with one Linux x86_64 package. Prefer `x86_64-unknown-linux-musl`; if musl
becomes unreliable, use `x86_64-unknown-linux-gnu` and explain the reason in the
release notes.

Each archive contains:

```text
codex-sticky
LICENSE
NOTICE
```

Publish `SHA256SUMS` for downloadable artifacts.

## Install Path

Do not replace official `codex`. Install this fork side by side:

```bash
mkdir -p ~/.local/bin
tar -xzf codex-sticky-<target>.tar.gz
install -m 0755 codex-sticky ~/.local/bin/codex-sticky
~/.local/bin/codex-sticky
```

## Boundaries

A Sticky release must not push to `upstream`, modify OpenAI releases, delete old
tags, overwrite `v0.1.0`, publish hidden artifacts, or copy the official complex
multi-platform pipeline without a clear maintenance need.
