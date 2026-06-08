use ratatui::layout::Rect;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::TextArea;
use super::TextAreaState;

impl TextArea {
    pub(crate) fn byte_offset_for_mouse(
        &self,
        area: Rect,
        state: TextAreaState,
        column: u16,
        row: u16,
    ) -> Option<usize> {
        if area.is_empty()
            || column < area.x
            || column >= area.right()
            || row < area.y
            || row >= area.bottom()
        {
            return None;
        }

        let lines = self.wrapped_lines(area.width);
        let effective_scroll = self.effective_scroll(area.height, &lines, state.scroll);
        let line_index =
            usize::from(effective_scroll).saturating_add(usize::from(row.saturating_sub(area.y)));
        let Some(line) = lines.get(line_index) else {
            return Some(self.text.len());
        };

        let line_end = line.end.saturating_sub(1).min(self.text.len());
        let target_col = usize::from(column.saturating_sub(area.x));
        let offset = byte_offset_for_display_col(&self.text[line.start..line_end], target_col);
        Some(self.clamp_pos_to_nearest_boundary(line.start + offset))
    }

    pub(crate) fn selected_text_for_byte_range(&self, start: usize, end: usize) -> String {
        let start = self.clamp_pos_to_nearest_boundary(start.min(self.text.len()));
        let end = self.clamp_pos_to_nearest_boundary(end.min(self.text.len()));
        let (start, end) = if start <= end {
            (start, end)
        } else {
            (end, start)
        };
        if start >= end {
            return String::new();
        }
        self.text[start..end].to_string()
    }
}

fn byte_offset_for_display_col(text: &str, target_col: usize) -> usize {
    if target_col == 0 {
        return 0;
    }

    let mut width = 0usize;
    for (idx, grapheme) in text.grapheme_indices(true) {
        let grapheme_width = UnicodeWidthStr::width(grapheme);
        if width + grapheme_width > target_col {
            return idx;
        }
        width += grapheme_width;
        if width == target_col {
            return idx + grapheme.len();
        }
    }
    text.len()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use ratatui::layout::Rect;

    use super::*;

    fn ta_with(text: &str) -> TextArea {
        let mut textarea = TextArea::new();
        textarea.insert_str(text);
        textarea
    }

    #[test]
    fn mouse_offset_maps_wrapped_visible_rows() {
        let textarea = ta_with("abcdef");
        let area = Rect::new(10, 5, 3, 4);
        let state = TextAreaState::default();

        assert_eq!(
            textarea.byte_offset_for_mouse(area, state, /*column*/ 11, /*row*/ 5),
            Some(1)
        );
        assert_eq!(
            textarea.byte_offset_for_mouse(area, state, /*column*/ 12, /*row*/ 6),
            Some(5)
        );
    }

    #[test]
    fn mouse_offset_clamps_to_utf8_and_element_boundaries() {
        let mut textarea = ta_with("你abc");
        textarea.set_cursor("你a".len());
        let element_start = textarea.cursor();
        textarea.insert_element("ELEMENT");
        let area = Rect::new(0, 0, 20, 2);
        let state = TextAreaState::default();

        assert_eq!(
            textarea.byte_offset_for_mouse(area, state, /*column*/ 1, /*row*/ 0),
            Some(0)
        );
        assert_eq!(
            textarea.byte_offset_for_mouse(area, state, /*column*/ 5, /*row*/ 0),
            Some(element_start)
        );
    }

    #[test]
    fn mouse_offset_on_empty_visible_line_selects_end() {
        let textarea = ta_with("abc");
        let area = Rect::new(0, 0, 10, 3);
        let state = TextAreaState::default();

        assert_eq!(
            textarea.byte_offset_for_mouse(area, state, /*column*/ 4, /*row*/ 2),
            Some(textarea.text().len())
        );
    }

    #[test]
    fn selected_text_normalizes_reversed_ranges() {
        let textarea = ta_with("abcdef");

        assert_eq!(textarea.selected_text_for_byte_range(5, 1), "bcde");
    }
}
