# Codex Sticky Transcript

Sticky Transcript is an optional Codex TUI layout mode. It keeps the native composer fixed at the bottom while PageUp, PageDown, Ctrl+End, mouse wheel scrolling, and mouse drag selection operate on the transcript viewport above it. This is useful when you are reading earlier assistant output while composing the next request.

## Native Mode

The normal entry point is unchanged:

```bash
codex
```

With the default configuration, `tui.sticky_transcript` is `false`, so Codex keeps the native transcript and terminal scrollback behavior.

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

## Session Toggle

Inside the TUI, Sticky Transcript can be toggled without restarting or clearing the session:

```text
/sticky
/sticky on
/sticky off
/sticky status
```

These commands are local TUI commands. They do not submit a model request and do not modify the current composer draft.

## Mouse And Clipboard

Sticky mode uses a narrow mouse-reporting setup for the transcript viewport:

```text
CSI ? 1006 h
CSI ? 1002 h
OSC 22 ; text ST
```

This enables SGR mouse coordinates and button/drag/wheel events without enabling all-motion reporting. The mouse wheel scrolls the transcript. A left-button drag highlights transcript text; releasing the button copies the selected plain text through OSC 52:

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

Termius Desktop must allow OSC 52 clipboard writes for local Windows clipboard integration. If copying works without tmux but not inside tmux, check the tmux server:

```bash
tmux show -s set-clipboard
tmux info | grep 'Ms:'
tmux -V
```

For tmux sessions that allow applications inside tmux to update the outer terminal clipboard, this is commonly required:

```tmux
set -s set-clipboard on
```

Also ensure the `Ms` capability is present. Sticky wraps OSC 52 for tmux passthrough when `$TMUX` is set; if your tmux build or configuration blocks passthrough, you may also need `set -g allow-passthrough on`. Selection highlighting still works when forwarding is blocked, but clipboard forwarding must be fixed in tmux/terminal configuration.

## Persistent Config

To make Sticky Transcript the default for a Codex config, edit `~/.codex/config.toml` manually:

```toml
[tui]
sticky_transcript = true
```

The `/sticky` command does not persist changes.

## Install

Install the published Linux x86_64 GNU package side by side with official
Codex:

```bash
curl -fsSL https://raw.githubusercontent.com/Jurio0304/codex-sticky/main/scripts/install.sh | bash
```

To pin this release:

```bash
curl -fsSL https://raw.githubusercontent.com/Jurio0304/codex-sticky/main/scripts/install.sh -o /tmp/codex-sticky-install.sh
CODEX_STICKY_VERSION=0.137.0-sticky.1 bash /tmp/codex-sticky-install.sh
```

The installer downloads `codex-sticky-<version>-x86_64-unknown-linux-gnu.tar.gz`
and `SHA256SUMS`, verifies the checksum, and writes only:

```text
~/.local/bin/codex-sticky
```

## Side By Side With Official Codex

The installer does not overwrite the official `codex` entry:

```bash
which codex
which codex-sticky
```

Confirm that these are different paths before relying on both commands.

## Known Boundaries

Sticky Transcript and raw output mode are intentionally not combined in this first version. If raw mode is enabled, `/sticky on` is rejected; if Sticky Transcript is enabled, `/raw on` is rejected.

Alternate-screen behavior is unchanged. Sticky Transcript only changes the in-TUI transcript viewport and leaves the native composer, status rows, popups, approvals, file search, shell commands, MCP, skills, plugins, hooks, and subagents on their existing code paths.

In tmux and SSH, mouse wheel and drag selection depend on terminal mouse reporting reaching Codex, and clipboard copy depends on OSC 52 reaching the local terminal. PageUp, PageDown, and Ctrl+End remain the primary keyboard controls. During streaming output, scrolling away from the bottom keeps the current viewport locked and shows a low-noise new output hint until Ctrl+End or scrolling back to the bottom restores follow-tail. Terminal resize recalculates the transcript viewport and keeps the composer at the bottom.
