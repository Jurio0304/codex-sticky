# Codex Sticky Releasing

Codex Sticky releases are unofficial fork releases; they are not OpenAI releases
and are not maintained, sponsored, or endorsed by OpenAI.

## Version Name

Use:

```text
<upstream-version>-sticky.<revision>
```

For the first formal release, `0.137.0-sticky.1` means:

- `0.137.0` maps to the official OpenAI Codex tag `rust-v0.137.0`.
- `-sticky.1` is the first Sticky enhancement patchset on top of that tag.

Do not use a `v` prefix for new Sticky releases. Preserve historical `v0.1.0`;
never delete, overwrite, or reuse it for the new scheme.

## Manual Release Flow

Codex Sticky uses local builds and manual GitHub Release uploads. The former GitHub Actions release workflow has been removed because the first
tag-triggered build was not stable enough for this fork.

Release steps:

```text
1. Create and push a fixed release tag such as 0.137.0-sticky.1.
2. Build and package locally on the maintainer machine.
3. Verify SHA256SUMS and archive contents locally.
4. Open GitHub Releases and choose Draft a new release.
5. Select the existing tag.
6. Upload the tar.gz archive and SHA256SUMS.
7. Publish release after verifying the draft metadata and assets.
```

Do not overwrite an already published tag. If a release needs a rebuild after
publication, use a new Sticky revision such as `0.137.0-sticky.2`.

## Artifacts

The current package scope is one Linux x86_64 GNU archive:

```text
codex-sticky-0.137.0-sticky.1-x86_64-unknown-linux-gnu.tar.gz
SHA256SUMS
```

The archive contains:

```text
codex-sticky
libexec/codex-sticky-bin
LICENSE
NOTICE
```

`SHA256SUMS` records the archive basename so users can run
`sha256sum -c SHA256SUMS` in the download directory.

## Install Path

Do not replace official `codex`. Install this fork side by side:

```bash
mkdir -p ~/.local/bin ~/.local/libexec
tar -xzf codex-sticky-0.137.0-sticky.1-x86_64-unknown-linux-gnu.tar.gz
install -m 0755 codex-sticky ~/.local/bin/codex-sticky
install -m 0755 libexec/codex-sticky-bin ~/.local/libexec/codex-sticky-bin
chmod 0755 ~/.local/bin/codex-sticky
~/.local/bin/codex-sticky --version
```

Or use the installer:

```bash
curl -fsSL https://raw.githubusercontent.com/Jurio0304/codex-sticky/main/scripts/install.sh -o /tmp/codex-sticky-install.sh
CODEX_STICKY_VERSION=0.137.0-sticky.1 bash /tmp/codex-sticky-install.sh
```

## Boundaries

A Sticky release must not push to `upstream`, modify OpenAI releases, delete old
tags, overwrite `v0.1.0`, overwrite published Sticky tags, publish hidden
artifacts, or copy the official complex multi-platform pipeline without a clear
maintenance need.
