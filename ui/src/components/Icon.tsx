import type { ReactNode } from "react";

/** The names of the inline icons. One per screen, plus a few for actions and status. */
export type IconName =
  | "disc"
  | "crate"
  | "mic"
  | "ticket"
  | "cassette"
  | "users"
  | "compass"
  | "sliders"
  | "more"
  | "check"
  | "alert"
  | "play"
  | "plus"
  | "minus";

// 24 x 24 line drawings, stroke only (except where a fill is set). Drawn for this app.
const DRAWINGS: Readonly<Record<IconName, ReactNode>> = {
  disc: (
    <>
      <circle cx="12" cy="12" r="9" />
      <circle cx="12" cy="12" r="2.5" />
      <path d="M12 6.5a5.5 5.5 0 0 1 5.5 5.5" />
    </>
  ),
  crate: (
    <>
      <rect x="3" y="10" width="18" height="10" rx="1.5" />
      <path d="M6.5 10V5M10.5 10V4M14.5 10V6M18 10V5" />
    </>
  ),
  mic: (
    <>
      <rect x="9" y="3" width="6" height="11" rx="3" />
      <path d="M5.5 11a6.5 6.5 0 0 0 13 0M12 17.5V21M9 21h6" />
    </>
  ),
  ticket: (
    <>
      <path d="M3 7.5A1.5 1.5 0 0 1 4.5 6h15A1.5 1.5 0 0 1 21 7.5V10a2 2 0 0 0 0 4v2.5a1.5 1.5 0 0 1-1.5 1.5h-15A1.5 1.5 0 0 1 3 16.5V14a2 2 0 0 0 0-4z" />
      <path d="M15 7v10" strokeDasharray="2 2" />
    </>
  ),
  cassette: (
    <>
      <rect x="3" y="5" width="18" height="14" rx="2" />
      <circle cx="8.5" cy="11" r="1.75" />
      <circle cx="15.5" cy="11" r="1.75" />
      <path d="M10.25 11h3.5M7 19l1.5-3h7l1.5 3" />
    </>
  ),
  users: (
    <>
      <circle cx="9" cy="8" r="3.5" />
      <path d="M2.5 20a6.5 6.5 0 0 1 13 0M16 4.6a3.5 3.5 0 0 1 0 6.8M18 14.2a6.5 6.5 0 0 1 3.5 5.8" />
    </>
  ),
  compass: (
    <>
      <circle cx="12" cy="12" r="9" />
      <path d="m15.5 8.5-2 5-5 2 2-5z" />
    </>
  ),
  sliders: (
    <>
      <path d="M4 6h10M18 6h2M4 12h4M12 12h8M4 18h12" />
      <circle cx="16" cy="6" r="2" />
      <circle cx="10" cy="12" r="2" />
      <circle cx="18" cy="18" r="2" />
    </>
  ),
  more: (
    <>
      <circle cx="6" cy="12" r="1.25" fill="currentColor" />
      <circle cx="12" cy="12" r="1.25" fill="currentColor" />
      <circle cx="18" cy="12" r="1.25" fill="currentColor" />
    </>
  ),
  check: <path d="m5 12.5 4.5 4.5L19 7.5" />,
  alert: <path d="M12 3.5 21.5 20h-19zM12 10v4.5M12 17.5v.01" />,
  play: <path d="M8 5.5v13l10.5-6.5z" fill="currentColor" />,
  plus: <path d="M12 5v14M5 12h14" />,
  minus: <path d="M5 12h14" />,
};

/** A decorative icon. It is hidden from screen readers: the button or text next to it names it. */
export function Icon({ name }: { name: IconName }) {
  return (
    <svg
      className="icon"
      viewBox="0 0 24 24"
      width="20"
      height="20"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.75"
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden="true"
      focusable="false"
    >
      {DRAWINGS[name]}
    </svg>
  );
}
