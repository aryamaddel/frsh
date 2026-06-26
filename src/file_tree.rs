use std::path::PathBuf;
use std::rc::Rc;
use gpui::{
    App, Context, Entity, FocusHandle, Focusable, InteractiveElement, Render,
    UniformListScrollHandle, Window, actions, div, prelude::*, px, rgb, uniform_list,
};
use crate::vault::Vault;

actions!(
    file_tree,
    [SelectNext, SelectPrev, OpenSelected, DeleteFile, NewNoteAction]
);

pub struct FileTreeView {
    vault: Entity<Vault>,
    on_file_click: Rc<dyn Fn(PathBuf, &mut Window, &mut App) + 'static>,
    selected_index: Option<usize>,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
    /// Filter string for the search box
    filter: String,
    #[allow(dead_code)]
    filter_focus: FocusHandle,
}

impl FileTreeView {
    pub fn new(
        vault: Entity<Vault>,
        on_file_click: impl Fn(PathBuf, &mut Window, &mut App) + 'static,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe(&vault, |_, _, cx| cx.notify()).detach();

        Self {
            vault,
            on_file_click: Rc::new(on_file_click),
            selected_index: None,
            focus_handle: cx.focus_handle(),
            scroll_handle: UniformListScrollHandle::new(),
            filter: String::new(),
            filter_focus: cx.focus_handle(),
        }
    }

    /// Sorted + filtered list of file paths
    pub fn visible_files(&self, cx: &App) -> Vec<PathBuf> {
        let vault = self.vault.read(cx);
        let files = vault.files();
        let filter = self.filter.to_lowercase();
        files
            .iter()
            .filter(|p| {
                if filter.is_empty() { return true; }
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.to_lowercase().contains(&filter))
                    .unwrap_or(false)
            })
            .cloned()
            .collect()
    }

    pub fn vault(&self) -> &Entity<Vault> {
        &self.vault
    }

    pub fn create_new_note(&mut self, cx: &mut Context<Self>) {
        self.vault.update(cx, |v, cx| {
            let mut index = 1usize;
            let mut name = "Untitled.md".to_string();
            while v.files().iter().any(|f| {
                f.file_name().and_then(|n| n.to_str()).unwrap_or("") == name
            }) {
                name = format!("Untitled {index}.md");
                index += 1;
            }
            if let Err(e) = v.create_file(&name) {
                eprintln!("Create file error: {e:?}");
            }
            cx.notify();
        });
    }

    fn delete_selected(&mut self, cx: &mut Context<Self>) {
        let files = self.visible_files(cx);
        if let Some(idx) = self.selected_index {
            if let Some(path) = files.get(idx) {
                let path = path.clone();
                self.vault.update(cx, |v, cx| {
                    if let Err(e) = v.delete_file(&path) {
                        eprintln!("Delete file error: {e:?}");
                    }
                    cx.notify();
                });
                self.selected_index = None;
            }
        }
    }

    fn open_selected_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let files = self.visible_files(cx);
        if let Some(idx) = self.selected_index {
            if let Some(path) = files.get(idx).cloned() {
                let handler = self.on_file_click.clone();
                handler(path, window, cx);
            }
        }
    }

    fn select_next_action(&mut self, _: &SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        let count = self.visible_files(cx).len();
        if count == 0 { return; }
        let next = self.selected_index
            .map(|i| (i + 1).min(count - 1))
            .unwrap_or(0);
        self.selected_index = Some(next);
        self.scroll_handle.scroll_to_item(next, gpui::ScrollStrategy::Nearest);
        cx.notify();
    }

    fn select_prev_action(&mut self, _: &SelectPrev, _: &mut Window, cx: &mut Context<Self>) {
        let count = self.visible_files(cx).len();
        if count == 0 { return; }
        let prev = self.selected_index
            .map(|i| i.saturating_sub(1))
            .unwrap_or(0);
        self.selected_index = Some(prev);
        self.scroll_handle.scroll_to_item(prev, gpui::ScrollStrategy::Nearest);
        cx.notify();
    }

    fn open_selected_action(&mut self, _: &OpenSelected, window: &mut Window, cx: &mut Context<Self>) {
        self.open_selected_file(window, cx);
    }

    fn delete_file_action(&mut self, _: &DeleteFile, _: &mut Window, cx: &mut Context<Self>) {
        self.delete_selected(cx);
    }

    fn new_note_action(&mut self, _: &NewNoteAction, _: &mut Window, cx: &mut Context<Self>) {
        self.create_new_note(cx);
    }
}

impl Focusable for FileTreeView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for FileTreeView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let files = self.visible_files(cx);
        let file_count = files.len();
        let selected_index = self.selected_index;
        let click_handler = self.on_file_click.clone();

        // Pre-build file names for the uniform_list closure
        let file_names: Vec<(PathBuf, String)> = files
            .into_iter()
            .map(|p| {
                let name = p.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Untitled")
                    .to_string();
                (p, name)
            })
            .collect();

        let files_for_list = std::rc::Rc::new(file_names);

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf8f9fa))
            .border_r_1()
            .border_color(rgb(0xe5e5e5))
            .key_context("FileTree")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_next_action))
            .on_action(cx.listener(Self::select_prev_action))
            .on_action(cx.listener(Self::open_selected_action))
            .on_action(cx.listener(Self::delete_file_action))
            .on_action(cx.listener(Self::new_note_action))
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(48.))
                    .px_4()
                    .border_b_1()
                    .border_color(rgb(0xe5e5e5))
                    .flex_shrink_0()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_size(px(14.))
                            .text_color(rgb(0x222222))
                            .child("Notes")
                    )
                    .child(
                        // New note button
                        div()
                            .id("new-note-btn")
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(26.))
                            .h(px(26.))
                            .rounded_md()
                            .cursor_pointer()
                            .bg(rgb(0x4b6fff))
                            .text_color(rgb(0xffffff))
                            .text_size(px(18.))
                            .font_weight(gpui::FontWeight::BOLD)
                            .hover(|s| s.bg(rgb(0x3a5ddd)))
                            .child("+")
                            .on_click(cx.listener(|view, _, _window, cx| {
                                view.create_new_note(cx);
                            }))
                    )
            )
            // Search / filter box
            .child(
                div()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(rgb(0xe5e5e5))
                    .flex_shrink_0()
                    .child(
                        div()
                            .id("search-box")
                            .flex()
                            .items_center()
                            .h(px(28.))
                            .px_2()
                            .rounded_md()
                            .border_1()
                            .border_color(rgb(0xd1d5db))
                            .bg(rgb(0xffffff))
                            .text_size(px(12.))
                            .text_color(rgb(0x374151))
                            .child(
                                if self.filter.is_empty() {
                                    div()
                                        .text_color(rgb(0xaaaaaa))
                                        .child("Filter notes...")
                                        .into_any_element()
                                } else {
                                    div()
                                        .text_color(rgb(0x374151))
                                        .child(self.filter.clone())
                                        .into_any_element()
                                }
                            )
                    )
            )
            // File count
            .child(
                div()
                    .px_4()
                    .py_1()
                    .text_size(px(11.))
                    .text_color(rgb(0x9ca3af))
                    .flex_shrink_0()
                    .child(format!("{} notes", file_count))
            )
            // Virtual file list
            .child(
                uniform_list(
                    "file-list",
                    file_count,
                    {
                        let files_for_list = files_for_list.clone();
                        let click_handler = click_handler.clone();
                        cx.processor(move |_this, range: std::ops::Range<usize>, _window, cx| {
                            range.map(|ix| {
                                let (path, name) = &files_for_list[ix];
                                let is_selected = selected_index == Some(ix);
                                let path_clone = path.clone();
                                let handler = click_handler.clone();

                                div()
                                    .id(ix)
                                    .flex()
                                    .items_center()
                                    .h(px(32.))
                                    .px_3()
                                    .mx_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .when(is_selected, |el| el.bg(rgb(0xe8edff)))
                                    .when(!is_selected, |el| el.hover(|s| s.bg(rgb(0xf0f2ff))))
                                    .on_click(move |_, window, cx| {
                                        handler(path_clone.clone(), window, cx);
                                    })
                                    .on_mouse_down(
                                        gpui::MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            this.selected_index = Some(ix);
                                            cx.notify();
                                        }),
                                    )
                                    .child(
                                        // File icon (emoji)
                                        div()
                                            .mr_2()
                                            .text_size(px(13.))
                                            .child(if name.ends_with(".md") { "📄" } else { "📃" })
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .text_size(px(13.))
                                            .text_color(if is_selected { rgb(0x2d3fe0) } else { rgb(0x374151) })
                                            .font_weight(if is_selected { gpui::FontWeight::MEDIUM } else { gpui::FontWeight::NORMAL })
                                            .truncate()
                                            .child(name.clone())
                                    )
                            }).collect()
                        })
                    }
                )
                .flex_1()
                .track_scroll(&self.scroll_handle)
            )
            // Bottom: keyboard hints
            .child(
                div()
                    .px_3()
                    .py_2()
                    .border_t_1()
                    .border_color(rgb(0xe5e5e5))
                    .flex_shrink_0()
                    .text_size(px(10.))
                    .text_color(rgb(0xbbbbbbb))
                    .child("↑↓ navigate  ↵ open  Del delete  Ctrl+N new")
            )
    }
}
