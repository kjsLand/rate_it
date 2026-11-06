import { useState } from "react";
import "./RatingMeter.css";

const VALUES = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

/**
 * Ten rating buttons drawn as a level meter: the bars rise from 1 to 10.
 * The bars up to the pointer light up. Each bar shows its number, so height is not the only cue.
 */
export function RatingMeter({
  value,
  disabled,
  onRate,
}: {
  value: number | null;
  disabled: boolean;
  onRate: (rating: number) => void;
}) {
  const [hover, setHover] = useState<number | null>(null);
  const lit = hover ?? value ?? 0;

  return (
    <div
      className="meter"
      role="group"
      aria-label="Rating"
      data-preview={hover !== null}
      onMouseLeave={() => setHover(null)}
    >
      {VALUES.map((n) => (
        <button
          key={n}
          type="button"
          className="meter-bar"
          data-lit={n <= lit}
          aria-label={`Rate ${n}`}
          aria-pressed={value === n}
          disabled={disabled}
          onMouseEnter={() => setHover(n)}
          onFocus={() => setHover(n)}
          onBlur={() => setHover(null)}
          onClick={() => onRate(n)}
        >
          <span className="meter-fill" />
          <span className="meter-num" aria-hidden="true">
            {n}
          </span>
        </button>
      ))}
    </div>
  );
}
