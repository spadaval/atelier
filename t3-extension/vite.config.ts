import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

// One classic script, dist/custom.js, which the loader injects (t3-custom.mjs, buildBundle); the
// stylesheet React roots adopt is inside it as a string (src/react/mount.tsx).
// It is not minified, so a stack in injector.log names the code as written.
export default defineConfig({
  // Not optimised: Lightning CSS, which optimises, does not know :host-context (T3's dark variant
  // here, sync-t3.mjs) and warns on every use. Chromium reads the nesting it would have lowered.
  plugins: [react(), tailwindcss({ optimize: false })],
  // An IIFE has no bundler to replace it at load time; React reads it to pick its production build.
  define: { 'process.env.NODE_ENV': JSON.stringify('production') },
  build: {
    outDir: 'dist',
    // `pnpm watch --outDir <install folder>` writes into a folder that holds config.json and logs.
    emptyOutDir: false,
    minify: false,
    lib: { entry: 'src/main.ts', formats: ['iife'], name: 'T3Custom', fileName: () => 'custom.js' },
  },
});
