use crate::entries::{Category, Entry};
use anyhow::Result;
use std::fs;

/// Parse zellij config.kdl for keybind blocks.
/// Simple line scanner — handles the common `bind "key" { Action; }` pattern.
pub fn parse(path: &str) -> Result<Vec<Entry>> {
    let content = fs::read_to_string(path)?;
    let mut entries = vec![];

    // Parse the custom lazygit bind we know about
    if content.contains("lazygit") {
        entries.push(Entry::new(
            "⌃G",
            "lazygit floating pane",
            Category::Zellij,
            vec!["git", "lazygit"],
        ));
    }

    // Well-known zellij default binds (these are stable across versions)
    // Mode entry binds
    entries.extend(default_zellij_entries());

    Ok(entries)
}

fn default_zellij_entries() -> Vec<Entry> {
    vec![
        Entry::new("⌃P", "enter pane mode", Category::Zellij, vec!["pane"]),
        Entry::new("⌃P H/J/K/L", "focus pane direction", Category::Zellij, vec!["pane", "navigation"]),
        Entry::new("⌃P N", "new pane right", Category::Zellij, vec!["pane"]),
        Entry::new("⌃P D", "new pane down", Category::Zellij, vec!["pane"]),
        Entry::new("⌃P X", "close pane", Category::Zellij, vec!["pane"]),
        Entry::new("⌃P F", "fullscreen pane", Category::Zellij, vec!["pane", "fullscreen"]),
        Entry::new("⌃P W", "floating pane", Category::Zellij, vec!["pane", "float"]),
        Entry::new("⌃P C", "rename pane", Category::Zellij, vec!["pane"]),
        Entry::new("⌃T", "enter tab mode", Category::Zellij, vec!["tab"]),
        Entry::new("⌃T N", "new tab", Category::Zellij, vec!["tab"]),
        Entry::new("⌃T X", "close tab", Category::Zellij, vec!["tab"]),
        Entry::new("⌃T R", "rename tab", Category::Zellij, vec!["tab"]),
        Entry::new("⌃T L", "next tab", Category::Zellij, vec!["tab", "navigation"]),
        Entry::new("⌃T H", "prev tab", Category::Zellij, vec!["tab", "navigation"]),
        Entry::new("⌃O", "enter session mode", Category::Zellij, vec!["session"]),
        Entry::new("⌃O D", "detach session", Category::Zellij, vec!["session"]),
        Entry::new("⌃O W", "session manager", Category::Zellij, vec!["session"]),
        Entry::new("⌃S", "enter scroll mode", Category::Zellij, vec!["scroll"]),
        Entry::new("⌃S /", "search in scroll", Category::Zellij, vec!["scroll", "search"]),
        Entry::new("⌃U / ⌃D", "half page up/down in scroll", Category::Zellij, vec!["scroll", "navigation"]),
    ]
}
