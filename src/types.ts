export type Category = "skhd" | "nvim" | "zellij" | "tmux" | "cli";
export type MuxPref = "tmux" | "zellij";

export interface Entry {
  id: string;
  key: string;
  desc: string;
  category: Category;
  tags: string[];
  command: string | null;
}

export const CATEGORY_COLORS: Record<Category, string> = {
  skhd:   "var(--accent)",
  nvim:   "var(--green)",
  zellij: "var(--peach)",
  tmux:   "var(--peach)",
  cli:    "var(--mauve)",
};

export const CATEGORY_ORDER: Category[] = ["skhd", "nvim", "tmux", "zellij", "cli"];
