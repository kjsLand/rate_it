import { useState } from "react";

const VALUES = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

/** Ten star buttons. The stars up to the pointer light up. */
export function Stars({
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
    <div className="stars" role="group" aria-label="Rating" onMouseLeave={() => setHover(null)}>
      {VALUES.map((n) => (
        <button
          key={n}
          type="button"
          className={n <= lit ? "star lit" : "star"}
          aria-label={`Rate ${n}`}
          aria-pressed={value === n}
          disabled={disabled}
          onMouseEnter={() => setHover(n)}
          onClick={() => onRate(n)}
        >
          ★
        </button>
      ))}
    </div>
  );
}
