mod commands;
mod entries;
mod parsers;
mod watcher;

use tauri::Manager;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Register ⌥+/ global shortcut to toggle the HUD
            let shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Slash);
            app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    let window = _app.get_webview_window("hud").expect("hud window not found");
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            })?;

            // macOS: hide from dock, no menu bar icon
            #[cfg(target_os = "macos")]
            {
                use tauri::ActivationPolicy;
                app_handle.set_activation_policy(ActivationPolicy::Accessory);
            }

            // Enable autostart on login
            let autostart = app.autolaunch();
            if !autostart.is_enabled().unwrap_or(false) {
                let _ = autostart.enable();
            }

            // Load initial entries and start file watcher
            let handle = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                watcher::start_watcher(handle).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_entries,
            commands::hide_window,
            commands::run_command,
            commands::copy_to_clipboard,
            commands::work_get_status,
            commands::work_clock_in,
            commands::work_clock_out,
            commands::work_set_time,
        ])
        .run(tauri::generate_context!())
        .expect("error while running keybind-hud");
}
