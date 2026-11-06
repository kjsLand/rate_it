import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, expect, test } from "vitest";

import { applyTheme, storedTheme } from "../theme/useTheme";
import { ThemeSwitch } from "./ThemeSwitch";

afterEach(() => {
  window.localStorage.clear();
  applyTheme("system");
});

test("applies and saves the theme choice", async () => {
  render(<ThemeSwitch />);
  expect(screen.getByRole("radio", { name: "System" })).toHaveProperty("checked", true);

  await userEvent.click(screen.getByRole("radio", { name: "Dark" }));
  expect(document.documentElement.dataset["theme"]).toBe("dark");
  expect(storedTheme()).toBe("dark");

  await userEvent.click(screen.getByRole("radio", { name: "System" }));
  expect(document.documentElement.dataset["theme"]).toBeUndefined();
  expect(storedTheme()).toBe("system");
});

test("starts with the saved choice and ignores unknown values", () => {
  window.localStorage.setItem("rate_it.theme", "light");
  render(<ThemeSwitch />);
  expect(screen.getByRole("radio", { name: "Light" })).toHaveProperty("checked", true);

  window.localStorage.setItem("rate_it.theme", "sepia");
  expect(storedTheme()).toBe("system");
});
