import Fuse from "fuse.js";
import { useMemo } from "react";
import type { Category, Entry } from "./types";

const FUSE_OPTIONS = {
  keys: [
    { name: "desc",     weight: 0.4 },
    { name: "key",      weight: 0.25 },
    { name: "tags",     weight: 0.25 },
    { name: "category", weight: 0.1 },
  ],
  threshold: 0.35,
  includeScore: true,
};

export function useSearch(
  entries: Entry[],
  query: string,
  activeCategory: Category | null
): Entry[] {
  const filtered = useMemo(
    () => activeCategory ? entries.filter((e) => e.category === activeCategory) : entries,
    [entries, activeCategory]
  );

  const fuse = useMemo(() => new Fuse(filtered, FUSE_OPTIONS), [filtered]);

  return useMemo(() => {
    if (!query.trim()) return filtered;
    return fuse.search(query.trim()).map((r) => r.item);
  }, [fuse, query, filtered]);
}
