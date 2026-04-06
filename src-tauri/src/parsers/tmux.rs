use crate::entries::{Category, Entry};
use anyhow::Result;
use regex::Regex;
use std::fs;

/// Parse `~/.config/tmux/tmux.conf` for bind / bind-key lines.
pub fn parse(conf_path: &str) -> Result<Vec<Entry>> {
    let content = fs::read_to_string(conf_path)?;
    let mut entries = vec![];

    // prefix stored in config (default C-d here, but we read it)
    let mut prefix = "⌃D".to_string();
    let prefix_re = Regex::new(r"(?m)^set\s+-g\s+prefix\s+(\S+)")?;
    if let Some(cap) = prefix_re.captures(&content) {
        prefix = format_key(cap[1].trim());
    }

    // bind -n (no-prefix) lines
    let no_prefix_re = Regex::new(r"(?m)^bind\s+-n\s+(\S+)\s+(.+?)(?:\s*#\s*(.+))?$")?;
    for cap in no_prefix_re.captures_iter(&content) {
        let raw_key = cap[1].trim();
        let cmd = cap[2].trim();
        let comment = cap.get(3).map(|m| m.as_str().trim().to_string());
        let key = format_key(raw_key);
        let desc = comment.unwrap_or_else(|| describe_tmux_cmd(cmd));
        let tags = infer_tags(&key, &desc, cmd);
        entries.push(Entry::new(key, desc, Category::Tmux, tags));
    }

    // bind (with prefix) lines — skip -T copy-mode variants
    let bind_re = Regex::new(r"(?m)^bind\s+(?!-T\s)(?!-n\s)(\S+)\s+(.+?)(?:\s*#\s*(.+))?$")?;
    for cap in bind_re.captures_iter(&content) {
        let raw_key = cap[1].trim();
        let cmd = cap[2].trim();
        let comment = cap.get(3).map(|m| m.as_str().trim().to_string());
        let key = format!("{}+{}", prefix, format_key(raw_key));
        let desc = comment.unwrap_or_else(|| describe_tmux_cmd(cmd));
        let tags = infer_tags(&key, &desc, cmd);
        entries.push(Entry::new(key, desc, Category::Tmux, tags));
    }

    Ok(entries)
}

fn format_key(raw: &str) -> String {
    raw.replace("M-", "⌥")
        .replace("C-", "⌃")
        .replace("S-", "⇧")
        .replace("Enter", "↵")
        .replace("Escape", "⎋")
        .replace("Space", "␣")
        .replace("BSpace", "⌫")
        .replace("Left", "←")
        .replace("Right", "→")
        .replace("Up", "↑")
        .replace("Down", "↓")
        .replace("NPage", "⇟")
        .replace("PPage", "⇞")
}

fn describe_tmux_cmd(cmd: &str) -> String {
    let cmd = cmd.trim();
    if cmd.contains("split-window") && cmd.contains("-h") { return "split pane vertical".into(); }
    if cmd.contains("split-window") { return "split pane horizontal".into(); }
    if cmd.contains("select-pane -L") { return "focus pane left".into(); }
    if cmd.contains("select-pane -R") { return "focus pane right".into(); }
    if cmd.contains("select-pane -U") { return "focus pane up".into(); }
    if cmd.contains("select-pane -D") { return "focus pane down".into(); }
    if cmd.contains("resize-pane -L") { return "resize pane left".into(); }
    if cmd.contains("resize-pane -R") { return "resize pane right".into(); }
    if cmd.contains("resize-pane -U") { return "resize pane up".into(); }
    if cmd.contains("resize-pane -D") { return "resize pane down".into(); }
    if cmd.contains("next-window") { return "next window".into(); }
    if cmd.contains("previous-window") { return "prev window".into(); }
    if cmd.contains("new-window") { return "new window".into(); }
    if cmd.contains("kill-window") { return "kill window".into(); }
    if cmd.contains("kill-pane") { return "kill pane".into(); }
    if cmd.contains("new-session") { return "new session".into(); }
    if cmd.contains("kill-session") { return "kill session".into(); }
    if cmd.contains("choose-session") { return "list sessions".into(); }
    if cmd.contains("detach") { return "detach session".into(); }
    if cmd.contains("copy-mode") { return "enter copy mode".into(); }
    if cmd.contains("copy-pipe") { return "copy selection".into(); }
    if cmd.contains("select-window") {
        if let Some(n) = cmd.split(":").last() { return format!("jump to window {}", n.trim()); }
    }
    if cmd.contains("display-popup") && cmd.contains("lazygit") { return "lazygit popup".into(); }
    if cmd.contains("source-file") { return "reload config".into(); }
    if cmd.contains("command-prompt") { return "command prompt".into(); }
    // fallback: truncate
    if cmd.len() > 40 { format!("{}…", &cmd[..40]) } else { cmd.to_string() }
}

fn infer_tags<'a>(key: &str, desc: &str, cmd: &str) -> Vec<&'a str> {
    let mut tags: Vec<&str> = vec![];
    let combined = format!("{} {} {}", key, desc, cmd).to_lowercase();
    if combined.contains("pane") { tags.push("pane"); }
    if combined.contains("window") { tags.push("window"); }
    if combined.contains("session") { tags.push("session"); }
    if combined.contains("split") { tags.push("split"); }
    if combined.contains("focus") || combined.contains("select-pane") { tags.push("navigation"); }
    if combined.contains("resize") { tags.push("resize"); }
    if combined.contains("copy") || combined.contains("copy-mode") { tags.push("copy"); }
    if combined.contains("lazygit") { tags.push("git"); tags.push("lazygit"); }
    if combined.contains("detach") { tags.push("detach"); }
    if combined.contains("reload") || combined.contains("source-file") { tags.push("config"); }
    tags
}
