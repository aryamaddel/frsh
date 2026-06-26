use gpui::{KeyBinding, actions};

// Workspace-level actions
actions!(
    frsh,
    [
        NewNote,
        CloseTab,
        ToggleSidebar,
        NextTab,
        PrevTab,
    ]
);

pub fn register_keybindings(cx: &mut gpui::App) {
    cx.bind_keys([
        // App-wide (workspace context)
        KeyBinding::new("ctrl-s", crate::editor::Save, Some("Workspace")),
        KeyBinding::new("ctrl-n", NewNote, Some("Workspace")),
        KeyBinding::new("ctrl-w", CloseTab, Some("Workspace")),
        KeyBinding::new("ctrl-b", ToggleSidebar, Some("Workspace")),
        KeyBinding::new("ctrl-tab", NextTab, Some("Workspace")),
        KeyBinding::new("ctrl-shift-tab", PrevTab, Some("Workspace")),

        // Editor: bound GLOBALLY (None context) — same pattern as
        // gpui/examples/input.rs. This is what makes typing actually work.
        KeyBinding::new("ctrl-e", crate::editor::TogglePreview, Some("NoteEditor")),
        KeyBinding::new("backspace", crate::editor::Backspace, Some("NoteEditor")),
        KeyBinding::new("delete", crate::editor::Delete, Some("NoteEditor")),
        KeyBinding::new("left", crate::editor::Left, Some("NoteEditor")),
        KeyBinding::new("right", crate::editor::Right, Some("NoteEditor")),
        KeyBinding::new("up", crate::editor::Up, Some("NoteEditor")),
        KeyBinding::new("down", crate::editor::Down, Some("NoteEditor")),
        KeyBinding::new("shift-left", crate::editor::SelectLeft, Some("NoteEditor")),
        KeyBinding::new("shift-right", crate::editor::SelectRight, Some("NoteEditor")),
        KeyBinding::new("shift-up", crate::editor::SelectUp, Some("NoteEditor")),
        KeyBinding::new("shift-down", crate::editor::SelectDown, Some("NoteEditor")),
        KeyBinding::new("ctrl-a", crate::editor::SelectAll, Some("NoteEditor")),
        KeyBinding::new("home", crate::editor::Home, Some("NoteEditor")),
        KeyBinding::new("end", crate::editor::End, Some("NoteEditor")),
        KeyBinding::new("enter", crate::editor::NewLine, Some("NoteEditor")),
        KeyBinding::new("ctrl-v", crate::editor::Paste, Some("NoteEditor")),
        KeyBinding::new("ctrl-c", crate::editor::Copy, Some("NoteEditor")),
        KeyBinding::new("ctrl-x", crate::editor::Cut, Some("NoteEditor")),

        // File tree navigation
        KeyBinding::new("j", crate::file_tree::SelectNext, Some("FileTree")),
        KeyBinding::new("k", crate::file_tree::SelectPrev, Some("FileTree")),
        KeyBinding::new("down", crate::file_tree::SelectNext, Some("FileTree")),
        KeyBinding::new("up", crate::file_tree::SelectPrev, Some("FileTree")),
        KeyBinding::new("enter", crate::file_tree::OpenSelected, Some("FileTree")),
        KeyBinding::new("delete", crate::file_tree::DeleteFile, Some("FileTree")),
    ]);
}
