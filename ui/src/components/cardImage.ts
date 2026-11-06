/** Draws a rating card on a canvas and returns it as a PNG data URL. */

const WIDTH = 600;
const PADDING = 32;
const COVER = 240;
const LINE = 28;

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

/**
 * Draws the title, the lines and the cover (a `data:` URL) on a new canvas.
 * Throws when the WebView has no canvas support.
 */
export async function cardPng(
  title: string,
  lines: string[],
  cover: string | null,
): Promise<string> {
  const canvas = document.createElement("canvas");
  const coverHeight = cover === null ? 0 : COVER + PADDING;

  canvas.width = WIDTH;
  canvas.height = PADDING * 2 + LINE * (lines.length + 1) + coverHeight;
  const context = canvas.getContext("2d");

  if (context === null) {
    throw new Error("This system cannot draw images");
  }

  context.fillStyle = "#75BAFF";
  context.fillRect(0, 0, canvas.width, canvas.height);
  context.fillStyle = "#111111";
  context.font = "bold 24px sans-serif";
  context.fillText(title, PADDING, PADDING + LINE / 2, WIDTH - PADDING * 2);
  let y = PADDING + LINE / 2;

  if (cover !== null) {
    const image = await loadImage(cover);

    context.drawImage(image, PADDING, y + PADDING / 2, COVER, COVER);
    y += coverHeight;
  }

  context.font = "18px sans-serif";

  for (const line of lines) {
    y += LINE;
    context.fillText(line, PADDING, y, WIDTH - PADDING * 2);
  }

  return canvas.toDataURL("image/png");
}
