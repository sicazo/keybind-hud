import { useEffect, useRef } from "react";
import type { Entry } from "../types";
import { CATEGORY_COLORS } from "../types";

interface Props {
  results: Entry[];
  selectedIndex: number;
  onSelect: (e: Entry) => void;
  onHover: (i: number) => void;
  query: string;
}

function highlight(text: string, query: string): React.ReactNode {
  if (!query.trim()) return text;
  const idx = text.toLowerCase().indexOf(query.toLowerCase().trim());
  if (idx === -1) return text;
  return (
    <>
      {text.slice(0, idx)}
      <mark>{text.slice(idx, idx + query.trim().length)}</mark>
      {text.slice(idx + query.trim().length)}
    </>
  );
}

export function ResultList({ results, selectedIndex, onSelect, onHover, query }: Props) {
  const listRef = useRef<HTMLDivElement>(null);
  const selectedRef = useRef<HTMLDivElement>(null);

  // Scroll selected item into view
  useEffect(() => {
    selectedRef.current?.scrollIntoView({ block: "nearest" });
  }, [selectedIndex]);

  return (
    <div className="result-list" ref={listRef}>
      {results.map((entry, i) => (
        <div
          key={entry.id}
          ref={i === selectedIndex ? selectedRef : undefined}
          className={`result-item ${i === selectedIndex ? "selected" : ""}`}
          onClick={() => onSelect(entry)}
          onMouseEnter={() => onHover(i)}
        >
          <span
            className="result-cat-dot"
            style={{ background: CATEGORY_COLORS[entry.category] }}
          />
          <span className="result-key">
            {highlight(entry.key, query)}
          </span>
          <span className="result-desc">
            {highlight(entry.desc, query)}
          </span>
          <span
            className="result-badge"
            style={{
              background: `color-mix(in srgb, ${CATEGORY_COLORS[entry.category]} 12%, transparent)`,
              color: CATEGORY_COLORS[entry.category],
            }}
          >
            {entry.category}
          </span>
          <span className="result-action-hint">{entry.command ? "↵ run" : "↵ copy"}</span>
        </div>
      ))}
      {results.length > 0 && (
        <div className="hud-footer">
          <span><kbd>↑↓</kbd> navigate</span>
          <span><kbd>↵</kbd> copy keybind</span>
          <span><kbd>Tab</kbd> filter category</span>
          <span><kbd>Esc</kbd> dismiss</span>
        </div>
      )}
    </div>
  );
}
