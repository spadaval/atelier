import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";

// T3 Code's stylesheet, for the page the check injects into.
export default defineConfig({
  plugins: [tailwindcss()],
  build: { outDir: "dist-reference", lib: { entry: "src/reference.ts", formats: ["es"], fileName: () => "reference.js", cssFileName: "reference" } },
});
