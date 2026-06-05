use super::*;
use crate::history_cell::HistoryCell;
use crate::history_cell::PlainHistoryCell;
use crossterm::event::MouseButton;
use crossterm::event::MouseEvent;
use crossterm::event::MouseEventKind;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::text::Line;
use std::sync::Arc;

fn mouse_scroll(kind: MouseEventKind) -> MouseEvent {
    mouse(kind, /*column*/ 0, /*row*/ 0)
}

fn mouse(kind: MouseEventKind, column: u16, row: u16) -> MouseEvent {
    MouseEvent {
        kind,
        column,
        row,
        modifiers: KeyModifiers::NONE,
    }
}

fn transcript_cells(count: usize) -> Vec<Arc<dyn HistoryCell>> {
    (0..count)
        .map(|index| {
            Arc::new(PlainHistoryCell::new(vec![
                format!("transcript line {index:02}").into(),
            ])) as Arc<dyn HistoryCell>
        })
        .collect()
}

fn render_sticky(
    chat: &ChatWidget,
    cells: &[Arc<dyn HistoryCell>],
    width: u16,
    height: u16,
) -> String {
    let buf = render_sticky_buf(chat, cells, width, height);
    buffer_to_lines(&buf, Rect::new(0, 0, width, height)).join("\n")
}

fn render_sticky_buf(
    chat: &ChatWidget,
    cells: &[Arc<dyn HistoryCell>],
    width: u16,
    height: u16,
) -> Buffer {
    let area = Rect::new(0, 0, width, height);
    let mut buf = Buffer::empty(area);
    chat.render_sticky_transcript(area, &mut buf, cells);
    buf
}

fn buffer_to_lines(buf: &Buffer, area: Rect) -> Vec<String> {
    (0..area.height)
        .map(|row| {
            let mut line = String::new();
            for col in 0..area.width {
                let symbol = buf[(area.x + col, area.y + row)].symbol();
                if symbol.is_empty() {
                    line.push(' ');
                } else {
                    line.push_str(symbol);
                }
            }
            line.trim_end().to_string()
        })
        .collect::<Vec<_>>()
}

#[tokio::test]
async fn sticky_transcript_state_tracks_scroll_follow_and_unread_output() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let cells = transcript_cells(12);

    assert!(!chat.sticky_transcript_enabled());

    chat.set_sticky_transcript_enabled(/*enabled*/ true);
    assert!(chat.sticky_transcript_enabled());
    assert!(chat.sticky_transcript_follow_tail());
    assert_eq!(chat.sticky_transcript_scroll_offset(), 0);
    assert!(!chat.sticky_transcript_has_unread_output());

    let _ = render_sticky(&chat, &cells, /*width*/ 40, /*height*/ 8);
    assert!(chat.handle_sticky_transcript_key_event(KeyEvent::from(KeyCode::PageUp)));
    assert!(!chat.sticky_transcript_follow_tail());
    assert!(chat.sticky_transcript_scroll_offset() > 0);

    let locked_offset = chat.sticky_transcript_scroll_offset();
    chat.note_sticky_transcript_output_appended();
    assert!(chat.sticky_transcript_has_unread_output());
    assert_eq!(chat.sticky_transcript_scroll_offset(), locked_offset);

    assert!(!chat.handle_sticky_transcript_key_event(KeyEvent::from(KeyCode::End)));
    assert!(
        chat.handle_sticky_transcript_key_event(KeyEvent::new(KeyCode::End, KeyModifiers::CONTROL))
    );
    assert!(chat.sticky_transcript_follow_tail());
    assert_eq!(chat.sticky_transcript_scroll_offset(), 0);
    assert!(!chat.sticky_transcript_has_unread_output());

    chat.set_sticky_transcript_enabled(/*enabled*/ false);
    assert!(!chat.sticky_transcript_enabled());
}

#[tokio::test]
async fn sticky_transcript_mouse_scroll_stays_isolated_from_composer() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let cells = transcript_cells(16);
    chat.set_sticky_transcript_enabled(/*enabled*/ true);
    chat.bottom_pane
        .set_composer_text("draft text".to_string(), Vec::new(), Vec::new());

    let _ = render_sticky(&chat, &cells, /*width*/ 40, /*height*/ 8);
    let before_text = chat.bottom_pane.composer_text();
    let before_cursor = chat.sticky_transcript_cursor_pos(Rect::new(0, 0, 40, 8));

    assert!(chat.handle_sticky_transcript_mouse_event(mouse_scroll(MouseEventKind::ScrollUp)));

    assert_eq!(chat.bottom_pane.composer_text(), before_text);
    assert_eq!(
        chat.sticky_transcript_cursor_pos(Rect::new(0, 0, 40, 8)),
        before_cursor
    );
    assert!(chat.sticky_transcript_scroll_offset() > 0);
    assert!(chat.no_modal_or_popup_active());
}

#[tokio::test]
async fn sticky_transcript_page_down_restores_follow_when_bottom_reached() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let cells = transcript_cells(12);
    chat.set_sticky_transcript_enabled(/*enabled*/ true);

    let _ = render_sticky(&chat, &cells, /*width*/ 40, /*height*/ 8);
    assert!(chat.handle_sticky_transcript_mouse_event(mouse_scroll(MouseEventKind::ScrollUp)));
    assert!(!chat.sticky_transcript_follow_tail());

    for _ in 0..8 {
        let _ = chat.handle_sticky_transcript_key_event(KeyEvent::from(KeyCode::PageDown));
    }

    assert!(chat.sticky_transcript_follow_tail());
    assert_eq!(chat.sticky_transcript_scroll_offset(), 0);
}

#[tokio::test]
async fn sticky_transcript_render_snapshots() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let cells = transcript_cells(14);
    chat.bottom_pane
        .set_composer_text("composer stays here".to_string(), Vec::new(), Vec::new());
    chat.set_sticky_transcript_enabled(/*enabled*/ true);

    let initial = render_sticky(&chat, &cells, /*width*/ 48, /*height*/ 10);
    assert_snapshot!(
        sticky_render_summary(&initial),
        @"composer=true, unread=false, latest=true, older=false"
    );

    let _ = chat.handle_sticky_transcript_key_event(KeyEvent::from(KeyCode::PageUp));
    let scrolled = render_sticky(&chat, &cells, /*width*/ 48, /*height*/ 10);
    assert_snapshot!(
        sticky_render_summary(&scrolled),
        @"composer=true, unread=false, latest=false, older=true"
    );

    chat.note_sticky_transcript_output_appended();
    let unread = render_sticky(&chat, &cells, /*width*/ 48, /*height*/ 10);
    assert_snapshot!(
        sticky_render_summary(&unread),
        @"composer=true, unread=true, latest=false, older=false"
    );

    let _ =
        chat.handle_sticky_transcript_key_event(KeyEvent::new(KeyCode::End, KeyModifiers::CONTROL));
    let followed = render_sticky(&chat, &cells, /*width*/ 48, /*height*/ 10);
    assert_snapshot!(
        sticky_render_summary(&followed),
        @"composer=true, unread=false, latest=true, older=false"
    );

    let resized = render_sticky(&chat, &cells, /*width*/ 36, /*height*/ 7);
    assert_snapshot!(
        sticky_render_summary(&resized),
        @"composer=true, unread=false, latest=true, older=false"
    );

    chat.set_sticky_transcript_enabled(/*enabled*/ false);
    assert!(!chat.sticky_transcript_enabled());
}

#[tokio::test]
async fn sticky_transcript_selection_highlights_visible_rows() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let cells = transcript_cells(4);
    chat.set_sticky_transcript_enabled(/*enabled*/ true);
    let _ = render_sticky_buf(&chat, &cells, /*width*/ 40, /*height*/ 8);

    assert!(chat.handle_sticky_transcript_mouse_event(mouse(
        MouseEventKind::Down(MouseButton::Left),
        /*column*/ 1,
        /*row*/ 0,
    )));
    assert!(chat.handle_sticky_transcript_mouse_event(mouse(
        MouseEventKind::Drag(MouseButton::Left),
        /*column*/ 5,
        /*row*/ 1,
    )));
    let buf = render_sticky_buf(&chat, &cells, /*width*/ 40, /*height*/ 8);

    assert!(
        buf[(1, 0)]
            .style()
            .add_modifier
            .contains(Modifier::REVERSED)
    );
    assert!(
        buf[(4, 1)]
            .style()
            .add_modifier
            .contains(Modifier::REVERSED)
    );
    assert!(
        !buf[(0, 0)]
            .style()
            .add_modifier
            .contains(Modifier::REVERSED)
    );
}

#[tokio::test]
async fn sticky_transcript_adds_layout_spacer_around_unread_hint() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    let cells = transcript_cells(4);
    chat.bottom_pane
        .set_composer_text("composer".to_string(), Vec::new(), Vec::new());
    chat.set_sticky_transcript_enabled(/*enabled*/ true);

    let rendered = render_sticky(&chat, &cells, /*width*/ 48, /*height*/ 10);
    let lines = rendered.lines().collect::<Vec<_>>();
    let composer_row = lines
        .iter()
        .position(|line| line.contains("composer"))
        .expect("composer row");
    assert_eq!(lines[composer_row.saturating_sub(1)], "");

    let _ = chat.handle_sticky_transcript_key_event(KeyEvent::from(KeyCode::PageUp));
    chat.note_sticky_transcript_output_appended();
    let rendered = render_sticky(&chat, &cells, /*width*/ 48, /*height*/ 10);
    let lines = rendered.lines().collect::<Vec<_>>();
    let unread_row = lines
        .iter()
        .position(|line| line.contains("New output"))
        .expect("unread row");
    assert_eq!(lines[unread_row.saturating_sub(1)], "");
    assert_eq!(lines[unread_row + 1], "");
}

#[tokio::test]
async fn sticky_transcript_full_width_line_style_fills_right_padding() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
    chat.set_sticky_transcript_enabled(/*enabled*/ true);
    let style = Style::default().bg(Color::Blue);
    let cells: Vec<Arc<dyn HistoryCell>> = vec![Arc::new(PlainHistoryCell::new(vec![
        Line::from("styled").style(style),
    ]))];

    let buf = render_sticky_buf(&chat, &cells, /*width*/ 20, /*height*/ 6);

    assert_eq!(buf[(19, 0)].style().bg, Some(Color::Blue));
}

fn sticky_render_summary(rendered: &str) -> String {
    format!(
        "composer={}, unread={}, latest={}, older={}",
        rendered.contains("composer stays here"),
        rendered.contains("New output"),
        rendered.contains("transcript line 13"),
        rendered.contains("transcript line 09"),
    )
}
