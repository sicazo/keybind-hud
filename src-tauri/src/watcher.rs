use crate::parsers;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::time::Duration;
use tauri::{AppHandle, Emitter};

/// Watch config files and emit `keybinds-updated` to the frontend when they change.
pub async fn start_watcher(app: AppHandle) {
    let home = std::env::var("HOME").unwrap_or_default();

    let paths = vec![
        format!("{}/.skhdrc", home),
        format!("{}/.config/nvim/lua/config/keymaps.lua", home),
        format!("{}/.config/nvim/lua/plugins", home),
        format!("{}/.config/zellij/config.kdl", home),
    ];

    let app_clone = app.clone();
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        tx,
        Config::default().with_poll_interval(Duration::from_secs(2)),
    )
    .expect("failed to create file watcher");

    for path in &paths {
        let _ = watcher.watch(std::path::Path::new(path), RecursiveMode::Recursive);
    }

    // Keep watcher alive in a thread
    std::thread::spawn(move || {
        let _watcher = watcher; // keep alive
        for res in rx {
            match res {
                Ok(event) if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) => {
                    // Re-parse all and push updated entries to frontend
                    let entries = parsers::parse_all();
                    let _ = app_clone.emit("keybinds-updated", entries);
                }
                _ => {}
            }
        }
    });
}
