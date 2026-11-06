/**
 * Dev-only fake backend. `pnpm --dir ui dev` in a normal browser has no Tauri, so every command
 * would fail. `main.tsx` loads this module only in dev mode outside Tauri, to preview the screens
 * with sample data. It is not part of the production bundle.
 */
import { Channel } from "@tauri-apps/api/core";
import type { InvokeArgs } from "@tauri-apps/api/core";
import { mockIPC } from "@tauri-apps/api/mocks";

import { brick, neutral, rose, umber } from "../theme/palette";
import type {
  AccountStatus,
  ArtistDetails,
  ArtistSummaryDto,
  Collaborator,
  HistoryEntry,
  NowPlaying,
  Progress,
  RatedTrackDto,
} from "../bindings";

/** A square cover drawn as SVG: a gradient, a ring and the title. */
function cover(from: string, to: string, title: string): string {
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
<defs><linearGradient id="g" x1="0" y1="0" x2="1" y2="1">
<stop offset="0" stop-color="${from}"/><stop offset="1" stop-color="${to}"/></linearGradient></defs>
<rect width="300" height="300" fill="url(#g)"/>
<circle cx="190" cy="120" r="70" fill="none" stroke="${neutral.paper}" stroke-opacity=".4" stroke-width="2"/>
<text x="24" y="272" font-family="Georgia, serif" font-style="italic" font-size="30"
 fill="${neutral.paper}">${title}</text></svg>`;

  return `data:image/svg+xml,${encodeURIComponent(svg)}`;
}

const COVER = cover(brick[500], umber[800], "Rare Pleasure");
const ARTIST_IMAGE = cover(rose[400], umber[700], "Yot Club");

const ARTISTS: ArtistSummaryDto[] = [
  { id: "a1", name: "Yot Club", ratedCount: 5, average: 8.4 },
  { id: "a2", name: "Men I Trust", ratedCount: 4, average: 9 },
  { id: "a3", name: "Joji", ratedCount: 3, average: 7 },
  { id: "a4", name: "Mild High Club", ratedCount: 2, average: 5.5 },
];

// Sorted by artist, then by rating (high first), like the Library command.
const SONGS: [trackId: string, name: string, artistId: string, rating: number][] = [
  ["t2", "YKWIM", "a1", 10],
  ["t1", "Human Nature", "a1", 9],
  ["t3", "Rare Pleasure", "a1", 8],
  ["t5", "Morning Flower", "a1", 8],
  ["t4", "Inside Out", "a1", 7],
  ["t6", "Show Me How", "a2", 10],
  ["t7", "Tailwhip", "a2", 9],
  ["t9", "Lauren", "a2", 9],
  ["t8", "Numb", "a2", 8],
  ["t10", "Glimpse of Us", "a3", 8],
  ["t11", "Slow Dancing in the Dark", "a3", 7],
  ["t12", "Sanctuary", "a3", 6],
  ["t13", "Homage", "a4", 6],
  ["t14", "Windowpane", "a4", 5],
];

function toDto(song: (typeof SONGS)[number], ratedAt: string): RatedTrackDto {
  const [trackId, trackName, artistId, rating] = song;

  return {
    trackId,
    trackName,
    albumName: "Rare Pleasure",
    artistId,
    artistName: ARTISTS.find((a) => a.id === artistId)?.name ?? "",
    rating,
    ratedAt,
  };
}

let current: number | null = null;
let cancelled = false;

const log: RatedTrackDto[] = SONGS.slice(1, 9).map((song, i) =>
  toDto(song, `2026-11-0${5 - Math.floor(i / 3)}T1${i}:00:00.000Z`),
);

function tracksOf(artistId: string): RatedTrackDto[] {
  return SONGS.flatMap((song) =>
    song[2] === artistId ? [toDto(song, "2026-10-01T09:00:00.000Z")] : [],
  );
}

function playing(): NowPlaying {
  return {
    trackId: "t1",
    name: "Human Nature",
    artists: [{ id: "a1", name: "Yot Club" }],
    albumId: "al1",
    albumName: "Rare Pleasure",
    imageUrl: COVER,
    rating: current,
  };
}

function artistDetails(): ArtistDetails {
  return {
    id: "a1",
    name: "Yot Club",
    imageUrl: ARTIST_IMAGE,
    average: 8.4,
    ratedCount: 5,
    totalCount: 42,
    top: tracksOf("a1").slice(0, 5),
  };
}

function history(): HistoryEntry[] {
  return [
    { rating: 9, ratedAt: "2026-11-05T10:00:00.000Z", source: "app" },
    { rating: 7, ratedAt: "2026-09-12T10:00:00.000Z", source: "app" },
    { rating: 6, ratedAt: "2026-03-01T10:00:00.000Z", source: "import" },
  ];
}

const COLLABORATORS: Collaborator[] = [
  { id: "c1", name: "Mac DeMarco", trackCount: 3 },
  { id: "c2", name: "Clairo", trackCount: 2 },
  { id: "c3", name: "Rex Orange County", trackCount: 1 },
  { id: "c4", name: "Hope Tala", trackCount: 1 },
];

let account: AccountStatus = { signedIn: true, clientId: "dev-client-id", displayName: "Kevin" };

/** A command argument as text ("" when it is missing). */
function textArg(payload: InvokeArgs | undefined, key: string): string {
  if (payload === undefined || Array.isArray(payload) || payload instanceof ArrayBuffer) return "";

  if (payload instanceof Uint8Array) return "";

  return String(payload[key] ?? "");
}

function rated(trackId: string, rating: number): RatedTrackDto {
  const song = SONGS.find((s) => s[0] === trackId) ?? SONGS[0];
  const artistId = song?.[2] ?? "a1";

  return {
    trackId,
    trackName: song?.[1] ?? "",
    albumName: "Rare Pleasure",
    artistId,
    artistName: ARTISTS.find((a) => a.id === artistId)?.name ?? "",
    rating,
    ratedAt: new Date().toISOString(),
  };
}

function channelArg(payload: InvokeArgs | undefined): Channel<Progress> | null {
  if (payload === undefined || Array.isArray(payload) || payload instanceof ArrayBuffer)
    return null;

  if (payload instanceof Uint8Array) return null;

  const channel = payload["onProgress"];

  return channel instanceof Channel ? channel : null;
}

const wait = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

/** Sends progress for 12 albums, then returns. Throws a "cancelled" error after cancel_task. */
async function longTask(payload: InvokeArgs | undefined, task: string): Promise<void> {
  const channel = channelArg(payload);

  cancelled = false;

  for (let done = 0; done <= 12; done += 1) {
    if (cancelled) throw { kind: "cancelled", message: "Cancelled" };

    channel?.onmessage({ task, done, total: 12 });
    // oxlint-disable-next-line no-await-in-loop -- the steps must come one after another, like real progress.
    await wait(150);
  }
}

async function handle(cmd: string, payload: InvokeArgs | undefined) {
  await wait(250);

  switch (cmd) {
    case "now_playing":
      return playing();
    case "rate_current":
      current = Number(textArg(payload, "rating"));
      log.unshift(rated("t1", current));

      return playing();
    case "recent_ratings":
      return log.slice(0, 10);
    case "library_artists":
      return ARTISTS;
    case "library_tracks":
      return tracksOf(textArg(payload, "artistId"));
    case "rerate_track":
      return rated(textArg(payload, "trackId"), Number(textArg(payload, "rating")));
    case "track_history":
      return history();
    case "artist_details":
      return artistDetails();
    case "song_card":
      return {
        name: "Human Nature",
        artistName: "Yot Club",
        albumName: "Rare Pleasure",
        imageUrl: COVER,
        rating: current ?? 9,
      };
    case "album_card":
      return {
        name: "Rare Pleasure",
        artistName: "Yot Club",
        imageUrl: COVER,
        average: 8.4,
        tracks: tracksOf("a1").map((t) => ({
          trackId: t.trackId,
          name: t.trackName,
          rating: t.rating,
        })),
      };
    case "cover_data_url":
      return COVER;
    case "find_collaborators":
      await longTask(payload, "collaborators");

      return COLLABORATORS;
    case "discover_playlist":
      await longTask(payload, "discover");

      return { playlistId: "p1", name: "Yot Club", trackCount: 42 };
    case "cancel_task":
      cancelled = true;

      return null;
    case "build_playlist":
      return { playlistId: "p2", name: "Side A", trackCount: 18 };
    case "queue_current_album":
    case "queue_my_top_ten":
      return { added: 10 };
    case "account_status":
    case "save_client_id":
      return account;
    case "sign_in":
    case "sign_out":
      account = { ...account, signedIn: cmd === "sign_in" };

      return account;
    case "import_legacy_file":
    case "import_ratings_file":
      return { imported: 5, skipped: 2, failed: 0 };
    case "export_ratings_file":
      return "rate_it-export.json";
    default:
      return null;
  }
}

mockIPC(handle);
