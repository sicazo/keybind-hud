mod commands;
mod config;
mod entries;
mod parsers;
mod watcher;

use config::MuxPref;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

/// Rebuild the tray menu reflecting the current mux preference.
fn build_tray_menu(app: &AppHandle, mux: &MuxPref) -> tauri::Result<Menu<tauri::Wry>> {
    let use_tmux = matches!(mux, MuxPref::Tmux);

    let tmux_item = CheckMenuItem::with_id(app, "mux_tmux", "Use tmux", true, use_tmux, None::<&str>)?;
    let zellij_item = CheckMenuItem::with_id(app, "mux_zellij", "Use zellij", true, !use_tmux, None::<&str>)?;
    let sep = PredefinedMenuItem::separator(app)?;
    let show_item = MenuItem::with_id(app, "show_hud", "Show HUD  ⌥/", true, None::<&str>)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit keybind-hud", true, None::<&str>)?;

    Menu::with_items(app, &[&tmux_item, &zellij_item, &sep, &show_item, &sep2, &quit_item])
}

fn toggle_hud(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("hud") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            // ── Global hotkey ⌥+/ ─────────────────────────────────────────
            let shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Slash);
            app.global_shortcut().on_shortcut(shortcut, move |app, _shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    toggle_hud(app);
                }
            })?;

            // ── macOS: hide from dock ──────────────────────────────────────
            #[cfg(target_os = "macos")]
            {
                use tauri::ActivationPolicy;
                let _ = app_handle.set_activation_policy(ActivationPolicy::Accessory);
            }

            // ── Autostart on login ─────────────────────────────────────────
            let autostart = app.autolaunch();
            if !autostart.is_enabled().unwrap_or(false) {
                let _ = autostart.enable();
            }

            // ── System tray ────────────────────────────────────────────────
            let cfg = config::load();
            let menu = build_tray_menu(app.handle(), &cfg.mux)?;

            let tray_handle = app_handle.clone();
            TrayIconBuilder::with_id("main")
                .tooltip("keybind-hud")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(move |app, event| {
                    match event.id.as_ref() {
                        "mux_tmux" => {
                            let mut cfg = config::load();
                            cfg.mux = MuxPref::Tmux;
                            config::save(&cfg);
                            let entries = parsers::parse_all();
                            let _ = app.emit("keybinds-updated", entries);
                            let _ = app.emit("mux-changed", "tmux");
                            // Rebuild menu to reflect new check state
                            if let Ok(menu) = build_tray_menu(app, &MuxPref::Tmux) {
                                if let Some(tray) = app.tray_by_id("main") {
                                    let _ = tray.set_menu(Some(menu));
                                }
                            }
                        }
                        "mux_zellij" => {
                            let mut cfg = config::load();
                            cfg.mux = MuxPref::Zellij;
                            config::save(&cfg);
                            let entries = parsers::parse_all();
                            let _ = app.emit("keybinds-updated", entries);
                            let _ = app.emit("mux-changed", "zellij");
                            if let Ok(menu) = build_tray_menu(app, &MuxPref::Zellij) {
                                if let Some(tray) = app.tray_by_id("main") {
                                    let _ = tray.set_menu(Some(menu));
                                }
                            }
                        }
                        "show_hud" => toggle_hud(app),
                        "quit" => app.exit(0),
                        _ => {}
                    }
                })
                .on_tray_icon_event(move |tray, event| {
                    // Right-click also shows menu (left-click already handled above)
                    if let TrayIconEvent::Click { button: MouseButton::Right, button_state: MouseButtonState::Up, .. } = event {
                        let _ = tray.app_handle();
                    }
                })
                .build(app)?;

            // ── File watcher ───────────────────────────────────────────────
            let handle = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                watcher::start_watcher(handle).await;
            });

            // Listen for mux-changed events to rebuild tray menu
            let tray_handle2 = tray_handle.clone();
            let _ = tray_handle2;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_entries,
            commands::hide_window,
            commands::run_command,
            commands::copy_to_clipboard,
            commands::get_mux_pref,
            commands::set_mux_pref,
            commands::quit_app,
            commands::work_get_status,
            commands::work_clock_in,
            commands::work_clock_out,
            commands::work_set_time,
        ])
        .run(tauri::generate_context!())
        .expect("error while running keybind-hud");
}
