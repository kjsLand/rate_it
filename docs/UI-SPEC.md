# rate_it — UI Overhaul Spec ("Liner Notes")

Nov 5, 2026 · Kevin Land

> Companion to `SPEC.md`. This spec changes only the look of the React UI in `ui/`.
> It does not change Rust commands, IPC, data or screen behavior.

## Summary

Replace the colors of the Python app (grey, blue, light blue, yellow) with a warm palette
built from three colors:

| Name      | Hex       | OKLCH                   | Role                                                  |
| --------- | --------- | ----------------------- | ----------------------------------------------------- |
| **Umber** | `#7e6255` | `oklch(0.521 0.042 46)` | Structure: text accents, borders, quiet surfaces      |
| **Rose**  | `#c9a7ac` | `oklch(0.760 0.040 8)`  | Warmth: tinted surfaces, selection, text in dark mode |
| **Brick** | `#c34332` | `oklch(0.560 0.167 31)` | Action: primary buttons, the rating, focus, progress  |

The design concept is **Liner Notes**: the app looks like the sleeve, the booklet and the ticket
of a record. Each screen takes one object from that world as its theme (a stage, a record crate,
a gig poster, a ticket, a mixtape, a credits page, a record bin, the fine print). The shared
parts — palette, type, spacing, buttons — stay the same on all screens, so the app reads as one
product.

Three decisions shape this spec. Read them first:

1. **Brick is the only loud color.** Umber and brick have almost the same lightness (contrast
   1.11 : 1). They must never be the only difference between two states. Brick marks action and
   rating; umber marks structure.
2. **Rose is a surface in light mode and a text color in dark mode.** Rose on light paper is
   1.9 : 1 and cannot carry text. On the dark background it is 8.7 : 1.
3. **Album art supplies the extra color.** Like the current Spotify and Apple Music apps, the
   chrome stays tonal and the cover art gives the variety. Now Playing and Artist use the cover
   as a blurred backdrop.

## Goals and non-goals

**Goals**

- A warm, editorial look that is clearly different from the Spotify green-on-black look.
- A light theme ("Sleeve") and a dark theme ("Listening Room"). Default: follow the system.
- All text meets WCAG 2.2 AA (4.5 : 1 for body text, 3 : 1 for large text and UI parts).
- Each screen has its own theme, but uses only the shared tokens.
- No change to roles, accessible names or visible text that the Vitest tests use.
- Works at 375 px wide (phone) and up to 1600 px wide (desktop).

**Non-goals**

- No new features and no new Rust commands (except the theme setting, see Open questions).
- No CSS framework (Tailwind, CSS-in-JS). Plain CSS files with custom properties.
- No change to screen names. (The sidebar groups the screens; see App shell.)

## Research

| Source                                                                         | What it says                                                                                         | What this spec takes from it                                          |
| ------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| [Updivision, UI color trends 2026][updivision]                                 | "Soft light UI": warm whites, muted neutrals, small accent areas, soft shadows.                      | Warm paper background, not pure white. Brick used in small amounts.   |
| [Pantone 2026 / Lummi UI trends 2026][lummi]                                   | Cloud Dancer (warm off-white) is the color of the year; tonal palettes are common.                   | The paper color `#fef3f0` and one-hue tonal scales.                   |
| [Spotify design tokens (third-party)][spotify-tokens]                          | Album art is the main color source; chrome stays neutral.                                            | Cover-art backdrop on Now Playing and Artist.                         |
| [Apple Music design summary (third-party)][apple-music]                        | One signature accent on a quiet canvas.                                                              | Brick is the only signature accent.                                   |
| [Sleevenote (The Vinyl Factory)][sleevenote]                                   | Digital music app that shows sleeve art and track lists, "better than vinyl".                        | The whole Liner Notes concept: sleeve, booklet, credits.              |
| [shadcn music album notes block][album-notes]                                  | Album liner notes with a vintage paper look.                                                         | Paper grain texture, serif display type, dot-leader credits.          |
| [NoteBox case study][notebox]                                                  | Music rating app modeled on Letterboxd reviews.                                                      | Large rating numerals and a visible rating history.                   |
| [Flavio Copes, OKLCH and color-mix][oklch]; [OKLCH blueprint][oklch-blueprint] | Build scales by stepping OKLCH lightness and lowering chroma at the ends. Lower chroma in dark mode. | The scale method below, and `color-mix(in oklch, …)` for hover tints. |
| [Retro 90s music app (Dribbble)][dribbble-90s]                                 | Brown and warm tones read as retro and analog.                                                       | Umber as the structure color; cassette and ticket motifs.             |

The trend sources are blogs and third-party summaries. They give direction, not exact values.
All values in this spec come from the three input colors and are checked by the contrast table
below.

## Color system

### Tonal scales

Each scale steps OKLCH lightness and keeps the hue of the input color. Chroma goes down toward
the light and dark ends. The input color sits at its own lightness (marked **bold**).

| Step | Umber (h 46)  | Rose (h 8)    | Brick (h 31)  |
| ---- | ------------- | ------------- | ------------- |
| 50   | `#fff4ee`     | `#fff0f4`     | `#ffe2d4`     |
| 100  | `#f6e8e1`     | `#ffe8ec`     | `#ffd2c4`     |
| 200  | `#e1d0c8`     | `#f4d5d9`     | `#ffb7a8`     |
| 300  | `#c7b3a9`     | `#debec2`     | `#f99584`     |
| 400  | `#a58c81`     | **`#c9a7ac`** | `#e06f5d`     |
| 500  | **`#7e6255`** | `#a08488`     | **`#c34332`** |
| 600  | `#674f44`     | `#80676b`     | `#a53325`     |
| 700  | `#523d33`     | `#614c4f`     | `#882418`     |
| 800  | `#3a2921`     | `#443335`     | `#6d150b`     |
| 900  | `#271913`     | `#2b1d20`     | `#4e0400`     |
| 950  | `#170c07`     | `#190e10`     | `#350000`     |

Note: Rose is a light color, so it sits at step 400, not 500.

Neutral surfaces (low-chroma warm tones between umber and rose):

| Token        | Hex       | OKLCH                                    | Use                          |
| ------------ | --------- | ---------------------------------------- | ---------------------------- |
| `paper`      | `#fef3f0` | `oklch(0.972 0.012 35)`                  | Light page background        |
| `paper-2`    | `#f9e9e6` | `oklch(0.945 0.018 30)`                  | Light raised or sunken area  |
| `night`      | `#150e0a` | `oklch(0.170 0.014 45)`                  | Dark page background         |
| `night-2`    | `#211713` | `oklch(0.215 0.018 42)`                  | Dark card                    |
| `night-3`    | `#2e211c` | `oklch(0.260 0.022 40)`                  | Dark raised card, hover      |
| `night-tint` | `#322622` | rose-400 12 % mixed into night-2 (OKLab) | Dark rose area (log, ticket) |

### Semantic tokens

Components use only semantic tokens, never scale steps. The theme changes the token values.

| Token              | Light ("Sleeve")   | Dark ("Listening Room") | Use                                 |
| ------------------ | ------------------ | ----------------------- | ----------------------------------- |
| `--bg`             | paper              | night                   | Page                                |
| `--surface`        | `#ffffff`          | night-2                 | Cards, inputs                       |
| `--surface-raised` | paper-2            | night-3                 | Hover, selected row, sidebar        |
| `--surface-tint`   | rose-100           | night-tint              | Rose areas (log, ticket)            |
| `--text`           | umber-950          | rose-50                 | Body text                           |
| `--text-muted`     | umber-700          | umber-300               | Secondary text                      |
| `--text-subtle`    | umber-500          | rose-400                | Captions, meta (≥ 14 px)            |
| `--border`         | umber-200          | night-3                 | Dividers (decoration only)          |
| `--border-strong`  | umber-500          | umber-500               | Input borders (UI, ≥ 3 : 1)         |
| `--accent`         | brick-500          | brick-400               | Fills: primary button, rating, bars |
| `--accent-text`    | brick-600          | brick-300               | Brick text and links                |
| `--on-accent`      | `#ffffff`          | night                   | Text on an `--accent` fill          |
| `--accent-hover`   | brick-600          | brick-300               | Primary button hover                |
| `--focus`          | brick-500          | brick-300               | Focus ring                          |
| `--success`        | umber-700 + ✓ icon | umber-200 + ✓ icon      | "Saved", "Signed in"                |
| `--danger`         | brick-700          | brick-300               | Error text, with ⚠ icon             |

Success and error do not use green and red. The palette has no green, and brick already means
"action". Errors use brick-700 text plus an icon plus `role="alert"`, so color is never the only
signal.

### Contrast check

Measured with the WCAG 2.x relative-luminance formula. AA body text needs 4.5 : 1; large text
(≥ 24 px, or ≥ 19 px bold) and UI parts need 3 : 1.

| Pair (foreground / background) | Ratio | Use allowed                          |
| ------------------------------ | ----- | ------------------------------------ |
| umber-950 / paper              | 17.67 | All text                             |
| umber-700 / paper              | 9.30  | All text                             |
| umber-500 / paper              | 5.12  | Body text, input borders             |
| umber-500 / paper-2            | 4.73  | Body text                            |
| brick-600 / paper              | 6.22  | Brick text and links                 |
| brick-500 / paper              | 4.63  | Fills, large text, focus ring        |
| white / brick-500              | 5.04  | Primary button label                 |
| umber-900 / rose-400           | 7.78  | Text on a rose fill                  |
| brick-700 / rose-100           | 7.79  | Brick text on a rose tint            |
| rose-50 / night                | 17.31 | All text (dark)                      |
| umber-300 / night              | 9.51  | Muted text (dark)                    |
| rose-400 / night-2             | 8.03  | Subtle text (dark)                   |
| brick-300 / night              | 8.79  | Brick text (dark)                    |
| brick-400 / night              | 6.01  | Fills (dark); night text on it: 6.01 |
| brick-400 / night-3            | 4.89  | Brick fill on a raised card (dark)   |
| brick-500 / night              | 3.79  | **UI parts and large text only**     |
| umber-400 / paper              | 2.89  | **Decoration only** — fails 3 : 1    |
| rose-400 / paper               | ~1.9  | **Never for text or borders**        |
| brick-500 / umber-500          | 1.11  | **Never as the only difference**     |

Add this table as a test: `ui/src/theme/contrast.test.ts` reads the hex values from
`ui/src/theme/palette.ts` and checks each "allowed" pair with the same formula.

## Typography

All fonts are SIL Open Font License, self-hosted through Fontsource. The CSP is
`default-src 'self'`, so the app cannot load Google Fonts from the network.

| Role     | Font                                            | Package                                | Use                                                       |
| -------- | ----------------------------------------------- | -------------------------------------- | --------------------------------------------------------- |
| Display  | **Fraunces** (variable: `opsz`, `SOFT`, `WONK`) | `@fontsource-variable/fraunces`        | Screen titles, song and artist names, big rating numerals |
| UI, body | **Instrument Sans** (variable)                  | `@fontsource-variable/instrument-sans` | Everything else                                           |
| Numbers  | **IBM Plex Mono**                               | `@fontsource/ibm-plex-mono` (400, 500) | Rating log, counts, dates, progress                       |

Type scale (1.250, major third), in `rem` so the system text size applies:

| Token          | Size     | Line height | Font, weight                                                              |
| -------------- | -------- | ----------- | ------------------------------------------------------------------------- |
| `--fs-display` | 3.815rem | 1.0         | Fraunces 600, `opsz 144`, `SOFT 50`, `WONK 1`                             |
| `--fs-h1`      | 2.441rem | 1.1         | Fraunces 600, `opsz 72`                                                   |
| `--fs-h2`      | 1.563rem | 1.2         | Fraunces 500, `opsz 36`                                                   |
| `--fs-h3`      | 1.25rem  | 1.3         | Instrument Sans 600                                                       |
| `--fs-body`    | 1rem     | 1.5         | Instrument Sans 400                                                       |
| `--fs-small`   | 0.875rem | 1.45        | Instrument Sans 500                                                       |
| `--fs-micro`   | 0.75rem  | 1.3         | Instrument Sans 600, uppercase, `letter-spacing: 0.08em` (eyebrow labels) |

All numbers use `font-variant-numeric: tabular-nums`, so columns of ratings line up.

## Space, shape, depth, texture

- **Space:** 4 px base. Tokens `--space-1` (4) … `--space-8` (64): 4, 8, 12, 16, 24, 32, 48, 64.
- **Radius:** `--radius-s` 6 px (inputs, chips), `--radius-m` 12 px (cards), `--radius-l`
  20 px (hero panels), `--radius-pill` 999 px. Record and sleeve shapes use 2 px (square, like
  cardboard).
- **Shadow (light):** warm, never grey. `--shadow-1: 0 1px 2px rgb(60 35 25 / 8%), 0 2px 8px rgb(60 35 25 / 6%)`.
  Use precomputed values, not `color-mix()` or `oklch()`: the build target is Safari 15.
  `--shadow-2` doubles the offsets. Dark theme: no shadows; use `--surface-raised` and a 1 px
  `--border` instead.
- **Paper grain:** a 2 % noise layer on `--bg` in the light theme only. Use one small SVG
  `feTurbulence` file in `ui/src/assets/grain.svg` as a `background-image`. Not on cards, so text
  stays crisp.
- **Hairlines:** 1 px `--border` between list rows; 2 px brick rule under each screen title
  (40 px wide), like a section mark in a booklet.

## Motion

- Durations: `--dur-fast` 120 ms (hover, press), `--dur-base` 200 ms (panels), `--dur-slow`
  400 ms (record slide-out on Discover).
- Easing: `--ease-out: cubic-bezier(0.2, 0.8, 0.2, 1)`.
- Under `prefers-reduced-motion: reduce`, set all durations to 0 ms and stop the spin on
  Now Playing. No information may depend on animation.

## Shared components

| Component            | Look                                                                                                                                       |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| **Primary button**   | `--accent` fill, `--on-accent` text, `--radius-pill`, 40 px high (44 px on touch). Hover `--accent-hover`; press scale 0.98.               |
| **Secondary button** | `--surface` fill, 1 px `--border-strong`, `--text` label. Hover `--surface-raised`.                                                        |
| **Quiet button**     | No border, `--accent-text` label, underline on hover. For "History", "Clear", "Refresh".                                                   |
| **Input, select**    | `--surface`, 1 px `--border-strong`, `--radius-s`, 40 px high. `aria-invalid="true"` → 2 px `--danger` border + message.                   |
| **Focus ring**       | `outline: 2px solid var(--focus); outline-offset: 2px` on `:focus-visible` for all controls. Never removed.                                |
| **Rating chip**      | Pill with the number in IBM Plex Mono. Color by band (below). Always shows the number.                                                     |
| **Status line**      | `LoadStatus` / `ActionStatus`: small text with a leading icon (spinner, ✓, ⚠). Same text and roles as now.                                 |
| **Progress**         | 8 px track (`--surface-raised`), `--accent` fill, mono label "12 / 40 albums", quiet Cancel button. Keep native `<progress>` and style it. |
| **Eyebrow**          | `--fs-micro` uppercase label above titles, `--text-subtle`. Example: "NOW SPINNING".                                                       |
| **Empty state**      | Centered, a small line drawing (record, crate, ticket — one inline SVG per screen), one sentence, one action if useful.                    |

**Rating bands** (the number always shows; color is extra):

| Rating | Chip fill (light) | Chip text (light) | Chip fill (dark) | Chip text (dark) |
| ------ | ----------------- | ----------------- | ---------------- | ---------------- |
| 1–4    | umber-100         | umber-700         | night-3          | umber-300        |
| 5–7    | rose-200          | umber-900         | rose-800         | rose-50          |
| 8–9    | brick-100         | brick-700         | brick-900        | brick-200        |
| 10     | brick-500         | white + ★         | brick-400        | night + ★        |

## App shell

- **Sidebar ("the spine")**, desktop ≥ 900 px: 232 px wide, `--surface-raised`, full height.
  - Brand: "rate_it" in Fraunces italic 600, with a brick dot for the underscore.
  - Items: icon + label, 40 px high, `--radius-s`. Active item: `--surface` fill, brick 3 px bar
    on the left, label weight 600. Keep `aria-current="page"` as the only active signal in code.
  - Two groups: "Listen" (Now Playing, Library, Artist, Discover) without a heading, then
    "Make" (Share, Build Playlist, Collaborators, Settings) under a micro-caps heading.
  - Not built: a mini "now spinning" strip. The shell does not load `nowPlaying`, and a second
    poll is not worth it.
- **Tablet 641–899 px:** icon-only rail, 72 px wide. The labels stay in the DOM (visually
  hidden) and in `title`. The brand becomes a small record mark.
- **Phone ≤ 640 px:** bottom tab bar, 64 px + safe-area inset. Five tabs: Now Playing, Library,
  Artist, Discover, More. "More" opens a sheet with Share, Build Playlist, Collaborators,
  Settings; Escape or a choice closes it. Desktop, tablet and phone use the **same** `<nav>`
  and buttons, styled per breakpoint, so each screen button exists once (`App.test.tsx` finds
  them by name).
- **Icons:** inline SVG line drawings in `components/Icon.tsx` (no dependency), 20 px,
  1.75 stroke: disc, crate, mic, compass, ticket, cassette, users, sliders, plus more, check,
  alert, play, plus, minus.
- **Main area:** max content width 1120 px, padding `--space-6` (desktop) / `--space-4` (phone).
- **Screen accent hook:** `App` sets `data-screen="<id>"` on `<main>`. Screen CSS scopes its
  rules with `[data-screen="now"] …`. This is the only change to `App.tsx` logic.

## Screen themes

Each screen keeps its current components, roles and text. Only class names, wrappers and CSS
change. "Test impact" lists what the Vitest tests depend on.

### 1. Now Playing — "The Stage"

The song is on stage. The cover is the light source.

- **Backdrop:** a hero panel (`--radius-l`) with the cover image behind it at `blur(60px)
saturate(1.2)`, 140 % size, and a gradient overlay from `--bg` at 0 % opacity (top) to 100 %
  (bottom). In the light theme the overlay is paper at 55 %; in the dark theme night at 65 %.
  Without a cover: a flat `--surface-tint` panel.
- **Layout (desktop):** cover 280 px left, text right. Eyebrow "NOW SPINNING". Song name in
  `--fs-display` Fraunces. Artists in `--fs-h3`, album in `--text-muted`.
- **Cover:** 280 px square, 2 px radius, `--shadow-2`. Behind it, a black vinyl disc
  (`repeating-radial-gradient` grooves, brick label in the center) that sticks out 24 px to the
  right. The disc turns slowly (20 s per turn) while a song plays; it stops under reduced motion.
- **Rating — "the level meter":** the ten `Stars` buttons become ten vertical bars that rise in
  height (24 px to 60 px), like a VU meter. Lit bars: `--accent`. Unlit: `--border`. Hover preview
  lights bars in `--accent` at 55 % opacity. The selected value shows next to the meter as a big
  Fraunces numeral ("8" + small "/10"). Each bar keeps `aria-label="Rate n"`, `aria-pressed`, and
  shows its number under it in `--fs-micro`, so the meaning does not depend on bar height.
- **"Not rated"** shows as the text plus an empty dashed numeral box.
- **Rating log — "the setlist":** a `--surface-tint` card with a perforated top edge (CSS
  `radial-gradient` notches). Each row: mono index `01`, song, artist in `--text-muted`, rating
  chip at the right. The newest row fades in (`--dur-base`).
- **Nothing playing:** empty state with the still record drawing and "Nothing playing".
- **Test impact:** `aria-label="Rate n"`, "Nothing playing", "Last 10 ratings", the status text.
  No change.

### 2. Library — "The Crate"

Flip through a crate of records, artist by artist.

- **Layout:** two panes on desktop (artist list 320 px, tracks fill the rest). On phone, one
  pane at a time with a back button; the selected artist pushes the track pane.
- **Artist list — "crate dividers":** each artist button is a divider card: a large Fraunces
  initial letter in a rose-200 tab at the left, name, then "12 rated" in mono and a thin average
  bar (brick fill, width = average / 10). Selected: `aria-pressed="true"` → `--surface` card,
  brick 3 px left bar, `--shadow-1`. Visible text stays
  `Name (count, average x.x)` — keep it in the DOM for the tests and for screen readers; show the
  parts in separate spans.
- **Track list:** a table-like list. Columns: song, current rating chip, "New rating" stepper,
  Save (primary, small), History (quiet). Row hover `--surface-raised`.
- **New rating input:** keep `<input type="number">` with the label "New rating". Style it as a
  56 px mono field with − and + buttons beside it (the buttons change the text; they are extra).
  Invalid → brick-700 border and "1–10" hint.
- **History — "the run-out groove":** a vertical timeline. A 2 px umber-200 line, a dot per
  entry (brick dot for the current rating, umber dots for older), mono date, source as a small
  tag ("app" / "import").
- **Empty:** crate drawing and "No ratings yet".
- **Test impact:** button names with the full text, "New rating" label, "History"/"Hide history",
  "Rating history" list label. No change.

### 3. Artist — "The Gig Poster"

A bold letterpress poster for the artist that plays now.

- **Hero:** the artist image as a **duotone** (umber-950 shadows, rose-200 highlights). Make it
  with `filter: grayscale(1) contrast(1.1)` on the image and a brick-to-rose gradient layer with
  `mix-blend-mode: multiply` (light) / `screen` (dark). Image 100 % wide, 320 px high,
  `--radius-l`, `object-fit: cover`.
- **Name:** `--fs-display` Fraunces with `WONK 1`, overlapping the bottom of the image by 24 px,
  text on a `--bg` band so contrast does not depend on the image.
- **Stats row — "the bill":** two stat blocks side by side.
  - Average: huge Fraunces numeral ("7.8"), eyebrow "AVERAGE". "none" when no rating.
  - Rated: a 96 px ring (`conic-gradient` brick for rated, `--border` for the rest) with
    "12 / 40" in mono at the center, eyebrow "RATED".
  - The text "Average rating: x" and "Rated: a / b" stays in the DOM (visually hidden labels
    plus the visible numerals), so tests and screen readers get the same text.
- **Top 5 — "the setlist":** numbered list with big outlined Fraunces numerals 1–5
  (`-webkit-text-stroke` 1.5 px brick, transparent fill; fallback solid brick-600). Song, rating
  chip, and a round 40 px Play button (brick fill, play icon, `aria-label="Play <song>"` kept).
- **Test impact:** "Play <song>" labels, status text. No change.

### 4. Share — "The Ticket Stub"

A rating card looks like a concert ticket.

- **Controls:** a segmented control "Song card | Album card" (two buttons, keep names), then
  "Copy image" (primary) and "Clear" (quiet).
- **Card:** 640 px max, two parts split by a vertical perforation (dashed 2 px umber-300 line and
  two half-circle notches cut with `radial-gradient` masks).
  - Left part (stub body): `--surface-tint`, cover 200 px, title in Fraunces `--fs-h2`, lines in
    Instrument Sans.
  - Right part (the tear-off, 120 px): brick-500 fill, the rating as a big white Fraunces numeral
    rotated 0°, "/10" under it, "RATE_IT" in micro caps. Album card: the average, and the track
    lines move to the left part as a mono list with dot leaders.
- **Exported image:** update `ui/src/components/cardImage.ts` to draw the same ticket: paper
  background, rose-100 body, brick tear-off, umber-950 text. Read the hex values from
  `ui/src/theme/palette.ts` (a canvas cannot read CSS variables of a detached element reliably).
  Load the Fraunces and Instrument Sans font faces with `document.fonts.load()` before drawing;
  fall back to `serif` / `sans-serif`.
- **Test impact:** "Song card", "Album card", "Copy image", "Clear", `aria-label="Rating card"`,
  the status text. No change.

### 5. Build Playlist — "The Mixtape"

Make a tape for a friend.

- **Name field — "the J-card":** a wide card that looks like a cassette label: rose-100 fill,
  three faint ruled lines (`repeating-linear-gradient`), two small reel circles at the left.
  The `<input>` inside it uses Fraunces italic `--fs-h2`, no border, a 2 px brick underline on
  focus. Placeholder: "Side A — …". Label "Playlist name" stays (visible as an eyebrow).
- **Artist list — "the track picks":** each artist is a row card. The checkbox is styled as a
  20 px rounded square (brick fill + check when on). The minimum-rating `<select>` stays native
  (good on mobile) but styled as a mono pill "≥ 8". Disabled state at 45 % opacity.
- **Sticky summary bar:** at the bottom of the screen: "3 artists selected" in mono, and the
  "Make playlist" primary button. On phone it sits above the tab bar.
- **Success:** the status line shows the result text, with a small cassette icon.
- **Test impact:** checkbox labels (artist names), "Minimum rating for <name>", "Playlist name",
  "Make playlist", status text. No change. The "3 artists selected" text is new and decorative
  (`aria-live` off).

### 6. Collaborators — "The Credits"

The back of a booklet: who played on what.

- **Intro:** the current paragraph as a lead in `--fs-h3` italic Fraunces, `--text-muted`.
- **List — dot leaders:** each row: name (Instrument Sans 600) … dotted leader
  (`border-bottom: 2px dotted var(--border)` on a flex spacer) … "3 songs" in mono … quiet
  "My Top 10" button with a queue icon. Rows 48 px. Two columns on desktop ≥ 1100 px.
- **Progress — "the groove":** the shared progress bar, plus a small record icon that turns while
  the task runs (stops under reduced motion). Label "12 / 40 albums".
- **Empty (before the first search):** credits drawing and the "Find collaborators" primary
  button centered.
- **Test impact:** "Find collaborators", "Clear", "My Top 10", "Collaborators" list label,
  progress label "Progress", "Cancel". No change.

### 7. Discover — "The Record Bins"

Flip through four record sleeves.

- **Cards as sleeves:** a 2 × 2 grid (1 column on phone). Each `discover-card` is a square
  sleeve (2 px radius, `--shadow-1`) with a vinyl disc that slides 30 % out of the top edge on
  hover and on focus-within (`--dur-slow`). The button sits at the bottom of the sleeve.
- **One tone per sleeve** (all pairs checked):

  | Sleeve           | Fill (light) | Text      | Fill (dark) | Text     | Disc label |
  | ---------------- | ------------ | --------- | ----------- | -------- | ---------- |
  | Full Discography | umber-500    | white     | umber-700   | rose-50  | rose-400   |
  | Unrated Songs    | rose-400     | umber-950 | rose-800    | rose-50  | brick-500  |
  | Album            | brick-500    | white     | brick-800   | brick-50 | paper      |
  | My Top 10        | umber-950    | rose-50   | night-3     | rose-50  | brick-400  |

  The button on each sleeve is the inverse: `--surface` fill and `--text` label, so it is
  always readable.

- **Typography:** the card title in Fraunces `--fs-h2`; the description in `--fs-small`.
- **Progress:** under the grid, full width.
- **Test impact:** the four button names, headings, status text. No change.

### 8. Settings — "The Fine Print"

The back cover: calm and clear.

- **Layout:** one column, 640 px max. Sections as `--surface` cards with an eyebrow title:
  "SPOTIFY ACCOUNT", "RATINGS", "APPEARANCE".
- **Account card:** a status pill — "Signed in as …" (umber-100 fill, umber-700 text, ✓ icon) or
  "Not signed in" (paper-2 fill, umber-700 text). Client ID field in mono. The hint text in
  `--fs-small` with the redirect URIs in `<code>` chips (mono, paper-2 fill), each with a copy
  icon button (extra, optional). "Sign in with Spotify" as the primary button, "Sign out" as
  secondary.
- **Ratings card:** three secondary buttons in a row with icons (file-up, file-up, file-down).
  The import report shows as three mono stats: imported / skipped / failed.
- **Appearance card (new):** a segmented control "System | Light | Dark" (radio group,
  `aria-label="Theme"`). See Open questions for storage.
- **Test impact:** all current names and labels stay. The Appearance card adds new controls:
  add one test for it.

## Theme switching

- `<html data-theme="light|dark">`. With no attribute, the CSS uses
  `@media (prefers-color-scheme: dark)`.
- Set `color-scheme: light dark` on `:root`, so native controls (select, scrollbars, number
  spinners) follow the theme.
- A small `useTheme()` hook in `ui/src/theme/useTheme.ts` reads and writes the choice and sets the
  attribute. It is UI state only.
- Tauri window: set the window background color in `tauri.conf.json` to `#fef3f0` (light) so no
  white flash shows before the WebView paints. Dark-mode flash: set the background from Rust with
  the system theme at window creation (optional, small).

## File structure

```text
ui/src/
  theme/
    palette.ts          Hex values of all scale steps (for canvas and the contrast test)
    contrast.test.ts    Checks every "allowed" pair in this spec
    useTheme.ts         Theme choice + data-theme attribute
  styles/
    tokens.css          Scales, semantic tokens (light + dark), type, space, motion
    base.css            Reset, fonts, body, headings, focus ring, reduced motion
    shell.css           Sidebar, rail, bottom tab bar, main area
    components.css      Buttons, inputs, chips, status, progress, empty states
  screens/
    NowPlaying.css … Settings.css   One file per screen, scoped by [data-screen]
  assets/
    grain.svg, empty-*.svg          Texture and empty-state drawings
```

- `styles.css` is replaced by `styles/*.css`, imported in order in `main.tsx`.
- Each screen imports its own CSS file at the top of its `.tsx`.
- Keep each CSS file under 300 lines (same rule as TypeScript files).
- `palette.ts` is the single source of hex values; `tokens.css` must use the same values. The
  contrast test also checks that each hex in `tokens.css` exists in `palette.ts` (parse the file
  as text).
- No module in `ui/src/theme/` may import `@tauri-apps/api` (dependency-cruiser rule stays).

## New dependencies

| Package                                | License | Why                       |
| -------------------------------------- | ------- | ------------------------- |
| `@fontsource-variable/fraunces`        | OFL-1.1 | Display font, self-hosted |
| `@fontsource-variable/instrument-sans` | OFL-1.1 | UI font, self-hosted      |
| `@fontsource/ibm-plex-mono`            | OFL-1.1 | Numbers, self-hosted      |

Pin exact versions, like the current `package.json`. `knip` must see each one used.
Icons are inline SVG (see App shell), so `lucide-react` is not needed.

## Dev preview

`pnpm --dir ui dev` in a normal browser has no Tauri, so every command fails. `main.tsx` then
loads `ui/src/ipc/devBackend.ts`, which installs `mockIPC` with sample data (covers are inline
SVG). The check is `import.meta.env.DEV && !("__TAURI_INTERNALS__" in window)`, so `tauri dev`
and production builds never load it; Vite drops it from the production bundle.

## Implementation plan

Status: steps 1 and 2 done (Nov 5, 2026). The old screen rules live on, recolored, in
`screens/*.css` until step 3 replaces them.

Do the work in four steps. Run `pnpm check` after each step. Each step is one commit.

1. **Tokens and base.** Add `theme/palette.ts`, `contrast.test.ts`, `styles/tokens.css`,
   `base.css`, fonts. Replace the old colors. Screens look plain but use the new palette.
   - Exit: contrast test passes; all current Vitest tests pass without change.
2. **Shell and shared components.** Sidebar, rail, bottom tab bar, `data-screen`, buttons,
   inputs, chips, status, progress, icons, theme switch (`useTheme` + Settings card).
   - Exit: navigation works at 375, 768 and 1280 px; keyboard focus is visible on every control.
3. **Screen themes.** One screen per sub-commit, in this order: Now Playing, Library, Artist,
   Discover, Share (with `cardImage.ts`), Build Playlist, Collaborators, Settings.
   - Exit: each screen matches its section in this spec in both themes.
4. **Polish.** Empty-state drawings, grain, motion, reduced-motion check, window background.
   - Exit: acceptance criteria below are all true.

## Acceptance criteria

- [ ] No hex value from the old palette (`#c0c0c0`, `#75baff`, `#d0e8ff`, `#ffc90e`, `#ffff9f`)
      is left in `ui/src`.
- [ ] `contrast.test.ts` passes for every allowed pair.
- [ ] All existing Vitest tests pass with no change to their queries.
- [ ] Each screen renders in light and dark themes at 375 px, 768 px and 1280 px with no
      horizontal scroll and no overlap.
- [ ] Every control has a visible `:focus-visible` ring.
- [ ] With `prefers-reduced-motion: reduce`, nothing moves.
- [ ] The exported share image uses the ticket design.
- [ ] `pnpm check` passes.

## Open questions

Decided without a human in the loop, from the recommendations. Change them here if the owner
decides differently.

- [x] **Theme storage.** `localStorage` key `rate_it.theme` (UI only, no Rust change).
- [x] **Phone navigation.** Bottom bar with a "More" sheet.
- [x] **Icons.** Inline SVG in `components/Icon.tsx`, no new dependency.
- [x] **Now Playing disc spin.** Slow spin while a song plays; none under reduced motion.

## Sources

- [Updivision — UI color trends to watch in 2026][updivision]
- [Lummi — UI trends 2026][lummi]
- [Open Design — Spotify design system tokens (third-party)][spotify-tokens]
- [Web Design Hot — Apple Music design summary (third-party)][apple-music]
- [The Vinyl Factory — Sleevenote, "better than vinyl"][sleevenote]
- [shadcn.io — Music album notes block][album-notes]
- [NoteBox case study][notebox]
- [Flavio Copes — Modern CSS colors: oklch() and color-mix()][oklch]
- [DEV — Universal OKLCH color system blueprint][oklch-blueprint]
- [Dribbble — Music mobile app 90s][dribbble-90s]
- [W3C — WCAG 2.2 contrast minimum (1.4.3) and non-text contrast (1.4.11)][wcag]

[updivision]: https://updivision.com/blog/post/ui-color-trends-to-watch-in-2026
[lummi]: https://staticpages.lummi.ai/blog/ui-trends-2026
[spotify-tokens]: https://open-design.ai/systems/spotify/
[apple-music]: https://www.webdesignhot.com/design.md/apple-music/
[sleevenote]: https://thevinylfactory.com/?p=61639
[album-notes]: https://www.shadcn.io/blocks/music-album-notes
[notebox]: https://pedro-conde.github.io/Portfolio_Final/notebox.html
[oklch]: https://flaviocopes.com/css-oklch-color-mix/
[oklch-blueprint]: https://dev.to/okabrionz/universal-oklch-color-system-blueprint-one-base-color-design-tokens-3639
[dribbble-90s]: https://WWW.dribbble.com/shots/18026249-Music-Mobile-App-90s
[wcag]: https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html
