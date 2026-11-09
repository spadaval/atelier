import { defineConfig, devices } from "@playwright/test";

const port = Number(process.env.PORT ?? 7342);

// Loads every page from a built `atelier serve` over the fixture
// repositories (e2e/serve.sh), at a phone's width.
export default defineConfig({
  testDir: "e2e",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  reporter: process.env.CI ? "list" : "line",
  use: {
    baseURL: `http://127.0.0.1:${port}`,
    ...devices["Desktop Chrome"],
    viewport: { width: 390, height: 844 },
  },
  webServer: {
    command: "e2e/serve.sh",
    url: `http://127.0.0.1:${port}/api/repos`,
    env: { PORT: String(port) },
    timeout: 600_000,
    reuseExistingServer: false,
    stdout: "pipe",
    stderr: "pipe",
  },
});
