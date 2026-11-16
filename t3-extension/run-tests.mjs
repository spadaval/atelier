// Runs every test in this folder that needs no real T3 Code: each `test-*.mjs` but those listed in
// `needsT3`, one at a time, since they share the browser's debugging port. CI runs this; a new test
// is a new `test-*.mjs` file. Exits non-zero naming each test that failed.
//
//   node run-tests.mjs                # the tests start a fixture Desk from this checkout
//   DESK=<url> node run-tests.mjs     # or use a Desk serving the fixture repository
import fs from 'node:fs/promises';
import path from 'node:path';
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const root = path.dirname(fileURLToPath(import.meta.url));
// Tests that drive a running T3 Code, run by hand after a T3 update (README.md).
const needsT3 = ['test-contract.mjs'];

const tests = (await fs.readdir(root)).filter(name => /^test-.*\.mjs$/.test(name) && !needsT3.includes(name)).sort();
const failed = [];
for (const test of tests) {
  console.log(`=== ${test}`);
  const started = Date.now();
  const code = await new Promise(resolve => spawn(process.execPath, [path.join(root, test)], { stdio: 'inherit' }).on('close', resolve));
  const seconds = Math.round((Date.now() - started) / 1000);
  console.log(`=== ${test}: ${code === 0 ? 'passed' : `FAILED (exit ${code})`} in ${seconds}s`);
  if (code !== 0) failed.push(test);
}
if (failed.length) {
  console.error(`${failed.length} of ${tests.length} tests failed: ${failed.join(', ')}`);
  process.exit(1);
}
console.log(`All ${tests.length} tests passed: ${tests.join(', ')}`);
