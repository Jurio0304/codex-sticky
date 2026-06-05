> **Unofficial lightweight fork: Codex Sticky**
>
> This repository is an unofficial community fork of `openai/codex`.
> It adds an optional Sticky Transcript TUI mode and is not maintained,
> sponsored, or endorsed by OpenAI.
>
> - `origin/main`: Sticky's long-term maintenance branch.
> - `upstream/main`: OpenAI's official baseline for comparison and syncs.
> - `codex`: official Codex CLI behavior remains available.
> - `codex-sticky`: Sticky Transcript enabled by default, installed side by
>   side at `~/.local/bin/codex-sticky` without replacing official `codex`.
>
> Sticky maintenance tracks official stable tags periodically instead of
> continuously chasing `upstream/main`. Current Sticky code comes from an
> initial `upstream/main` migration at
> `55aa071b17c825bdb66fac99cde2e7a7acfbdee7`; it is not a standard release
> based on the current GitHub stable release `rust-v0.137.0`.
> Future standardized Sticky releases should first sync an official stable tag.
>
> See [`docs/codex-sticky.md`](docs/codex-sticky.md) for user-facing Sticky
> behavior and [`docs/codex-sticky/`](docs/codex-sticky/) for fork maintenance.

<p align="center"><strong>Codex CLI</strong> is a coding agent from OpenAI that runs locally on your computer.
<p align="center">
  <img src="https://github.com/openai/codex/blob/main/.github/codex-cli-splash.png" alt="Codex CLI splash" width="80%" />
</p>
</br>
If you want Codex in your code editor (VS Code, Cursor, Windsurf), <a href="https://developers.openai.com/codex/ide">install in your IDE.</a>
</br>If you want the desktop app experience, run <code>codex app</code> or visit <a href="https://chatgpt.com/codex?app-landing-page=true">the Codex App page</a>.
</br>If you are looking for the <em>cloud-based agent</em> from OpenAI, <strong>Codex Web</strong>, go to <a href="https://chatgpt.com/codex">chatgpt.com/codex</a>.</p>

---

## Quickstart

### Installing Codex Sticky from this fork

When a Sticky release is available, download the `codex-sticky` archive from
this fork's GitHub Releases and install it under your user-local bin directory:

```shell
mkdir -p ~/.local/bin
tar -xzf codex-sticky-<release-tag>-x86_64-unknown-linux-gnu.tar.gz
install -m 0755 codex-sticky ~/.local/bin/codex-sticky
~/.local/bin/codex-sticky
```

This intentionally does not overwrite the official `codex` command. Keep both
commands side by side if you use the official CLI too.

### Installing and running official Codex CLI

Run the following on Mac or Linux to install official Codex CLI:

```shell
curl -fsSL https://chatgpt.com/codex/install.sh | sh
```

Run the following on Windows to install official Codex CLI:

```
powershell -ExecutionPolicy ByPass -c "irm https://chatgpt.com/codex/install.ps1 | iex"
```

Official Codex CLI can also be installed via the following package managers:

```shell
# Install using npm
npm install -g @openai/codex
```

```shell
# Install using Homebrew
brew install --cask codex
```

Then simply run `codex` to get started with official Codex CLI.

<details>
<summary>You can also go to the <a href="https://github.com/openai/codex/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `codex-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `codex-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `codex-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `codex-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `codex-x86_64-unknown-linux-musl`), so you likely want to rename it to `codex` after extracting it.

</details>

### Using Codex with your ChatGPT plan

Run `codex` and select **Sign in with ChatGPT**. We recommend signing into your ChatGPT account to use Codex as part of your Plus, Pro, Business, Edu, or Enterprise plan. [Learn more about what's included in your ChatGPT plan](https://help.openai.com/en/articles/11369540-codex-in-chatgpt).

You can also use Codex with an API key, but this requires [additional setup](https://developers.openai.com/codex/auth#sign-in-with-an-api-key).

## Docs

- [**Codex Documentation**](https://developers.openai.com/codex)
- [**Sticky Transcript user notes**](./docs/codex-sticky.md)
- [**Sticky patchset**](./docs/codex-sticky/PATCHSET.md)
- [**Sticky maintenance**](./docs/codex-sticky/MAINTENANCE.md)
- [**Sticky releasing**](./docs/codex-sticky/RELEASING.md)
- [**Sticky GitHub settings**](./docs/codex-sticky/GITHUB_SETTINGS.md)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)
- [**Open source fund**](./docs/open-source-fund.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
