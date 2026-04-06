import type { Category, Entry, MuxPref } from "../types";

interface Props {
  active: Category | null;
  onChange: (c: Category | null) => void;
  entries: Entry[];
  muxPref: MuxPref;
}

const LABELS: Record<Category, string> = {
  skhd:   "skhd",
  nvim:   "nvim",
  zellij: "zellij",
  tmux:   "tmux",
  cli:    "cli",
};

export function CategoryFilter({ active, onChange, entries, muxPref }: Props) {
  // Show the active mux category, hide the other one
  const visibleCats: Category[] = ["skhd", "nvim", muxPref as Category, "cli"];

  const counts = Object.fromEntries(
    visibleCats.map((c) => [c, entries.filter((e) => e.category === c).length])
  );

  return (
    <div className="category-bar">
      <button
        className={`cat-btn ${active === null ? "active" : ""}`}
        onClick={() => onChange(null)}
      >
        all <span className="cat-count">{entries.length}</span>
      </button>
      {visibleCats.map((cat) => (
        <button
          key={cat}
          data-cat={cat}
          className={`cat-btn ${active === cat ? "active" : ""}`}
          onClick={() => onChange(active === cat ? null : cat)}
        >
          {LABELS[cat]} <span className="cat-count">{counts[cat] ?? 0}</span>
        </button>
      ))}
    </div>
  );
}
