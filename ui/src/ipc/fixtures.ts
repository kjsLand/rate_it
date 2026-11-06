/** Fake command results for the screen tests. Only tests import this file. */
import type { FakeResult } from "./testing";

export const playing = {
  trackId: "t1",
  name: "Human Nature",
  artists: [{ id: "a1", name: "Yot Club" }],
  albumId: "al1",
  albumName: "Rare Pleasure",
  imageUrl: null,
  rating: null,
} satisfies FakeResult;

export function ratedTrack(trackId: string, trackName: string, rating: number) {
  return {
    trackId,
    trackName,
    albumName: "Rare Pleasure",
    artistId: "a1",
    artistName: "Yot Club",
    rating,
    ratedAt: "2026-11-05T10:00:00.000Z",
  } satisfies FakeResult;
}

export const artists = [
  { id: "a1", name: "Yot Club", ratedCount: 2, average: 9 },
  { id: "a2", name: "Joji", ratedCount: 1, average: 8 },
] satisfies FakeResult;

export const account = {
  signedIn: false,
  clientId: "abc123",
  displayName: null,
} satisfies FakeResult;
