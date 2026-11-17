import { defineConfig } from 'vite';

// One classic script, dist/custom.js, which the loader injects (t3-custom.mjs, buildBundle).
// It is not minified, so a stack in injector.log names the code as written.
export default defineConfig({
  build: {
    outDir: 'dist',
    // `pnpm watch --outDir <install folder>` writes into a folder that holds config.json and logs.
    emptyOutDir: false,
    minify: false,
    lib: { entry: 'src/main.ts', formats: ['iife'], name: 'T3Custom', fileName: () => 'custom.js' },
  },
});
