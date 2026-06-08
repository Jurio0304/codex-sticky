# Codex Sticky Transcript

Codex Sticky Transcript is an unofficial community fork of `openai/codex`.
It adds an optional Sticky Transcript TUI mode while keeping the upstream
Codex CLI source and behavior available. This project is not maintained or
endorsed by OpenAI.

## Entry Points

The normal Codex entry point is unchanged:

```bash
codex
```

`codex` keeps the official Codex CLI behavior. With the default configuration,
`tui.sticky_transcript` is `false`.

## Sticky Mode

The `codex-sticky` entry point starts Codex with Sticky Transcript enabled by
default for that process:

```bash
codex-sticky
```

It uses the existing CLI override syntax:

```bash
-c 'tui.sticky_transcript=true'
```

This keeps `codex` and `codex-sticky` side by side. The wrapper does not
overwrite the official `codex` command.

## Sticky Transcript Mode

Sticky Transcript keeps the native composer fixed at the bottom while the
transcript above it scrolls independently. It is useful when reading earlier
assistant output while composing the next request.

Supported controls include:

- `PageUp` and `PageDown` to browse the transcript.
- `Ctrl+End` to jump back to the newest output and resume follow-tail.
- Mouse wheel scrolling inside the transcript viewport.
- Mouse drag selection inside the transcript viewport.
- A low-noise new-output hint when streaming continues while scrolled away from
  the bottom.

Sticky Transcript and raw output mode are intentionally mutually exclusive. If
raw mode is enabled, `/sticky on` is rejected. If Sticky Transcript is enabled,
`/raw on` is rejected.

## Session Commands

Sticky Transcript can be toggled without restarting or clearing the session:

```text
/sticky
/sticky on
/sticky off
/sticky status
```

These commands are local TUI commands. They do not submit a model request and
do not modify the current composer draft.

The `/sticky` command does not persist changes. To make Sticky Transcript the
default for a Codex config, edit `~/.codex/config.toml` manually:

```toml
[tui]
sticky_transcript = true
```

## Mouse Selection And Clipboard

Sticky mode enables a narrow mouse-reporting setup for the transcript viewport:

```text
CSI ? 1006 h
CSI ? 1002 h
OSC 22 ; text ST
```

This enables SGR mouse coordinates and button, drag, and wheel events without
enabling all-motion reporting. The mouse wheel scrolls the transcript. A
left-button drag highlights transcript text; releasing the button copies the
selected plain text through OSC 52:

```text
OSC 52 ; c ; <base64 payload> BEL
```

The copy path is only triggered by completing a drag selection in the Sticky
transcript viewport or the bottom composer. It does not read the clipboard or
run in the background. Transcript selection does not include the composer,
unread hint, layout spacer, ANSI styling, or selection highlight; composer
selection copies only the selected draft text.

While mouse reporting is enabled, many terminals show the pointer as a default
arrow instead of the native text-selection I-beam. Sticky sends OSC 22 `text`
as a best-effort pointer shape hint and resets it to `default` when mouse
reporting is disabled, but support is terminal-specific.

## tmux And OSC 52

Clipboard forwarding depends on both the terminal and any terminal multiplexer
in the path. If copying works outside tmux but not inside tmux, inspect the tmux
server:

```bash
tmux show -s set-clipboard
tmux info | grep 'Ms:'
tmux -V
```

For tmux sessions that allow applications inside tmux to update the outer
terminal clipboard, this is commonly required:

```tmux
set -s set-clipboard on
```

Also ensure the `Ms` capability is present. Sticky wraps OSC 52 for tmux
passthrough when `$TMUX` is set. If a tmux build or configuration blocks
passthrough, `set -g allow-passthrough on` may also be required. Selection
highlighting still works when forwarding is blocked, but clipboard forwarding
must be fixed in tmux or terminal configuration.

## Build And Install From Source

This project currently documents source builds only. Do not assume a prebuilt
GitHub Release binary exists until one is explicitly published.

```bash
git clone <public-repository>
cd codex-sticky
cargo build --manifest-path codex-rs/Cargo.toml --release --bin codex
bash scripts/install-codex-sticky.sh
codex-sticky
```

The install script copies the locally built binary to:

```text
~/.local/libexec/codex-sticky-bin
```

and writes the wrapper to:

```text
~/.local/bin/codex-sticky
```

Confirm that `codex` and `codex-sticky` resolve to different entries before
relying on both commands:

```bash
which codex
which codex-sticky
```

## Uninstall

Remove the Sticky wrapper and its copied binary:

```bash
rm -f ~/.local/bin/codex-sticky
rm -f ~/.local/libexec/codex-sticky-bin
```

This does not remove or modify the official `codex` command.

## Build Cache Cleanup

Source builds create a Rust `target/` directory under `codex-rs/`. To reclaim
disk space after building:

```bash
cd codex-rs
cargo clean
```

## Syncing Upstream

When maintaining a fork, keep the upstream remote pointing at `openai/codex` and
sync from upstream before rebuilding:

```bash
git fetch upstream
git rebase upstream/main
bash scripts/install-codex-sticky.sh
```

## Validation Scope

Tested:

- Linux x86_64
- SSH terminals
- Termius
- tmux 3.6a

Best effort:

- Other OSC 52 compatible terminals
- Other tmux versions
- macOS
- WSL2

Not fully validated:

- Native Windows legacy console

## Known Limits

- OSC 52 clipboard copy depends on terminal support.
- tmux may require `set-clipboard`, the `Ms` capability, and passthrough support.
- Mouse reporting must reach Codex for wheel scrolling and drag selection.
- `PageUp`, `PageDown`, and `Ctrl+End` remain the primary keyboard controls.
- Alternate-screen behavior is unchanged.
- Sticky Transcript only changes the in-TUI transcript viewport and leaves the
  native composer, status rows, popups, approvals, file search, shell commands,
  MCP, skills, plugins, hooks, and subagents on their existing code paths.
