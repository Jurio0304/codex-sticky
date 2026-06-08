use std::fmt;
#[cfg(windows)]
use std::io::Result;

use crossterm::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EnableStickyMouseCapture;

impl Command for EnableStickyMouseCapture {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        // Sticky only needs SGR coordinates plus button/drag/wheel reporting.
        // OSC 22 is a best-effort pointer shape hint; unsupported terminals ignore it.
        f.write_str("\x1b[?1006h\x1b[?1002h\x1b]22;text\x1b\\")
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
        f.write_str("\x1b[?1002l\x1b[?1006l\x1b]22;default\x1b\\")
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
    fn sticky_mouse_enable_requests_mouse_reporting_and_text_pointer_hint() {
        let sequence = ansi(EnableStickyMouseCapture);

        assert_eq!(sequence, "\x1b[?1006h\x1b[?1002h\x1b]22;text\x1b\\");
    }

    #[test]
    fn sticky_mouse_disable_reverses_mouse_reporting_and_pointer_hint() {
        let sequence = ansi(DisableStickyMouseCapture);

        assert_eq!(sequence, "\x1b[?1002l\x1b[?1006l\x1b]22;default\x1b\\");
    }
}
