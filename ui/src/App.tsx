import { useState } from "react";
import type { ComponentType } from "react";

import { Icon } from "./components/Icon";
import type { IconName } from "./components/Icon";
import { Artist } from "./screens/Artist";
import { BuildPlaylist } from "./screens/BuildPlaylist";
import { Collaborators } from "./screens/Collaborators";
import { Discover } from "./screens/Discover";
import { Library } from "./screens/Library";
import { NowPlaying } from "./screens/NowPlaying";
import { Settings } from "./screens/Settings";
import { Share } from "./screens/Share";

type ScreenId =
  "now" | "library" | "artist" | "discover" | "share" | "playlist" | "collaborators" | "settings";

type Screen = {
  readonly id: ScreenId;
  readonly label: string;
  readonly icon: IconName;
  readonly view: ComponentType;
};

/** Listening screens: in the phone tab bar. */
const LISTEN = [
  { id: "now", label: "Now Playing", icon: "disc", view: NowPlaying },
  { id: "library", label: "Library", icon: "crate", view: Library },
  { id: "artist", label: "Artist", icon: "mic", view: Artist },
  { id: "discover", label: "Discover", icon: "compass", view: Discover },
] as const satisfies readonly Screen[];

/** Making screens and Settings: in the sidebar's second group, and the phone "More" sheet. */
const MAKE = [
  { id: "share", label: "Share", icon: "ticket", view: Share },
  { id: "playlist", label: "Build Playlist", icon: "cassette", view: BuildPlaylist },
  { id: "collaborators", label: "Collaborators", icon: "users", view: Collaborators },
  { id: "settings", label: "Settings", icon: "sliders", view: Settings },
] as const satisfies readonly Screen[];

const SCREENS: readonly Screen[] = [...LISTEN, ...MAKE];

function NavList({
  screens,
  current,
  onSelect,
}: {
  screens: readonly Screen[];
  current: ScreenId;
  onSelect: (id: ScreenId) => void;
}) {
  return (
    <ul className="nav-list">
      {screens.map((s) => (
        <li key={s.id}>
          <button
            type="button"
            className="nav-item"
            title={s.label}
            aria-current={s.id === current ? "page" : undefined}
            onClick={() => onSelect(s.id)}
          >
            <Icon name={s.icon} />
            <span className="nav-label">{s.label}</span>
          </button>
        </li>
      ))}
    </ul>
  );
}

/** The app frame: navigation and the selected screen. */
export function App() {
  const [screen, setScreen] = useState<ScreenId>("now");
  const [moreOpen, setMoreOpen] = useState(false);
  const current = SCREENS.find((s) => s.id === screen) ?? LISTEN[0];
  const inMore = MAKE.some((s) => s.id === screen);
  const View = current.view;

  function select(id: ScreenId) {
    setScreen(id);
    setMoreOpen(false);
  }

  return (
    <div className="app">
      <nav
        className="nav"
        aria-label="Screens"
        data-more={moreOpen ? "open" : "closed"}
        onKeyDown={(e) => {
          if (e.key === "Escape") setMoreOpen(false);
        }}
      >
        <p className="brand">
          rate<span className="brand-dot">_</span>it
        </p>
        <NavList screens={LISTEN} current={screen} onSelect={select} />
        <button
          type="button"
          className="nav-item nav-more"
          aria-expanded={moreOpen}
          aria-controls="nav-sheet"
          data-active={inMore}
          onClick={() => setMoreOpen((open) => !open)}
        >
          <Icon name="more" />
          <span className="nav-label">More</span>
        </button>
        <div className="nav-sheet" id="nav-sheet">
          <p className="nav-group">Make</p>
          <NavList screens={MAKE} current={screen} onSelect={select} />
        </div>
      </nav>
      <main data-screen={screen}>
        <div className="main-inner">
          <View />
        </div>
      </main>
    </div>
  );
}
