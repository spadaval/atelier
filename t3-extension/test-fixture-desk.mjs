// The fixture Desk's address is read only from a whole "serving on" line: atelier-desk's output
// arrives in chunks that can end anywhere, and a partial address made the tests fetch the wrong one.
import assert from 'node:assert/strict';
import { servingUrl } from './fixture-desk.mjs';

const line = 'atelier-desk: built from 43f07d6, serving on http://127.0.0.1:32875\n';
for (let end = 0; end < line.length; end++) assert.equal(servingUrl(line.slice(0, end)), null, `a partial line gave an address: ${JSON.stringify(line.slice(0, end))}`);
assert.equal(servingUrl(line), 'http://127.0.0.1:32875');
assert.equal(servingUrl(line.replace('\n', '\r\n')), 'http://127.0.0.1:32875');
console.log('PASS: the fixture Desk\'s address is read only from a whole line, however its output is split');
