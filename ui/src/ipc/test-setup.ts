/** Vitest setup: clean the DOM and the IPC fakes after each test. */
import { cleanup } from "@testing-library/react";
import { afterEach } from "vitest";

import { resetFakes } from "./testing";

afterEach(() => {
  cleanup();
  resetFakes();
});
