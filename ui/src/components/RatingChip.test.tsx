import { render, screen } from "@testing-library/react";
import { expect, test } from "vitest";

import { RatingChip, ratingBand } from "./RatingChip";

test("puts each rating in its band", () => {
  expect([1, 4, 5, 7, 8, 9, 10].map(ratingBand)).toEqual([
    "low",
    "low",
    "mid",
    "mid",
    "high",
    "high",
    "top",
  ]);
});

test("shows the number, and a dash with no rating", () => {
  render(
    <>
      <RatingChip rating={10} label="Rating 10" />
      <RatingChip rating={null} label="No rating" />
    </>,
  );
  expect(screen.getByLabelText("Rating 10").textContent).toBe("10");
  expect(screen.getByLabelText("Rating 10").dataset["band"]).toBe("top");
  expect(screen.getByLabelText("No rating").textContent).toBe("–");
});
