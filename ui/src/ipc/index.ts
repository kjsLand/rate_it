/**
 * The only module that calls Rust. Screens import typed functions from here.
 * Each function returns the data, or throws an `IpcError` with the error class.
 */
import { Channel } from "@tauri-apps/api/core";

import { commands } from "../bindings";
import type {
  CommandError,
  DiscoverMode,
  ErrorKind,
  PlaylistRequest,
  PlaylistResult,
  Progress,
} from "../bindings";

export type {
  AccountStatus,
  AlbumCard,
  ArtistSummaryDto,
  Collaborator,
  DiscoverMode,
  ErrorKind,
  ImportReportDto,
  NowPlaying,
  PlaylistRequest,
  PlaylistResult,
  Progress,
  RatedTrackDto,
  SongCard,
} from "../bindings";

/** An error that a command returned. */
export class IpcError extends Error {
  readonly kind: ErrorKind;

  constructor(error: CommandError) {
    super(error.message);
    this.name = "IpcError";
    this.kind = error.kind;
  }
}

type CommandResult<T> = { status: "ok"; data: T } | { status: "error"; error: CommandError };

async function unwrap<T>(pending: Promise<CommandResult<T>>): Promise<T> {
  const result = await pending;

  if (result.status === "error") {
    throw new IpcError(result.error);
  }

  return result.data;
}

function progressChannel(onProgress: (progress: Progress) => void): Channel<Progress> {
  return new Channel<Progress>(onProgress);
}

export const nowPlaying = () => unwrap(commands.nowPlaying());
export const rateCurrent = (rating: number) => unwrap(commands.rateCurrent(rating));
export const recentRatings = () => unwrap(commands.recentRatings());
export const libraryArtists = () => unwrap(commands.libraryArtists());
export const libraryTracks = (artistId: string) => unwrap(commands.libraryTracks(artistId));
export const trackHistory = (trackId: string) => unwrap(commands.trackHistory(trackId));
export const artistDetails = (artistId: string | null) => unwrap(commands.artistDetails(artistId));
export const playTrack = (trackId: string) => unwrap(commands.playTrack(trackId));
export const queueCurrentAlbum = () => unwrap(commands.queueCurrentAlbum());
export const queueMyTopTen = (artistId: string | null) => unwrap(commands.queueMyTopTen(artistId));
export const cancelTask = (task: "collaborators" | "discover") => unwrap(commands.cancelTask(task));
export const songCard = () => unwrap(commands.songCard());
export const albumCard = () => unwrap(commands.albumCard());
export const coverDataUrl = (url: string) => unwrap(commands.coverDataUrl(url));
export const copyCardImage = (pngBase64: string) => unwrap(commands.copyCardImage(pngBase64));
export const accountStatus = () => unwrap(commands.accountStatus());
export const saveClientId = (clientId: string) => unwrap(commands.saveClientId(clientId));
export const signIn = () => unwrap(commands.signIn());
export const signOut = () => unwrap(commands.signOut());
export const importLegacyFile = () => unwrap(commands.importLegacyFile());
export const importRatingsFile = () => unwrap(commands.importRatingsFile());
export const exportRatingsFile = () => unwrap(commands.exportRatingsFile());

export function rerateTrack(trackId: string, rating: number) {
  return unwrap(commands.rerateTrack(trackId, rating));
}

export function buildPlaylist(request: PlaylistRequest): Promise<PlaylistResult> {
  return unwrap(commands.buildPlaylist(request));
}

export function findCollaborators(
  artistId: string | null,
  onProgress: (progress: Progress) => void,
) {
  return unwrap(commands.findCollaborators(artistId, progressChannel(onProgress)));
}

export function discoverPlaylist(
  mode: DiscoverMode,
  onProgress: (progress: Progress) => void,
): Promise<PlaylistResult> {
  return unwrap(commands.discoverPlaylist(null, mode, progressChannel(onProgress)));
}
