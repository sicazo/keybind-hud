import type { Category, Entry } from "../types";
import { CATEGORY_ORDER } from "../types";

interface Props {
  active: Category | null;
  onChange: (c: Category | null) => void;
  entries: Entry[];
}

const LABELS: Record<Category, string> = {
  skhd: "skhd",
  nvim: "nvim",
  zellij: "zellij",
  cli: "cli",
};

export function CategoryFilter({ active, onChange, entries }: Props) {
  const counts = Object.fromEntries(
    CATEGORY_ORDER.map((c) => [c, entries.filter((e) => e.category === c).length])
  );

  return (
    <div className="category-bar">
      <button
        className={`cat-btn ${active === null ? "active" : ""}`}
        onClick={() => onChange(null)}
      >
        all <span className="cat-count">{entries.length}</span>
      </button>
      {CATEGORY_ORDER.map((cat) => (
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
