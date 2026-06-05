use super::*;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use crossterm::event::MouseButton;
use crossterm::event::MouseEvent;
use crossterm::event::MouseEventKind;
use pretty_assertions::assert_eq;
use ratatui::layout::Rect;
use ratatui::text::Line;

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

fn seed_snapshot(state: &StickyTranscriptState) {
    state.sync_rendered_content_height(/*total_rows*/ 30, /*viewport_height*/ 10);
    state.update_render_snapshot(StickyTranscriptSnapshot {
        area: Rect::new(0, 0, 40, 10),
        width: 40,
        viewport_start_row: 0,
        rows: (0..30)
            .map(|index| StickyTranscriptRow {
                line: Line::from(format!("row {index}")),
                text: format!("row {index}"),
            })
            .collect(),
    });
}

fn seed_rows(state: &StickyTranscriptState, rows: &[&str]) {
    state.sync_rendered_content_height(rows.len(), rows.len());
    state.update_render_snapshot(StickyTranscriptSnapshot {
        area: Rect::new(0, 0, 40, rows.len() as u16),
        width: 40,
        viewport_start_row: 0,
        rows: rows
            .iter()
            .map(|row| StickyTranscriptRow {
                line: Line::from((*row).to_string()),
                text: (*row).to_string(),
            })
            .collect(),
    });
}

#[test]
fn sticky_state_defaults_disabled() {
    let state = StickyTranscriptState::default();

    assert!(!state.enabled());
    assert!(state.follow_tail());
    assert_eq!(state.scroll_offset(), 0);
    assert!(!state.has_unread_output());
}

#[test]
fn sticky_state_scroll_lock_unread_and_follow_latest() {
    let state = StickyTranscriptState::new(/*enabled*/ true);
    state.sync_rendered_content_height(/*total_rows*/ 30, /*viewport_height*/ 10);

    assert!(state.handle_key_event(KeyEvent::from(KeyCode::PageUp)));
    assert!(!state.follow_tail());
    assert!(state.scroll_offset() > 0);

    let locked_offset = state.scroll_offset();
    state.note_output_appended();
    assert!(state.has_unread_output());
    assert_eq!(state.scroll_offset(), locked_offset);

    assert!(!state.handle_key_event(KeyEvent::from(KeyCode::End)));
    assert!(state.handle_key_event(KeyEvent::new(KeyCode::End, KeyModifiers::CONTROL)));
    assert!(state.follow_tail());
    assert_eq!(state.scroll_offset(), 0);
    assert!(!state.has_unread_output());
}

#[test]
fn sticky_state_mouse_and_page_down_restore_follow_tail_at_bottom() {
    let state = StickyTranscriptState::new(/*enabled*/ true);
    seed_snapshot(&state);

    assert_eq!(
        state.handle_mouse_event(mouse_scroll(MouseEventKind::ScrollUp)),
        StickyMouseAction::Redraw
    );
    assert!(!state.follow_tail());
    assert!(state.scroll_offset() > 0);

    while state.scroll_offset() > 0 {
        assert!(state.handle_key_event(KeyEvent::from(KeyCode::PageDown)));
    }

    assert!(state.follow_tail());
    assert_eq!(state.scroll_offset(), 0);
}

#[test]
fn sticky_selection_drag_release_extracts_multiline_text() {
    let state = StickyTranscriptState::new(/*enabled*/ true);
    seed_rows(&state, &["alpha", "beta", "gamma"]);

    assert_eq!(
        state.handle_mouse_event(mouse(
            MouseEventKind::Down(MouseButton::Left),
            /*column*/ 1,
            /*row*/ 0,
        )),
        StickyMouseAction::Redraw
    );
    assert_eq!(
        state.handle_mouse_event(mouse(
            MouseEventKind::Drag(MouseButton::Left),
            /*column*/ 3,
            /*row*/ 1,
        )),
        StickyMouseAction::Redraw
    );

    assert_eq!(
        state.handle_mouse_event(mouse(
            MouseEventKind::Up(MouseButton::Left),
            /*column*/ 3,
            /*row*/ 1,
        )),
        StickyMouseAction::CopySelection("lpha\nbet".to_string())
    );
}

#[test]
fn sticky_selection_text_trims_trailing_padding_and_handles_unicode_columns() {
    let rows = vec![
        StickyTranscriptRow {
            line: Line::from("  padded   "),
            text: "  padded   ".to_string(),
        },
        StickyTranscriptRow {
            line: Line::from("中abc"),
            text: "中abc".to_string(),
        },
    ];
    let selection = TranscriptSelection {
        anchor: TranscriptPosition { row: 0, column: 0 },
        cursor: TranscriptPosition { row: 1, column: 3 },
        active: false,
    };

    assert_eq!(selected_text_from_rows(&rows, &selection), "  padded\n中a");
}
