import { forwardRef } from "react";

interface Props {
  value: string;
  onChange: (v: string) => void;
  resultCount: number;
  copied: boolean;
}

export const SearchBar = forwardRef<HTMLInputElement, Props>(
  ({ value, onChange, resultCount, copied }, ref) => (
    <div className="search-wrap">
      <span className="search-icon">⌕</span>
      <input
        ref={ref}
        className="search-input"
        type="text"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder="search keybinds…"
        autoComplete="off"
        autoCorrect="off"
        spellCheck={false}
      />
      <span className={`search-meta ${copied ? "copied" : ""}`}>
        {copied ? "copied ✓" : value ? `${resultCount} result${resultCount !== 1 ? "s" : ""}` : ""}
      </span>
    </div>
  )
);

SearchBar.displayName = "SearchBar";
