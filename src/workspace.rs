use std::path::PathBuf;
use gpui::prelude::*;
use gpui::{
    div, Render, Context, rgb, px, Entity, Window, Focusable,
};
use crate::vault::Vault;
use crate::file_tree::FileTreeView;
use crate::editor::EditorView;

pub struct Workspace {
    vault: Entity<Vault>,
    file_tree: Entity<FileTreeView>,
    active_editor: Option<Entity<EditorView>>,
}

impl Workspace {
    // Helper to open a file in the workspace
    pub fn open_file(&mut self, path: PathBuf, window: &mut Window, cx: &mut Context<Self>) {
        let editor = cx.new(|cx| {
            EditorView::new(self.vault.clone(), path, cx)
        });
        
        // Focus the editor automatically
        editor.update(cx, |editor, cx| {
            editor.focus_handle(cx).focus(window, cx);
        });
        
        self.active_editor = Some(editor);
        cx.notify();
    }

    pub fn new_with_callback(vault: Entity<Vault>, cx: &mut Context<Self>) -> Self {
        let workspace_handle = cx.entity().downgrade();
        
        let file_tree = cx.new(|cx| {
            FileTreeView::new(
                vault.clone(),
                move |path, window, cx| {
                    if let Some(workspace) = workspace_handle.upgrade() {
                        workspace.update(cx, |ws, cx| {
                            ws.open_file(path, window, cx);
                        });
                    }
                },
                cx,
            )
        });

        Self {
            vault,
            file_tree,
            active_editor: None,
        }
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .bg(rgb(0xffffff))
            .child(
                // Left Sidebar (File Tree)
                div()
                    .w(px(240.))
                    .h_full()
                    .child(self.file_tree.clone())
            )
            .child(
                // Right Main Area (Editor or Placeholder)
                div()
                    .flex_1()
                    .h_full()
                    .child(
                        if let Some(ref editor) = self.active_editor {
                            div().size_full().child(editor.clone())
                        } else {
                            // Placeholder welcome screen
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .size_full()
                                .bg(rgb(0xfdfdfd))
                                .gap_3()
                                .child(
                                    div()
                                        .text_size(px(20.))
                                        .font_weight(gpui::FontWeight::BOLD)
                                        .text_color(rgb(0x111111))
                                        .child("Welcome to your Notes")
                                )
                                .child(
                                    div()
                                        .text_size(px(13.))
                                        .text_color(rgb(0x666666))
                                        .child("Select a file from the sidebar or click '+' to create a new note.")
                                )
                        }
                    )
            )
    }
}
