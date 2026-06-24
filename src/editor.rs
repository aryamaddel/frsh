use std::path::PathBuf;
use gpui::prelude::*;
use gpui::{
    div, FocusHandle, Focusable, InteractiveElement, KeyDownEvent, rgb, px, Render, Context, App, Window, Entity,
};
use crate::vault::Vault;

pub struct EditorView {
    vault: Entity<Vault>,
    file_path: PathBuf,
    content: String,
    cursor_pos: usize,
    focus_handle: FocusHandle,
    is_dirty: bool,
}

impl EditorView {
    pub fn new(vault: Entity<Vault>, file_path: PathBuf, cx: &mut Context<Self>) -> Self {
        let content = vault.read_with(cx, |v, _| {
            v.read_file(&file_path).unwrap_or_default()
        });

        Self {
            vault,
            file_path,
            cursor_pos: content.len(),
            content,
            focus_handle: cx.focus_handle(),
            is_dirty: false,
        }
    }

    pub fn save(&mut self, cx: &mut Context<Self>) {
        let path = self.file_path.clone();
        let content = self.content.clone();
        self.vault.update(cx, |v, cx| {
            if let Err(e) = v.save_file(&path, &content) {
                eprintln!("Failed to save file: {:?}", e);
            } else {
                cx.notify();
            }
        });
        self.is_dirty = false;
        cx.notify();
    }

    fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
        let keystroke = &event.keystroke;
        
        // Handle shortcuts (like save with Ctrl+S / Cmd+S)
        if (keystroke.modifiers.control || keystroke.modifiers.platform) && keystroke.key == "s" {
            self.save(cx);
            return;
        }

        match keystroke.key.as_str() {
            "backspace" => {
                if self.cursor_pos > 0 {
                    // Handle UTF-8 boundary correctly
                    let mut prev_char_pos = self.cursor_pos - 1;
                    while prev_char_pos > 0 && !self.content.is_char_boundary(prev_char_pos) {
                        prev_char_pos -= 1;
                    }
                    self.content.drain(prev_char_pos..self.cursor_pos);
                    self.cursor_pos = prev_char_pos;
                    self.is_dirty = true;
                }
            }
            "delete" => {
                if self.cursor_pos < self.content.len() {
                    let mut next_char_pos = self.cursor_pos + 1;
                    while next_char_pos < self.content.len() && !self.content.is_char_boundary(next_char_pos) {
                        next_char_pos += 1;
                    }
                    self.content.drain(self.cursor_pos..next_char_pos);
                    self.is_dirty = true;
                }
            }
            "enter" => {
                self.content.insert(self.cursor_pos, '\n');
                self.cursor_pos += 1;
                self.is_dirty = true;
            }
            "space" => {
                self.content.insert(self.cursor_pos, ' ');
                self.cursor_pos += 1;
                self.is_dirty = true;
            }
            "left" => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                    while self.cursor_pos > 0 && !self.content.is_char_boundary(self.cursor_pos) {
                        self.cursor_pos -= 1;
                    }
                }
            }
            "right" => {
                if self.cursor_pos < self.content.len() {
                    self.cursor_pos += 1;
                    while self.cursor_pos < self.content.len() && !self.content.is_char_boundary(self.cursor_pos) {
                        self.cursor_pos += 1;
                    }
                }
            }
            "home" => {
                // Move to start of current line
                let before_cursor = &self.content[..self.cursor_pos];
                if let Some(last_newline) = before_cursor.rfind('\n') {
                    self.cursor_pos = last_newline + 1;
                } else {
                    self.cursor_pos = 0;
                }
            }
            "end" => {
                // Move to end of current line
                let after_cursor = &self.content[self.cursor_pos..];
                if let Some(next_newline) = after_cursor.find('\n') {
                    self.cursor_pos += next_newline;
                } else {
                    self.cursor_pos = self.content.len();
                }
            }
            _ => {
                // Type characters
                if let Some(ref c) = keystroke.key_char {
                    if !keystroke.modifiers.control && !keystroke.modifiers.platform {
                        self.content.insert_str(self.cursor_pos, c);
                        self.cursor_pos += c.len();
                        self.is_dirty = true;
                    }
                }
            }
        }
        cx.notify();
    }
}

impl Focusable for EditorView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for EditorView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_focused = self.focus_handle.is_focused(window);
        let file_name = self.file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled");

        // Split text into lines and render them
        let mut lines_elements = Vec::new();
        let mut current_byte_idx = 0;
        
        let lines: Vec<&str> = self.content.split('\n').collect();
        let total_lines = lines.len();

        for (i, line) in lines.into_iter().enumerate() {
            let line_len = line.len();
            let line_end_idx = current_byte_idx + line_len;

            let is_cursor_in_this_line = self.cursor_pos >= current_byte_idx 
                && (self.cursor_pos <= line_end_idx || (i == total_lines - 1 && self.cursor_pos == self.content.len()));

            let line_el = div()
                .flex()
                .h(px(24.))
                .items_center()
                .font_family("Consolas")
                .text_size(px(14.))
                .child(
                    // Line number
                    div()
                        .w(px(32.))
                        .text_color(rgb(0x888888))
                        .text_align(gpui::TextAlign::Right)
                        .mr(px(12.))
                        .child(format!("{}", i + 1))
                );

            let line_content_el = if is_cursor_in_this_line {
                let relative_cursor_pos = self.cursor_pos - current_byte_idx;
                let before = &line[..relative_cursor_pos];
                let after = &line[relative_cursor_pos..];

                div()
                    .flex()
                    .items_center()
                    .child(before.to_string())
                    .child(
                        // The blinking / static cursor
                        div()
                            .w(px(2.))
                            .h(px(18.))
                            .bg(if is_focused { rgb(0x4b6fff) } else { rgb(0x888888) })
                    )
                    .child(after.to_string())
            } else {
                div().child(line.to_string())
            };

            lines_elements.push(
                line_el.child(line_content_el)
            );

            current_byte_idx = line_end_idx + 1; // +1 for the newline character
        }

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xffffff))
            .child(
                // Tab Header / Title
                div()
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
                            .child(file_name.to_string())
                            .when(self.is_dirty, |parent| {
                                parent.child(
                                    div()
                                        .w(px(8.))
                                        .h(px(8.))
                                        .rounded_full()
                                        .bg(rgb(0xff5555))
                                )
                            })
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(rgb(0x888888))
                            .child("Ctrl+S to save")
                    )
            )
            .child(
                // Scrollable Text Area
                div()
                    .id("text-area")
                    .flex_1()
                    .p_4()
                    .overflow_y_scroll()
                    .track_focus(&self.focus_handle)
                    .on_key_down(cx.listener(|editor, event, _window, cx| {
                        editor.handle_key_down(event, cx);
                    }))
                    .on_click(cx.listener(|editor, _event, window, cx| {
                        editor.focus_handle(cx).focus(window, cx);
                    }))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .children(lines_elements)
                    )
            )
    }
}
