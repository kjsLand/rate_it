/// <reference types="vitest/config" />
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// See https://v2.tauri.app/start/frontend/vite/
export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: { port: 5173, strictPort: true },
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  build: { target: "safari15", outDir: "dist", emptyOutDir: true },
  test: {
    environment: "jsdom",
    setupFiles: ["src/ipc/test-setup.ts"],
    include: ["src/**/*.test.{ts,tsx}"],
  },
});
