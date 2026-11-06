import { useState } from "react";

import { Artist } from "./screens/Artist";
import { BuildPlaylist } from "./screens/BuildPlaylist";
import { Collaborators } from "./screens/Collaborators";
import { Discover } from "./screens/Discover";
import { Library } from "./screens/Library";
import { NowPlaying } from "./screens/NowPlaying";
import { Settings } from "./screens/Settings";
import { Share } from "./screens/Share";

const SCREENS = [
  { id: "now", label: "Now Playing", view: NowPlaying },
  { id: "library", label: "Library", view: Library },
  { id: "artist", label: "Artist", view: Artist },
  { id: "share", label: "Share", view: Share },
  { id: "playlist", label: "Build Playlist", view: BuildPlaylist },
  { id: "collaborators", label: "Collaborators", view: Collaborators },
  { id: "discover", label: "Discover", view: Discover },
  { id: "settings", label: "Settings", view: Settings },
] as const;

type ScreenId = (typeof SCREENS)[number]["id"];

/** The app frame: navigation and the selected screen. */
export function App() {
  const [screen, setScreen] = useState<ScreenId>("now");
  const current = SCREENS.find((s) => s.id === screen) ?? SCREENS[0];
  const View = current.view;

  return (
    <div className="app">
      <nav aria-label="Screens">
        <p className="brand">rate_it</p>
        {SCREENS.map((s) => (
          <button
            key={s.id}
            type="button"
            aria-current={s.id === screen ? "page" : undefined}
            onClick={() => setScreen(s.id)}
          >
            {s.label}
          </button>
        ))}
      </nav>
      <main>
        <View />
      </main>
    </div>
  );
}
