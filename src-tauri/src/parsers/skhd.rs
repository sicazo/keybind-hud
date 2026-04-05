use crate::entries::{Category, Entry};
use anyhow::Result;
use regex::Regex;
use std::fs;

/// Parses ~/.skhdrc into Entry list.
/// Handles lines like: `alt - h : yabai ...` or `alt + shift - h : ...`
pub fn parse(path: &str) -> Result<Vec<Entry>> {
    let content = fs::read_to_string(path)?;
    let mut entries = vec![];

    // Matches: [modifiers] - key : description_comment_or_command
    let re = Regex::new(
        r"(?m)^([a-z_\s+]+)\s*-\s*([a-z0-9\[\];',./\\`\-=]+)\s*:\s*(.+)$",
    )?;

    // Optional inline comment as human desc: `# open lazygit`
    let comment_re = Regex::new(r"#\s*(.+)$")?;

    for cap in re.captures_iter(&content) {
        let modifiers = cap[1].trim();
        let key = cap[2].trim();
        let rest = cap[3].trim();

        let display_key = format_skhd_key(modifiers, key);

        // Use trailing comment if present, else derive from command
        let desc = if let Some(c) = comment_re.captures(rest) {
            c[1].trim().to_string()
        } else {
            derive_desc_from_command(rest)
        };

        let tags = infer_skhd_tags(&desc, rest);

        // Strip inline comment from command so we get the raw shell command
        let command = comment_re.replace(rest, "").trim().to_string();

        entries.push(Entry::new(display_key, desc, Category::Skhd, tags).with_command(command));
    }

    Ok(entries)
}

fn format_skhd_key(modifiers: &str, key: &str) -> String {
    let mut parts: Vec<&str> = modifiers.split('+').map(str::trim).collect();
    parts.push(key);

    parts
        .iter()
        .map(|p| match *p {
            "alt" => "⌥",
            "shift" => "⇧",
            "cmd" => "⌘",
            "ctrl" => "⌃",
            "return" => "↵",
            "space" => "Space",
            "slash" => "/",
            "backslash" => "\\",
            "comma" => ",",
            other => other,
        })
        .collect::<Vec<_>>()
        .join("+")
}

fn derive_desc_from_command(cmd: &str) -> String {
    // best-effort: take first meaningful words
    let trimmed = cmd.trim_start_matches("yabai -m ").trim();
    if trimmed.len() > 48 {
        format!("{}…", &trimmed[..48])
    } else {
        trimmed.to_string()
    }
}

fn infer_skhd_tags<'a>(desc: &str, cmd: &str) -> Vec<&'a str> {
    let lower = format!("{} {}", desc.to_lowercase(), cmd.to_lowercase());
    let mut tags = vec![];
    if lower.contains("focus") || lower.contains("window") { tags.push("window"); }
    if lower.contains("focus") { tags.push("focus"); }
    if lower.contains("swap") || lower.contains("move") { tags.push("swap"); }
    if lower.contains("resize") { tags.push("resize"); }
    if lower.contains("layout") || lower.contains("bsp") || lower.contains("stack") { tags.push("layout"); }
    if lower.contains("space") { tags.push("spaces"); }
    if lower.contains("launch") || lower.contains("open") { tags.push("launch"); }
    if lower.contains("git") { tags.push("git"); }
    if lower.contains("term") || lower.contains("ghostty") { tags.push("terminal"); }
    tags
}
