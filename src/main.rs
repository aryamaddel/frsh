#![cfg_attr(target_family = "wasm", no_main)]

mod vault;
mod editor;
mod file_tree;
mod workspace;

use std::path::PathBuf;
use gpui::{
    App, WindowBounds, WindowOptions, prelude::*, px, size, Bounds,
};
use crate::vault::Vault;
use crate::workspace::Workspace;

fn run_app() {
    gpui_platform::application().run(|cx: &mut App| {
        // We will default to a folder called "vault" in the current working directory
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let vault_path = current_dir.join("vault");
        
        // Setup window options
        let bounds = Bounds::centered(None, size(px(1024.0), px(768.0)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };

        cx.open_window(options, |_, cx| {
            // Initialize the Vault model
            let vault = cx.new(|cx| Vault::new(vault_path, cx));
            
            // Initialize the Workspace view with the vault
            cx.new(|cx| Workspace::new_with_callback(vault, cx))
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
