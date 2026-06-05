# Codex Sticky Releasing

Codex Sticky releases are unofficial fork releases; they are not OpenAI releases
and are not maintained, sponsored, or endorsed by OpenAI.

## Version Names

Use:

```text
v<upstream-version>-sticky.<revision>
```

The upstream version must come from an official stable tag that was explicitly
synced into Sticky main. For example, after syncing `rust-v0.138.0`, the first
Sticky release would be `v0.138.0-sticky.1`. Preserve historical `v0.1.0`;
never delete, overwrite, or reuse it for the new scheme.

The current Sticky `main` is an initial `upstream/main` migration, not a
standard release based on `rust-v0.137.0`. Do not create a current release tag
and do not name this code `v0.137.0-sticky.1`.

## Preflight

Before creating a release tag:

```bash
bash scripts/sticky/check-release.sh v<upstream-version>-sticky.<revision>
```

The script is only a guardrail. It must not push tags, create GitHub Releases,
upload binaries, build artifacts, or modify GitHub settings.

## Artifacts

Start with one Linux x86_64 package using `x86_64-unknown-linux-gnu`. Local
validation of `x86_64-unknown-linux-musl` failed because the Rusty V8 musl
prebuilt archive was unavailable or unstable during download, so the first-stage
Sticky release workflow uses the host GNU build path. Re-evaluate musl later only
if the Rusty V8 assets become reliable.

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
tar -xzf codex-sticky-<release-tag>-x86_64-unknown-linux-gnu.tar.gz
install -m 0755 codex-sticky ~/.local/bin/codex-sticky
~/.local/bin/codex-sticky
```

## Boundaries

A Sticky release must not push to `upstream`, modify OpenAI releases, delete old
tags, overwrite `v0.1.0`, publish hidden artifacts, or copy the official complex
multi-platform pipeline without a clear maintenance need.
