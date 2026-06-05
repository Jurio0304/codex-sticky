use std::io;
use std::io::Write;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;

pub(super) const MAX_OSC52_TEXT_BYTES: usize = 100 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ClipboardWrite {
    Written,
    Empty,
    TooLarge { limit: usize },
}

#[cfg(not(test))]
pub(super) fn copy_text_to_clipboard(text: &str) -> io::Result<ClipboardWrite> {
    let mut writer = control_terminal_writer();
    write_osc52_with_tmux(&mut writer, text, std::env::var_os("TMUX").is_some())
}

#[cfg(test)]
pub(super) fn copy_text_to_clipboard(text: &str) -> io::Result<ClipboardWrite> {
    if text.is_empty() {
        return Ok(ClipboardWrite::Empty);
    }
    if text.len() > MAX_OSC52_TEXT_BYTES {
        return Ok(ClipboardWrite::TooLarge {
            limit: MAX_OSC52_TEXT_BYTES,
        });
    }
    Ok(ClipboardWrite::Written)
}

#[cfg(not(test))]
fn control_terminal_writer() -> Box<dyn Write> {
    #[cfg(unix)]
    {
        if let Ok(file) = std::fs::OpenOptions::new().write(true).open("/dev/tty") {
            return Box::new(file);
        }
    }
    Box::new(std::io::stdout())
}

#[cfg(test)]
pub(super) fn write_osc52<W: Write>(writer: &mut W, text: &str) -> io::Result<ClipboardWrite> {
    write_osc52_with_tmux(writer, text, /*tmux*/ false)
}

fn write_osc52_with_tmux<W: Write>(
    writer: &mut W,
    text: &str,
    tmux: bool,
) -> io::Result<ClipboardWrite> {
    if text.is_empty() {
        return Ok(ClipboardWrite::Empty);
    }
    if text.len() > MAX_OSC52_TEXT_BYTES {
        return Ok(ClipboardWrite::TooLarge {
            limit: MAX_OSC52_TEXT_BYTES,
        });
    }

    let payload = STANDARD.encode(text.as_bytes());
    if tmux {
        write!(writer, "\x1bPtmux;\x1b\x1b]52;c;{payload}\x07\x1b\\")?;
    } else {
        write!(writer, "\x1b]52;c;{payload}\x07")?;
    }
    writer.flush()?;
    Ok(ClipboardWrite::Written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn osc52_writer_frames_base64_payload_and_flushes() {
        let mut output = Vec::new();

        let result = write_osc52(&mut output, "hello").expect("write osc52");

        assert_eq!(result, ClipboardWrite::Written);
        assert_eq!(
            String::from_utf8(output).expect("utf8"),
            "\x1b]52;c;aGVsbG8=\x07"
        );
    }

    #[test]
    fn osc52_writer_wraps_tmux_passthrough_when_requested() {
        let mut output = Vec::new();

        let result =
            write_osc52_with_tmux(&mut output, "hello", /*tmux*/ true).expect("write osc52");

        assert_eq!(result, ClipboardWrite::Written);
        assert_eq!(
            String::from_utf8(output).expect("utf8"),
            "\x1bPtmux;\x1b\x1b]52;c;aGVsbG8=\x07\x1b\\"
        );
    }

    #[test]
    fn osc52_writer_skips_empty_selection() {
        let mut output = Vec::new();

        let result = write_osc52(&mut output, "").expect("write osc52");

        assert_eq!(result, ClipboardWrite::Empty);
        assert!(output.is_empty());
    }

    #[test]
    fn osc52_writer_rejects_oversized_selection() {
        let mut output = Vec::new();
        let text = "x".repeat(MAX_OSC52_TEXT_BYTES + 1);

        let result = write_osc52(&mut output, &text).expect("write osc52");

        assert_eq!(
            result,
            ClipboardWrite::TooLarge {
                limit: MAX_OSC52_TEXT_BYTES
            }
        );
        assert!(output.is_empty());
    }
}
