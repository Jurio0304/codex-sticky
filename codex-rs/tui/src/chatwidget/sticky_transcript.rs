//! Sticky transcript state and render adapter for `ChatWidget`.

use std::cell::Cell;
use std::cell::RefCell;
use std::sync::Arc;

use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;
use crossterm::event::MouseButton;
use crossterm::event::MouseEvent;
use crossterm::event::MouseEventKind;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Modifier;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::widgets::Clear;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::bottom_pane::BottomPaneMouseAction;

use super::sticky_clipboard::ClipboardWrite;
use super::sticky_clipboard::copy_text_to_clipboard;
use super::*;

const DEFAULT_SCROLL_LINES: usize = 3;
const COPY_FLASH_SECS: u64 = 2;

#[derive(Debug)]
pub(super) struct StickyTranscriptState {
    enabled: bool,
    scroll_offset: Cell<usize>,
    follow_tail: Cell<bool>,
    has_unread_output: Cell<bool>,
    last_total_rows: Cell<Option<usize>>,
    last_viewport_height: Cell<usize>,
    render_snapshot: RefCell<StickyTranscriptSnapshot>,
    selection: RefCell<Option<TranscriptSelection>>,
}

#[derive(Clone, Debug, Default)]
struct StickyTranscriptSnapshot {
    area: Rect,
    width: u16,
    viewport_start_row: usize,
    rows: Vec<StickyTranscriptRow>,
}

impl StickyTranscriptSnapshot {
    fn position_for_mouse(&self, column: u16, row: u16) -> Option<TranscriptPosition> {
        if self.area.is_empty()
            || column < self.area.x
            || column >= self.area.right()
            || row < self.area.y
            || row >= self.area.bottom()
        {
            return None;
        }

        let visible_row = usize::from(row.saturating_sub(self.area.y));
        let transcript_row = self.viewport_start_row.saturating_add(visible_row);
        self.rows.get(transcript_row).map(|_| TranscriptPosition {
            row: transcript_row,
            column: usize::from(column.saturating_sub(self.area.x)),
        })
    }
}

#[derive(Clone, Debug, Default)]
struct StickyTranscriptRow {
    line: Line<'static>,
    text: String,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct TranscriptPosition {
    row: usize,
    column: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TranscriptSelection {
    anchor: TranscriptPosition,
    cursor: TranscriptPosition,
    active: bool,
}

impl TranscriptSelection {
    fn normalized(&self) -> (TranscriptPosition, TranscriptPosition) {
        if self.anchor <= self.cursor {
            (self.anchor, self.cursor)
        } else {
            (self.cursor, self.anchor)
        }
    }

    fn is_empty(&self) -> bool {
        let (start, end) = self.normalized();
        start == end
    }
}

impl Default for StickyTranscriptState {
    fn default() -> Self {
        Self {
            enabled: false,
            scroll_offset: Cell::new(0),
            follow_tail: Cell::new(true),
            has_unread_output: Cell::new(false),
            last_total_rows: Cell::new(None),
            last_viewport_height: Cell::new(0),
            render_snapshot: RefCell::new(StickyTranscriptSnapshot::default()),
            selection: RefCell::new(None),
        }
    }
}

impl StickyTranscriptState {
    pub(super) fn new(enabled: bool) -> Self {
        let mut state = Self::default();
        state.set_enabled(enabled);
        state
    }

    pub(super) fn enabled(&self) -> bool {
        self.enabled
    }

    pub(super) fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.scroll_offset.set(0);
        self.follow_tail.set(true);
        self.has_unread_output.set(false);
        self.last_total_rows.set(None);
        self.last_viewport_height.set(0);
        self.render_snapshot
            .replace(StickyTranscriptSnapshot::default());
        self.selection.borrow_mut().take();
    }

    fn page_size(&self) -> usize {
        self.last_viewport_height.get().saturating_sub(1).max(1)
    }

    fn scroll_up(&self, rows: usize) {
        if rows == 0 {
            return;
        }
        self.scroll_offset
            .set(self.scroll_offset.get().saturating_add(rows));
        self.follow_tail.set(false);
    }

    fn scroll_down(&self, rows: usize) {
        let next = self.scroll_offset.get().saturating_sub(rows);
        self.scroll_offset.set(next);
        if next == 0 {
            self.follow_latest();
        }
    }

    pub(super) fn follow_latest(&self) {
        self.scroll_offset.set(0);
        self.follow_tail.set(true);
        self.has_unread_output.set(false);
    }

    pub(super) fn note_output_appended(&self) {
        if self.enabled && !self.follow_tail.get() {
            self.has_unread_output.set(true);
        }
    }

    pub(super) fn sync_rendered_content_height(&self, total_rows: usize, viewport_height: usize) {
        self.last_viewport_height.set(viewport_height);
        let max_offset = total_rows.saturating_sub(viewport_height);
        let previous_total_rows = self.last_total_rows.replace(Some(total_rows));

        if self.follow_tail.get() {
            self.follow_latest();
            return;
        }

        let mut offset = self.scroll_offset.get();
        if let Some(previous) = previous_total_rows
            && total_rows > previous
        {
            offset = offset.saturating_add(total_rows - previous);
            self.has_unread_output.set(true);
        }
        offset = offset.min(max_offset);
        self.scroll_offset.set(offset);
        if offset == 0 {
            self.follow_latest();
        }
    }

    fn update_render_snapshot(&self, snapshot: StickyTranscriptSnapshot) {
        let previous = self.render_snapshot.borrow();
        let width_changed = previous.width != 0 && previous.width != snapshot.width;
        drop(previous);

        if width_changed {
            self.selection.borrow_mut().take();
        } else if let Some(selection) = self.selection.borrow().as_ref() {
            let (_, end) = selection.normalized();
            if end.row >= snapshot.rows.len() {
                self.selection.borrow_mut().take();
            }
        }

        self.render_snapshot.replace(snapshot);
    }

    fn clear_selection(&self) {
        self.selection.borrow_mut().take();
    }

    fn handle_key_event(&self, key_event: KeyEvent) -> bool {
        if !self.enabled || !matches!(key_event.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
            return false;
        }

        match key_event.code {
            KeyCode::PageUp => {
                self.scroll_up(self.page_size());
                true
            }
            KeyCode::PageDown => {
                self.scroll_down(self.page_size());
                true
            }
            KeyCode::End if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.follow_latest();
                true
            }
            _ => false,
        }
    }

    fn handle_mouse_event(&self, mouse_event: MouseEvent) -> StickyMouseAction {
        if !self.enabled {
            return StickyMouseAction::Ignored;
        }
        match mouse_event.kind {
            MouseEventKind::ScrollUp => {
                if self.mouse_in_transcript(mouse_event) {
                    self.scroll_up(DEFAULT_SCROLL_LINES);
                    StickyMouseAction::Redraw
                } else {
                    StickyMouseAction::Ignored
                }
            }
            MouseEventKind::ScrollDown => {
                if self.mouse_in_transcript(mouse_event) {
                    self.scroll_down(DEFAULT_SCROLL_LINES);
                    StickyMouseAction::Redraw
                } else {
                    StickyMouseAction::Ignored
                }
            }
            MouseEventKind::Down(MouseButton::Left) => {
                if let Some(position) = self.mouse_position(mouse_event) {
                    self.selection.replace(Some(TranscriptSelection {
                        anchor: position,
                        cursor: position,
                        active: true,
                    }));
                    StickyMouseAction::Redraw
                } else {
                    StickyMouseAction::Ignored
                }
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if let Some(position) = self.mouse_position(mouse_event) {
                    let mut selection = self.selection.borrow_mut();
                    if let Some(selection) = selection.as_mut()
                        && selection.active
                    {
                        selection.cursor = position;
                        return StickyMouseAction::Redraw;
                    }
                }
                StickyMouseAction::Ignored
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if let Some(position) = self.mouse_position(mouse_event) {
                    let mut selection_ref = self.selection.borrow_mut();
                    if let Some(selection) = selection_ref.as_mut()
                        && selection.active
                    {
                        selection.cursor = position;
                        selection.active = false;
                        if selection.is_empty() {
                            selection_ref.take();
                            return StickyMouseAction::Redraw;
                        }
                        drop(selection_ref);
                        let text = self.selected_text();
                        return StickyMouseAction::CopySelection(text);
                    }
                }
                StickyMouseAction::Ignored
            }
            _ => StickyMouseAction::Ignored,
        }
    }

    fn mouse_in_transcript(&self, mouse_event: MouseEvent) -> bool {
        let snapshot = self.render_snapshot.borrow();
        !snapshot.area.is_empty()
            && mouse_event.column >= snapshot.area.x
            && mouse_event.column < snapshot.area.right()
            && mouse_event.row >= snapshot.area.y
            && mouse_event.row < snapshot.area.bottom()
    }

    fn mouse_position(&self, mouse_event: MouseEvent) -> Option<TranscriptPosition> {
        self.render_snapshot
            .borrow()
            .position_for_mouse(mouse_event.column, mouse_event.row)
    }

    fn selected_text(&self) -> String {
        let snapshot = self.render_snapshot.borrow();
        let selection = self.selection.borrow();
        let Some(selection) = selection.as_ref() else {
            return String::new();
        };
        selected_text_from_rows(&snapshot.rows, selection)
    }

    pub(super) fn has_unread_output(&self) -> bool {
        self.has_unread_output.get()
    }

    pub(super) fn scroll_offset(&self) -> usize {
        self.scroll_offset.get()
    }

    #[cfg(test)]
    pub(super) fn follow_tail(&self) -> bool {
        self.follow_tail.get()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum StickyMouseAction {
    Ignored,
    Redraw,
    CopySelection(String),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct StickyLayout {
    transcript: Rect,
    spacer_above_unread: Rect,
    unread: Rect,
    spacer_below_unread: Rect,
    bottom: Rect,
    right_reserve: u16,
}

impl ChatWidget {
    pub(crate) fn sticky_transcript_enabled(&self) -> bool {
        self.sticky_transcript.enabled()
    }

    pub(crate) fn set_sticky_transcript_enabled(&mut self, enabled: bool) {
        self.sticky_transcript.set_enabled(enabled);
        self.config.tui_sticky_transcript = enabled;
    }

    pub(crate) fn clear_sticky_transcript_selection(&self) {
        self.sticky_transcript.clear_selection();
    }

    pub(crate) fn set_sticky_transcript_enabled_and_notify(&mut self, enabled: bool) {
        if enabled && self.raw_output_mode() {
            self.add_error_message(
                "Sticky Transcript mode is unavailable while raw output mode is enabled."
                    .to_string(),
            );
            return;
        }
        let before = self.sticky_transcript.enabled();
        self.set_sticky_transcript_enabled(enabled);
        self.add_info_message(sticky_transcript_mode_notice(enabled).to_string(), None);
        if enabled != before {
            self.emit_sticky_transcript_mode_changed(enabled);
        }
    }

    pub(crate) fn toggle_sticky_transcript_enabled_and_notify(&mut self) -> bool {
        let enabled = !self.sticky_transcript.enabled();
        self.set_sticky_transcript_enabled_and_notify(enabled);
        self.sticky_transcript.enabled()
    }

    pub(crate) fn handle_sticky_transcript_key_event(&self, key_event: KeyEvent) -> bool {
        self.sticky_transcript.handle_key_event(key_event)
    }

    pub(crate) fn handle_sticky_transcript_mouse_event(&mut self, mouse_event: MouseEvent) -> bool {
        match self.sticky_transcript.handle_mouse_event(mouse_event) {
            StickyMouseAction::Ignored => false,
            StickyMouseAction::Redraw => true,
            StickyMouseAction::CopySelection(text) => {
                self.finish_sticky_transcript_copy(text);
                true
            }
        }
    }

    pub(crate) fn handle_sticky_composer_mouse_event(&mut self, mouse_event: MouseEvent) -> bool {
        match self.bottom_pane.handle_mouse_event(mouse_event) {
            BottomPaneMouseAction::Ignored => false,
            BottomPaneMouseAction::Redraw => true,
            BottomPaneMouseAction::CopySelection(text) => {
                self.finish_sticky_transcript_copy(text);
                true
            }
        }
    }

    pub(crate) fn note_sticky_transcript_output_appended(&self) {
        self.sticky_transcript.note_output_appended();
    }

    pub(crate) fn render_sticky_transcript(
        &self,
        area: Rect,
        buf: &mut Buffer,
        transcript_cells: &[Arc<dyn HistoryCell>],
    ) {
        self.last_rendered_width.set(Some(area.width as usize));
        let layout = self.sticky_layout(area);

        self.render_sticky_transcript_area(layout.transcript, buf, transcript_cells);
        Clear.render(layout.spacer_above_unread, buf);
        render_unread_hint(
            layout.unread,
            buf,
            self.sticky_transcript.has_unread_output(),
        );
        Clear.render(layout.spacer_below_unread, buf);
        self.bottom_pane.render_with_composer_right_reserve(
            layout.bottom,
            buf,
            layout.right_reserve,
        );
    }

    fn finish_sticky_transcript_copy(&mut self, text: String) {
        if text.is_empty() {
            return;
        }

        let line = match copy_text_to_clipboard(&text) {
            Ok(ClipboardWrite::Written) => "Copied selection".dim(),
            Ok(ClipboardWrite::Empty) => return,
            Ok(ClipboardWrite::TooLarge { limit }) => {
                format!("Selection too large to copy (limit {limit} bytes)").red()
            }
            Err(err) => format!("Failed to copy selection: {err}").red(),
        };
        self.bottom_pane
            .show_footer_flash(Line::from(line), Duration::from_secs(COPY_FLASH_SECS));
    }

    pub(crate) fn sticky_transcript_cursor_pos(&self, area: Rect) -> Option<(u16, u16)> {
        let layout = self.sticky_layout(area);
        self.bottom_pane
            .cursor_pos_with_composer_right_reserve(layout.bottom, layout.right_reserve)
    }

    pub(crate) fn sticky_transcript_cursor_style(
        &self,
        area: Rect,
    ) -> crossterm::cursor::SetCursorStyle {
        let layout = self.sticky_layout(area);
        self.bottom_pane
            .cursor_style_with_composer_right_reserve(layout.bottom, layout.right_reserve)
    }

    fn sticky_layout(&self, area: Rect) -> StickyLayout {
        let right_reserve = self.ambient_pet_wrap_reserved_cols();
        let bottom_height = self
            .bottom_pane
            .desired_height_with_composer_right_reserve(area.width, right_reserve)
            .min(area.height);
        let bottom = Rect::new(
            area.x,
            area.bottom().saturating_sub(bottom_height),
            area.width,
            bottom_height,
        );

        let mut remaining = area.height.saturating_sub(bottom_height);
        let unread_height = if self.sticky_transcript.has_unread_output() && remaining > 0 {
            remaining = remaining.saturating_sub(1);
            1
        } else {
            0
        };
        let spacer_below_unread_height = if remaining > 0 {
            remaining = remaining.saturating_sub(1);
            1
        } else {
            0
        };
        let spacer_above_unread_height = if unread_height > 0 && remaining > 0 {
            remaining = remaining.saturating_sub(1);
            1
        } else {
            0
        };

        let transcript = Rect::new(area.x, area.y, area.width, remaining);
        let spacer_above_unread = Rect::new(
            area.x,
            transcript.bottom(),
            area.width,
            spacer_above_unread_height,
        );
        let unread = Rect::new(
            area.x,
            spacer_above_unread.bottom(),
            area.width,
            unread_height,
        );
        let spacer_below_unread = Rect::new(
            area.x,
            unread.bottom(),
            area.width,
            spacer_below_unread_height,
        );

        StickyLayout {
            transcript,
            spacer_above_unread,
            unread,
            spacer_below_unread,
            bottom,
            right_reserve,
        }
    }

    fn render_sticky_transcript_area(
        &self,
        area: Rect,
        buf: &mut Buffer,
        transcript_cells: &[Arc<dyn HistoryCell>],
    ) {
        Clear.render(area, buf);
        if area.is_empty() {
            self.sticky_transcript
                .update_render_snapshot(StickyTranscriptSnapshot {
                    area,
                    width: area.width,
                    viewport_start_row: 0,
                    rows: Vec::new(),
                });
            return;
        }

        let rows = self.sticky_transcript_rows(transcript_cells, area.width);
        let total_rows = rows.len();
        self.sticky_transcript
            .sync_rendered_content_height(total_rows, usize::from(area.height));
        let overflow = total_rows.saturating_sub(usize::from(area.height));
        let viewport_start_row = overflow.saturating_sub(self.sticky_transcript.scroll_offset());
        let visible_count =
            usize::from(area.height).min(total_rows.saturating_sub(viewport_start_row));

        self.sticky_transcript
            .update_render_snapshot(StickyTranscriptSnapshot {
                area,
                width: area.width,
                viewport_start_row,
                rows: rows.clone(),
            });

        for (screen_row, row) in rows
            .iter()
            .skip(viewport_start_row)
            .take(visible_count)
            .enumerate()
        {
            let Ok(screen_row) = u16::try_from(screen_row) else {
                break;
            };
            let row_area = Rect::new(area.x, area.y + screen_row, area.width, 1);
            Block::default().style(row.line.style).render(row_area, buf);
            row.line.clone().render(row_area, buf);
        }
        render_selection_overlay(area, buf, &self.sticky_transcript);
    }

    fn sticky_transcript_rows(
        &self,
        transcript_cells: &[Arc<dyn HistoryCell>],
        width: u16,
    ) -> Vec<StickyTranscriptRow> {
        let mut lines = Vec::new();
        let mut has_content = false;
        for cell in transcript_cells {
            let cell_lines = crate::terminal_hyperlinks::visible_lines(
                cell.display_hyperlink_lines_for_mode(width, self.history_render_mode()),
            );
            if !cell_lines.is_empty() && !cell.is_stream_continuation() {
                if has_content {
                    lines.push(Line::from(""));
                } else {
                    has_content = true;
                }
            }
            lines.extend(cell_lines);
        }

        if let Some(active) = self.transcript.active_cell.as_ref() {
            let active_lines = crate::terminal_hyperlinks::visible_lines(
                active.display_hyperlink_lines_for_mode(width, self.history_render_mode()),
            );
            if !active_lines.is_empty() && !active.is_stream_continuation() && has_content {
                lines.push(Line::from(""));
            }
            lines.extend(active_lines);
        }

        wrap_sticky_lines(lines, width)
    }

    fn emit_sticky_transcript_mode_changed(&self, enabled: bool) {
        self.app_event_tx
            .send(AppEvent::StickyTranscriptModeChanged { enabled });
    }
}

fn wrap_sticky_lines(lines: Vec<Line<'static>>, width: u16) -> Vec<StickyTranscriptRow> {
    let width = usize::from(width.max(1));
    let mut rows = Vec::new();
    for line in lines {
        if line_plain_text(&line).is_empty() {
            rows.push(StickyTranscriptRow {
                line: Line::from("").style(line.style),
                text: String::new(),
            });
            continue;
        }

        let wrapped =
            crate::wrapping::word_wrap_line(&line, crate::wrapping::RtOptions::new(width));
        for wrapped_line in wrapped {
            let line = crate::render::line_utils::line_to_static(&wrapped_line);
            rows.push(StickyTranscriptRow {
                text: line_plain_text(&line),
                line,
            });
        }
    }
    rows
}

fn line_plain_text(line: &Line<'_>) -> String {
    line.spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect::<String>()
}

fn render_selection_overlay(area: Rect, buf: &mut Buffer, state: &StickyTranscriptState) {
    let snapshot = state.render_snapshot.borrow();
    let selection = state.selection.borrow();
    let Some(selection) = selection.as_ref() else {
        return;
    };
    if selection.is_empty() {
        return;
    }

    for visible_row in 0..area.height {
        let transcript_row = snapshot.viewport_start_row + usize::from(visible_row);
        let Some((start, end)) = selection_columns_for_row(
            selection,
            &snapshot.rows,
            transcript_row,
            usize::from(area.width),
        ) else {
            continue;
        };
        let start = start.min(usize::from(area.width));
        let end = end.min(usize::from(area.width));
        if start >= end {
            continue;
        }
        for column in start..end {
            let Ok(column) = u16::try_from(column) else {
                break;
            };
            let x = area.x + column;
            let y = area.y + visible_row;
            let style = buf[(x, y)].style().add_modifier(Modifier::REVERSED);
            buf[(x, y)].set_style(style);
        }
    }
}

fn selection_columns_for_row(
    selection: &TranscriptSelection,
    rows: &[StickyTranscriptRow],
    row: usize,
    viewport_width: usize,
) -> Option<(usize, usize)> {
    let (start, end) = selection.normalized();
    if row < start.row || row > end.row {
        return None;
    }

    if start.row == end.row {
        return Some((
            start.column,
            end_column_for_selected_cell(rows, end.row, end.column, viewport_width),
        ));
    }
    if row == start.row {
        return Some((start.column, viewport_width));
    }
    if row == end.row {
        return Some((
            0,
            end_column_for_selected_cell(rows, end.row, end.column, viewport_width),
        ));
    }
    Some((0, viewport_width))
}

fn selected_text_from_rows(
    rows: &[StickyTranscriptRow],
    selection: &TranscriptSelection,
) -> String {
    let (start, end) = selection.normalized();
    if start == end || rows.is_empty() {
        return String::new();
    }

    let mut selected = Vec::new();
    for row_index in start.row..=end.row {
        let Some(row) = rows.get(row_index) else {
            break;
        };
        let row_width = UnicodeWidthStr::width(row.text.as_str());
        let Some((start_column, end_column)) =
            selection_columns_for_row(selection, rows, row_index, row_width)
        else {
            continue;
        };

        let start_byte = byte_index_for_column(&row.text, start_column.min(row_width));
        let end_byte = byte_index_for_column(&row.text, end_column.min(row_width));
        let segment = if start_byte <= end_byte {
            &row.text[start_byte..end_byte]
        } else {
            ""
        };
        selected.push(segment.trim_end().to_string());
    }
    selected.join("\n")
}

fn end_column_for_selected_cell(
    rows: &[StickyTranscriptRow],
    row: usize,
    column: usize,
    max_column: usize,
) -> usize {
    let Some(row) = rows.get(row) else {
        return column.min(max_column);
    };
    let row_width = UnicodeWidthStr::width(row.text.as_str());
    if column >= row_width {
        return column.saturating_add(1).min(max_column);
    }

    let mut width = 0usize;
    for grapheme in row.text.graphemes(true) {
        let next_width = width + UnicodeWidthStr::width(grapheme);
        if column < next_width {
            return next_width.min(max_column);
        }
        width = next_width;
    }
    row_width.min(max_column)
}

fn byte_index_for_column(text: &str, column: usize) -> usize {
    if column == 0 {
        return 0;
    }

    let mut width = 0usize;
    for (idx, grapheme) in text.grapheme_indices(true) {
        let grapheme_width = UnicodeWidthStr::width(grapheme);
        if width + grapheme_width > column {
            return idx;
        }
        width += grapheme_width;
        if width == column {
            return idx + grapheme.len();
        }
    }
    text.len()
}

fn render_unread_hint(area: Rect, buf: &mut Buffer, visible: bool) {
    if area.is_empty() {
        return;
    }
    Clear.render(area, buf);
    if visible {
        Paragraph::new(Line::from("↓ New output · Ctrl+End to follow").dim()).render(area, buf);
    }
}

pub(super) fn sticky_transcript_mode_notice(enabled: bool) -> &'static str {
    if enabled {
        "Sticky Transcript mode on: composer stays fixed while transcript history scrolls."
    } else {
        "Sticky Transcript mode off: native terminal scrollback restored."
    }
}

#[cfg(test)]
#[path = "sticky_transcript_tests.rs"]
mod tests;
