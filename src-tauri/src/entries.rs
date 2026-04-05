use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Skhd,
    Nvim,
    Zellij,
    Cli,
}

impl Category {
    pub fn label(&self) -> &'static str {
        match self {
            Category::Skhd => "skhd",
            Category::Nvim => "nvim",
            Category::Zellij => "zellij",
            Category::Cli => "cli",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub key: String,
    pub desc: String,
    pub category: Category,
    pub tags: Vec<String>,
    /// Shell command to run on select (skhd entries only).
    pub command: Option<String>,
}

impl Entry {
    pub fn new(
        key: impl Into<String>,
        desc: impl Into<String>,
        category: Category,
        tags: Vec<&str>,
    ) -> Self {
        let k = key.into();
        let d = desc.into();
        let id = format!(
            "{}-{}",
            category.label(),
            k.replace(' ', "_").replace('+', "plus")
        );
        Self {
            id,
            key: k,
            desc: d,
            category,
            tags: tags.into_iter().map(String::from).collect(),
            command: None,
        }
    }

    pub fn with_command(mut self, cmd: impl Into<String>) -> Self {
        self.command = Some(cmd.into());
        self
    }
}

/// Fallback hardcoded entries used when config files can't be parsed.
/// These mirror what's in the cheatsheet and serve as a safe baseline.
pub fn hardcoded_entries() -> Vec<Entry> {
    let mut e = vec![];

    // ── skhd / yabai ────────────────────────────────────────────────────
    macro_rules! sk {
        ($key:expr, $desc:expr, $($tag:expr),*) => {
            e.push(Entry::new($key, $desc, Category::Skhd, vec![$($tag),*]));
        };
    }
    sk!("⌥+H", "focus west", "focus", "window", "navigation");
    sk!("⌥+J", "focus south", "focus", "window", "navigation");
    sk!("⌥+K", "focus north", "focus", "window", "navigation");
    sk!("⌥+L", "focus east", "focus", "window", "navigation");
    sk!("⌥+⇧H", "swap window west", "swap", "window");
    sk!("⌥+⇧J", "swap window south", "swap", "window");
    sk!("⌥+⇧K", "swap window north", "swap", "window");
    sk!("⌥+⇧L", "swap window east", "swap", "window");
    sk!("⌥+⌃H", "resize shrink left", "resize", "window");
    sk!("⌥+⌃L", "resize grow right", "resize", "window");
    sk!("⌥+⌃J", "resize grow down", "resize", "window");
    sk!("⌥+⌃K", "resize shrink up", "resize", "window");
    sk!("⌥+⇧B", "bsp layout", "layout");
    sk!("⌥+S", "stack layout", "layout");
    sk!("⌥+F", "fullscreen toggle", "layout", "fullscreen");
    sk!("⌥+T", "toggle float / tile", "layout", "float");
    sk!("⌥+E", "toggle split direction", "layout");
    sk!("⌥+1–9", "switch to space 1–9", "spaces");
    sk!("⌥+⇧1–9", "send window to space 1–9", "spaces");
    sk!("⌥+⌘H", "insert direction west", "insert");
    sk!("⌥+⌘J", "insert direction south", "insert");
    sk!("⌥+⌘K", "insert direction north", "insert");
    sk!("⌥+⌘L", "insert direction east", "insert");
    sk!("⌥+↵", "open terminal (ghostty)", "launch", "terminal");
    sk!("⌥+B", "open browser", "launch", "browser");
    sk!("⌃G", "open lazygit", "git", "lazygit", "launch");
    sk!("⌥+/", "open keybind HUD", "launch", "hud");
    sk!("⌥+W", "close window", "window");
    sk!("⌥+R", "reload skhd", "system");
    sk!("⌥+⇧R", "restart yabai + skhd + bar", "system");
    sk!("⌥+⇧W", "clock in / out", "timer", "work");
    sk!("⌥+⌃W", "set work start time", "timer", "work");

    // ── Zellij ──────────────────────────────────────────────────────────
    macro_rules! zj {
        ($key:expr, $desc:expr, $($tag:expr),*) => {
            e.push(Entry::new($key, $desc, Category::Zellij, vec![$($tag),*]));
        };
    }
    zj!("⌃G", "lazygit floating pane", "git", "lazygit");
    zj!("⌃P", "enter pane mode", "pane");
    zj!("⌃P H/J/K/L", "focus pane direction", "pane", "navigation");
    zj!("⌃P N", "new pane right", "pane");
    zj!("⌃P D", "new pane down", "pane");
    zj!("⌃P X", "close pane", "pane");
    zj!("⌃P F", "fullscreen pane", "pane", "fullscreen");
    zj!("⌃P W", "floating pane", "pane", "float");
    zj!("⌃T", "enter tab mode", "tab");
    zj!("⌃T N", "new tab", "tab");
    zj!("⌃T X", "close tab", "tab");
    zj!("⌃T R", "rename tab", "tab");
    zj!("⌃T L", "next tab", "tab", "navigation");
    zj!("⌃T H", "prev tab", "tab", "navigation");
    zj!("⌃O", "enter session mode", "session");
    zj!("⌃O D", "detach session", "session");
    zj!("⌃O W", "session manager", "session");
    zj!("⌃S", "enter scroll mode", "scroll");
    zj!("⌃S /", "search in scroll", "scroll", "search");

    // ── Nvim ────────────────────────────────────────────────────────────
    macro_rules! nv {
        ($key:expr, $desc:expr, $($tag:expr),*) => {
            e.push(Entry::new($key, $desc, Category::Nvim, vec![$($tag),*]));
        };
    }
    // navigation
    nv!("⌃P", "find files (telescope)", "telescope", "files", "navigation");
    nv!("⌃F", "live grep (telescope)", "telescope", "search", "grep");
    nv!("⌃O", "open buffers (telescope)", "telescope", "buffers");
    nv!("⌃N", "file explorer (oil)", "oil", "files", "navigation");
    nv!("⌃S", "save file", "save");
    nv!("⌃E", "harpoon menu", "harpoon", "navigation");
    nv!("spc+a", "harpoon add file", "harpoon");
    nv!("spc+1–4", "harpoon jump to mark 1–4", "harpoon", "navigation");
    nv!("spc+hp", "harpoon prev mark", "harpoon");
    nv!("spc+hn", "harpoon next mark", "harpoon");
    nv!("s", "flash jump (label any char)", "flash", "navigation", "jump");
    nv!("S", "flash treesitter node", "flash", "treesitter");
    nv!("⌃H/J/K/L", "focus window split", "window", "navigation");
    nv!("⇧J/K", "jump 5 lines", "navigation");
    nv!("spc+tc", "close tab", "tab");
    nv!("spc+tn", "next tab", "tab");
    nv!("spc+tp", "prev tab", "tab");
    nv!("spc+cl", "clear search highlight", "search");
    // lsp
    nv!("gd", "go to definition", "lsp", "definition");
    nv!("cr", "go to references", "lsp", "references");
    nv!("gI", "go to implementation", "lsp", "implementation");
    nv!("spc+gd", "go to declaration", "lsp", "declaration");
    nv!("spc+cd", "hover docs", "lsp", "hover", "docs");
    nv!("spc+ca", "code action", "lsp", "action");
    nv!("spc+rn", "rename symbol", "lsp", "rename");
    nv!("spc+ff", "format buffer", "format");
    nv!("spc+ds", "document symbols", "lsp", "symbols");
    nv!("spc+ws", "workspace symbols", "lsp", "symbols");
    nv!("spc+th", "toggle inlay hints", "lsp", "hints");
    nv!("spc+dn", "next diagnostic", "diagnostics");
    nv!("spc+dp", "prev diagnostic", "diagnostics");
    nv!("spc+xx", "workspace diagnostics (trouble)", "diagnostics", "trouble");
    nv!("spc+xX", "buffer diagnostics (trouble)", "diagnostics", "trouble");
    nv!("spc+xr", "LSP references panel (trouble)", "lsp", "trouble");
    nv!("spc+cs", "symbols panel (trouble)", "symbols", "trouble");
    nv!("spc+xQ", "quickfix list (trouble)", "quickfix");
    nv!("spc+xL", "location list (trouble)", "location");
    // git
    nv!("spc+gv", "open diffview", "git", "diff");
    nv!("spc+gV", "close diffview", "git", "diff");
    nv!("spc+gh", "file git history", "git", "history");
    nv!("spc+gH", "repo git history", "git", "history");
    nv!("⌃G", "open lazygit", "git", "lazygit");
    // tests
    nv!("spc+tt", "run nearest test", "test", "neotest");
    nv!("spc+tf", "run test file", "test", "neotest");
    nv!("spc+ts", "test summary panel", "test", "neotest");
    nv!("spc+to", "test output", "test", "neotest");
    nv!("spc+tS", "stop test", "test", "neotest");
    nv!("]n", "next failed test", "test", "navigation");
    nv!("[n", "prev failed test", "test", "navigation");
    // tasks & todos
    nv!("spc+or", "overseer run task", "task", "overseer");
    nv!("spc+ot", "overseer toggle panel", "task", "overseer");
    nv!("]t", "next todo comment", "todo");
    nv!("[t", "prev todo comment", "todo");
    nv!("spc+xt", "todo list (trouble)", "todo", "trouble");
    nv!("spc+ft", "find todos (telescope)", "todo", "telescope");
    // ai
    nv!("spc+ac", "toggle Claude", "ai", "claude");
    nv!("spc+af", "focus Claude", "ai", "claude");
    nv!("spc+ar", "resume Claude session", "ai", "claude");
    nv!("spc+ab", "add buffer to Claude", "ai", "claude");
    nv!("spc+as", "send selection to Claude", "ai", "claude");
    nv!("spc+aa", "accept Claude diff", "ai", "claude", "diff");
    nv!("spc+ad", "deny Claude diff", "ai", "claude", "diff");
    // text objects
    nv!("ia / aa", "inner/around func args", "text-object", "mini-ai");
    nv!("if / af", "inner/around function", "text-object", "mini-ai");
    nv!("ic / ac", "inner/around class", "text-object", "mini-ai");

    // ── CLI tools ────────────────────────────────────────────────────────
    macro_rules! cl {
        ($key:expr, $desc:expr, $($tag:expr),*) => {
            e.push(Entry::new($key, $desc, Category::Cli, vec![$($tag),*]));
        };
    }
    cl!("eza / ls", "modern ls with icons + git", "files");
    cl!("bat / cat", "cat with syntax highlighting", "files");
    cl!("zoxide / cd", "smart cd, learns paths", "navigation");
    cl!("yazi", "terminal file manager (vim keys)", "files", "navigation");
    cl!("dust / du", "disk usage analyzer", "system");
    cl!("fzf", "fuzzy find (⌃R shell history, ⌃T files)", "search", "fuzzy");
    cl!("atuin", "searchable shell history (↑)", "search", "history");
    cl!("lazygit / lg", "terminal git UI", "git");
    cl!("delta", "syntax-aware diff pager (auto via git)", "git", "diff");
    cl!("yadm", "dotfile manager", "dotfiles", "git");
    cl!("git absorb", "auto-fixup into prior commits", "git");
    cl!("jq", "JSON processor", "data", "json");
    cl!("xh / http", "fast HTTP client", "network", "api");
    cl!("mise", "manage runtimes (node, python…)", "dev");
    cl!("direnv", "auto-load .env per directory", "dev", "env");
    cl!("watchexec", "run cmd on file changes", "dev");
    cl!("just", "project command runner", "dev", "scripts");
    cl!("zellij / zj", "terminal multiplexer", "terminal");
    cl!("glow", "render markdown in terminal", "docs");
    cl!("tldr", "simplified man pages", "docs");
    cl!("bottom / btm", "system monitor with graphs", "system");
    cl!("procs / ps", "modern ps with color + tree", "system");
    cl!("just sync", "pull + brew bundle dotfiles", "dotfiles", "scripts");
    cl!("just backup", "commit + push dotfiles", "dotfiles", "scripts");
    cl!("just upgrade", "brew update + upgrade + cleanup", "scripts");
    cl!("just restart", "restart yabai/skhd/bar/borders", "system", "scripts");
    cl!(":email", "espanso → coding@sicazo.dev", "espanso", "snippet");
    cl!(":gh", "espanso → github.com/sicazo", "espanso", "snippet");
    cl!(":dbg / :pln / :clg", "espanso debug print (rust/js)", "espanso", "snippet");
    cl!(":todo / :fixme", "espanso code comment tags", "espanso", "snippet");

    e
}
