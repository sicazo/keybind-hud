use crate::entries::{Category, Entry};
use anyhow::Result;
use regex::Regex;
use std::fs;

/// Parse keymaps.lua: lines like `keymap("n", "<C-p>", function() ... end, { desc = "Find files" })`
pub fn parse_keymaps(keymaps_path: &str, plugins_dir: &str) -> Result<Vec<Entry>> {
    let mut entries = vec![];

    // Parse keymaps.lua
    if let Ok(content) = fs::read_to_string(keymaps_path) {
        entries.extend(parse_keymap_calls(&content));
    }

    // Parse plugin files for `keys = { ... }` tables
    if let Ok(dir) = fs::read_dir(plugins_dir) {
        for entry in dir.flatten() {
            if entry.path().extension().and_then(|e| e.to_str()) == Some("lua") {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    entries.extend(parse_plugin_keys(&content));
                }
            }
        }
    }

    Ok(entries)
}

/// Matches: keymap("mode", "key", ..., { desc = "description" })
fn parse_keymap_calls(content: &str) -> Vec<Entry> {
    let re = Regex::new(
        r#"keymap\s*\(\s*["']([^"']+)["']\s*,\s*["']([^"']+)["'][^)]*desc\s*=\s*["']([^"']+)["']"#,
    )
    .unwrap();

    re.captures_iter(content)
        .map(|cap| {
            let key = format_nvim_key(&cap[2]);
            let desc = cap[3].trim().to_string();
            let tags = infer_nvim_tags(&key, &desc);
            Entry::new(key, desc, Category::Nvim, tags)
        })
        .collect()
}

/// Matches plugin keys tables: { "key", function, desc = "description" }
fn parse_plugin_keys(content: &str) -> Vec<Entry> {
    let re = Regex::new(
        r#"\{\s*["']([^"']+)["'][^}]*desc\s*=\s*["']([^"']+)["'][^}]*\}"#,
    )
    .unwrap();

    re.captures_iter(content)
        .map(|cap| {
            let key = format_nvim_key(&cap[1]);
            let desc = cap[2].trim().to_string();
            let tags = infer_nvim_tags(&key, &desc);
            Entry::new(key, desc, Category::Nvim, tags)
        })
        .collect()
}

fn format_nvim_key(raw: &str) -> String {
    raw.replace("<leader>", "spc+")
        .replace("<C-", "⌃")
        .replace("<S-", "⇧")
        .replace("<A-", "⌥")
        .replace("<CR>", "↵")
        .replace("<Tab>", "⇥")
        .replace("<BS>", "⌫")
        .replace("<Esc>", "Esc")
        .replace(">", "")
        .replace("<", "")
}

fn infer_nvim_tags<'a>(key: &str, desc: &str) -> Vec<&'a str> {
    let lower = format!("{} {}", key.to_lowercase(), desc.to_lowercase());
    let mut tags = vec![];
    if lower.contains("lsp") || lower.contains("definition") || lower.contains("reference") { tags.push("lsp"); }
    if lower.contains("telescope") || lower.contains("find") || lower.contains("grep") { tags.push("telescope"); }
    if lower.contains("git") || lower.contains("diff") { tags.push("git"); }
    if lower.contains("test") || lower.contains("neotest") { tags.push("test"); }
    if lower.contains("harpoon") { tags.push("harpoon"); }
    if lower.contains("flash") || lower.contains("jump") { tags.push("navigation"); }
    if lower.contains("claude") || lower.contains("ai") { tags.push("ai"); }
    if lower.contains("diagnostic") || lower.contains("trouble") { tags.push("diagnostics"); }
    if lower.contains("format") { tags.push("format"); }
    if lower.contains("todo") { tags.push("todo"); }
    if lower.contains("overseer") || lower.contains("task") { tags.push("task"); }
    tags
}
