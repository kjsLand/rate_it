import { useTheme } from "../theme/useTheme";
import type { ThemeChoice } from "../theme/useTheme";

const CHOICES: readonly { value: ThemeChoice; label: string }[] = [
  { value: "system", label: "System" },
  { value: "light", label: "Light" },
  { value: "dark", label: "Dark" },
];

/** A segmented control for the theme: System, Light or Dark. */
export function ThemeSwitch() {
  const [theme, setTheme] = useTheme();

  return (
    <div className="segmented" role="radiogroup" aria-label="Theme">
      {CHOICES.map((choice) => (
        <label key={choice.value} className="segment">
          <input
            type="radio"
            name="theme"
            value={choice.value}
            checked={theme === choice.value}
            onChange={() => setTheme(choice.value)}
          />
          <span>{choice.label}</span>
        </label>
      ))}
    </div>
  );
}
