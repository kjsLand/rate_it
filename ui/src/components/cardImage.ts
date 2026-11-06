/** Draws a rating card as a concert ticket on a canvas and returns it as a PNG data URL. */

import { brick, neutral, rose, umber } from "../theme/palette";

/** The text of a ticket: the body (title, subtitle, track lines) and the score on the stub. */
export type TicketText = {
  title: string;
  subtitle: string;
  lines: string[];
  score: string;
  scoreText: string;
};

const WIDTH = 720;
const MARGIN = 24;
const PAD = 28;
const STUB = 168;
const COVER = 176;
const LINE = 26;
const BODY_RIGHT = WIDTH - MARGIN - STUB;
const DISPLAY = "'Fraunces Variable', Georgia, serif";
const SANS = "'Instrument Sans Variable', system-ui, sans-serif";
const MONO = "'IBM Plex Mono', ui-monospace, monospace";

function loadImage(source: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const image = new Image();

    image.addEventListener("load", () => resolve(image), { once: true });
    image.addEventListener("error", () => reject(new Error("Cannot read the cover image")), {
      once: true,
    });
    image.src = source;
  });
}

/** Loads the app fonts for the canvas. A font that does not load falls back to the next one. */
async function loadFonts(): Promise<void> {
  try {
    await Promise.all([
      document.fonts.load(`600 30px ${DISPLAY}`),
      document.fonts.load(`500 18px ${SANS}`),
      document.fonts.load(`400 16px ${MONO}`),
    ]);
  } catch {
    // The fallback fonts are good enough for the image.
  }
}

/** A rectangle path with round corners (`roundRect` needs Safari 16). */
function roundedRect(
  context: CanvasRenderingContext2D,
  [x, y, w, h]: [number, number, number, number],
  radius: number,
): void {
  context.beginPath();
  context.moveTo(x + radius, y);
  context.arcTo(x + w, y, x + w, y + h, radius);
  context.arcTo(x + w, y + h, x, y + h, radius);
  context.arcTo(x, y + h, x, y, radius);
  context.arcTo(x, y, x + w, y, radius);
  context.closePath();
}

/** The paper, the rose body, the brick stub, the perforation and its two notches. */
function drawTicket(context: CanvasRenderingContext2D, height: number): void {
  const ticketHeight = height - MARGIN * 2;

  context.fillStyle = neutral.paper;
  context.fillRect(0, 0, WIDTH, height);
  context.fillStyle = rose[100];
  roundedRect(context, [MARGIN, MARGIN, WIDTH - MARGIN * 2, ticketHeight], 16);
  context.fill();
  context.save();
  context.clip();
  context.fillStyle = brick[500];
  context.fillRect(BODY_RIGHT, MARGIN, STUB, ticketHeight);
  context.restore();

  context.fillStyle = neutral.paper;

  for (const y of [MARGIN, height - MARGIN]) {
    context.beginPath();
    context.arc(BODY_RIGHT, y, 12, 0, Math.PI * 2);
    context.fill();
  }

  context.strokeStyle = umber[300];
  context.lineWidth = 2;
  context.setLineDash([6, 6]);
  context.beginPath();
  context.moveTo(BODY_RIGHT, MARGIN + 18);
  context.lineTo(BODY_RIGHT, height - MARGIN - 18);
  context.stroke();
  context.setLineDash([]);
}

/** The score on the stub: a big numeral, "/10" and the app name. */
function drawStub(context: CanvasRenderingContext2D, score: string, height: number): void {
  const center = BODY_RIGHT + STUB / 2;

  context.textAlign = "center";
  context.fillStyle = neutral.white;
  context.font = `600 72px ${DISPLAY}`;
  context.fillText(score, center, height / 2 + 12, STUB - 24);
  context.font = `400 16px ${MONO}`;
  context.fillText("/10", center, height / 2 + 40);
  context.font = `600 12px ${SANS}`;
  context.fillText("R A T E _ I T", center, height - MARGIN - 20);
  context.textAlign = "start";
}

function bodyHeight(text: TicketText, hasCover: boolean): number {
  const top = Math.max(hasCover ? COVER : 0, 96);

  return MARGIN * 2 + PAD * 2 + top + (text.lines.length > 0 ? PAD : 0) + text.lines.length * LINE;
}

/**
 * Draws the ticket with the cover (a `data:` URL) on a new canvas.
 * Throws when the WebView has no canvas support.
 */
export async function cardPng(text: TicketText, cover: string | null): Promise<string> {
  const canvas = document.createElement("canvas");
  const height = bodyHeight(text, cover !== null);

  canvas.width = WIDTH;
  canvas.height = height;
  const context = canvas.getContext("2d");

  if (context === null) {
    throw new Error("This system cannot draw images");
  }

  await loadFonts();
  drawTicket(context, height);
  const left = MARGIN + PAD;
  let textLeft = left;

  if (cover !== null) {
    context.drawImage(await loadImage(cover), left, MARGIN + PAD, COVER, COVER);
    textLeft = left + COVER + 24;
  }

  const textWidth = BODY_RIGHT - PAD - textLeft;

  context.fillStyle = umber[950];
  context.font = `600 30px ${DISPLAY}`;
  context.fillText(text.title, textLeft, MARGIN + PAD + 30, textWidth);
  context.fillStyle = umber[700];
  context.font = `500 18px ${SANS}`;
  context.fillText(text.subtitle, textLeft, MARGIN + PAD + 62, textWidth);

  context.fillStyle = umber[900];
  context.font = `400 16px ${MONO}`;
  let y = MARGIN + PAD + Math.max(cover === null ? 0 : COVER, 96) + PAD;

  for (const line of text.lines) {
    context.fillText(line, left, y, BODY_RIGHT - PAD - left);
    y += LINE;
  }

  drawStub(context, text.score, height);

  return canvas.toDataURL("image/png");
}
