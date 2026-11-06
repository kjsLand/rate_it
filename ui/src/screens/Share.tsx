import "./Share.css";
import { useState } from "react";

import { cardPng } from "../components/cardImage";
import { useAction } from "../components/async";
import { ActionStatus } from "../components/Status";
import { albumCard, copyCardImage, coverDataUrl, songCard } from "../ipc";
import type { AlbumCard, SongCard } from "../ipc";

type Card = { kind: "song"; card: SongCard } | { kind: "album"; card: AlbumCard };

/** The text of a card: a title and the lines under it. */
type CardText = { title: string; lines: string[] };

function cardLines(card: Card): CardText {
  if (card.kind === "song") {
    const { name, artistName, albumName, rating } = card.card;

    return {
      title: name,
      lines: [
        `${artistName} — ${albumName}`,
        rating === null ? "Not rated" : `Rating: ${rating} / 10`,
      ],
    };
  }

  const { name, artistName, average, tracks } = card.card;

  return {
    title: `${name} — ${artistName}`,
    lines: [
      average === null ? "Overall rating: none" : `Overall rating: ${average.toFixed(1)}`,
      ...tracks.map((t) => `${t.name}: ${t.rating ?? "Not rated"}`),
    ],
  };
}

/** Share: a song card or an album card. Copy an image of the card on desktop. */
export function Share() {
  const [card, setCard] = useState<Card | null>(null);
  const [cover, setCover] = useState<string | null>(null);
  const { action, run } = useAction();

  async function show(load: () => Promise<Card>) {
    setCover(null);
    await run(load, (loaded) => {
      setCard(loaded);
      const url = loaded.card.imageUrl;

      if (url !== null) {
        coverDataUrl(url).then(setCover, () => setCover(null));
      }

      return "Card ready";
    });
  }

  async function copy() {
    if (card === null) return;
    const { title, lines } = cardLines(card);

    await run(
      async () => copyCardImage(await cardPng(title, lines, cover)),
      () => "Image copied. Paste it in a message.",
    );
  }

  const text = card === null ? null : cardLines(card);

  return (
    <section aria-labelledby="share-title">
      <header className="screen-header">
        <h1 id="share-title">Share</h1>
      </header>
      <div className="button-row">
        <button
          type="button"
          onClick={() => void show(async () => ({ kind: "song", card: await songCard() }))}
        >
          Song card
        </button>
        <button
          type="button"
          onClick={() => void show(async () => ({ kind: "album", card: await albumCard() }))}
        >
          Album card
        </button>
        <button type="button" disabled={card === null} onClick={() => void copy()}>
          Copy image
        </button>
        <button type="button" onClick={() => setCard(null)}>
          Clear
        </button>
      </div>
      <ActionStatus action={action} />
      {text !== null && (
        <figure className="card" aria-label="Rating card">
          {cover !== null && <img src={cover} alt="" className="cover" />}
          <figcaption>
            <strong>{text.title}</strong>
            {text.lines.map((line) => (
              <span key={line}>{line}</span>
            ))}
          </figcaption>
        </figure>
      )}
    </section>
  );
}
