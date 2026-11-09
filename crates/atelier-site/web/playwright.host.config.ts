import { defineConfig, devices } from "@playwright/test";

// Loads every mission's brief and debrief from a running `atelier serve`
// over this host's repositories, named by ATELIER_SERVE:
//   ATELIER_SERVE=http://127.0.0.1:7341 pnpm exec playwright test -c playwright.host.config.ts
export default defineConfig({
  testDir: "e2e/host",
  reporter: "line",
  workers: 2,
  timeout: 120_000,
  use: {
    baseURL: process.env.ATELIER_SERVE ?? "http://127.0.0.1:7341",
    ...devices["Desktop Chrome"],
    viewport: { width: 390, height: 844 },
  },
});
