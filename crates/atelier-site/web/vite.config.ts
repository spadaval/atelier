/// <reference types="vitest/config" />
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// `pnpm dev` proxies /api to the `atelier serve` that ATELIER_SERVE names,
// by default the one `atelier serve` starts on its default address.
const api = process.env.ATELIER_SERVE ?? "http://127.0.0.1:7341";

export default defineConfig({
  plugins: [react()],
  base: "/",
  server: {
    proxy: { "/api": { target: api, changeOrigin: true } },
  },
  build: { outDir: "dist", assetsDir: "assets", emptyOutDir: true },
  test: {
    environment: "jsdom",
    setupFiles: ["src/test/setup.ts"],
    include: ["src/**/*.test.{ts,tsx}"],
  },
});
