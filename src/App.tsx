import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useCallback, useEffect, useRef, useState } from "react";
import "./app.css";
import { CategoryFilter } from "./components/CategoryFilter";
import { ResultList } from "./components/ResultList";
import { SearchBar } from "./components/SearchBar";
import { WorkTimer } from "./components/WorkTimer";
import type { Category, Entry, MuxPref } from "./types";
import { useSearch } from "./useSearch";

const QUIT_ENTRY: Entry = {
  id: "__quit__",
  key: "quit",
  desc: "quit keybind hud",
  category: "skhd",
  tags: ["quit", "exit", "close"],
  command: null,
};

export default function App() {
  const [entries, setEntries] = useState<Entry[]>([]);
  const [query, setQuery] = useState("");
  const [activeCategory, setActiveCategory] = useState<Category | null>(null);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [copied, setCopied] = useState(false);
  const [muxPref, setMuxPref] = useState<MuxPref>("tmux");
  const searchRef = useRef<HTMLInputElement>(null);
  const resultsLenRef = useRef(0);

  // Load entries + mux preference on mount, subscribe to live updates
  useEffect(() => {
    invoke<Entry[]>("get_entries").then(setEntries).catch(console.error);
    invoke<string>("get_mux_pref").then((p) => setMuxPref(p as MuxPref)).catch(console.error);

    const unlistenEntries = listen<Entry[]>("keybinds-updated", (e) => setEntries(e.payload));
    const unlistenMux = listen<string>("mux-changed", (e) => setMuxPref(e.payload as MuxPref));
    return () => {
      unlistenEntries.then((fn) => fn());
      unlistenMux.then((fn) => fn());
    };
  }, []);

  const searchResults = useSearch(entries, query, activeCategory);
  const quitMatches = !activeCategory && (!query || "quit".includes(query.toLowerCase()) || "exit".includes(query.toLowerCase()));
  const results = quitMatches ? [...searchResults, QUIT_ENTRY] : searchResults;
  resultsLenRef.current = results.length;

  // Reset selection when query/filter changes
  useEffect(() => { setSelectedIndex(0); }, [query, activeCategory]);

  const dismiss = useCallback(() => {
    invoke("hide_window").catch(console.error);
    setQuery("");
    setSelectedIndex(0);
    setCopied(false);
  }, []);

  // Autofocus search on every window show, close on blur
  useEffect(() => {
    const win = getCurrentWindow();
    const unlistenFocus = win.onFocusChanged(({ payload: focused }) => {
      if (focused) {
        searchRef.current?.focus();
      } else {
        dismiss();
      }
    });
    searchRef.current?.focus();
    return () => { unlistenFocus.then((fn) => fn()); };
  }, [dismiss]);

  const handleSelect = useCallback(
    (entry: Entry) => {
      if (entry.id === "__quit__") {
        invoke("quit_app").catch(console.error);
        return;
      }
      if (entry.command) {
        invoke("run_command", { command: entry.command }).catch(console.error);
        dismiss();
      } else {
        invoke("copy_to_clipboard", { text: entry.key }).catch(console.error);
        setCopied(true);
        setTimeout(dismiss, 600);
      }
    },
    [dismiss]
  );

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      const inSearch = document.activeElement === searchRef.current;
      // Tab cycles through: all → skhd → nvim → active mux → cli
      const muxCat = muxPref as Category;
      const cats: (Category | null)[] = [null, "skhd", "nvim", muxCat, "cli"];

      switch (e.key) {
        case "ArrowDown":
          e.preventDefault();
          setSelectedIndex((i) => Math.min(i + 1, resultsLenRef.current - 1));
          break;
        case "ArrowUp":
          e.preventDefault();
          setSelectedIndex((i) => Math.max(i - 1, 0));
          break;
        case "j":
          if (!inSearch) { e.preventDefault(); setSelectedIndex((i) => Math.min(i + 1, resultsLenRef.current - 1)); }
          break;
        case "k":
          if (!inSearch) { e.preventDefault(); setSelectedIndex((i) => Math.max(i - 1, 0)); }
          break;
        case "Enter":
          if (results[selectedIndex]) handleSelect(results[selectedIndex]);
          break;
        case "Escape":
          if (query) { setQuery(""); } else { dismiss(); }
          break;
        case "Tab": {
          e.preventDefault();
          setActiveCategory((c) => {
            const idx = cats.indexOf(c);
            return cats[(idx + 1) % cats.length];
          });
          break;
        }
      }
    },
    [selectedIndex, query, results, handleSelect, dismiss, muxPref]
  );

  return (
    <div className="hud" onKeyDown={handleKeyDown} tabIndex={-1}>
      <div className="hud-inner">
        <SearchBar
          ref={searchRef}
          value={query}
          onChange={setQuery}
          resultCount={results.length}
          copied={copied}
        />
        <CategoryFilter
          active={activeCategory}
          onChange={setActiveCategory}
          entries={entries}
          muxPref={muxPref}
        />
        <ResultList
          results={results}
          selectedIndex={selectedIndex}
          onSelect={handleSelect}
          onHover={setSelectedIndex}
          query={query}
        />
        {results.length === 0 && query && (
          <div className="empty">no matches for "<span>{query}</span>"</div>
        )}
        <WorkTimer />
      </div>
    </div>
  );
}
