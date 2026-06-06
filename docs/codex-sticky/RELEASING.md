# Codex Sticky Releasing

Codex Sticky releases are unofficial fork releases; they are not OpenAI releases
and are not maintained, sponsored, or endorsed by OpenAI.

## Version Name

Use:

```text
<upstream-version>-sticky.<revision>
```

For this branch, the release version is `0.137.0-sticky.1`:

- `0.137.0` maps to the official OpenAI Codex tag `rust-v0.137.0`.
- `-sticky.1` means the first Sticky enhancement patchset on top of that tag.

Do not use a `v` prefix for new Sticky releases. Preserve historical `v0.1.0`;
never delete, overwrite, or reuse it for the new scheme.

## Expected Tag

Only after review and validation, a maintainer may create the local/remote tag:

```text
0.137.0-sticky.1
```

The release workflow rejects any `v`-prefixed Sticky tag, prerelease strings,
and revision `0`.

## Artifacts

This first-stage workflow builds one Linux x86_64 GNU package:

```text
codex-sticky-0.137.0-sticky.1-x86_64-unknown-linux-gnu.tar.gz
SHA256SUMS
```

The archive is flat and contains:

```text
codex-sticky
LICENSE
NOTICE
```

`SHA256SUMS` records the archive basename so users can run
`sha256sum -c SHA256SUMS` in the download directory.

## Install Path

Do not replace official `codex`. Install this fork side by side:

```bash
mkdir -p ~/.local/bin
tar -xzf codex-sticky-0.137.0-sticky.1-x86_64-unknown-linux-gnu.tar.gz
install -m 0755 codex-sticky ~/.local/bin/codex-sticky
~/.local/bin/codex-sticky
```

Or use the installer:

```bash
curl -fsSL https://raw.githubusercontent.com/Jurio0304/codex-sticky/main/scripts/install.sh -o /tmp/codex-sticky-install.sh
CODEX_STICKY_VERSION=0.137.0-sticky.1 bash /tmp/codex-sticky-install.sh
```

## Boundaries

A Sticky release must not push to `upstream`, modify OpenAI releases, delete old
tags, overwrite `v0.1.0`, publish hidden artifacts, or copy the official complex
multi-platform pipeline without a clear maintenance need.
