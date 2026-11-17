// A T3 Code test instance on this Linux host, so an agent can see the extension in the real T3 Code
// without the Windows test machine. It runs a pinned T3 Code Nightly on a virtual display, isolated
// from the T3 Code server this host runs, with a project, a thread answered by a free OpenCode model,
// a new-thread draft, a fixture Desk naming that thread, and the injector attached with the built
// extension. See "A test instance (Linux)" in README.md.
//
//   node t3-instance.mjs start [--model opencode/beam-free] [--port 9333] [--backend-port 47331]
//   node t3-instance.mjs status
//   node t3-instance.mjs shot [--out <dir>]     # screenshots in light and in dark
//   node t3-instance.mjs stop
//   node t3-instance.mjs isolation [--model …]  # start, thread, stop; checks nothing of the human's changed
//
// Everything lives under T3_TEST_ROOT, else /var/tmp/atelier-t3-test-<user>: apps/<tag>/ holds the
// unpacked T3 Code, fetched once; instance/ holds one instance, made afresh by each start, with
// state.json naming every process it started. The root must not be inside the human's home: OpenCode
// looks for configuration and skills (.opencode, .claude/skills, .agents/skills) in every folder above
// the one it runs in, so an instance under ~ would read the human's.
import fs from 'node:fs/promises';
import fsSync from 'node:fs';
import net from 'node:net';
import os from 'node:os';
import path from 'node:path';
import { spawn, execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { setTimeout as delay } from 'node:timers/promises';
import { connectPage } from './devtools.mjs';
import { fixture, deskBuild, writeFixtureDesk, servingUrl } from './fixture-desk.mjs';

const here = path.dirname(fileURLToPath(import.meta.url));
// The T3 Code this instance runs. A newer nightly is a change to this line and a contract run.
const release = 'v0.1.11-nightly.20261117.4851';
const appImage = 'T3-Code-Nightly-x86_64.AppImage';
const root = path.resolve(process.env.T3_TEST_ROOT ?? path.join('/var/tmp', `atelier-t3-test-${os.userInfo().username}`));
const insideHome = folder => folder === os.homedir() || folder.startsWith(os.homedir() + '/');
const appDir = path.join(root, 'apps', release, 'squashfs-root');
const dir = path.join(root, 'instance');
const statePath = path.join(dir, 'state.json');
// The instance's home. HOME, T3CODE_HOME and the XDG folders all point inside it.
const home = path.join(dir, 'home');
const option = (name, fallback) => { const index = process.argv.indexOf(name); return index < 0 ? fallback : process.argv[index + 1]; };
const log = message => console.log(`[t3-instance] ${message}`);

// The environment T3 Code and everything it starts get: an allowlist, not the caller's environment
// minus some names. An agent's shell inside T3 carries T3CODE_* (one of them turns on Tailscale
// serve), and API keys in the caller's environment would let OpenCode reach a paid model.
function instanceEnvironment(extra = {}) {
  return {
    PATH: process.env.PATH, LANG: process.env.LANG ?? 'C.UTF-8', USER: os.userInfo().username, LOGNAME: os.userInfo().username, SHELL: '/bin/bash',
    HOME: home, T3CODE_HOME: path.join(home, '.t3'),
    XDG_CONFIG_HOME: path.join(home, '.config'), XDG_DATA_HOME: path.join(home, '.local', 'share'),
    XDG_CACHE_HOME: path.join(home, '.cache'), XDG_STATE_HOME: path.join(home, '.local', 'state'),
    ...extra,
  };
}

// --- Processes -------------------------------------------------------------------------------------

// A process's start time in clock ticks since boot (/proc/<pid>/stat field 22): with the PID, it names
// one process, so a PID the system has since given to another is never signalled.
function startTime(pid) {
  try {
    const stat = fsSync.readFileSync(`/proc/${pid}/stat`, 'utf8');
    return stat.slice(stat.lastIndexOf(')') + 2).split(' ')[19];
  } catch { return null; }
}
const alive = ({ pid, start }) => startTime(pid) === start;
const processRecord = pid => ({ pid, start: startTime(pid) });
function parentOf(pid) {
  try { const stat = fsSync.readFileSync(`/proc/${pid}/stat`, 'utf8'); return Number(stat.slice(stat.lastIndexOf(')') + 2).split(' ')[1]); } catch { return null; }
}
function environOf(pid) {
  try { return Object.fromEntries(fsSync.readFileSync(`/proc/${pid}/environ`, 'utf8').split('\0').filter(Boolean).map(entry => [entry.slice(0, entry.indexOf('=')), entry.slice(entry.indexOf('=') + 1)])); } catch { return null; }
}
const allPids = () => fsSync.readdirSync('/proc').filter(name => /^\d+$/.test(name)).map(Number);
// Every live process of this instance: the ones state.json names, their descendants, and any process
// whose HOME is the instance's home, which catches what was reparented away from T3 (OpenCode's
// background service detaches itself). Each is found by PID and checked by its start time.
function instanceProcesses(state) {
  const found = new Map();
  for (const record of Object.values(state.processes ?? {})) if (alive(record)) found.set(record.pid, record);
  const pids = allPids();
  for (let grew = true; grew;) {
    grew = false;
    for (const pid of pids) if (!found.has(pid) && found.has(parentOf(pid))) { found.set(pid, processRecord(pid)); grew = true; }
  }
  for (const pid of pids) if (!found.has(pid) && environOf(pid)?.HOME === home) found.set(pid, processRecord(pid));
  found.delete(process.pid);
  return [...found.values()].filter(record => record.start);
}
function signal(records, name) {
  for (const record of records) if (alive(record)) { try { process.kill(record.pid, name); } catch {} }
}

const portOpen = port => new Promise(resolve => {
  const socket = net.connect(port, '127.0.0.1');
  socket.once('connect', () => { socket.destroy(); resolve(true); });
  socket.once('error', () => resolve(false));
});
async function until(what, check, ms) {
  const end = Date.now() + ms;
  for (;;) {
    const value = await check();
    if (value) return value;
    if (Date.now() > end) throw new Error(`Timed out after ${ms / 1000} s waiting for ${what}`);
    await delay(250);
  }
}
// Starts a detached process with its output in a log file under the instance and returns its record.
async function launch(name, command, args, env) {
  const output = await fs.open(path.join(dir, `${name}.log`), 'a');
  const child = spawn(command, args, { cwd: dir, env, detached: true, stdio: ['ignore', output.fd, output.fd] });
  await new Promise((resolve, reject) => { child.once('spawn', resolve); child.once('error', error => reject(new Error(`Starting ${name} (${command}): ${error.message}`))); });
  child.unref(); await output.close();
  return processRecord(child.pid);
}
const readState = async () => JSON.parse(await fs.readFile(statePath, 'utf8').catch(() => 'null'));
const writeState = state => fs.writeFile(statePath, JSON.stringify(state, null, 2) + '\n');
const running = state => Boolean(state?.processes?.t3 && alive(state.processes.t3));

// --- T3 Code ---------------------------------------------------------------------------------------

const which = program => (process.env.PATH ?? '').split(':').map(folder => path.join(folder, program)).find(file => { try { fsSync.accessSync(file, fsSync.constants.X_OK); return true; } catch { return false; } });

// Downloads the pinned AppImage once and unpacks it (no FUSE needed) into apps/<tag>/.
async function fetchApp() {
  if (fsSync.existsSync(path.join(appDir, 't3code'))) return;
  const url = `https://github.com/pingdotgg/t3code/releases/download/${release}/${appImage}`;
  const staging = path.join(root, 'apps', `${release}.partial`);
  await fs.rm(staging, { recursive: true, force: true });
  await fs.mkdir(staging, { recursive: true });
  log(`Fetching ${url}`);
  const response = await fetch(url);
  if (!response.ok) throw new Error(`GET ${url}: HTTP ${response.status} ${response.statusText}`);
  const file = path.join(staging, appImage);
  await fs.writeFile(file, response.body, { mode: 0o755 });
  execFileSync(file, ['--appimage-extract'], { cwd: staging, stdio: 'ignore' });
  await fs.rm(file);
  await fs.mkdir(path.dirname(appDir), { recursive: true });
  await fs.rename(path.join(staging, 'squashfs-root'), appDir);
  await fs.rm(staging, { recursive: true, force: true });
}

// A free display number: no socket and no lock file for it.
function freeDisplay() {
  for (let number = 70; number < 100; number++) if (!fsSync.existsSync(`/tmp/.X11-unix/X${number}`) && !fsSync.existsSync(`/tmp/.X${number}-lock`)) return number;
  throw new Error('No free X display between :70 and :99');
}
// T3 Code needs a real X display: with --ozone-platform=headless its main process crashes and leaves
// children holding the debugging port.
function displayServers() {
  const xvnc = which('Xvnc'); const xvfb = which('Xvfb');
  if (!xvnc && !xvfb) throw new Error('A virtual X display is needed and neither Xvnc nor Xvfb is on PATH: install tigervnc-standalone-server (Xvnc) or xvfb.');
  return { xvnc, xvfb };
}
async function startDisplay() {
  const number = freeDisplay();
  const { xvnc, xvfb } = displayServers();
  // Xvnc also lets a person watch: a VNC viewer on 127.0.0.1:<5900 + display>, local connections only.
  const args = xvnc ? [`:${number}`, '-geometry', '1440x900', '-depth', '24', '-SecurityTypes', 'None', '-localhost', '-rfbport', String(5900 + number)] : [`:${number}`, '-screen', '0', '1440x900x24', '-nolisten', 'tcp'];
  const record = await launch('display', xvnc ?? xvfb, args, { PATH: process.env.PATH, HOME: home });
  await until(`the X display :${number} (${path.join(dir, 'display.log')})`, () => fsSync.existsSync(`/tmp/.X11-unix/X${number}`) || (!alive(record) && Promise.reject(new Error(`${xvnc ?? xvfb} exited; read ${path.join(dir, 'display.log')}`))), 15000);
  return { record, display: `:${number}`, vnc: xvnc ? 5900 + number : null, server: path.basename(xvnc ?? xvfb) };
}

// The settings T3 Code reads at its first start. Only OpenCode is enabled, with the free model chosen
// for threads and for the titles T3 writes; onboarding is done and the boring sidebar is on.
async function seedHome(model) {
  const userdata = path.join(home, '.t3', 'userdata');
  for (const folder of [userdata, path.join(home, '.config'), path.join(home, '.local', 'share'), path.join(home, '.local', 'state'), path.join(home, '.cache')]) await fs.mkdir(folder, { recursive: true });
  const selection = { instanceId: 'opencode', model };
  await fs.writeFile(path.join(userdata, 'settings.json'), JSON.stringify({
    defaultModelSelection: selection, textGenerationModelSelection: selection,
    providerInstances: {
      opencode: { driver: 'opencode', enabled: true, config: { binaryPath: 'opencode' } },
      claudeAgent: { driver: 'claudeAgent', enabled: false },
      codex: { driver: 'codex', enabled: false },
    },
  }, null, 2));
  await fs.writeFile(path.join(userdata, 'client-settings.json'), JSON.stringify({ onboardingCompletedAt: new Date().toISOString(), boringSidebarEnabled: true }));
  // The window fills the display, so screenshots show what a person would see.
  await fs.writeFile(path.join(userdata, 'desktop-settings.json'), JSON.stringify({ mainWindowBounds: { x: 0, y: 0, width: 1440, height: 900 } }));
  // Without this T3 offers to put its `t3` command on PATH at the first launch.
  await fs.writeFile(path.join(userdata, 'cli-command-first-launch'), '');
}

// The models the instance's OpenCode lists with no credential: OpenCode Zen's free ones. --standalone
// keeps it from leaving a background service behind. It runs in the instance's home, so it reads no
// opencode.json or skills of the caller's working folder or of any folder above it.
function freeModels() {
  const output = execFileSync('opencode', ['models', '--standalone'], { cwd: home, env: instanceEnvironment(), encoding: 'utf8', timeout: 120000 });
  return output.split('\n').map(line => line.trim()).filter(line => /^[\w.-]+\/[\w.:-]+$/.test(line));
}
function credentials() {
  return execFileSync('opencode', ['auth', 'list', '--standalone'], { cwd: home, env: instanceEnvironment(), encoding: 'utf8', timeout: 60000 }).trim();
}

// The project T3 opens: a Git repository whose origin gives it the fixture's identity, so the
// extension maps it to the fixture Desk's repository. Its origin is never fetched.
async function writeProject() {
  const project = path.join(dir, 'project');
  await fs.mkdir(project, { recursive: true });
  await fs.writeFile(path.join(project, 'README.md'), '# Test project\n\nThe T3 Code test instance\'s project.\n');
  const git = (...args) => execFileSync('git', ['-c', 'user.name=fixture', '-c', 'user.email=fixture@example.invalid', ...args], { cwd: project, stdio: 'pipe' });
  git('init', '-q', '-b', 'master'); git('remote', 'add', 'origin', fixture.origin); git('add', '-A'); git('commit', '-qm', 'Test project');
  return project;
}

// --- Driving T3 through its debugging port --------------------------------------------------------

async function page(port) {
  const connected = await connectPage(port);
  const key = async (key, code, keyCode) => { for (const type of ['keyDown', 'keyUp']) await connected.call('Input.dispatchKeyEvent', { type, key, code, windowsVirtualKeyCode: keyCode }); };
  return {
    ...connected,
    // Waits for an expression to be truthy in the page and returns its value.
    waitFor: (what, expression, ms = 30000) => until(what, () => connected.evaluate(expression).catch(() => null), ms),
    // Clicks the first option, button or palette item whose text starts with `text`.
    click: text => connected.evaluate(`(() => { const node = [...document.querySelectorAll('[role=option],button,[cmdk-item]')].find(node => node.innerText?.trim().startsWith(${JSON.stringify(text)})); node?.click(); return Boolean(node); })()`),
    type: text => connected.call('Input.insertText', { text }),
    enter: () => key('Enter', 'Enter', 13),
  };
}

// From a fresh T3 to a thread with an answer and a new-thread draft of its project, all through T3's
// own screens: the palette's Add project, Local folder, the composer, and the folder's New thread.
async function makeThread(port, project) {
  const t3 = await page(port);
  try {
    await t3.waitFor('T3 Code to finish loading', `document.readyState === 'complete' && !location.hash.startsWith('#/welcome') && [...document.querySelectorAll('button')].some(node => node.innerText.trim() === 'Add project')`, 60000)
      .catch(async error => { throw new Error(`${error.message}; the page shows: ${(await t3.evaluate('location.hash + " " + document.body.innerText.slice(0, 300)')).replace(/\s+/g, ' ')}`); });
    await t3.click('Add project');
    await t3.waitFor('the Add project palette', `[...document.querySelectorAll('[role=option],[cmdk-item],button')].some(node => node.innerText?.trim().startsWith('Local folder'))`);
    await t3.click('Local folder');
    await delay(500);
    await t3.type(project);
    await delay(500);
    await t3.enter();
    const draftRoute = await t3.waitFor('the project\'s first draft', `location.hash.startsWith('#/draft/') && location.hash`, 30000);
    await t3.waitFor('the composer', `Boolean(document.querySelector('[data-testid="composer-editor"]'))`);
    await t3.evaluate(`document.querySelector('[data-testid="composer-editor"] [contenteditable=true], [data-testid="composer-editor"][contenteditable=true]')?.focus()`);
    await t3.type('Reply with the single word: ready. Do not run any tool.');
    await delay(300);
    await t3.enter();
    const route = await t3.waitFor(`the thread made from ${draftRoute}`, `/^#\\/[^/]+\\/[^/]+$/.test(location.hash) && !location.hash.startsWith('#/draft/') && location.hash`, 30000);
    const [environmentId, threadId] = route.slice(2).split('/').map(decodeURIComponent);
    // The answer: an assistant message in the timeline, after T3 shows the turn has ended.
    const answer = await t3.waitFor('an assistant message in the thread', `(() => { const rows = [...document.querySelectorAll('[data-timeline-row-id]')]; const text = document.querySelector('[data-workspace-area="conversation"]')?.innerText ?? ''; return /Worked for/.test(text) && rows.length >= 2 && rows.at(-1).innerText.trim(); })()`, 180000);
    // A new-thread draft of the project, kept in T3's draft store, then back to the thread.
    await t3.evaluate(`document.querySelector('[aria-label="New thread in project"]')?.click()`);
    const draft = (await t3.waitFor('a new-thread draft of the project', `location.hash.startsWith('#/draft/') && location.hash`, 15000)).slice('#/draft/'.length);
    await t3.evaluate(`location.hash = ${JSON.stringify(route.slice(1))}`);
    await t3.waitFor('the thread again', `location.hash === ${JSON.stringify(route)}`);
    return { environmentId, threadId, draft: decodeURIComponent(draft), answer };
  } finally { t3.close(); }
}

// --- Commands --------------------------------------------------------------------------------------

function describe(state) {
  const lines = [
    `T3 Code test instance running: T3 Code Nightly ${state.release}, debugging port ${state.port}.`,
    `  Thread ${state.thread.environmentId}/${state.thread.threadId}, draft ${state.draft}, model ${state.model}.`,
    `  Check the hooks: node test-contract.mjs --port ${state.port} --thread ${state.thread.environmentId}/${state.thread.threadId} --draft ${state.draft}`,
    `  Fixture Desk ${state.desk}; injector home ${state.extension}; display ${state.display}${state.vnc ? ` (VNC on 127.0.0.1:${state.vnc})` : ''}.`,
    `  Look: node t3-instance.mjs shot. Stop: node t3-instance.mjs stop. Logs: ${dir}.`,
  ];
  return lines.join('\n');
}

// One start at a time. The lock is a file linked into place whole, naming its owner by PID and start
// time; a second start waits for the first and then finds its instance running. A lock whose owner
// has exited is removed.
async function start(options) {
  if (insideHome(root)) throw new Error(`The test instance's root ${root} is inside ${os.homedir()}; OpenCode would read the configuration and skills in the folders above it. Set T3_TEST_ROOT to a folder outside it.`);
  await fs.mkdir(root, { recursive: true, mode: 0o700 });
  const lock = path.join(root, 'start.lock');
  const mine = path.join(root, `start.lock.${process.pid}`);
  await fs.writeFile(mine, JSON.stringify(processRecord(process.pid)));
  try {
    for (let waiting = false; ;) {
      try { await fs.link(mine, lock); break; } catch (error) { if (error.code !== 'EEXIST') throw error; }
      const owner = JSON.parse(await fs.readFile(lock, 'utf8').catch(() => 'null'));
      if (owner && alive(owner)) {
        if (!waiting) { log(`Another start (PID ${owner.pid}) is making the instance; waiting for it`); waiting = true; }
        await delay(1000);
      } else await fs.rm(lock, { force: true });
    }
  } finally { await fs.rm(mine, { force: true }); }
  try { return await startLocked(options); } finally { await fs.rm(lock, { force: true }); }
}

async function startLocked({ quiet = false } = {}) {
  const model = option('--model', 'opencode/beam-free');
  const port = Number(option('--port', 9333));
  const backendPort = Number(option('--backend-port', 47331));
  const existing = await readState();
  if (running(existing)) { console.log(`Already running; not starting another.\n${describe(existing)}`); return existing; }
  if (existing) await stop({ quiet: true });

  for (const [what, file, how] of [['the built extension', path.join(here, 'dist', 'custom.js'), `in ${here}, run pnpm install && pnpm build`]]) {
    await fs.access(file).catch(() => { throw new Error(`No ${what} at ${file}: ${how}.`); });
  }
  await deskBuild();
  displayServers();
  if (!which('opencode')) throw new Error('OpenCode (`opencode`) is not on PATH; the instance makes its thread with it. Install it from https://opencode.ai.');
  for (const [name, value] of [['debugging', port], ['backend', backendPort]]) if (await portOpen(value)) throw new Error(`The ${name} port ${value} is already in use; pass --${name === 'debugging' ? 'port' : 'backend-port'} <free port>.`);
  await fetchApp();

  await fs.rm(dir, { recursive: true, force: true });
  await fs.mkdir(dir, { recursive: true });
  await seedHome(model);
  const free = freeModels();
  if (!free.includes(model)) throw new Error(`Refusing model ${model}: it is not one of the free models the instance's OpenCode lists without a credential: ${free.join(', ')}.`);
  const project = await writeProject();

  const state = { release, model, port, backendPort, root, startedAt: new Date().toISOString(), processes: {} };
  const save = () => writeState(state);
  try {
    const display = await startDisplay();
    Object.assign(state, { display: display.display, vnc: display.vnc, displayServer: display.server });
    state.processes.display = display.record; await save();

    // Chromium's sandbox needs its SUID helper installed by root on this host, so it is off (README.md).
    state.processes.t3 = await launch('t3code', path.join(appDir, 't3code'), ['--no-sandbox', '--password-store=basic', '--remote-debugging-address=127.0.0.1', `--remote-debugging-port=${port}`],
      instanceEnvironment({ DISPLAY: display.display, T3CODE_PORT: String(backendPort), T3CODE_DISABLE_AUTO_UPDATE: '1' }));
    await save();
    log(`T3 Code started (PID ${state.processes.t3.pid}); making a project and a thread with ${model}`);
    await until(`T3 Code's renderer on debugging port ${port} (${path.join(dir, 't3code.log')})`, async () => {
      if (!alive(state.processes.t3)) throw new Error(`T3 Code exited; read ${path.join(dir, 't3code.log')}`);
      const targets = await fetch(`http://127.0.0.1:${port}/json/list`).then(response => response.json()).catch(() => []);
      return targets.some(target => target.type === 'page' && target.url.startsWith('t3code://app/'));
    }, 60000);
    const made = await makeThread(port, project);
    Object.assign(state, { project, thread: { environmentId: made.environmentId, threadId: made.threadId }, draft: made.draft, answer: made.answer });
    await save();
    if (!quiet) log(`Thread ${made.threadId} answered: ${JSON.stringify(made.answer.slice(0, 80))}`);

    // The fixture Desk, naming this thread as the lead of its open mission and the implementer of its building item.
    const deskDir = path.join(dir, 'desk');
    const { binary, args } = await writeFixtureDesk(deskDir, { ...fixture.threads, lead: made.threadId, implementer: made.threadId });
    state.processes.desk = await launch('desk', binary, args, { PATH: process.env.PATH, HOME: home });
    await save();
    state.desk = await until(`the fixture Desk (${path.join(dir, 'desk.log')})`, async () => {
      if (!alive(state.processes.desk)) throw new Error(`atelier-desk exited; read ${path.join(dir, 'desk.log')}`);
      return servingUrl(await fs.readFile(path.join(dir, 'desk.log'), 'utf8'));
    }, 30000);

    // The injector, in a home of its own with the built extension and a config naming this instance.
    const extension = path.join(dir, 'extension');
    await fs.mkdir(extension, { recursive: true });
    for (const [from, to] of [['t3-custom.mjs', 't3-custom.mjs'], ['dist/custom.js', 'custom.js'], ['custom.css', 'custom.css']]) await fs.copyFile(path.join(here, from), path.join(extension, to));
    await fs.writeFile(path.join(extension, 'config.json'), JSON.stringify({
      appPath: path.join(appDir, 't3code'), debugPort: port, targetPrefixes: ['t3code://app/'],
      desk: { urls: [state.desk], repositories: [] },
      settings: { enabled: true, references: true, followProject: true, autoUpdate: false, pinnedRepo: null, mappings: {} },
    }, null, 2) + '\n');
    const prepared = JSON.parse(execFileSync(process.execPath, [path.join(extension, 't3-custom.mjs'), 'prepare', '--home', extension], { encoding: 'utf8', env: { PATH: process.env.PATH, HOME: home } }));
    state.processes.injector = processRecord(prepared.pid);
    state.extension = extension;
    await save();
    const status = await until(`the injector to apply the extension (${path.join(extension, 'injector.log')})`, async () => {
      const result = JSON.parse(execFileSync(process.execPath, [path.join(extension, 't3-custom.mjs'), 'status', '--home', extension], { encoding: 'utf8' }));
      if (result.lastError) throw new Error(`The injector failed: ${result.lastError}`);
      return result.targets?.length && result.lastAppliedAt && result;
    }, 30000);
    state.extensionVersion = status.version;
    await save();
    if (!quiet) console.log(describe(state));
    return state;
  } catch (error) {
    await save();
    if (instanceProcesses(state).length) error.message += `\nThe instance was left running to look at; stop it with: node t3-instance.mjs stop`;
    throw error;
  }
}

async function stop({ quiet = false } = {}) {
  const state = await readState();
  if (!state) { if (!quiet) console.log(`No test instance: ${statePath} does not exist.`); return; }
  // The injector first, so it removes the extension from a live window, then everything else.
  if (state.processes?.injector && alive(state.processes.injector) && state.extension) {
    try { execFileSync(process.execPath, [path.join(state.extension, 't3-custom.mjs'), 'stop', '--home', state.extension], { stdio: 'ignore', timeout: 15000 }); } catch {}
  }
  let processes = instanceProcesses(state);
  const all = new Map(processes.map(record => [record.pid, record]));
  signal(processes, 'SIGTERM');
  const end = Date.now() + 10000;
  while ((processes = instanceProcesses(state)).length && Date.now() < end) {
    for (const record of processes) all.set(record.pid, record);
    await delay(250);
  }
  if (processes.length) { signal(processes, 'SIGKILL'); await delay(500); }
  const left = instanceProcesses(state);
  const ports = [];
  for (const port of [state.port, state.backendPort].filter(Boolean)) if (await portOpen(port)) ports.push(port);
  if (left.length || ports.length) throw new Error(`Stopping left ${left.map(record => `PID ${record.pid}`).join(', ') || 'no process'}${ports.length ? `, and ports ${ports.join(', ')} still open` : ''}`);
  await fs.rm(statePath);
  if (!quiet) console.log(`Stopped the test instance: ${all.size} processes (${[...all.keys()].join(', ')}), by PID. Its files stay in ${dir} until the next start.`);
}

async function status() {
  const state = await readState();
  if (!running(state)) { console.log(state ? `Not running; ${statePath} names processes that have exited. Run stop to clear it.` : 'No test instance.'); process.exitCode = 1; return; }
  console.log(describe(state));
  console.log(`  Processes: ${instanceProcesses(state).map(record => record.pid).join(', ')}.`);
}

// Screenshots of the T3 window as T3 draws it in light and in dark. T3 follows the system scheme,
// which the debugging port sets for the page alone; the scheme is put back afterwards.
async function shot() {
  const state = await readState();
  if (!running(state)) throw new Error('No test instance is running: start one with node t3-instance.mjs start');
  const out = path.resolve(option('--out', path.join(here, 'artifacts', 'test-instance')));
  await fs.mkdir(out, { recursive: true });
  const t3 = await page(state.port);
  try {
    const files = [];
    for (const scheme of ['light', 'dark']) {
      await t3.call('Emulation.setEmulatedMedia', { features: [{ name: 'prefers-color-scheme', value: scheme }] });
      await t3.waitFor(`T3's ${scheme} theme`, `document.documentElement.classList.contains('dark') === ${scheme === 'dark'}`, 5000);
      await delay(500);
      const { data } = await t3.call('Page.captureScreenshot', { format: 'png' });
      const file = path.join(out, `t3-${scheme}.png`);
      await fs.writeFile(file, Buffer.from(data, 'base64'));
      files.push(file);
    }
    await t3.call('Emulation.setEmulatedMedia', { features: [] });
    console.log(`Saved ${files.join(' and ')} (${await t3.evaluate('location.hash')}).`);
  } finally { t3.close(); }
}

// --- The isolation check ---------------------------------------------------------------------------

// The human's folders the instance must not touch. The checkout running this check is left out: it
// may itself sit under ~/.t3/worktrees, and the Desk and the injector run from it.
const checkout = path.resolve(here, '..');
const guardedPath = file => guarded.some(folder => file === folder || file.startsWith(folder + '/')) && file !== checkout && !file.startsWith(checkout + '/');
const guarded = [path.join(os.homedir(), '.t3'), path.join(os.homedir(), '.config', 'opencode'), path.join(os.homedir(), '.local', 'share', 'opencode')];
// What in the guarded folders was modified after `marker` was: each path, with no folder pruned.
// ~/.t3 holds millions of entries (worktrees), too many to keep two listings of; `find -newer` walks it
// once and names only what changed. The host's own T3 server writes under ~/.t3 all the time, so what
// changed is checked for any sign of the instance, not required to be empty (see isolation()).
function changedSince(marker) {
  return guarded.filter(folder => fsSync.existsSync(folder)).flatMap(folder =>
    execFileSync('find', [folder, '-xdev', '-path', checkout, '-prune', '-o', '-newer', marker, '-print'], { encoding: 'utf8', maxBuffer: 2 ** 28 }).split('\n').filter(Boolean));
}
// The top three levels of the guarded folders, to find anything removed.
function topLevels() {
  return new Set(guarded.filter(folder => fsSync.existsSync(folder)).flatMap(folder =>
    execFileSync('find', [folder, '-xdev', '-maxdepth', '3', '-path', checkout, '-prune', '-o', '-print'], { encoding: 'utf8' }).split('\n').filter(Boolean)));
}
// The first of `marks` the file holds, or null.
function containsAny(file, marks) {
  try {
    const stat = fsSync.statSync(file);
    if (!stat.isFile()) return null;
    // Read in pieces; the end of the last piece is kept, so a mark split across two is still found.
    const handle = fsSync.openSync(file, 'r'); const buffer = Buffer.alloc(1 << 22);
    try {
      for (let offset = 0, tail = ''; ; ) {
        const read = fsSync.readSync(handle, buffer, 0, buffer.length, offset);
        if (!read) return null;
        offset += read;
        const text = tail + buffer.toString('latin1', 0, read);
        const found = marks.find(mark => text.includes(mark));
        if (found) return found;
        tail = text.slice(-512);
      }
    } finally { fsSync.closeSync(handle); }
  } catch { return null; }
}

async function isolation() {
  if (running(await readState())) throw new Error('A test instance is running; the check starts its own. Stop it first: node t3-instance.mjs stop');
  await fs.mkdir(root, { recursive: true });
  const marker = path.join(root, 'isolation-marker');
  await fs.writeFile(marker, '');
  const before = topLevels();
  log(`Listed ${guarded.join(', ')} (${before.size} entries in their top three levels); starting an instance`);
  // Quiet: the instance's IDs are what the check looks for, and an agent running it inside the host's
  // T3 Code would have them written into its own thread's log if they were printed before the end.
  const state = await start({ quiet: true });
  const problems = [];
  // While it runs: no process of the instance has a guarded folder open, as a file, its working
  // directory or a mapping, and each runs with the instance's HOME, T3CODE_HOME and XDG folders.
  const processes = instanceProcesses(state);
  for (const record of processes) {
    // Chromium's children (--type=zygote, renderer, gpu-process, utility) rewrite the memory /proc
    // reads their environment and arguments from, and inherit the main process's; a process under bwrap (T3's
    // image loader, glycin) has the sandbox's own HOME. Their open files are still checked below.
    const cmdline = file => { try { return fsSync.readFileSync(file, 'utf8').split('\0'); } catch { return []; } };
    const sandboxed = /(^|\s)--type=/.test(cmdline(`/proc/${record.pid}/cmdline`).join(' ')) || cmdline(`/proc/${parentOf(record.pid)}/cmdline`)[0] === 'bwrap';
    const env = sandboxed ? null : environOf(record.pid);
    if (env && env.HOME !== home) problems.push(`PID ${record.pid} runs with HOME=${env.HOME}`);
    for (const name of ['T3CODE_HOME', 'XDG_CONFIG_HOME', 'XDG_DATA_HOME', 'XDG_CACHE_HOME', 'XDG_STATE_HOME']) if (env?.[name] && !env[name].startsWith(home + '/')) problems.push(`PID ${record.pid} runs with ${name}=${env[name]}`);
    const open = [];
    try { for (const fd of fsSync.readdirSync(`/proc/${record.pid}/fd`)) { try { open.push(fsSync.readlinkSync(`/proc/${record.pid}/fd/${fd}`)); } catch {} } } catch {}
    try { open.push(fsSync.readlinkSync(`/proc/${record.pid}/cwd`)); } catch {}
    try { open.push(...fsSync.readFileSync(`/proc/${record.pid}/maps`, 'utf8').split('\n').map(line => line.split(/\s+/).slice(5).join(' ')).filter(Boolean)); } catch {}
    for (const file of open) if (guardedPath(file)) problems.push(`PID ${record.pid} has ${file} open`);
  }
  log(`Checked the open files and environment of ${processes.length} instance processes`);
  await stop();
  const changed = changedSince(marker);
  const removed = [...before].filter(file => !fsSync.existsSync(file));
  // Fresh random IDs, written nowhere but by the instance: its folder's path is not one, since the
  // command that runs this check names it, and the host's T3 logs that command.
  const marks = [state.thread.threadId, state.thread.environmentId];
  for (const file of [...changed, ...removed]) {
    const inName = marks.find(mark => file.includes(mark));
    if (inName) problems.push(`${file} names the instance (${inName})`);
  }
  for (const file of changed) { const found = containsAny(file, marks); if (found) problems.push(`${file} holds the instance's ${found}`); }
  // OpenCode logs each folder it watches or boots in (path=, directory=); none may be in the human's home.
  const logs = path.join(home, '.local', 'share', 'opencode', 'log');
  const read = new Set();
  for (const name of await fs.readdir(logs).catch(() => [])) {
    for (const match of (await fs.readFile(path.join(logs, name), 'utf8')).matchAll(/\b(?:path|directory)=(\S+)/g)) if (insideHome(match[1])) read.add(match[1]);
  }
  for (const folder of read) problems.push(`The instance's OpenCode used ${folder} (${logs})`);
  const held = credentials();
  if (held !== 'No authenticated integrations') problems.push(`The instance's OpenCode holds credentials: ${held}`);
  console.log(`After the start, a thread and the stop, ${changed.length} entries under the guarded folders were changed or added and ${removed.length} removed; the instance's thread ${state.thread.threadId} and environment ${state.thread.environmentId} were looked for in each name and file.`);
  if (changed.length || removed.length) console.log(`Changed while the check ran, by the host's own T3 server unless named below:\n${[...changed, ...removed].slice(0, 15).map(file => `  ${file}`).join('\n')}${changed.length + removed.length > 15 ? `\n  … and ${changed.length + removed.length - 15} more` : ''}`);
  console.log(`The instance's OpenCode, asked for its credentials (opencode auth list): ${held}`);
  if (problems.length) { console.error(`Isolation broken:\n${problems.map(problem => `  ${problem}`).join('\n')}`); process.exitCode = 1; }
  else console.log('Isolation holds: no instance process opened a guarded folder, nothing changed in them names or holds the instance, and OpenCode used no folder in the human's home.');
}

const commands = { start, stop, status, shot, isolation };
const command = process.argv[2];
if (process.platform !== 'linux') { console.error('The test instance runs on Linux; on Windows see "A test instance (Windows)" in README.md.'); process.exit(1); }
if (!commands[command]) { console.error(`Usage: node t3-instance.mjs ${Object.keys(commands).join('|')} (see the top of ${fileURLToPath(import.meta.url)})`); process.exit(2); }
try { await commands[command](); } catch (error) { console.error(`t3-instance ${command} failed: ${error.message}`); process.exitCode = 1; }
// Exits once the command is done. Chromium sometimes leaves a DevTools WebSocket's close frame
// unanswered, and Node's WebSocket cannot be closed without that answer, so the open socket kept this
// process running for good after a finished start (seen in about one start in three).
process.exit();
