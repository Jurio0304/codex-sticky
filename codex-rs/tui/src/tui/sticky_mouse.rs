use std::fmt;
#[cfg(windows)]
use std::io::Result;

use crossterm::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EnableStickyMouseCapture;

impl Command for EnableStickyMouseCapture {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        // Sticky only needs SGR coordinates plus button/drag/wheel reporting.
        f.write_str("\x1b[?1006h\x1b[?1002h")
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        crossterm::event::EnableMouseCapture.execute_winapi()
    }

    #[cfg(windows)]
    fn is_ansi_code_supported(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DisableStickyMouseCapture;

impl Command for DisableStickyMouseCapture {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        // Disable button-event tracking before SGR mouse coordinates.
        f.write_str("\x1b[?1002l\x1b[?1006l")
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        crossterm::event::DisableMouseCapture.execute_winapi()
    }

    #[cfg(windows)]
    fn is_ansi_code_supported(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ansi(command: impl Command) -> String {
        let mut out = String::new();
        command.write_ansi(&mut out).expect("ansi");
        out
    }

    #[test]
    fn sticky_mouse_enable_only_requests_sgr_and_button_event_tracking() {
        let sequence = ansi(EnableStickyMouseCapture);

        assert_eq!(sequence, "\x1b[?1006h\x1b[?1002h");
    }

    #[test]
    fn sticky_mouse_disable_reverses_only_enabled_modes_in_safe_order() {
        let sequence = ansi(DisableStickyMouseCapture);

        assert_eq!(sequence, "\x1b[?1002l\x1b[?1006l");
    }
}
