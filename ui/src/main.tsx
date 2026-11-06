import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { App } from "./App";
import "@fontsource-variable/fraunces/full.css";
import "@fontsource-variable/fraunces/full-italic.css";
import "@fontsource-variable/instrument-sans/index.css";
import "@fontsource/ibm-plex-mono/400.css";
import "@fontsource/ibm-plex-mono/500.css";
import "./styles/tokens.css";
import "./styles/base.css";
import "./styles/shell.css";
import "./styles/components.css";
import { applyTheme, storedTheme } from "./theme/useTheme";

applyTheme(storedTheme());

// In `vite dev` in a normal browser there is no Tauri: load sample data to preview the screens.
// Vite removes this branch from the production build.
if (import.meta.env.DEV && !("__TAURI_INTERNALS__" in window)) {
  await import("./ipc/devBackend");
}

const root = document.getElementById("root");

if (root !== null) {
  createRoot(root).render(
    <StrictMode>
      <App />
    </StrictMode>,
  );
}
