import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// One classic script, with its stylesheet inside it as a string: what the loader injects.
export default defineConfig({
  plugins: [react(), tailwindcss()],
  define: { "process.env.NODE_ENV": JSON.stringify("production") },
  build: {
    outDir: "dist",
    lib: { entry: "src/main.tsx", formats: ["iife"], name: "AtelierT3", fileName: () => "custom.js" },
  },
});
