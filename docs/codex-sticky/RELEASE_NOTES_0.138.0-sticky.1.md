# Codex Sticky 0.138.0-sticky.1

Unofficial Codex Sticky build based on OpenAI Codex CLI `0.138.0`.

## Highlights

- Adds the Sticky Transcript patchset on top of upstream `0.138.0`.
- Keeps the official `codex` command untouched and installs this fork as `codex-sticky`.
- Fixes Sticky Transcript mouse-selection copying so the last selected character or punctuation mark is included.
- Handles forward and backward Sticky Transcript selections consistently, including Unicode display columns.

## Assets

Upload these files manually to the GitHub Release:

- `codex-sticky-0.138.0-sticky.1-x86_64-unknown-linux-gnu.tar.gz`
- `SHA256SUMS`

SHA256:

```text
c3d2d9757d18da27edd283027d414d34c85e7f3b160d578f0bcd89a20a5ffccd  codex-sticky-0.138.0-sticky.1-x86_64-unknown-linux-gnu.tar.gz
```

## Archive Contents

```text
codex-sticky
libexec/codex-sticky-bin
LICENSE
NOTICE
```

## Install

```bash
mkdir -p ~/.local/bin ~/.local/libexec
tar -xzf codex-sticky-0.138.0-sticky.1-x86_64-unknown-linux-gnu.tar.gz
install -m 0755 codex-sticky ~/.local/bin/codex-sticky
install -m 0755 libexec/codex-sticky-bin ~/.local/libexec/codex-sticky-bin
~/.local/bin/codex-sticky --version
```

Expected version output:

```text
codex-sticky 0.138.0-sticky.1
```

The wrapped upstream binary still reports:

```text
codex-cli 0.138.0
```

## Verification

```bash
sha256sum -c SHA256SUMS
```
