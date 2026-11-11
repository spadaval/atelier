import { defineConfig, devices } from "@playwright/test";

const port = Number(process.env.DESK_E2E_PORT ?? 4794);

// Loads the built app (`pnpm build` first) from Vite's preview server. Every
// /api request is answered in the browser from e2e/fixtures.ts, so no
// atelier-desk and no Rust build is needed.
export default defineConfig({
  testDir: "e2e",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  reporter: process.env.CI ? "list" : "line",
  use: { baseURL: `http://127.0.0.1:${port}` },
  projects: [
    { name: "desktop", use: { ...devices["Desktop Chrome"], viewport: { width: 1440, height: 900 } } },
    { name: "phone", use: { ...devices["Desktop Chrome"], viewport: { width: 390, height: 844 }, isMobile: true, hasTouch: true } },
  ],
  webServer: {
    command: `pnpm exec vite preview --host 127.0.0.1 --port ${port} --strictPort`,
    url: `http://127.0.0.1:${port}/`,
    reuseExistingServer: false,
    stdout: "pipe",
    stderr: "pipe",
  },
});
