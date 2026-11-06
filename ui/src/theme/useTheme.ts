import { useState } from "react";

/** The theme the user picked. "system" follows the operating system setting. */
export type ThemeChoice = "system" | "light" | "dark";

const KEY = "rate_it.theme";

function parseChoice(value: string | null): ThemeChoice {
  return value === "light" || value === "dark" ? value : "system";
}

/** The saved theme choice. "system" when nothing is saved or storage is not available. */
export function storedTheme(): ThemeChoice {
  try {
    return parseChoice(window.localStorage.getItem(KEY));
  } catch {
    return "system";
  }
}

/** Sets `data-theme` on `<html>`. "system" removes it, so the CSS media query decides. */
export function applyTheme(choice: ThemeChoice): void {
  const root = document.documentElement;

  if (choice === "system") {
    delete root.dataset["theme"];
  } else {
    root.dataset["theme"] = choice;
  }
}

function saveTheme(choice: ThemeChoice): void {
  try {
    if (choice === "system") {
      window.localStorage.removeItem(KEY);
    } else {
      window.localStorage.setItem(KEY, choice);
    }
  } catch {
    // Storage is not available (private mode). The choice lasts until the app closes.
  }
}

/** The theme choice, and a setter that applies and saves it. UI state only. */
export function useTheme(): [ThemeChoice, (choice: ThemeChoice) => void] {
  const [choice, setChoice] = useState<ThemeChoice>(storedTheme);

  function change(next: ThemeChoice) {
    applyTheme(next);
    saveTheme(next);
    setChoice(next);
  }

  return [choice, change];
}
