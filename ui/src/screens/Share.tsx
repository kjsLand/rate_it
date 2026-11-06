import { useState } from "react";

import { cardPng } from "../components/cardImage";
import type { TicketText } from "../components/cardImage";
import { useAction } from "../components/async";
import { Icon } from "../components/Icon";
import { ActionStatus } from "../components/Status";
import { albumCard, copyCardImage, coverDataUrl, songCard } from "../ipc";
import type { AlbumCard, SongCard } from "../ipc";
import "./Share.css";

type Card = { kind: "song"; card: SongCard } | { kind: "album"; card: AlbumCard };

/** One track line of the album card: the name and the rating text. */
type TrackLine = { id: string; name: string; rating: string };

function trackLines(card: Card): TrackLine[] {
  if (card.kind === "song") return [];

  return card.card.tracks.map((t) => ({
    id: t.trackId,
    name: t.name,
    rating: t.rating === null ? "Not rated" : String(t.rating),
  }));
}

function ticketText(card: Card): TicketText {
  if (card.kind === "song") {
    const { name, artistName, albumName, rating } = card.card;

    return {
      title: name,
      subtitle: `${artistName} — ${albumName}`,
      lines: [],
      score: rating === null ? "–" : String(rating),
      scoreText: rating === null ? "Not rated" : `Rating: ${rating} / 10`,
    };
  }

  const { name, artistName, average } = card.card;

  return {
    title: name,
    subtitle: artistName,
    lines: trackLines(card).map((t) => `${t.name}: ${t.rating}`),
    score: average === null ? "–" : average.toFixed(1),
    scoreText: average === null ? "Overall rating: none" : `Overall rating: ${average.toFixed(1)}`,
  };
}

/** Share ("The Ticket Stub"): a song card or an album card. Copy an image of the card on desktop. */
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

    await run(
      async () => copyCardImage(await cardPng(ticketText(card), cover)),
      () => "Image copied. Paste it in a message.",
    );
  }

  return (
    <section aria-labelledby="share-title">
      <header className="screen-header">
        <h1 id="share-title">Share</h1>
      </header>
      <div className="button-row">
        <div className="segmented" role="group" aria-label="Card">
          <button
            type="button"
            className="segment-button"
            aria-pressed={card?.kind === "song"}
            onClick={() => void show(async () => ({ kind: "song", card: await songCard() }))}
          >
            Song card
          </button>
          <button
            type="button"
            className="segment-button"
            aria-pressed={card?.kind === "album"}
            onClick={() => void show(async () => ({ kind: "album", card: await albumCard() }))}
          >
            Album card
          </button>
        </div>
        <button
          type="button"
          className="primary"
          disabled={card === null}
          onClick={() => void copy()}
        >
          Copy image
        </button>
        <button type="button" className="quiet" onClick={() => setCard(null)}>
          Clear
        </button>
      </div>
      <ActionStatus action={action} />
      {card === null ? (
        <div className="ticket-empty">
          <Icon name="ticket" />
          <p>Pick a card to make a ticket for the song or the album that plays now.</p>
        </div>
      ) : (
        <Ticket card={card} cover={cover} />
      )}
    </section>
  );
}

function Ticket({ card, cover }: { card: Card; cover: string | null }) {
  const text = ticketText(card);

  return (
    <figure className="ticket" aria-label="Rating card">
      <figcaption className="ticket-body">
        {cover !== null && <img src={cover} alt="" className="ticket-cover" />}
        <span className="ticket-text">
          <strong className="ticket-title">{text.title}</strong>
          <span className="ticket-subtitle">{text.subtitle}</span>
        </span>
        {card.kind === "album" && (
          <ul className="ticket-lines">
            {trackLines(card).map((line) => (
              <li key={line.id}>
                <span className="visually-hidden">
                  {line.name}: {line.rating}
                </span>
                <span aria-hidden="true">{line.name}</span>
                <span className="leader" aria-hidden="true" />
                <span aria-hidden="true">{line.rating}</span>
              </li>
            ))}
          </ul>
        )}
      </figcaption>
      <div className="ticket-stub">
        <span className="visually-hidden">{text.scoreText}</span>
        <span className="ticket-score" aria-hidden="true">
          {text.score}
        </span>
        <span className="ticket-max" aria-hidden="true">
          /10
        </span>
        <span className="ticket-brand" aria-hidden="true">
          rate_it
        </span>
      </div>
    </figure>
  );
}
