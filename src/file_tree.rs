use std::path::PathBuf;
use std::rc::Rc;
use gpui::prelude::*;
use gpui::{
    div, Render, Context, InteractiveElement, rgb, px, Window, App, Entity,
};
use crate::vault::Vault;

pub struct FileTreeView {
    vault: Entity<Vault>,
    on_file_click: Rc<dyn Fn(PathBuf, &mut Window, &mut App) + 'static>,
}

impl FileTreeView {
    pub fn new(
        vault: Entity<Vault>,
        on_file_click: impl Fn(PathBuf, &mut Window, &mut App) + 'static,
        cx: &mut Context<Self>,
    ) -> Self {
        // Observe the vault model so that when the file system updates, we re-render
        cx.observe(&vault, |_, _, cx| {
            cx.notify();
        }).detach();

        Self {
            vault,
            on_file_click: Rc::new(on_file_click),
        }
    }

    fn create_new_file(&mut self, cx: &mut Context<Self>) {
        self.vault.update(cx, |v, cx| {
            // Find a unique name like "Untitled.md"
            let mut index = 1;
            let mut name = "Untitled.md".to_string();
            while v.files().iter().any(|f| f.file_name().unwrap().to_str().unwrap() == name) {
                name = format!("Untitled {}.md", index);
                index += 1;
            }
            
            if let Ok(_path) = v.create_file(&name) {
                cx.notify();
            }
        });
    }
}

impl Render for FileTreeView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let vault = self.vault.read(cx);
        let files = vault.files().to_vec();

        let mut file_elements = Vec::new();
        for file in files {
            let file_name = file.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Untitled")
                .to_string();
            
            let file_path = file.clone();
            let click_handler = self.on_file_click.clone();
            let file_id = format!("file-item-{}", file_name);
            
            file_elements.push(
                div()
                    .id(file_id)
                    .flex()
                    .items_center()
                    .h(px(32.))
                    .px_3()
                    .rounded_md()
                    .cursor_pointer()
                    .text_size(px(13.))
                    .text_color(rgb(0x333333))
                    .hover(|this| this.bg(rgb(0xf0f0f0)))
                    .active(|this| this.bg(rgb(0xe0e0e0)))
                    .child(file_name)
                    .on_click(move |_, window, cx| {
                        click_handler(file_path.clone(), window, cx);
                    })
            );
        }

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf8f9fa))
            .border_r_1()
            .border_color(rgb(0xe5e5e5))
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(48.))
                    .px_4()
                    .border_b_1()
                    .border_color(rgb(0xe5e5e5))
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_size(px(14.))
                            .text_color(rgb(0x222222))
                            .child("Vault Files")
                    )
                    .child(
                        // New File button
                        div()
                            .id("new-file-btn")
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(24.))
                            .h(px(24.))
                            .rounded_md()
                            .cursor_pointer()
                            .bg(rgb(0x4b6fff))
                            .text_color(rgb(0xffffff))
                            .text_size(px(16.))
                            .hover(|this| this.bg(rgb(0x3a5ddd)))
                            .child("+")
                            .on_click(cx.listener(|view, _, _window, cx| {
                                view.create_new_file(cx);
                            }))
                    )
            )
            .child(
                // Scrollable File List
                div()
                    .id("file-list-scroll")
                    .flex_1()
                    .p_2()
                    .overflow_y_scroll()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .children(file_elements)
                    )
            )
    }
}
