pub mod nvim;
pub mod skhd;
pub mod tmux;
pub mod zellij;

use crate::config::{self, MuxPref};
use crate::entries::Entry;
use std::collections::HashSet;

pub fn parse_all() -> Vec<Entry> {
    let home = std::env::var("HOME").unwrap_or_default();
    let cfg = config::load();
    let mut entries = vec![];

    // skhd
    let skhd_path = format!("{}/.skhdrc", home);
    match skhd::parse(&skhd_path) {
        Ok(mut e) => entries.append(&mut e),
        Err(_) => entries.append(&mut crate::entries::hardcoded_entries()
            .into_iter()
            .filter(|e| e.category == crate::entries::Category::Skhd)
            .collect()),
    }

    // nvim keymaps
    let nvim_keymaps = format!("{}/.config/nvim/lua/config/keymaps.lua", home);
    let nvim_plugins = format!("{}/.config/nvim/lua/plugins", home);
    match nvim::parse_keymaps(&nvim_keymaps, &nvim_plugins) {
        Ok(mut e) => entries.append(&mut e),
        Err(_) => entries.append(&mut crate::entries::hardcoded_entries()
            .into_iter()
            .filter(|e| e.category == crate::entries::Category::Nvim)
            .collect()),
    }

    // multiplexer — serve tmux or zellij entries based on saved preference
    match cfg.mux {
        MuxPref::Tmux => {
            let tmux_path = format!("{}/.config/tmux/tmux.conf", home);
            match tmux::parse(&tmux_path) {
                Ok(mut e) => entries.append(&mut e),
                Err(_) => entries.append(&mut crate::entries::hardcoded_entries()
                    .into_iter()
                    .filter(|e| e.category == crate::entries::Category::Tmux)
                    .collect()),
            }
        }
        MuxPref::Zellij => {
            let zellij_path = format!("{}/.config/zellij/config.kdl", home);
            match zellij::parse(&zellij_path) {
                Ok(mut e) => entries.append(&mut e),
                Err(_) => entries.append(&mut crate::entries::hardcoded_entries()
                    .into_iter()
                    .filter(|e| e.category == crate::entries::Category::Zellij)
                    .collect()),
            }
        }
    }

    // CLI tools always from hardcoded list
    entries.append(&mut crate::entries::hardcoded_entries()
        .into_iter()
        .filter(|e| e.category == crate::entries::Category::Cli)
        .collect());

    // Deduplicate by id, preserving first occurrence
    let mut seen = HashSet::new();
    entries.retain(|e| seen.insert(e.id.clone()));

    entries
}
