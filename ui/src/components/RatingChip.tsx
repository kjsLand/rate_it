/** The color band of a rating (docs/UI-SPEC.md, "Rating bands"). The number always shows too. */
export type RatingBand = "low" | "mid" | "high" | "top";

export function ratingBand(rating: number): RatingBand {
  if (rating >= 10) return "top";

  if (rating >= 8) return "high";

  if (rating >= 5) return "mid";

  return "low";
}

/** A pill with the rating number, colored by its band. "–" when there is no rating. */
export function RatingChip({ rating, label }: { rating: number | null; label?: string }) {
  if (rating === null) {
    return (
      <span className="chip" data-band="none" aria-label={label}>
        –
      </span>
    );
  }

  return (
    <span className="chip" data-band={ratingBand(rating)} aria-label={label}>
      {rating}
      {rating >= 10 && (
        <span className="chip-star" aria-hidden="true">
          ★
        </span>
      )}
    </span>
  );
}
