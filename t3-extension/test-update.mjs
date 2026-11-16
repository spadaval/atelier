// Installing from a release bundle and updating from the releases list, with no T3 Code and no
// network: a local server answers the releases list and the bundles, and a stand-in T3 app whose
// executable runs Node takes the place of T3 Code's, named to install.py with --app.
// Needs python3 and node on PATH.
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import os from 'node:os';
import http from 'node:http';
import { createHash } from 'node:crypto';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { fileURLToPath } from 'node:url';
import { setTimeout as delay } from 'node:timers/promises';

const execute = promisify(execFile);
const root = path.dirname(fileURLToPath(import.meta.url));
// Short, since the injector's control socket is a file in the home.
const temp = await fs.mkdtemp(path.join(os.tmpdir(), 't3u-'));
const home = path.join(temp, 'home');
const sha256 = data => createHash('sha256').update(data).digest('hex');

// The stand-in for T3 Code's app, as scripts/t3-extension-release.sh verify makes it.
const app = path.join(temp, 'T3 Code.app');
await fs.mkdir(path.join(app, 'Contents/MacOS'), { recursive: true });
await execute('python3', ['-c', 'import plistlib, sys; plistlib.dump({"CFBundleExecutable": "T3 Code"}, open(sys.argv[1], "wb"))', path.join(app, 'Contents/Info.plist')]);
await fs.writeFile(path.join(app, 'Contents/MacOS/T3 Code'), `#!/bin/sh\nexec ${JSON.stringify(process.execPath)} "$@"\n`, { mode: 0o755 });

// A bundle as the release workflow builds it: this folder's files under t3-extension-<version>/
// plus VERSION, zipped, with its sha256sum line. Each version's custom.js differs, so an update
// visibly replaces it.
async function makeBundle(version) {
  const stage = path.join(temp, `stage-${version}`);
  const folder = path.join(stage, `t3-extension-${version}`);
  await fs.mkdir(folder, { recursive: true });
  for (const entry of await fs.readdir(root, { withFileTypes: true })) {
    if (entry.isFile() && entry.name !== 'config.json' && !entry.name.endsWith('.log')) await fs.copyFile(path.join(root, entry.name), path.join(folder, entry.name));
  }
  await fs.appendFile(path.join(folder, 'custom.js'), `\n// release ${version}\n`);
  await fs.writeFile(path.join(folder, 'VERSION'), `${version}\n`);
  const zip = path.join(stage, `t3-extension-${version}.zip`);
  await execute('python3', ['-m', 'zipfile', '-c', zip, `t3-extension-${version}`], { cwd: stage });
  const data = await fs.readFile(zip);
  return { folder, zip: data, sum: `${sha256(data)}  t3-extension-${version}.zip\n` };
}
const bundles = { '0.1.0': await makeBundle('0.1.0'), '0.2.0': await makeBundle('0.2.0') };

// The releases list holds atelier's own releases, a draft and a prerelease with higher extension
// versions, a tag that is not x.y.z, and older and newer extension releases out of order:
// 0.2.0 is the one to take.
const server = http.createServer((request, response) => {
  const url = new URL(request.url, base);
  if (url.pathname === '/releases') {
    if (fixture.lookupFails) { response.writeHead(500, { 'content-type': 'application/json' }); response.end('{"message":"broken"}'); return; }
    const assets = version => [`t3-extension-${version}.zip`, `t3-extension-${version}.zip.sha256`].map(name => ({ name, browser_download_url: `${base}/assets/${name}` }));
    response.writeHead(200, { 'content-type': 'application/json' });
    response.end(JSON.stringify([
      { tag_name: 'v9.0.0', draft: false, prerelease: false, assets: [{ name: 'atelier-cli-x86_64-unknown-linux-gnu.tar.xz', browser_download_url: `${base}/assets/atelier` }] },
      { tag_name: 't3-extension-v0.1.0', draft: false, prerelease: false, assets: assets('0.1.0') },
      { tag_name: 't3-extension-v0.9.0', draft: true, prerelease: false, assets: assets('0.9.0') },
      { tag_name: 't3-extension-v0.2.0', draft: false, prerelease: false, assets: assets('0.2.0') },
      { tag_name: 't3-extension-v0.8.0', draft: false, prerelease: true, assets: assets('0.8.0') },
      { tag_name: 't3-extension-v1.0.0-rc.1', draft: false, prerelease: false, assets: assets('1.0.0-rc.1') },
      { tag_name: 't3-extension-v0.0.9', draft: false, prerelease: false, assets: assets('0.0.9') },
      { tag_name: 'v0.7.0', draft: false, prerelease: false, assets: [] },
    ]));
    return;
  }
  const match = url.pathname.match(/^\/assets\/t3-extension-(.+)\.zip(\.sha256)?$/);
  const bundle = match && bundles[match[1]];
  if (!bundle) { response.writeHead(404); response.end(); return; }
  fixture.downloads.push(url.pathname);
  response.writeHead(200);
  response.end(match[2] ? (fixture.badChecksum ? `${'0'.repeat(64)}  t3-extension-${match[1]}.zip\n` : bundle.sum) : bundle.zip);
});
const fixture = { lookupFails: false, badChecksum: false, downloads: [] };
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
const base = `http://127.0.0.1:${server.address().port}`;

// The installed control command, which runs the installed t3-custom.mjs under the stand-in.
async function cli(...args) {
  try {
    const { stdout } = await execute(path.join(home, 't3-custom'), args);
    return { code: 0, output: stdout, json: JSON.parse(stdout) };
  } catch (error) {
    let json = null; try { json = JSON.parse(error.stdout); } catch {}
    return { code: error.code, output: error.stdout + error.stderr, json };
  }
}
const read = name => fs.readFile(path.join(home, name), 'utf8');
const exists = name => fs.access(path.join(home, name)).then(() => true, () => false);
const editConfig = async change => { const config = JSON.parse(await read('config.json')); change(config); await fs.writeFile(path.join(home, 'config.json'), JSON.stringify(config, null, 2)); };
async function until(check, label, timeout = 15000) {
  const end = Date.now() + timeout;
  let last;
  while (Date.now() < end) { last = await check(); if (last) return last; await delay(100); }
  throw new Error(`${label} timed out`);
}
const installFrom = version => execute('python3', [path.join(bundles[version].folder, 'install.py'), '--home', home, '--app', app]);

try {
  // An install from a bundle into an empty home.
  await installFrom('0.1.0');
  assert.equal((await read('VERSION')).trim(), '0.1.0');
  for (const name of ['t3-custom.mjs', 'host-adapter.js', 'custom.js', 'custom.css', 'README.md', 't3-custom']) assert(await exists(name), `${name} was not installed`);
  const installedConfig = JSON.parse(await read('config.json'));
  assert.equal(installedConfig.appPath, app);
  assert.equal(installedConfig.python, (await execute('python3', ['-c', 'import sys; print(sys.executable)'])).stdout.trim());
  assert.equal(installedConfig.settings.autoUpdate, false, 'automatic updates are on by default');
  assert.equal((await cli('check')).json.release, '0.1.0');
  // Settings of this device, which every later install and update must keep.
  await editConfig(config => { config.releases = `${base}/releases`; config.debugPort = 19333; config.desk.urls = ['http://desk.test:3780']; });
  console.log('PASS: the bundle installs into an empty home, records its version and Python, and has automatic updates off');

  // update --check reports the newest extension release and changes nothing.
  const checked = await cli('update', '--check');
  assert.equal(checked.code, 0, checked.output);
  assert.equal(checked.json.installed, '0.1.0');
  assert.deepEqual(checked.json.latest, { version: '0.2.0', tag: 't3-extension-v0.2.0' });
  assert.equal(checked.json.newer, true);
  assert.equal((await read('VERSION')).trim(), '0.1.0');
  assert(!(await exists('update.json')), 'update --check wrote update.json');
  assert.deepEqual(fixture.downloads, [], 'update --check downloaded a bundle');
  console.log('PASS: update --check takes 0.2.0 over atelier releases, a draft, a prerelease, a non-x.y.z tag and older releases, and changes nothing');

  // A bundle whose checksum does not match is refused, and the installed version stays.
  const customBefore = await read('custom.js');
  fixture.badChecksum = true;
  const refused = await cli('update');
  fixture.badChecksum = false;
  assert.equal(refused.code, 1, refused.output);
  assert.match(refused.json.error, /^Checking t3-extension-0\.2\.0\.zip: its SHA-256 is [0-9a-f]{64}, but t3-extension-0\.2\.0\.zip\.sha256 says 0{64}; nothing was installed$/);
  assert.equal((await read('VERSION')).trim(), '0.1.0');
  assert.equal(await read('custom.js'), customBefore);
  assert.equal(JSON.parse(await read('update.json')).error, refused.json.error);
  console.log('PASS: a bundle whose checksum does not match is refused by name and nothing changes');

  // A failed lookup is reported by name.
  fixture.lookupFails = true;
  const failed = await cli('update', '--check');
  fixture.lookupFails = false;
  assert.equal(failed.code, 1);
  assert.equal(failed.json.error, `Looking up releases: GET ${base}/releases: HTTP 500 Internal Server Error`);
  console.log('PASS: a failed lookup is reported by name');

  // update installs the newer release over this home and keeps its config.json.
  const updated = await cli('update');
  assert.equal(updated.code, 0, updated.output);
  assert.equal(updated.json.installed, '0.2.0');
  assert.equal(updated.json.reloaded, false, 'no injector runs, so none was reloaded');
  assert.equal(updated.json.restartNeeded, false, 't3-custom.mjs did not change');
  assert.equal((await read('VERSION')).trim(), '0.2.0');
  assert.match(await read('custom.js'), /\/\/ release 0\.2\.0\n$/);
  assert(!(await exists('custom.js.previous')), 'the update took the custom.js 0.1.0 installed for an edit');
  const keptConfig = JSON.parse(await read('config.json'));
  assert.equal(keptConfig.releases, `${base}/releases`);
  assert.deepEqual(keptConfig.desk.urls, ['http://desk.test:3780']);
  assert.equal(keptConfig.appPath, app);
  const again = await cli('update');
  assert.equal(again.json.newer, false); assert.equal(again.json.installedAt, undefined, 'an update reinstalled the same release');
  console.log('PASS: update installs 0.2.0 from its bundle and keeps config.json; a second update does nothing');

  // The injector checks for a release when it starts. A failed lookup is in status by name, and the
  // injector itself carries on.
  // An edited custom.js is kept beside the new one.
  await fs.appendFile(path.join(home, 'custom.js'), '// my edit\n');
  await installFrom('0.1.0');
  assert.match(await read('custom.js.previous'), /\/\/ my edit\n$/);
  assert.match(await read('custom.js'), /\/\/ release 0\.1\.0\n$/);
  await fs.rm(path.join(home, 'update.json'));
  fixture.lookupFails = true;
  assert.equal((await cli('prepare')).code, 0);
  let status = await until(async () => { const { json } = await cli('status'); return json?.update && json; }, 'a release check in status');
  assert.equal(status.update.error, `Looking up releases: GET ${base}/releases: HTTP 500 Internal Server Error`);
  assert.equal(status.lastError, null, 'a failed lookup became the injector\'s error');
  assert.equal(status.release, '0.1.0');
  await cli('stop');
  fixture.lookupFails = false;
  console.log('PASS: the injector reports a failed lookup in status by name and keeps running');

  // With autoUpdate off, the injector reports the newer release and installs nothing.
  await until(async () => (await cli('status')).code !== 0, 'the injector stopped');
  await fs.rm(path.join(home, 'update.json'));
  assert.equal((await cli('prepare')).code, 0);
  status = await until(async () => { const { json } = await cli('status'); return json?.update && json; }, 'a release check in status');
  assert.equal(status.update.error, null);
  assert.equal(status.update.newer, true);
  assert.equal(status.update.latest.version, '0.2.0');
  await delay(500);
  assert.equal((await read('VERSION')).trim(), '0.1.0', 'the injector installed a release with autoUpdate off');
  await cli('stop');
  console.log('PASS: with autoUpdate off the injector reports 0.2.0 and installs nothing');

  // With autoUpdate on, it installs the release when it starts and reloads.
  await until(async () => (await cli('status')).code !== 0, 'the injector stopped');
  await editConfig(config => { config.settings.autoUpdate = true; });
  assert.equal((await cli('prepare')).code, 0);
  status = await until(async () => { const { json } = await cli('status'); return json?.update?.installedAt && json; }, 'the release installed by the injector');
  assert.equal(status.update.installed, '0.2.0');
  assert.equal((await read('VERSION')).trim(), '0.2.0');
  assert.equal(JSON.parse(await read('config.json')).settings.autoUpdate, true, 'the update lost the setting');
  status = await until(async () => { const { json } = await cli('status'); return json?.release === '0.2.0' && json; }, 'the injector reloaded the release');
  assert.equal(status.restartNeeded, false);
  console.log('PASS: with autoUpdate on the injector installs 0.2.0 at start, keeps the setting and reports the new release');
} finally {
  await cli('stop');
  server.close();
  await fs.rm(temp, { recursive: true, force: true });
}
