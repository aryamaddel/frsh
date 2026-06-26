#![cfg_attr(target_family = "wasm", no_main)]

mod actions;
mod editor;
mod file_tree;
mod titlebar;
mod vault;
mod workspace;

use std::path::PathBuf;

use gpui::{App, Bounds, prelude::*, px, size};
use crate::titlebar::TitleBar;
use crate::vault::Vault;
use crate::workspace::Workspace;
use crate::actions::register_keybindings;

fn run_app() {
    gpui_platform::application().run(|cx: &mut App| {
        // Register all keybindings (editor, workspace, file tree)
        register_keybindings(cx);

        // Register application menus (native menu bar on macOS,
        // custom-rendered in title bar on Windows/Linux)
        titlebar::register(cx);

        // Vault folder: "vault/" next to the working directory
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let vault_path = current_dir.join("vault");

        let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);
        let options = titlebar::window_options(bounds);

        cx.open_window(options, |_window, cx| {
            cx.new(|cx| {
                let vault = cx.new(|cx| Vault::new(vault_path, cx));
                let mut workspace = Workspace::new_with_callback(vault, cx);
                workspace.title_bar = Some(cx.new(|cx| TitleBar::new(cx)));
                workspace
            })
        })
        .unwrap();

        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_app();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_app();
}
