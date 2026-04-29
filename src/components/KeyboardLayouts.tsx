import { useMemo, useState } from "react";

type LayoutName = "qwerty" | "programmers-dvorak" | "colemak-dh";

type KeyCell = {
  code: string;
  label: string;
};

const LAYOUTS: Record<LayoutName, { title: string; rows: KeyCell[][] }> = {
  qwerty: {
    title: "QWERTY",
    rows: [
      ["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"].map((k) => ({ code: k, label: k.toUpperCase() })),
      ["a", "s", "d", "f", "g", "h", "j", "k", "l", ";"].map((k) => ({ code: k, label: k.toUpperCase() })),
      ["z", "x", "c", "v", "b", "n", "m", ",", ".", "/"].map((k) => ({ code: k, label: k.toUpperCase() })),
    ],
  },
  "programmers-dvorak": {
    title: "Programmer's Dvorak",
    rows: [
      ["'", ",", ".", "p", "y", "f", "g", "c", "r", "l"].map((k) => ({ code: k, label: k.toUpperCase() })),
      ["a", "o", "e", "u", "i", "d", "h", "t", "n", "s"].map((k) => ({ code: k, label: k.toUpperCase() })),
      [";", "q", "j", "k", "x", "b", "m", "w", "v", "z"].map((k) => ({ code: k, label: k.toUpperCase() })),
    ],
  },
  "colemak-dh": {
    title: "Colemak-DH",
    rows: [
      ["q", "w", "f", "p", "b", "j", "l", "u", "y", ";"].map((k) => ({ code: k, label: k.toUpperCase() })),
      ["a", "r", "s", "t", "g", "m", "n", "e", "i", "o"].map((k) => ({ code: k, label: k.toUpperCase() })),
      ["x", "c", "d", "v", "z", "k", "h", ",", ".", "/"].map((k) => ({ code: k, label: k.toUpperCase() })),
    ],
  },
};

const OPTIONS: LayoutName[] = ["qwerty", "programmers-dvorak", "colemak-dh"];

export function KeyboardLayouts() {
  const [selectedLayout, setSelectedLayout] = useState<LayoutName>("qwerty");
  const activeLayout = useMemo(() => LAYOUTS[selectedLayout], [selectedLayout]);

  return (
    <section className="layout-pane" aria-label="keyboard layouts">
      <div className="layout-tabs" role="tablist" aria-label="layout selector">
        {OPTIONS.map((option) => (
          <button
            key={option}
            type="button"
            role="tab"
            className={`layout-tab ${selectedLayout === option ? "active" : ""}`}
            aria-selected={selectedLayout === option}
            onClick={() => setSelectedLayout(option)}
          >
            {LAYOUTS[option].title}
          </button>
        ))}
      </div>

      <div className="board-note">Kinesis Advantage360 Pro split + column-stagger reference</div>
      <div className="kinesis-board" role="img" aria-label={`${activeLayout.title} on a kinesis style board`}>
        <Half rows={activeLayout.rows.map((r) => r.slice(0, 5))} side="left" />
        <Half rows={activeLayout.rows.map((r) => r.slice(5))} side="right" />
      </div>
      <p className="layout-help">Tip: this is a visual memory aid. It does not yet auto-read firmware layers from the keyboard.</p>
    </section>
  );
}

function Half({ rows, side }: { rows: KeyCell[][]; side: "left" | "right" }) {
  return (
    <div className={`kinesis-half ${side}`}>
      {rows.map((row, idx) => (
        <div key={`${side}-${idx}`} className={`kinesis-row row-${idx + 1}`}>
          {row.map((key) => (
            <span className="kinesis-key" key={`${side}-${key.code}-${idx}`}>
              {key.label}
            </span>
          ))}
        </div>
      ))}
    </div>
  );
}
