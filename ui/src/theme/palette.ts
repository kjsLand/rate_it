/**
 * The hex values of the "Liner Notes" palette (see `docs/UI-SPEC.md`).
 * `styles/tokens.css` uses the same values; `contrast.test.ts` checks that the two agree.
 * Use these constants only where CSS variables are not available, for example on a canvas.
 */

/** Structure: text accents, borders, quiet surfaces. The input color is step 500. */
export const umber = {
  50: "#fff4ee",
  100: "#f6e8e1",
  200: "#e1d0c8",
  300: "#c7b3a9",
  400: "#a58c81",
  500: "#7e6255",
  600: "#674f44",
  700: "#523d33",
  800: "#3a2921",
  900: "#271913",
  950: "#170c07",
} as const;

/** Warmth: tinted surfaces in light mode, text in dark mode. The input color is step 400. */
export const rose = {
  50: "#fff0f4",
  100: "#ffe8ec",
  200: "#f4d5d9",
  300: "#debec2",
  400: "#c9a7ac",
  500: "#a08488",
  600: "#80676b",
  700: "#614c4f",
  800: "#443335",
  900: "#2b1d20",
  950: "#190e10",
} as const;

/** Action: buttons, the rating, focus, progress. The input color is step 500. */
export const brick = {
  50: "#ffe2d4",
  100: "#ffd2c4",
  200: "#ffb7a8",
  300: "#f99584",
  400: "#e06f5d",
  500: "#c34332",
  600: "#a53325",
  700: "#882418",
  800: "#6d150b",
  900: "#4e0400",
  950: "#350000",
} as const;

/** Low-chroma warm surfaces between umber and rose. */
export const neutral = {
  white: "#ffffff",
  paper: "#fef3f0",
  paper2: "#f9e9e6",
  night: "#150e0a",
  night2: "#211713",
  night3: "#2e211c",
  nightTint: "#322622",
} as const;
