import { describe, expect, it } from "vitest";

import tokensCss from "../styles/tokens.css?raw";
import { brick, neutral, rose, umber } from "./palette";

/** WCAG 2.x relative luminance of a `#rrggbb` color. */
function luminance(hex: string): number {
  const channels = [1, 3, 5].map((start) => {
    const value = Number.parseInt(hex.slice(start, start + 2), 16) / 255;

    return value <= 0.040_45 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
  });

  const [red = 0, green = 0, blue = 0] = channels;

  return 0.2126 * red + 0.7152 * green + 0.0722 * blue;
}

function contrast(foreground: string, background: string): number {
  const a = luminance(foreground);
  const b = luminance(background);

  return (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05);
}

/** WCAG AA: 4.5 for body text; 3 for large text and UI parts (borders, fills, focus rings). */
const TEXT = 4.5;
const UI = 3;

// The "allowed" rows of the contrast table in docs/UI-SPEC.md, plus the pairs that the
// semantic tokens, the rating chips and the Discover sleeves use.
const PAIRS: [label: string, foreground: string, background: string, minimum: number][] = [
  ["text on paper", umber[950], neutral.paper, TEXT],
  ["text on surface", umber[950], neutral.white, TEXT],
  ["muted text on paper", umber[700], neutral.paper, TEXT],
  ["subtle text on paper", umber[500], neutral.paper, TEXT],
  ["subtle text on paper-2", umber[500], neutral.paper2, TEXT],
  ["input border on paper", umber[500], neutral.paper, UI],
  ["brick text on paper", brick[600], neutral.paper, TEXT],
  ["brick text on paper-2", brick[600], neutral.paper2, TEXT],
  ["brick fill on paper", brick[500], neutral.paper, UI],
  ["label on primary button", neutral.white, brick[500], TEXT],
  ["label on primary hover", neutral.white, brick[600], TEXT],
  ["text on rose fill", umber[900], rose[400], TEXT],
  ["text on rose tint", umber[950], rose[100], TEXT],
  ["brick text on rose tint", brick[700], rose[100], TEXT],
  ["danger text on paper", brick[700], neutral.paper, TEXT],
  ["dark: text on night", rose[50], neutral.night, TEXT],
  ["dark: text on night-2", rose[50], neutral.night2, TEXT],
  ["dark: muted text on night", umber[300], neutral.night, TEXT],
  ["dark: muted text on night-3", umber[300], neutral.night3, TEXT],
  ["dark: subtle text on night-2", rose[400], neutral.night2, TEXT],
  ["dark: brick text on night", brick[300], neutral.night, TEXT],
  ["dark: brick fill on night", brick[400], neutral.night, TEXT],
  ["dark: label on primary button", neutral.night, brick[400], TEXT],
  ["dark: brick fill on night-3", brick[400], neutral.night3, UI],
  ["dark: input border on night", umber[500], neutral.night, UI],
  ["dark: input border on night-2", umber[500], neutral.night2, UI],
  ["dark: focus ring on night", brick[300], neutral.night, UI],
  ["dark: text on tint", rose[50], neutral.nightTint, TEXT],
  ["dark: muted text on tint", umber[300], neutral.nightTint, TEXT],
  ["chip 1-4", umber[700], umber[100], TEXT],
  ["chip 5-7", umber[900], rose[200], TEXT],
  ["chip 8-9", brick[700], brick[100], TEXT],
  ["dark: chip 5-7", rose[50], rose[800], TEXT],
  ["dark: chip 8-9", brick[200], brick[900], TEXT],
  ["sleeve: full discography", neutral.white, umber[500], TEXT],
  ["dark sleeve: full discography", rose[50], umber[700], TEXT],
  ["dark sleeve: album", brick[50], brick[800], TEXT],
  ["sleeve: my top 10", rose[50], umber[950], TEXT],
];

describe("palette contrast", () => {
  it.each(PAIRS)("%s meets its minimum", (_label, foreground, background, minimum) => {
    expect(contrast(foreground, background)).toBeGreaterThanOrEqual(minimum);
  });

  it("finds the known low pairs too low, so the formula is correct", () => {
    expect(contrast(brick[500], umber[500])).toBeLessThan(1.2);
    expect(contrast(rose[400], neutral.paper)).toBeLessThan(UI);
    expect(contrast(neutral.white, "#000000")).toBeCloseTo(21, 5);
  });
});

describe("tokens.css", () => {
  const known = new Set<string>(
    [umber, rose, brick, neutral].flatMap((scale) => Object.values(scale).map((hex) => hex)),
  );

  it("uses only hex values from palette.ts", () => {
    const used = tokensCss.match(/#[\da-f]{3,8}\b/giu) ?? [];

    expect(used.length).toBeGreaterThan(0);
    expect(used.map((hex) => hex.toLowerCase()).filter((hex) => !known.has(hex))).toEqual([]);
  });
});
