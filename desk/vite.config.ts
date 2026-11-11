import react from "@vitejs/plugin-react";
import { defineConfig } from "vitest/config";

// `pnpm dev` serves the app and sends /api to a running atelier-desk, so a
// change to the app needs no Rust build. ATELIER_DESK names that server.
const desk = process.env.ATELIER_DESK ?? "http://127.0.0.1:3780";

export default defineConfig({
  plugins: [react()],
  server: {
    host: "127.0.0.1",
    port: Number(process.env.DESK_DEV_PORT ?? 5173),
    strictPort: true,
    proxy: { "/api": { target: desk, changeOrigin: true } },
  },
  test: {
    include: ["src/**/*.test.ts"],
  },
});
