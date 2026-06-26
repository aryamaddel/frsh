//! Custom title bar — directly adapted from Zed's `platform_title_bar` and
//! `title_bar` crates.
//!
//! Renders a single integrated row: [drag area with menu buttons + app title]
//! [minimize] [maximize] [close]. Uses `window_control_area()` so the Windows
//! OS handles the caption buttons natively (snap layouts, hover states, etc.).

#![allow(unused)]

use gpui::{
    App, Context, InteractiveElement, IntoElement, Menu, MenuItem, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, Render, TitlebarOptions, Window,
    WindowControlArea, WindowOptions, actions, div, point, prelude::*, px, rgb, size,
};

use crate::actions::{CloseTab, NewNote, ToggleSidebar};
use crate::editor::{Copy, Cut, Paste, Save, TogglePreview};

// Window-level actions (title bar buttons + menu items)
actions!(titlebar, [Quit, About, ZoomToggle]);

/// The title bar height on Windows (matches Zed's constant).
pub const TITLE_BAR_HEIGHT: f32 = 32.0;

/// Top-level menu names rendered as buttons in the title bar.
const MENU_NAMES: &[&str] = &["File", "Edit", "View", "Help"];

pub struct TitleBar {
    /// Whether the window is being dragged (mouse-down on the drag region).
    should_move: bool,
    /// Which menu dropdown is currently open (by name), if any.
    open_menu: Option<String>,
}

impl TitleBar {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            should_move: false,
            open_menu: None,
        }
    }

    fn toggle_menu(&mut self, name: &str, _: &mut Window, cx: &mut Context<Self>) {
        if self.open_menu.as_deref() == Some(name) {
            self.open_menu = None;
        } else {
            self.open_menu = Some(name.to_string());
        }
        cx.notify();
    }

    fn on_mouse_down(&mut self, _: &MouseDownEvent, _: &mut Window, _: &mut Context<Self>) {
        self.should_move = true;
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, _: &mut Context<Self>) {
        self.should_move = false;
    }

    fn on_mouse_move(&mut self, _: &MouseMoveEvent, window: &mut Window, _: &mut Context<Self>) {
        if self.should_move {
            self.should_move = false;
            window.start_window_move();
        }
    }
}

impl Render for TitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_maximized = window.is_maximized();
        let open_menu = self.open_menu.clone();

        div()
            .id("title-bar")
            // The entire bar is a drag region — drag the window by it.
            .window_control_area(WindowControlArea::Drag)
            .w_full()
            .h(px(TITLE_BAR_HEIGHT))
            .flex()
            .flex_row()
            .items_center()
            .bg(rgb(0x1e1e2e))
            .border_b_1()
            .border_color(rgb(0x11111b))
            // Drag handling (copied from Zed's PlatformTitleBar)
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            // ── LEFT: Menu buttons (File, Edit, View, Help) ──────────────
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .h_full()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .children(MENU_NAMES.iter().map(|name| {
                        let is_open = open_menu.as_deref() == Some(*name);
                        let name_for_click = name.to_string();

                        div()
                            .id(format!("menu-{}", name))
                            .h_full()
                            .flex()
                            .items_center()
                            .px_3()
                            .cursor_pointer()
                            .text_size(px(12.))
                            .text_color(if is_open {
                                rgb(0xffffff)
                            } else {
                                rgb(0xcdd6f4)
                            })
                            .when(is_open, |el| el.bg(rgb(0x313244)))
                            .hover(|s| s.bg(rgb(0x313244)).text_color(rgb(0xffffff)))
                            .child(name.to_string())
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.toggle_menu(&name_for_click, window, cx);
                            }))
                    })),
            )
            // ── CENTER: App title (flex-1 spacer) ────────────────────────
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(12.))
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(rgb(0x6c7086))
                    .child("frsh"),
            )
            // ── RIGHT: Windows caption buttons (min / max / close) ───────
            .child(render_windows_caption_buttons(is_maximized))
    }
}

/// Render the three Windows caption buttons using `window_control_area`.
/// On Windows, the OS handles the actual click behavior (minimize, maximize,
/// close, snap layouts) via hit-testing — no on_click handlers needed.
///
/// Directly adapted from Zed's
/// `platform_title_bar/src/platforms/platform_windows.rs`.
fn render_windows_caption_buttons(is_maximized: bool) -> impl IntoElement {
    div()
        .id("windows-caption-buttons")
        .font_family("Segoe Fluent Icons")
        .flex()
        .flex_row()
        .h_full()
        .child(caption_button(
            "minimize",
            "\u{e921}",
            WindowControlArea::Min,
            false,
        ))
        .child(caption_button(
            "maximize",
            if is_maximized { "\u{e923}" } else { "\u{e922}" },
            WindowControlArea::Max,
            false,
        ))
        .child(caption_button(
            "close",
            "\u{e8bb}",
            WindowControlArea::Close,
            true,
        ))
}

/// A single Windows caption button.
/// Copied from Zed's `WindowsCaptionButton`.
fn caption_button(
    id: &'static str,
    icon: &'static str,
    area: WindowControlArea,
    is_close: bool,
) -> impl IntoElement {
    let hover_bg = if is_close {
        rgb(0xe81120)
    } else {
        rgb(0x313244)
    };
    let hover_fg = rgb(0xffffff);

    div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .w(px(46.))
        .h_full()
        .text_size(px(10.))
        .text_color(rgb(0xcdd6f4))
        .hover(move |s| s.bg(hover_bg).text_color(hover_fg))
        // Mark this element's hitbox so the OS handles the window control.
        .window_control_area(area)
        .child(icon.to_string())
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
}

// ── Menu definitions (registered via cx.set_menus for the native menu bar) ─

/// Build the application menus. These are registered with `cx.set_menus()`
/// so they appear in the native menu bar.
pub fn build_menus() -> Vec<Menu> {
    vec![
        Menu::new("File").items([
            MenuItem::action("New Note", NewNote),
            MenuItem::action("Save", Save),
            MenuItem::action("Close Tab", CloseTab),
            MenuItem::separator(),
            MenuItem::action("Quit", Quit),
        ]),
        Menu::new("Edit").items([
            MenuItem::action("Cut", Cut),
            MenuItem::action("Copy", Copy),
            MenuItem::action("Paste", Paste),
        ]),
        Menu::new("View").items([
            MenuItem::action("Toggle Sidebar", ToggleSidebar),
            MenuItem::action("Toggle Preview", TogglePreview),
            MenuItem::separator(),
            MenuItem::action("Zoom", ZoomToggle),
        ]),
        Menu::new("Help").items([MenuItem::action("About frsh", About)]),
    ]
}

/// Register all menus + the Quit action handler with the platform.
pub fn register(cx: &mut App) {
    let menus = build_menus();
    cx.set_menus(menus);
    cx.on_action(|_: &Quit, cx| cx.quit());
}

/// Window options that make the custom title bar work.
/// Directly adapted from Zed's `build_window_options` in `zed.rs`.
pub fn window_options(bounds: gpui::Bounds<gpui::Pixels>) -> WindowOptions {
    WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: None,
            // Hide the native title bar so we draw our own.
            appears_transparent: true,
            traffic_light_position: Some(point(px(9.0), px(9.0))),
        }),
        window_bounds: Some(gpui::WindowBounds::Windowed(bounds)),
        focus: true,
        show: true,
        kind: gpui::WindowKind::Normal,
        is_movable: true,
        is_resizable: true,
        is_minimizable: true,
        // Client-side decorations → we render our own caption buttons.
        window_decorations: Some(gpui::WindowDecorations::Client),
        window_min_size: Some(size(px(400.0), px(300.0))),
        ..Default::default()
    }
}
