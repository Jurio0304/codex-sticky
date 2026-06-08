<div align="center">
  <img src="docs/assets/readme/codex-sticky-logo.svg" alt="Codex Sticky logo" width="96" />
  <h1>Codex Sticky</h1>
  <p><a href="./README.zh-CN.md">简体中文</a> | English</p>
  <p><strong>A lightweight terminal-first enhancement for OpenAI Codex CLI.</strong></p>
  <p>
    <img alt="Terminal-first" src="https://img.shields.io/badge/terminal--first-0f766e" />
    <img alt="Linux x86_64 GNU" src="https://img.shields.io/badge/Linux-x86__64%20GNU-111827" />
    <img alt="Unofficial fork" src="https://img.shields.io/badge/unofficial-fork-f97316" />
  </p>
</div>

Codex Sticky keeps the Codex experience close to upstream while making long terminal sessions easier to drive: scroll back through older transcript content without losing access to the composer, stay productive over SSH and tmux, and install it side by side with the official `codex` command.

## 📰 News

2026-06-08 🖱️ `0.137.0-sticky.2` adds bottom-composer mouse drag selection/copy, sticky mouse routing, and a best-effort text pointer hint.

2026-06-05 📌 `0.137.0-sticky.1` introduced Sticky Transcript as a side-by-side Codex CLI release for Linux x86_64 GNU.

## ✨ Core Features

- Keeps the composer/input area reachable while browsing older transcript content.
- Improves long-conversation usability in pure terminal sessions.
- Fits SSH, tmux, remote server, and terminal-first development workflows.
- Installs as `codex-sticky` without overwriting the official `codex` binary.
- Tracks upstream with a thin patchset to keep maintenance cost low.

## 🎬 Demo

【TODO: Add terminal demo GIF at `docs/assets/readme/codex-sticky-demo.gif`】

【TODO: Add Sticky Transcript before/after comparison at `docs/assets/readme/sticky-transcript-before-after.png`】

【TODO: Add tmux / SSH demo video thumbnail at `docs/assets/readme/tmux-ssh-demo-thumbnail.png`】

No missing images are referenced yet.

## 👤 Who It Is For

Codex Sticky is most useful if you:

- spend most of your development time in a terminal;
- use Codex over SSH, inside tmux, or on a remote Linux server;
- often work with long agent conversations and need to read earlier output;
- want a small side-by-side enhancement instead of replacing your official Codex CLI install.

## 🧭 Current Support And Limits

Current formal release: `0.137.0-sticky.2`.

- `0.137.0` means the release is based on OpenAI Codex `0.137.0` / upstream tag `rust-v0.137.0`.
- `-sticky.2` means the second Sticky enhancement revision on top of that upstream version.

Supported today:

- Linux x86_64
- `x86_64-unknown-linux-gnu`
- terminal, SSH, tmux, and remote-server use cases

Not provided today:

- macOS prebuilt package
- Windows prebuilt package
- Linux ARM64 prebuilt package
- musl static package
- automatic updater

The GNU package expects a normal Linux glibc environment. Codex Sticky may lag behind the latest OpenAI Codex release; this project syncs selected stable upstream versions in stages instead of tracking every upstream commit.

## 🚀 Quick Start

### 0. Install Official Codex CLI First

Recommended flow:

1. Install and verify the official OpenAI Codex CLI.
2. Install Codex Sticky after official `codex` works.
3. Keep official `codex` and `codex-sticky` installed side by side.
4. Run `codex` or `codex-sticky` depending on the session.

This README intentionally does not duplicate the official installation guide. Use the official sources instead:

- OpenAI Codex repository: <https://github.com/openai/codex>
- Official Codex install/build docs: <https://github.com/openai/codex/blob/main/docs/install.md>
- OpenAI Codex developer docs: <https://developers.openai.com/codex>

### Option A: Ask Codex To Install It For You

If you can already run the official Codex CLI, the easiest path is to give Codex this prompt and let it install and verify Codex Sticky for you:

```text
Please help me install codex-sticky. Requirements:
1. Do not overwrite or uninstall my existing official codex.
2. Download the Linux x86_64 GNU archive and SHA256SUMS from the latest formal GitHub Release of Jurio0304/codex-sticky.
3. Verify SHA256.
4. Extract and install it as ~/.local/bin/codex-sticky.
5. If ~/.local/bin is not already in PATH, tell me how to configure it, but do not edit my shell config without confirmation.
6. Run codex-sticky --version to verify the installation.
7. Finally report whether official codex and codex-sticky can coexist.
```

### Option B: Use The Install Script

Safer review-first install:

```bash
curl -fsSL https://raw.githubusercontent.com/Jurio0304/codex-sticky/main/scripts/install.sh \
  -o install-codex-sticky.sh

less install-codex-sticky.sh
bash install-codex-sticky.sh
```

Quick install:

```bash
curl -fsSL https://raw.githubusercontent.com/Jurio0304/codex-sticky/main/scripts/install.sh | bash
```

The review-first form is recommended if you prefer to inspect a shell script before running it. The installer downloads the current Linux x86_64 GNU package, verifies `SHA256SUMS`, and writes only `~/.local/bin/codex-sticky`. It does not install or overwrite a binary named `codex`.

To pin the current release explicitly:

```bash
curl -fsSL https://raw.githubusercontent.com/Jurio0304/codex-sticky/main/scripts/install.sh \
  -o install-codex-sticky.sh
CODEX_STICKY_VERSION=0.137.0-sticky.2 bash install-codex-sticky.sh
```

### Option C: Install The Release Package Manually

Download the release assets from:

<https://github.com/Jurio0304/codex-sticky/releases/tag/0.137.0-sticky.2>

You need:

```text
codex-sticky-0.137.0-sticky.2-x86_64-unknown-linux-gnu.tar.gz
SHA256SUMS
```

Verify and install:

```bash
sha256sum -c SHA256SUMS

mkdir -p ~/.local/bin
tar -xzf codex-sticky-0.137.0-sticky.2-x86_64-unknown-linux-gnu.tar.gz
install -m 755 codex-sticky ~/.local/bin/codex-sticky

~/.local/bin/codex-sticky --version
```

If `~/.local/bin` is not on your `PATH`, add it in your shell profile:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

### Option D: Optional Alias

By default, Codex Sticky does not replace official Codex CLI. If you intentionally want `codex` to start Codex Sticky in your shell, you can add an alias yourself:

```bash
alias codex='codex-sticky'
```

This is optional. The default install keeps official `codex` untouched.

## 🔀 Running And Switching

Run official Codex CLI:

```bash
codex
```

Run Codex Sticky:

```bash
codex-sticky
```

Verify both commands resolve separately:

```bash
which codex
which codex-sticky
codex --version
codex-sticky --version
```

Temporarily switch by choosing the command for the current session. If you added an alias and want to bypass it once, use `command codex` or remove the alias in that shell.

Inside the TUI, Sticky Transcript can also be toggled for the current session:

```text
/sticky
/sticky on
/sticky off
/sticky status
```

## ⬆️ Updating

Use the installer again when a later Sticky release is published:

```bash
curl -fsSL https://raw.githubusercontent.com/Jurio0304/codex-sticky/main/scripts/install.sh | bash
```

To install a specific Sticky version, reuse the same script and set `CODEX_STICKY_VERSION`.

There is no automatic updater today. Check GitHub Releases for new versions:

<https://github.com/Jurio0304/codex-sticky/releases>

## 🧹 Uninstalling

Remove the side-by-side binary:

```bash
rm ~/.local/bin/codex-sticky
```

If you added an alias, remove that alias from your shell profile too. The official `codex` binary is not removed by this command.

## 🔄 Upstream Sync Policy

Codex Sticky aims to stay close to `openai/codex` while preserving a small terminal-workflow patchset. It does not chase every upstream commit. Instead, maintainers periodically choose stable upstream releases, review the delta, and publish a Sticky revision when the patchset is ready.

This keeps the fork lightweight, but it also means Codex Sticky can be behind the latest OpenAI Codex version.

## ❓ FAQ

### 1. Will it overwrite official `codex`?

No. The installer writes `~/.local/bin/codex-sticky` and does not install or overwrite a binary named `codex`.

### 2. Why install official Codex CLI first?

Codex Sticky is a small enhancement, not a full replacement. Installing official Codex first confirms that your account, authentication, model access, and base CLI workflow already work before you add this side-by-side binary.

### 3. Why only Linux x86_64 GNU right now?

The first formal release focuses on the environment this fork is meant to serve best: terminal, SSH, tmux, and remote Linux server workflows. macOS, Windows, ARM64, musl, and broader release automation are deferred until they can be supported without expanding maintenance cost too much.

### 4. Why does `codex-sticky --version` still show `codex-cli 0.137.0`?

The binary is based on the upstream Codex CLI version, so the CLI version output may still show the upstream package identity. The Sticky version is tracked by the GitHub Release tag and asset name, for example `0.137.0-sticky.2`.

### 5. Will Codex Sticky sync OpenAI Codex updates?

Yes, but in planned stages rather than every upstream commit.

## ⚠️ Disclaimer

Codex Sticky is an unofficial community fork of OpenAI Codex CLI. It is not an OpenAI product and is not maintained, sponsored, endorsed, or supported by OpenAI. Use the official OpenAI Codex CLI and documentation for the authoritative upstream project.

## 📄 License

This repository is licensed under the [Apache-2.0 License](LICENSE).
