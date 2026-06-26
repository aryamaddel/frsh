use std::path::PathBuf;
use gpui::{
    Context, Entity, FocusHandle, Focusable, Render, Window,
    div, prelude::*, px, rgb,
};
use crate::vault::Vault;
use crate::file_tree::FileTreeView;
use crate::editor::EditorView;
use crate::titlebar::TitleBar;
use crate::actions::{
    CloseTab, NewNote, NextTab, PrevTab, ToggleSidebar,
};

/// One open tab
pub struct NoteTab {
    path: PathBuf,
    editor: Entity<EditorView>,
}

pub struct Workspace {
    vault: Entity<Vault>,
    file_tree: Entity<FileTreeView>,
    pub tabs: Vec<NoteTab>,
    pub active_tab: usize,
    sidebar_visible: bool,
    focus_handle: FocusHandle,
    /// The custom title bar (set by main.rs after construction).
    pub title_bar: Option<Entity<TitleBar>>,
}

impl Workspace {
    pub fn new_with_callback(vault: Entity<Vault>, cx: &mut Context<Self>) -> Self {
        let workspace_handle = cx.entity().downgrade();

        let file_tree = cx.new(|cx| {
            FileTreeView::new(
                vault.clone(),
                move |path, window, cx| {
                    if let Some(ws) = workspace_handle.upgrade() {
                        ws.update(cx, |ws, cx| {
                            ws.open_or_focus_tab(path, window, cx);
                        });
                    }
                },
                cx,
            )
        });

        Self {
            vault,
            file_tree,
            tabs: Vec::new(),
            active_tab: 0,
            sidebar_visible: true,
            focus_handle: cx.focus_handle(),
            title_bar: None,
        }
    }

    /// Open a file in a new tab, or focus the existing tab if already open.
    pub fn open_or_focus_tab(
        &mut self,
        path: PathBuf,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(idx) = self.tabs.iter().position(|t| t.path == path) {
            self.active_tab = idx;
            self.focus_active_editor(window, cx);
            cx.notify();
            return;
        }

        let editor = cx.new(|cx| EditorView::new(self.vault.clone(), path.clone(), cx));
        self.tabs.push(NoteTab { path, editor });
        self.active_tab = self.tabs.len() - 1;
        self.focus_active_editor(window, cx);
        cx.notify();
    }

    fn focus_active_editor(&self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(tab) = self.tabs.get(self.active_tab) {
            tab.editor.update(cx, |editor, cx| {
                editor.focus_handle(cx).focus(window, cx);
            });
        }
    }

    fn close_tab(&mut self, idx: usize, _: &mut Window, cx: &mut Context<Self>) {
        if idx < self.tabs.len() {
            self.tabs.remove(idx);
            if self.active_tab >= self.tabs.len() && !self.tabs.is_empty() {
                self.active_tab = self.tabs.len() - 1;
            }
            cx.notify();
        }
    }

    // ── Actions ────────────────────────────────────────────────────────────

    fn action_save(&mut self, _: &crate::editor::Save, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(tab) = self.tabs.get(self.active_tab) {
            tab.editor.update(cx, |editor, cx| editor.save(window, cx));
        }
    }

    fn action_new_note(&mut self, _: &NewNote, _: &mut Window, cx: &mut Context<Self>) {
        let vault = self.file_tree.read(cx).vault().clone();
        vault.update(cx, |v, cx| {
            let mut i = 1;
            let mut name = "Untitled.md".to_string();
            while v.files().iter().any(|f| {
                f.file_name().and_then(|n| n.to_str()).unwrap_or("") == name
            }) {
                name = format!("Untitled {i}.md");
                i += 1;
            }
            let _ = v.create_file(&name);
            cx.notify();
        });
    }

    fn action_close_tab(&mut self, _: &CloseTab, window: &mut Window, cx: &mut Context<Self>) {
        let idx = self.active_tab;
        self.close_tab(idx, window, cx);
    }

    fn action_toggle_sidebar(&mut self, _: &ToggleSidebar, _: &mut Window, cx: &mut Context<Self>) {
        self.sidebar_visible = !self.sidebar_visible;
        cx.notify();
    }

    fn action_next_tab(&mut self, _: &NextTab, _: &mut Window, cx: &mut Context<Self>) {
        if !self.tabs.is_empty() {
            self.active_tab = (self.active_tab + 1) % self.tabs.len();
            cx.notify();
        }
    }

    fn action_prev_tab(&mut self, _: &PrevTab, _: &mut Window, cx: &mut Context<Self>) {
        if !self.tabs.is_empty() {
            self.active_tab = self.active_tab
                .checked_sub(1)
                .unwrap_or(self.tabs.len() - 1);
            cx.notify();
        }
    }
}

impl Focusable for Workspace {
    fn focus_handle(&self, _: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active = self.active_tab;
        let sidebar_visible = self.sidebar_visible;

        // ── Tab bar (includes sidebar toggle on the left) ──────────────
        let tabs_bar = div()
            .flex()
            .flex_row()
            .w_full()
            .h(px(36.))
            .bg(rgb(0xf7f7f8))
            .border_b_1()
            .border_color(rgb(0xe5e5e5))
            .flex_shrink_0()
            // Sidebar toggle button (leftmost)
            .child(
                div()
                    .id("sidebar-toggle")
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(36.))
                    .h_full()
                    .flex_shrink_0()
                    .cursor_pointer()
                    .text_size(px(14.))
                    .text_color(rgb(0x555555))
                    .hover(|s| s.bg(rgb(0xe8e8e8)))
                    .border_r_1()
                    .border_color(rgb(0xe5e5e5))
                    .on_click(cx.listener(|ws, _, _, cx| {
                        ws.sidebar_visible = !ws.sidebar_visible;
                        cx.notify();
                    }))
                    .child(if sidebar_visible { "◀" } else { "▶" })
            )
            .children(self.tabs.iter().enumerate().map(|(ix, tab)| {
                let is_active = ix == active;
                let name = tab.path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Untitled")
                    .to_string();
                let is_dirty = tab.editor.read(cx).is_dirty();

                div()
                    .id(ix)
                    .flex()
                    .flex_row()
                    .items_center()
                    .h_full()
                    .px_3()
                    .gap_2()
                    .cursor_pointer()
                    .border_r_1()
                    .border_color(rgb(0xe5e5e5))
                    .when(is_active, |el| {
                        el.bg(rgb(0xffffff))
                          .border_b_2()
                          .border_color(rgb(0x4b6fff))
                    })
                    .when(!is_active, |el| {
                        el.bg(rgb(0xf7f7f8))
                          .hover(|s| s.bg(rgb(0xeeeeee)))
                    })
                    .on_click(cx.listener(move |ws, _, _, cx| {
                        ws.active_tab = ix;
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(if is_active { rgb(0x111111) } else { rgb(0x555555) })
                            .font_weight(if is_active {
                                gpui::FontWeight::MEDIUM
                            } else {
                                gpui::FontWeight::NORMAL
                            })
                            .child(name)
                    )
                    .when(is_dirty, |el| {
                        el.child(
                            div()
                                .w(px(6.))
                                .h(px(6.))
                                .rounded_full()
                                .bg(rgb(0xf97316))
                        )
                    })
                    .child(
                        div()
                            .id(("close-tab", ix))
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(18.))
                            .h(px(18.))
                            .rounded_sm()
                            .text_size(px(14.))
                            .text_color(rgb(0x999999))
                            .hover(|s| s.bg(rgb(0xdddddd)).text_color(rgb(0x333333)))
                            .on_click(cx.listener(move |ws, _, window, cx| {
                                ws.close_tab(ix, window, cx);
                            }))
                            .child("×")
                    )
            }))
            .child(div().flex_1());

        // ── Root layout: title bar at top, then main content ───────────
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xffffff))
            .key_context("Workspace")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::action_save))
            .on_action(cx.listener(Self::action_new_note))
            .on_action(cx.listener(Self::action_close_tab))
            .on_action(cx.listener(Self::action_toggle_sidebar))
            .on_action(cx.listener(Self::action_next_tab))
            .on_action(cx.listener(Self::action_prev_tab))
            // ── Title bar (first child) ────────────────────────────────
            .when_some(self.title_bar.clone(), |root, title_bar| {
                root.child(title_bar)
            })
            // ── Main content area: sidebar + editor ─────────────────────
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .min_h_0()
                    .when(sidebar_visible, |el| {
                        el.child(
                            div()
                                .w(px(240.))
                                .h_full()
                                .flex_shrink_0()
                                .child(self.file_tree.clone())
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .h_full()
                            .min_w_0()
                            .child(tabs_bar)
                            .child(
                                if let Some(tab) = self.tabs.get(active) {
                                    div()
                                        .flex_1()
                                        .min_h_0()
                                        .child(tab.editor.clone())
                                        .into_any_element()
                                } else {
                                    div()
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .justify_center()
                                        .flex_1()
                                        .bg(rgb(0xfdfdfd))
                                        .gap_4()
                                        .child(
                                            div()
                                                .text_size(px(48.))
                                                .child("📓")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(22.))
                                                .font_weight(gpui::FontWeight::BOLD)
                                                .text_color(rgb(0x111111))
                                                .child("frsh")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(14.))
                                                .text_color(rgb(0x666666))
                                                .child("Select a note from the sidebar, or press Ctrl+N to create one.")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(12.))
                                                .text_color(rgb(0xaaaaaa))
                                                .child("Ctrl+N new  •  Ctrl+B toggle sidebar  •  Ctrl+E toggle preview")
                                        )
                                        .into_any_element()
                                }
                            )
                    )
            )
    }
}
