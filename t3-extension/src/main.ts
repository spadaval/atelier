// The extension's entry. Vite builds it into dist/custom.js, one classic script that defines
// T3Custom; the loader injects it and calls T3Custom.start(api) (t3-custom.mjs, buildBundle).
export { start } from './custom.js';
