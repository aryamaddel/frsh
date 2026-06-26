//! Multi-line text editor for notes.
//!
//! This is a faithful adaptation of Zed's official `gpui/examples/input.rs`
//! (the proven single-line TextInput), extended to handle multiple lines
//! for note-taking. The EntityInputHandler impl, action handlers, and
//! TextElement structure are copied directly from that example.

#![allow(unused)]

use std::ops::Range;

use gpui::{
    App, Bounds, ClipboardItem, Context, CursorStyle, ElementId, ElementInputHandler, Entity,
    EntityInputHandler, FocusHandle, Focusable, GlobalElementId, InspectorElementId, KeyBinding,
    LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PaintQuad, Pixels, Point,
    ShapedLine, SharedString, Style, TextRun, UTF16Selection, UnderlineStyle, Window, actions,
    black, div, fill, hsla, opaque_grey, point, prelude::*, px, relative, rgb, rgba, size, white,
};
use gpui_platform::application;
use unicode_segmentation::*;
use std::path::PathBuf;
use crate::vault::Vault;

actions!(
    note_editor,
    [
        Backspace,
        Delete,
        Left,
        Right,
        Up,
        Down,
        SelectLeft,
        SelectRight,
        SelectUp,
        SelectDown,
        SelectAll,
        Home,
        End,
        ShowCharacterPalette,
        Paste,
        Cut,
        Copy,
        Save,
        TogglePreview,
        NewLine,
    ]
);

// ── View mode ──────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EditorMode {
    Edit,
    Preview,
}

// ── Editor view ────────────────────────────────────────────────────────────

pub struct EditorView {
    pub vault: Entity<Vault>,
    pub file_path: PathBuf,
    focus_handle: FocusHandle,
    content: SharedString,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    /// One ShapedLine per visual line, plus its bounds from last paint.
    last_layout: Vec<ShapedLine>,
    last_bounds: Vec<Bounds<Pixels>>,
    is_selecting: bool,
    is_dirty: bool,
    pub mode: EditorMode,
}

impl EditorView {
    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }
    pub fn new(vault: Entity<Vault>, file_path: PathBuf, cx: &mut Context<Self>) -> Self {
        let content: SharedString = vault
            .read_with(cx, |v, _| v.read_file(&file_path).unwrap_or_default())
            .into();
        let len = content.len();
        Self {
            vault,
            file_path,
            focus_handle: cx.focus_handle(),
            content,
            selected_range: len..len,
            selection_reversed: false,
            marked_range: None,
            last_layout: Vec::new(),
            last_bounds: Vec::new(),
            is_selecting: false,
            is_dirty: false,
            mode: EditorMode::Edit,
        }
    }

    pub fn file_name(&self) -> &str {
        self.file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
    }

    pub fn save(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let path = self.file_path.clone();
        let content = self.content.to_string();
        self.vault.update(cx, |v, cx| {
            if let Err(e) = v.save_file(&path, &content) {
                eprintln!("Save error: {e:?}");
            } else {
                cx.notify();
            }
        });
        self.is_dirty = false;
        cx.notify();
    }

    pub fn save_action(&mut self, _: &Save, window: &mut Window, cx: &mut Context<Self>) {
        self.save(window, cx);
    }

    pub fn toggle_preview(&mut self, _: &TogglePreview, _: &mut Window, cx: &mut Context<Self>) {
        self.mode = match self.mode {
            EditorMode::Edit => EditorMode::Preview,
            EditorMode::Preview => EditorMode::Edit,
        };
        cx.notify();
    }

    // ── Cursor helpers (copied from input.rs example) ──────────────────────

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        cx.notify()
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset
        } else {
            self.selected_range.end = offset
        };
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        cx.notify()
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .rev()
            .find_map(|(idx, _)| (idx < offset).then_some(idx))
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .find_map(|(idx, _)| (idx > offset).then_some(idx))
            .unwrap_or(self.content.len())
    }

    /// Byte offset of the start of the line containing `offset`.
    fn line_start_offset(&self, offset: usize) -> usize {
        let upto = &self.content[..offset.min(self.content.len())];
        upto.rfind('\n').map(|i| i + 1).unwrap_or(0)
    }

    /// Byte offset of the end of the line (before the newline) containing `offset`.
    fn line_end_offset(&self, offset: usize) -> usize {
        let from = &self.content[offset.min(self.content.len())..];
        from.find('\n')
            .map(|i| offset + i)
            .unwrap_or(self.content.len())
    }

    /// Column (bytes from line start) of the given offset.
    fn column(&self, offset: usize) -> usize {
        offset - self.line_start_offset(offset)
    }

    /// Move the cursor vertically by `delta` lines, preserving the column.
    fn vertical_target(&self, offset: usize, delta: isize) -> usize {
        let col = self.column(offset);
        let mut current_line_start = self.line_start_offset(offset);

        // Walk lines to find the target line's start.
        let mut byte = 0usize;
        let lines: Vec<&str> = self.content.split('\n').collect();
        let mut line_idx = 0usize;
        let mut acc = 0usize;
        for (i, l) in lines.iter().enumerate() {
            if acc == current_line_start {
                line_idx = i;
                break;
            }
            acc += l.len() + 1;
        }

        let target_line = (line_idx as isize + delta).max(0) as usize;
        if target_line >= lines.len() {
            return self.content.len();
        }

        // Byte offset of the start of target_line
        let mut target_start = 0usize;
        for l in &lines[..target_line] {
            target_start += l.len() + 1;
        }
        let target_line_str = lines[target_line];
        let target_col = col.min(target_line_str.len());
        target_start + target_col
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }
        if self.last_bounds.is_empty() || self.last_layout.is_empty() {
            return 0;
        }
        // Find which line was clicked
        for (i, line_bounds) in self.last_bounds.iter().enumerate() {
            if position.y >= line_bounds.top() && position.y <= line_bounds.bottom() {
                let line = &self.last_layout[i];
                let local_x = position.x - line_bounds.left();
                let col = line.closest_index_for_x(local_x);
                // Convert (line_index, col) to absolute byte offset
                let mut byte = 0usize;
                let lines: Vec<&str> = self.content.split('\n').collect();
                for (j, l) in lines.iter().enumerate() {
                    if j == i {
                        return byte + col.min(l.len());
                    }
                    byte += l.len() + 1;
                }
            }
        }
        // Above the first line
        if position.y < self.last_bounds[0].top() {
            return 0;
        }
        // Below the last line
        self.content.len()
    }

    // ── UTF-16 conversion (copied from input.rs example) ───────────────────

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;
        for ch in self.content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }
        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;
        for ch in self.content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }
        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    // ── Action handlers (copied from input.rs example + vertical nav) ──────

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx)
        }
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), cx);
        } else {
            self.move_to(self.selected_range.end, cx)
        }
    }

    fn up(&mut self, _: &Up, _: &mut Window, cx: &mut Context<Self>) {
        let target = self.vertical_target(self.cursor_offset(), -1);
        self.move_to(target, cx);
    }

    fn down(&mut self, _: &Down, _: &mut Window, cx: &mut Context<Self>) {
        let target = self.vertical_target(self.cursor_offset(), 1);
        self.move_to(target, cx);
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    fn select_up(&mut self, _: &SelectUp, _: &mut Window, cx: &mut Context<Self>) {
        let target = self.vertical_target(self.cursor_offset(), -1);
        self.select_to(target, cx);
    }

    fn select_down(&mut self, _: &SelectDown, _: &mut Window, cx: &mut Context<Self>) {
        let target = self.vertical_target(self.cursor_offset(), 1);
        self.select_to(target, cx);
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
        self.select_to(self.content.len(), cx)
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.line_start_offset(self.cursor_offset()), cx);
    }

    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.line_end_offset(self.cursor_offset()), cx);
    }

    fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let prev = self.previous_boundary(self.cursor_offset());
            if self.cursor_offset() == prev {
                window.play_system_bell();
                return;
            }
            self.select_to(prev, cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let next = self.next_boundary(self.cursor_offset());
            if self.cursor_offset() == next {
                window.play_system_bell();
                return;
            }
            self.select_to(next, cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn new_line(&mut self, _: &NewLine, window: &mut Window, cx: &mut Context<Self>) {
        self.replace_text_in_range(None, "\n", window, cx);
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_selecting = true;
        if event.modifiers.shift {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        } else {
            self.move_to(self.index_for_mouse_position(event.position), cx)
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _: &mut Context<Self>) {
        self.is_selecting = false;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_selecting {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        }
    }

    fn show_character_palette(
        &mut self,
        _: &ShowCharacterPalette,
        window: &mut Window,
        _: &mut Context<Self>,
    ) {
        window.show_character_palette();
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.replace_text_in_range(None, &text, window, cx);
        }
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
        }
    }

    fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
            self.replace_text_in_range(None, "", window, cx)
        }
    }
}

// ── EntityInputHandler (copied verbatim from input.rs example) ─────────────

impl EntityInputHandler for EditorView {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                .into();
        self.selected_range = range.start + new_text.len()..range.start + new_text.len();
        self.marked_range.take();
        self.is_dirty = true;
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                .into();
        if !new_text.is_empty() {
            self.marked_range = Some(range.start..range.start + new_text.len());
        } else {
            self.marked_range = None;
        }
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.end)
            .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());

        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        // Return the overall editor bounds as a hint to the IME.
        Some(bounds)
    }

    fn character_index_for_point(
        &mut self,
        _point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        None
    }
}

impl Focusable for EditorView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// ── Multi-line TextElement (adapted from input.rs example) ─────────────────

struct TextElement {
    input: Entity<EditorView>,
}

struct PrepaintState {
    lines: Vec<ShapedLine>,
    line_origins: Vec<Point<Pixels>>,
    cursor: Option<PaintQuad>,
    selections: Vec<PaintQuad>,
    line_height: Pixels,
}

impl IntoElement for TextElement {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl gpui::Element for TextElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let input = self.input.read(cx);
        let line_count = input.content.lines().count().max(1) as f32;
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = (window.line_height() * line_count).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) -> PrepaintState {
        let input = self.input.read(cx);
        let line_height = window.line_height();
        let style = window.text_style();
        let font_size = style.font_size.to_pixels(window.rem_size());
        let font = style.font();
        let text_color = style.color;
        let cursor = input.cursor_offset();
        let selected_range = input.selected_range.clone();
        let is_focused = input.focus_handle.is_focused(window);
        let content = input.content.clone();

        // Split into lines and shape each one.
        let line_strs: Vec<&str> = content.split('\n').collect();
        let mut shaped_lines = Vec::with_capacity(line_strs.len());
        let mut line_origins = Vec::with_capacity(line_strs.len());

        for (i, line_str) in line_strs.iter().enumerate() {
            let display_text: SharedString = if line_str.is_empty() {
                " ".into()
            } else {
                (*line_str).into()
            };
            let run = TextRun {
                len: display_text.len(),
                font: font.clone(),
                color: text_color,
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            let runs = if let Some(marked_range) = input.marked_range.as_ref() {
                // Compute the portion of marked_range that overlaps this line.
                let mut byte = 0usize;
                let mut line_start_byte = 0usize;
                for (j, l) in line_strs.iter().enumerate() {
                    if j == i {
                        line_start_byte = byte;
                        break;
                    }
                    byte += l.len() + 1;
                }
                let line_end_byte = line_start_byte + line_str.len();
                let local_marked_start = marked_range.start.saturating_sub(line_start_byte);
                let local_marked_end = marked_range.end.clamp(line_start_byte, line_end_byte) - line_start_byte;

                if local_marked_end > local_marked_start && local_marked_start <= line_str.len() {
                    let ms = local_marked_start.min(display_text.len());
                    let me = local_marked_end.min(display_text.len());
                    vec![
                        TextRun { len: ms, ..run.clone() },
                        TextRun {
                            len: me - ms,
                            underline: Some(UnderlineStyle {
                                color: Some(run.color),
                                thickness: px(1.0),
                                wavy: false,
                            }),
                            ..run.clone()
                        },
                        TextRun { len: display_text.len() - me, ..run },
                    ]
                    .into_iter()
                    .filter(|r| r.len > 0)
                    .collect()
                } else {
                    vec![run]
                }
            } else {
                vec![run]
            };

            let shaped = window
                .text_system()
                .shape_line(display_text, font_size, &runs, None);
            let origin = point(bounds.left(), bounds.top() + line_height * i as f32);
            shaped_lines.push(shaped);
            line_origins.push(origin);
        }

        // Build selection + cursor quads across all lines.
        let mut selections: Vec<PaintQuad> = Vec::new();
        let mut cursor_quad: Option<PaintQuad> = None;

        for (i, (shaped, origin)) in shaped_lines.iter().zip(line_origins.iter()).enumerate() {
            let mut byte = 0usize;
            let mut line_start_byte = 0usize;
            for (j, l) in line_strs.iter().enumerate() {
                if j == i {
                    line_start_byte = byte;
                    break;
                }
                byte += l.len() + 1;
            }
            let line_end_byte = line_start_byte + line_strs[i].len();

            // Selection highlight on this line
            if !selected_range.is_empty() {
                let sel_start = selected_range.start;
                let sel_end = selected_range.end;
                if sel_start < line_end_byte && sel_end > line_start_byte {
                    let local_start = sel_start.max(line_start_byte) - line_start_byte;
                    let local_end = sel_end.min(line_end_byte) - line_start_byte;
                    let x0 = origin.x + shaped.x_for_index(local_start.min(shaped.text.len()));
                    let x1 = origin.x + shaped.x_for_index(local_end.min(shaped.text.len()));
                    selections.push(fill(
                        Bounds::from_corners(
                            point(x0, origin.y),
                            point(x1, origin.y + line_height),
                        ),
                        rgba(0x3311ff30),
                    ));
                }
            }

            // Cursor on this line
            if is_focused
                && cursor >= line_start_byte
                && cursor <= line_end_byte + 1
            {
                let col = cursor.saturating_sub(line_start_byte).min(shaped.text.len());
                let cursor_x = origin.x + shaped.x_for_index(col);
                cursor_quad = Some(fill(
                    Bounds::new(
                        point(cursor_x, origin.y),
                        size(px(2.), line_height),
                    ),
                    gpui::blue(),
                ));
            }
        }

        PrepaintState {
            lines: shaped_lines,
            line_origins,
            cursor: cursor_quad,
            selections,
            line_height,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut (),
        prepaint: &mut PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.input.clone()),
            cx,
        );

        // Paint selection highlights
        for sel in prepaint.selections.drain(..) {
            window.paint_quad(sel);
        }

        // Paint each text line
        let lh = prepaint.line_height;
        let lines = std::mem::take(&mut prepaint.lines);
        let origins = std::mem::take(&mut prepaint.line_origins);
        for (line, origin) in lines.iter().zip(origins.iter()) {
            line.paint(*origin, lh, gpui::TextAlign::Left, None, window, cx)
                .ok();
        }

        // Paint cursor on top
        if focus_handle.is_focused(window)
            && let Some(cursor) = prepaint.cursor.take()
        {
            window.paint_quad(cursor);
        }

        // Stash layout for mouse hit-testing
        let line_height = prepaint.line_height;
        let line_bounds: Vec<Bounds<Pixels>> = origins
            .iter()
            .map(|o| Bounds::new(*o, size(bounds.size.width, line_height)))
            .collect();
        self.input.update(cx, |input, _cx| {
            input.last_layout = lines;
            input.last_bounds = line_bounds;
        });
    }
}

// ── Simple markdown renderer for preview mode ──────────────────────────────

fn render_markdown_preview(source: &str) -> impl IntoElement {
    let mut elements: Vec<gpui::AnyElement> = Vec::new();

    for line in source.lines() {
        if line.starts_with("# ") {
            elements.push(
                div().mt_4().mb_2().text_size(px(26.)).font_weight(gpui::FontWeight::BOLD)
                    .child(line[2..].to_string()).into_any_element(),
            );
        } else if line.starts_with("## ") {
            elements.push(
                div().mt_3().mb_2().text_size(px(20.)).font_weight(gpui::FontWeight::BOLD)
                    .child(line[3..].to_string()).into_any_element(),
            );
        } else if line.starts_with("### ") {
            elements.push(
                div().mt_2().mb_1().text_size(px(17.)).font_weight(gpui::FontWeight::BOLD)
                    .child(line[4..].to_string()).into_any_element(),
            );
        } else if line.starts_with("> ") {
            elements.push(
                div().my_1().pl_3().border_l_2().border_color(rgb(0x7c3aed))
                    .italic().text_color(rgb(0x6b7280))
                    .child(line[2..].to_string()).into_any_element(),
            );
        } else if line.starts_with("- ") || line.starts_with("* ") {
            elements.push(
                div().pl_4().child(format!("• {}", &line[2..])).into_any_element(),
            );
        } else if line.trim().is_empty() {
            elements.push(div().h(px(8.)).into_any_element());
        } else {
            elements.push(
                div().my_px().text_size(px(15.)).text_color(rgb(0x374151))
                    .child(line.to_string()).into_any_element(),
            );
        }
    }

    div().flex().flex_col().children(elements)
}

// ── Render ─────────────────────────────────────────────────────────────────

impl Render for EditorView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let file_name = self.file_name().to_string();
        let is_dirty = self.is_dirty;
        let mode = self.mode;
        let mode_label = match mode {
            EditorMode::Edit => "EDIT",
            EditorMode::Preview => "PREVIEW",
        };

        // Header
        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .h(px(40.))
            .px_4()
            .border_b_1()
            .border_color(rgb(0xe5e5e5))
            .bg(rgb(0xfafafa))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div().text_size(px(14.)).font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(rgb(0x111111))
                            .child(file_name),
                    )
                    .when(is_dirty, |p| {
                        p.child(div().w(px(7.)).h(px(7.)).rounded_full().bg(rgb(0xf97316)))
                    }),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .id("mode-toggle")
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .border_1()
                            .border_color(rgb(0xd1d5db))
                            .bg(rgb(0xf3f4f6))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0xe5e7eb)))
                            .text_size(px(11.))
                            .text_color(rgb(0x6b7280))
                            .child(mode_label)
                            .on_click(cx.listener(|editor, _, _, cx| {
                                editor.mode = match editor.mode {
                                    EditorMode::Edit => EditorMode::Preview,
                                    EditorMode::Preview => EditorMode::Edit,
                                };
                                cx.notify();
                            })),
                    )
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(rgb(0xaaaaaa))
                            .child("Ctrl+E preview • Ctrl+S save"),
                    ),
            );

        let body = match mode {
            EditorMode::Edit => div()
                .id("editor-scroll")
                .flex_1()
                .min_h_0()
                .overflow_scroll()
                .p_4()
                .bg(rgb(0xffffff))
                // NOTE: key_context + track_focus make this input participate in focus dispatch
                .key_context("NoteEditor")
                .track_focus(&self.focus_handle)
                .cursor(CursorStyle::IBeam)
                // Register all action handlers (same pattern as input.rs example)
                .on_action(cx.listener(Self::backspace))
                .on_action(cx.listener(Self::delete))
                .on_action(cx.listener(Self::left))
                .on_action(cx.listener(Self::right))
                .on_action(cx.listener(Self::up))
                .on_action(cx.listener(Self::down))
                .on_action(cx.listener(Self::select_left))
                .on_action(cx.listener(Self::select_right))
                .on_action(cx.listener(Self::select_up))
                .on_action(cx.listener(Self::select_down))
                .on_action(cx.listener(Self::select_all))
                .on_action(cx.listener(Self::home))
                .on_action(cx.listener(Self::end))
                .on_action(cx.listener(Self::show_character_palette))
                .on_action(cx.listener(Self::paste))
                .on_action(cx.listener(Self::cut))
                .on_action(cx.listener(Self::copy))
                .on_action(cx.listener(Self::save_action))
                .on_action(cx.listener(Self::toggle_preview))
                .on_action(cx.listener(Self::new_line))
                .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
                .on_mouse_move(cx.listener(Self::on_mouse_move))
                .child(TextElement { input: cx.entity() })
                .into_any_element(),
            EditorMode::Preview => {
                let content = self.content.to_string();
                div()
                    .id("preview-scroll")
                    .flex_1()
                    .min_h_0()
                    .overflow_scroll()
                    .p_6()
                    .bg(rgb(0xffffff))
                    .max_w(px(780.))
                    .child(render_markdown_preview(&content))
                    .into_any_element()
            }
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xffffff))
            .child(header)
            .child(body)
    }
}
