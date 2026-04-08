use crate::config::{self, MuxPref};
use crate::entries::Entry;
use crate::parsers;
use serde::Serialize;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;

/// Return all parsed entries to the frontend.
#[tauri::command]
pub fn get_entries() -> Vec<Entry> {
    parsers::parse_all()
}

/// Hide the HUD window.
#[tauri::command]
pub fn hide_window(app: AppHandle) {
    if let Some(win) = app.get_webview_window("hud") {
        let _ = win.hide().ok();
    }
}

/// Run a shell command (used for skhd entries).
#[tauri::command]
pub fn run_command(command: String) -> Result<(), String> {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .spawn()
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// Copy text to the system clipboard.
#[tauri::command]
pub fn copy_to_clipboard(app: AppHandle, text: String) -> Result<(), String> {
    app.clipboard()
        .write_text(text)
        .map_err(|e| e.to_string())
}

/// Return the current multiplexer preference ("tmux" or "zellij").
#[tauri::command]
pub fn get_mux_pref() -> String {
    match config::load().mux {
        MuxPref::Tmux => "tmux".into(),
        MuxPref::Zellij => "zellij".into(),
    }
}

/// Set multiplexer preference and push refreshed entries to the frontend.
#[tauri::command]
pub fn set_mux_pref(app: AppHandle, mux: String) -> Result<(), String> {
    let pref = match mux.as_str() {
        "tmux" => MuxPref::Tmux,
        "zellij" => MuxPref::Zellij,
        other => return Err(format!("unknown mux: {other}")),
    };
    let mut cfg = config::load();
    cfg.mux = pref;
    config::save(&cfg);
    // push updated entries immediately
    let entries = parsers::parse_all();
    let _ = app.emit("keybinds-updated", entries);
    // also notify tray label update
    let _ = app.emit("mux-changed", cfg.mux);
    Ok(())
}

/// Quit the application.
#[tauri::command]
pub fn quit_app(app: AppHandle) {
    app.exit(0);
}

// ── Work timer ────────────────────────────────────────────────────────────────

const WORK_SECONDS: i64 = 7 * 3600 + 45 * 60; // 7h 45m

fn work_file() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(format!("{}/.config/sketchybar/work_start", home))
}

fn trigger_sketchybar() {
    let _ = std::process::Command::new("sketchybar")
        .args(["--trigger", "work_timer_update"])
        .spawn();
}

#[derive(Serialize)]
pub struct WorkStatus {
    pub clocked_in: bool,
    pub start_ts: Option<i64>,
    pub elapsed_secs: i64,
    pub remaining_secs: i64,
}

#[tauri::command]
pub fn work_get_status() -> WorkStatus {
    let path = work_file();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    if let Ok(contents) = std::fs::read_to_string(&path) {
        if let Ok(start) = contents.trim().parse::<i64>() {
            // Clear stale timer from a previous day
            let start_date = chrono_date(start);
            let today = chrono_date(now);
            if start_date != today {
                let _ = std::fs::remove_file(&path);
                return WorkStatus { clocked_in: false, start_ts: None, elapsed_secs: 0, remaining_secs: WORK_SECONDS };
            }
            let elapsed = (now - start).max(0);
            return WorkStatus {
                clocked_in: true,
                start_ts: Some(start),
                elapsed_secs: elapsed,
                remaining_secs: WORK_SECONDS - elapsed,
            };
        }
    }

    WorkStatus { clocked_in: false, start_ts: None, elapsed_secs: 0, remaining_secs: WORK_SECONDS }
}

#[tauri::command]
pub fn work_clock_in() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let _ = std::fs::write(work_file(), now.to_string());
    trigger_sketchybar();
}

#[tauri::command]
pub fn work_clock_out() {
    let _ = std::fs::remove_file(work_file());
    trigger_sketchybar();
}

/// Set start time from "HH:MM" string (today's date, 24h).
#[tauri::command]
pub fn work_set_time(time: String) -> Result<(), String> {
    let parts: Vec<&str> = time.trim().splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err("invalid format".into());
    }
    let h: i64 = parts[0].parse().map_err(|_| "bad hours")?;
    let m: i64 = parts[1].parse().map_err(|_| "bad minutes")?;
    if !(0..24).contains(&h) || !(0..60).contains(&m) {
        return Err("out of range".into());
    }

    // Get start of today (local midnight) as unix timestamp
    use chrono::{Local, Datelike, TimeZone};
    let local_now = Local::now();
    let local_midnight = Local
        .with_ymd_and_hms(local_now.year(), local_now.month(), local_now.day(), 0, 0, 0)
        .single()
        .ok_or("could not determine local midnight")?;
    let stamp = local_midnight.timestamp() + h * 3600 + m * 60;

    let _ = std::fs::write(work_file(), stamp.to_string());
    trigger_sketchybar();
    Ok(())
}

/// Returns "YYYY-MM-DD" (local time) for a unix timestamp.
fn chrono_date(ts: i64) -> String {
    use chrono::{Local, TimeZone};
    Local
        .timestamp_opt(ts, 0)
        .single()
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}
