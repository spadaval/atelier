import fs from 'node:fs/promises';
import { watch } from 'node:fs';
import net from 'node:net';
import path from 'node:path';
import os from 'node:os';
import { spawn, execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { fileURLToPath } from 'node:url';
import { createHash } from 'node:crypto';
import vm from 'node:vm';
import { setTimeout as delay } from 'node:timers/promises';

const execute = promisify(execFile);
const entry = fileURLToPath(import.meta.url);

// Everything that differs between Windows and macOS lives in this object.
const platform = process.platform === 'win32' ? {
  controlPath: home => `\\\\.\\pipe\\t3-custom-${createHash('sha256').update(home.toLowerCase()).digest('hex').slice(0, 12)}`,
  openFolder: folder => spawn('explorer.exe', [folder], { detached: true, stdio: 'ignore' }).unref(),
  dialog: (title, message) => execute('powershell.exe', ['-NoProfile', '-NonInteractive', '-Command',
    'Add-Type -AssemblyName PresentationFramework; [void][System.Windows.MessageBox]::Show($env:T3_CUSTOM_MESSAGE, $env:T3_CUSTOM_TITLE)'],
    { env: { ...process.env, T3_CUSTOM_MESSAGE: message, T3_CUSTOM_TITLE: title } }),
  // A second process for a data directory that is already in use hands off to
  // the running instance and exits, so an early exit means "already running".
  async launch(config, args) {
    const child = spawn(config.appPath, args, { detached: true, stdio: 'ignore', env: appEnvironment(config) });
    let exitCode = null;
    child.once('exit', code => { exitCode = code; });
    child.unref();
    for (let attempt = 0; attempt < 60 && exitCode === null; attempt++) {
      if (await debugPortOpen(config)) return null;
      await delay(500);
    }
    return exitCode === null ? null : `${config.appPath} exited with code ${exitCode} before opening its debugging port`;
  },
} : {
  controlPath: home => path.join(home, 'injector.sock'),
  openFolder: folder => execute('/usr/bin/open', [folder], { env: appEnvironment({}) }),
  dialog: (title, message) => execute('/usr/bin/osascript', ['-e', `display dialog ${JSON.stringify(message)} with title ${JSON.stringify(title)} buttons {"OK"} default button "OK"`]),
  async launch(config, args) {
    if (Object.keys(config.appEnv ?? {}).length) throw new Error('config.appEnv (a separate test instance) is only supported on Windows');
    const executable = path.join(config.appPath, 'Contents/MacOS', path.basename(config.appPath, '.app'));
    const { stdout } = await execute('/bin/ps', ['-axo', 'args=']);
    const running = stdout.split('\n').some(line => line.trim() === executable || line.trim().startsWith(`${executable} -`));
    if (running && !(await debugPortOpen(config))) return `${executable} is running`;
    await execute('/usr/bin/open', ['-a', config.appPath, '--args', ...args], { env: appEnvironment(config) });
    return null;
  },
};

const homeIndex = process.argv.indexOf('--home');
// The installed loader lives in its own home folder beside config.json.
const home = homeIndex < 0 ? path.dirname(entry) : path.resolve(process.argv[homeIndex + 1]);
const command = process.argv[2] ?? 'launch';
const controlPath = platform.controlPath(home);
// A shell inside T3 inherits T3CODE_* variables that would tie a new instance to the running one.
const appEnvironment = config => {
  const env = { ...process.env };
  for (const name of Object.keys(env)) if (/^(T3CODE_|ELECTRON_RUN_AS_NODE$|NODE_OPTIONS$)/i.test(name)) delete env[name];
  return { ...env, ...config.appEnv };
};
const configPath = path.join(home, 'config.json');
const loadConfig = async () => JSON.parse(await fs.readFile(configPath, 'utf8'));
// The release this home was installed from: install.py copies a bundle's VERSION
// file here. An install from a checkout has none.
const readRelease = async () => (await fs.readFile(path.join(home, 'VERSION'), 'utf8').catch(() => 'development')).trim();
// Where to look for releases: config.releases, else this repository's releases on GitHub, which
// also hold atelier's own; only t3-extension-v<major>.<minor>.<patch> tags are the extension's.
const defaultReleases = 'https://api.github.com/repos/spadaval/atelier/releases?per_page=100';
// The last release lookup or update, by this process or another: the injector and
// `t3-custom update` both write it, and status reports it.
const updatePath = path.join(home, 'update.json');
const readUpdate = async () => JSON.parse(await fs.readFile(updatePath, 'utf8').catch(() => 'null'));
// x.y.z compared as numbers; anything else, such as development or 0.0.0-dev.<commit>, is 0.0.0,
// older than every release.
const versionNumbers = version => version.match(/^(\d+)\.(\d+)\.(\d+)/)?.slice(1).map(Number) ?? [0, 0, 0];
function newer(version, than) {
  const [a, b] = [versionNumbers(version), versionNumbers(than)];
  const differs = a.findIndex((part, index) => part !== b[index]);
  return differs >= 0 && a[differs] > b[differs];
}
// The installed loader: a running injector keeps the one it started with until it is restarted.
const loaderHash = () => fs.readFile(path.join(home, 't3-custom.mjs')).then(text => createHash('sha256').update(text).digest('hex'), () => null);
// GET url and read its body as json, text or arrayBuffer. A failure names the operation and URL,
// including one while the body arrives, after fetch has already resolved on the headers.
async function get(url, operation, as) {
  let response;
  try { response = await fetch(url, { signal: AbortSignal.timeout(30000), headers: { 'user-agent': 't3-custom' } }); } catch (error) { throw new Error(`${operation}: GET ${url}: ${error.cause?.message ?? error.message}`); }
  if (!response.ok) throw new Error(`${operation}: GET ${url}: HTTP ${response.status} ${response.statusText}`);
  try { return await response[as](); } catch (error) { throw new Error(`${operation}: GET ${url}: reading the response: ${error.cause?.message ?? error.message}`); }
}
// The newest release that is not a draft or prerelease and whose tag is the extension's, or null.
async function newestRelease(config) {
  const url = config.releases ?? defaultReleases;
  const releases = await get(url, 'Looking up releases', 'json');
  if (!Array.isArray(releases)) throw new Error(`Looking up releases: GET ${url} did not return a list of releases`);
  let newest = null;
  for (const release of releases) {
    const version = !release.draft && !release.prerelease && release.tag_name?.match(/^t3-extension-v(\d+\.\d+\.\d+)$/)?.[1];
    if (version && (!newest || newer(version, newest.version))) newest = { version, tag: release.tag_name, assets: release.assets ?? [] };
  }
  return newest;
}
// Downloads a release's bundle, refuses it unless its SHA-256 matches the release's .sha256 file,
// and runs the bundle's own install.py on this home with the Python that installed it.
async function install(config, release) {
  const name = `t3-extension-${release.version}.zip`;
  const download = async (file, as) => {
    const asset = release.assets.find(asset => asset.name === file);
    if (!asset) throw new Error(`Downloading ${file}: release ${release.tag} has no asset ${file}`);
    return get(asset.browser_download_url, `Downloading ${file}`, as);
  };
  const zip = Buffer.from(await download(name, 'arrayBuffer'));
  const expected = (await download(`${name}.sha256`, 'text')).trim().split(/\s+/)[0];
  const actual = createHash('sha256').update(zip).digest('hex');
  if (actual !== expected) throw new Error(`Checking ${name}: its SHA-256 is ${actual}, but ${name}.sha256 says ${expected}; nothing was installed`);
  const work = await fs.mkdtemp(path.join(os.tmpdir(), 't3-custom-update-'));
  const python = config.python ?? (process.platform === 'win32' ? 'py' : 'python3');
  const run = async (operation, args) => {
    try { await execute(python, args, { env: appEnvironment({}) }); } catch (error) {
      throw new Error(`${operation}: ${python} ${args.join(' ')} failed (${error.code ?? error.signal}): ${(error.stderr || error.message).trim()}`);
    }
  };
  try {
    await fs.writeFile(path.join(work, name), zip);
    await run(`Unpacking ${name}`, ['-m', 'zipfile', '-e', path.join(work, name), work]);
    await run(`Installing ${release.version}`, [path.join(work, `t3-extension-${release.version}`, 'install.py'), '--home', home]);
  } finally { await fs.rm(work, { recursive: true, force: true }); }
}
// Looks up the newest release and, with apply, installs it when it is newer than this home's.
// With record it writes the outcome to update.json; a failure is in its error and nothing else.
async function update(config, { apply, record }) {
  const result = { checkedAt: new Date().toISOString(), installed: await readRelease(), latest: null, newer: false, error: null };
  try {
    const release = await newestRelease(config);
    result.latest = release && { version: release.version, tag: release.tag };
    result.newer = Boolean(release && newer(release.version, result.installed));
    if (apply && result.newer) {
      const before = await loaderHash();
      await install(config, release);
      Object.assign(result, { installed: await readRelease(), newer: false, installedAt: new Date().toISOString(), restartNeeded: before !== await loaderHash() });
    }
  } catch (error) { result.error = error.message; }
  if (record) await fs.writeFile(updatePath, JSON.stringify(result, null, 2) + '\n');
  return result;
}
// desk.urls lists the Desks; the older form has a single desk.baseUrl.
const deskUrls = config => (config.desk?.urls ?? [config.desk?.baseUrl ?? 'http://127.0.0.1:3780']).map(url => url.replace(/\/$/, ''));
// What the client sees of config.json: the Desks, manual repository overrides and the settings
// the Customizations page edits.
const clientConfig = config => ({ urls: deskUrls(config), repositories: config.desk?.repositories ?? [], settings: config.settings ?? {} });
// The settings the Customizations page may write, checked field by field.
function checkSettings(settings) {
  const fail = message => { throw new Error(`save-settings: ${message}`); };
  const isObject = value => value !== null && typeof value === 'object' && !Array.isArray(value);
  if (!isObject(settings)) fail('settings must be an object');
  const checks = {
    enabled: value => typeof value === 'boolean',
    references: value => typeof value === 'boolean',
    followProject: value => typeof value === 'boolean',
    pinnedRepo: value => value === null || typeof value === 'string',
    autoUpdate: value => typeof value === 'boolean',
    mappings: value => isObject(value) && Object.values(value).every(repo => typeof repo === 'string'),
  };
  const expected = { enabled: 'a boolean', references: 'a boolean', followProject: 'a boolean', autoUpdate: 'a boolean', pinnedRepo: 'a string or null', mappings: 'an object whose values are strings' };
  for (const [key, value] of Object.entries(settings)) {
    if (!Object.hasOwn(checks, key)) fail(`unknown setting ${key}`);
    if (!checks[key](value)) fail(`${key} must be ${expected[key]}, got ${JSON.stringify(value)}`);
  }
  return settings;
}
// A Desk read: only GET, only from a configured Desk, and only of these paths. A path must come out
// of URL parsing unchanged, so no dot segment, backslash or fragment gets through, and its query
// must have exactly the parameters, in the order, of one of its forms, each value of the shape
// below. Nothing else on the Desk is reachable from the page, so its writes are not.
const readablePaths = /^\/api\/(repos|inbox|sidebar|motion|records\/[a-z0-9-]+|repos\/[^/\\?#%]+\/missions|repos\/[^/\\?#%]+\/(items|missions)\/[a-z0-9-]+)$/;
const readableQueries = {
  '/api/launch': [['repo']],
  '/api/history': [['repo', 'days'], ['repo', 'ids']],
  '/api/checks': [['repo', 'pr']],
  '/api/roadmap': [['repo']],
  '/api/thread': [['id'], ['id', 'full']],
  '/api/ties': [['thread']],
};
// A thread identity is T3's (environment and thread, encoded); the Desk looks up what it is given.
const thread = value => value.length > 0 && value.length <= 1024;
const readableValues = { repo: /^[A-Za-z0-9._-]+$/, days: /^[0-9]{1,4}$/, ids: /^[a-z0-9-]+(,[a-z0-9-]+)*$/, pr: /^[0-9]{1,9}$/, full: /^1$/, id: thread, thread };
function deskReadUrl(config, desk, requested) {
  if (!deskUrls(config).includes(desk)) throw new Error(`Desk read is not allowed: ${desk} is not in desk.urls of ${configPath}`);
  if (typeof requested !== 'string') throw new Error(`Desk read is not allowed: the path is ${JSON.stringify(requested)}, not a string`);
  const refuse = reason => { throw new Error(`Desk read is not allowed: ${requested}: ${reason}`); };
  const [pathname, query = null] = requested.split(/\?(.*)/s);
  const forms = readableQueries[pathname];
  if (!forms && !readablePaths.test(pathname)) refuse('not a path the extension reads');
  if (requested.includes('#') || requested.includes('\\')) refuse('it has a fragment or a backslash');
  const url = new URL(desk + requested);
  if (url.pathname !== pathname || url.search !== (query === null ? '' : `?${query}`)) refuse(`it resolves to ${url.href}`);
  const keys = query === null ? [] : [...new URLSearchParams(query).keys()];
  if (!forms) {
    if (query !== null) refuse(`${pathname} takes no query`);
    return url.href;
  }
  if (!forms.some(form => form.join() === keys.join())) refuse(`${pathname} takes ${forms.map(form => form.join(' and ')).join(', or ')}, in that order; it has ${keys.join(', ') || 'none'}`);
  for (const [key, value] of url.searchParams) {
    const check = readableValues[key];
    if (!(typeof check === 'function' ? check(value) : check.test(value))) refuse(`${key}=${JSON.stringify(value)} is not a ${key} the Desk is read with`);
  }
  return url.href;
}
const log = (message, extra = {}) => console.log(JSON.stringify({ time: new Date().toISOString(), message, ...extra }));
async function debugPortOpen(config) {
  try {
    const response = await fetch(`http://127.0.0.1:${config.debugPort}/json/version`, { signal: AbortSignal.timeout(1000) });
    return Boolean((await response.json()).webSocketDebuggerUrl);
  } catch { return false; }
}

function control(action) {
  return new Promise((resolve, reject) => {
    const socket = net.createConnection(controlPath);
    let text = '';
    socket.setTimeout(3000, () => socket.destroy(new Error(`Injector control timed out: ${controlPath}`)));
    socket.on('connect', () => socket.write(`${JSON.stringify({ action })}\n`));
    socket.on('data', chunk => {
      text += chunk;
      if (!text.includes('\n')) return;
      socket.end();
      try { resolve(JSON.parse(text.split('\n')[0])); } catch (error) { reject(error); }
    });
    socket.on('error', reject);
  });
}

class CDP {
  constructor(url, onEvent, onClose) {
    this.pending = new Map();
    this.nextId = 0;
    this.socket = new WebSocket(url);
    this.ready = new Promise((resolve, reject) => {
      this.socket.addEventListener('open', resolve, { once: true });
      this.socket.addEventListener('error', () => reject(new Error(`Could not connect to ${url}`)), { once: true });
    });
    this.socket.addEventListener('message', event => {
      const message = JSON.parse(event.data);
      if (!message.id) { onEvent(message); return; }
      const pending = this.pending.get(message.id);
      if (!pending) return;
      this.pending.delete(message.id);
      clearTimeout(pending.timer);
      message.error ? pending.reject(new Error(`${pending.method}: ${message.error.message}`)) : pending.resolve(message.result);
    });
    this.socket.addEventListener('close', () => {
      for (const pending of this.pending.values()) {
        clearTimeout(pending.timer);
        pending.reject(new Error(`${pending.method}: debugging connection closed`));
      }
      this.pending.clear();
      onClose();
    });
  }
  async call(method, params = {}) {
    await this.ready;
    const id = ++this.nextId;
    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => { this.pending.delete(id); reject(new Error(`${method}: no response after 10 seconds`)); }, 10000);
      this.pending.set(id, { resolve, reject, timer, method });
      this.socket.send(JSON.stringify({ id, method, params }));
    });
  }
  close() { this.socket.close(); }
}

async function buildBundle(config) {
  const jsPath = path.join(home, 'custom.js');
  const cssPath = path.join(home, 'custom.css');
  // custom.js is the extension built from src/ (pnpm build), adapter included: a classic script
  // that defines T3Custom, whose start(api) runs it.
  const [js, css] = await Promise.all([fs.readFile(jsPath, 'utf8'), fs.readFile(cssPath, 'utf8')]);
  const version = createHash('sha256').update(js).update(css).update(JSON.stringify(config)).digest('hex').slice(0, 12);
  // Everything but the settings, to tell a change of settings alone.
  const base = createHash('sha256').update(js).update(css).update(JSON.stringify({ ...config, settings: null })).digest('hex');
  const source = `(() => {
    if (window.top !== window || !${JSON.stringify(config.targetPrefixes)}.some(prefix => location.href.startsWith(prefix))) return;
    globalThis.__t3Custom?.dispose();
    const controller = new AbortController();
    const cleanups = []; const requests = new Map();
    const state = { version: ${JSON.stringify(version)}, appliedAt: new Date().toISOString(), error: null, dispose() {
      controller.abort();
      for (const request of requests.values()) { clearTimeout(request.timer); request.reject(new Error('Customization was reloaded')); }
      requests.clear();
      for (const cleanup of cleanups.reverse()) { try { cleanup(); } catch (error) { console.error('[T3 Custom] Cleanup failed', error); } }
      if (globalThis.__t3Custom === state) delete globalThis.__t3Custom;
    }};
    globalThis.__t3Custom = state;
    state.answer = (id, response) => {
      const request = requests.get(id); if (!request) return;
      requests.delete(id); clearTimeout(request.timer);
      if (response.error) { const error = new Error(response.error.message); Object.assign(error, response.error); request.reject(error); }
      else request.resolve(response.data);
    };
    const report = (type, data = {}) => globalThis.__t3CustomReport(JSON.stringify({ type, version: state.version, ...data }));
    const run = async () => {
      if (controller.signal.aborted) return;
      const style = document.createElement('style');
      style.id = 't3-custom-styles'; style.textContent = ${JSON.stringify(css)};
      document.head.append(style); cleanups.push(() => style.remove());
      const root = document.createElement('div'); root.id = 't3-custom-root';
      document.body.append(root); cleanups.push(() => root.remove());
      const api = { root: root.attachShadow({ mode: 'open' }), signal: controller.signal,
        onCleanup: callback => cleanups.push(callback),
        log: (...args) => console.info('[T3 Custom]', ...args),
        reloadScripts: () => report('reload-request'),
        config: ${JSON.stringify(clientConfig(config))},
        home: ${JSON.stringify(home)},
        request: (action, payload = {}) => new Promise((resolve, reject) => {
          const id = crypto.randomUUID();
          const timer = setTimeout(() => { requests.delete(id); reject(new Error(action + ': no reply from injector after 20 seconds')); }, 20000);
          requests.set(id, { resolve, reject, timer }); report('request', { id, action, payload });
        }),
        reportError: error => { state.error = String(error); report('script-error', { error: state.error }); },
        reportProbes: probes => report('probes', { probes }),
        version: state.version
      };
      try {
        await (async function(api) {\n${js}\nif (typeof T3Custom === 'undefined' || typeof T3Custom.start !== 'function') throw new Error(${JSON.stringify(`${jsPath} defines no T3Custom.start: it is not the extension built from src/ (pnpm build)`)});\nreturn T3Custom.start(api);\n})(api);
        if (!controller.signal.aborted) report('applied');
      } catch (error) { api.reportError(error?.stack ?? error); }
    };
    if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', run, { once: true, signal: controller.signal });
    else void run();
  })();\n//# sourceURL=t3-custom-loader.js`;
  new vm.Script(source, { filename: jsPath });
  return { source, version, base };
}

async function daemon() {
  let config = await loadConfig();
  const targets = new Map();
  const state = { pid: process.pid, startedAt: new Date().toISOString(), release: await readRelease(), port: config.debugPort, phase: 'waiting', version: null, lastAppliedAt: null, lastError: null, probes: null, targets: [], home };
  let bundle;
  let stopping = false;
  let watcher;
  let updateTimer;
  let releaseTimer;
  let discoveryError;
  let reloadQueue = Promise.resolve();
  // config.json as save-settings last wrote it: the page already applied that change itself.
  let savedConfig = null;
  // Saves run one at a time, in the order they arrive, and replace config.json whole (a temporary
  // file renamed over it), so the last save wins and savedConfig is always what the file holds.
  let saveQueue = Promise.resolve();
  function saveSettings(settings) {
    const save = saveQueue.then(async () => {
      const next = await loadConfig();
      next.settings = settings;
      const text = JSON.stringify(next, null, 2) + '\n';
      const temporary = `${configPath}.${process.pid}.tmp`;
      // The replacement keeps config.json's permissions; a new file would get the defaults.
      const { mode } = await fs.stat(configPath);
      await fs.writeFile(temporary, text, { mode });
      await fs.chmod(temporary, mode);
      await fs.rename(temporary, configPath);
      savedConfig = text;
      return { saved: configPath };
    });
    saveQueue = save.catch(() => {});
    return save;
  }

  // Release checks and updates run one at a time. A failure is kept apart from lastError, which
  // launch treats as fatal: the extension keeps running whatever the lookup says.
  let updating = Promise.resolve();
  function checkRelease(apply = config.settings?.autoUpdate === true) {
    const run = updating.then(async () => {
      const result = await update(config, { apply, record: true });
      if (result.error) log(`Release check: ${result.error}`);
      else log(result.installedAt ? `Installed release ${result.installed}` : 'Release checked', { installed: result.installed, latest: result.latest?.version ?? null });
      if (result.installedAt) await reload(true);
      return result;
    });
    updating = run.catch(() => {});
    return run;
  }
  const startedLoader = await loaderHash();
  const fullState = async () => ({ ...state, update: await readUpdate(), restartNeeded: (await loaderHash()) !== startedLoader });

  function problem(operation, error, extra = {}) {
    state.lastError = `${operation}: ${error.message ?? error}`;
    log(state.lastError, extra);
  }
  // The script a reloaded T3 document runs.
  async function register(target) {
    if (target.scriptId) await target.cdp.call('Page.removeScriptToEvaluateOnNewDocument', { identifier: target.scriptId });
    const result = await target.cdp.call('Page.addScriptToEvaluateOnNewDocument', { source: bundle.source });
    target.scriptId = result.identifier;
  }
  async function apply(target) {
    await register(target);
    const evaluated = await target.cdp.call('Runtime.evaluate', { expression: bundle.source, returnByValue: true });
    if (evaluated.exceptionDetails) throw new Error(evaluated.exceptionDetails.exception?.description ?? evaluated.exceptionDetails.text);
  }
  function reload(force = false) {
    reloadQueue = reloadQueue.then(() => reloadFiles(force));
    return reloadQueue;
  }
  async function reloadFiles(force) {
    try {
      const raw = await fs.readFile(configPath, 'utf8');
      config = JSON.parse(raw);
      const next = await buildBundle(config);
      if (!force && bundle?.version === next.version) return;
      // A save from the Customizations section changed only settings the page has already
      // applied, so the running page is left alone; only a reloaded document needs the new bundle.
      const inPlace = !force && raw === savedConfig && bundle?.base === next.base;
      bundle = next;
      state.version = next.version;
      state.release = await readRelease();
      state.lastError = null;
      for (const target of targets.values()) {
        if (target.ready) await (inPlace ? register(target) : apply(target));
      }
      log(inPlace ? 'Settings saved; applied in place' : 'Customization files loaded', { version: next.version });
    } catch (error) { problem('Loading custom.js/custom.css', error); }
  }
  async function request(target, report, executionContextId) {
    let response;
    try {
      let data;
      if (report.action === 'status') data = { ...(await fullState()), home, targets: targets.size };
      else if (report.action === 'update') data = await checkRelease(true);
      else if (report.action === 'open-folder') {
        await platform.openFolder(home); data = { opened: true };
      } else if (report.action === 'desk-get') {
        // Only reads, only from a configured Desk, only of the paths deskReadUrl admits.
        const url = deskReadUrl(config, report.payload.desk, report.payload.path);
        const result = await fetch(url, { signal: AbortSignal.timeout(15000), headers: { accept: 'application/json' }, redirect: 'error' });
        const body = await result.json();
        if (!result.ok) { const error = new Error(`GET ${url}: HTTP ${result.status}: ${body.error ?? result.statusText}`); error.status = result.status; throw error; }
        data = body;
      } else if (report.action === 'save-settings') {
        data = await saveSettings(checkSettings(report.payload));
      } else throw new Error(`Unsupported injector request: ${report.action}`);
      response = { data };
    } catch (error) { response = { error: { message: error.message, status: error.status ?? null } }; }
    await target.cdp.call('Runtime.evaluate', { expression: `globalThis.__t3Custom?.answer(${JSON.stringify(report.id)}, ${JSON.stringify(response)})`, contextId: executionContextId }).catch(error => log('Request recipient disappeared', { action: report.action, error: error.message }));
  }
  async function attach(info) {
    // The binding exists in every context of the page, so a report counts only from the main world
    // of the top frame (whose frame ID is the page target's ID) while it shows T3, where the extension runs.
    const target = { id: info.id, url: info.url, ready: false, scriptId: null, topContexts: new Set() };
    targets.set(info.id, target);
    target.cdp = new CDP(info.webSocketDebuggerUrl, message => {
      if (message.method === 'Runtime.executionContextCreated') {
        const context = message.params.context;
        // Only while the top frame shows T3: after a navigation elsewhere, that page's context does not count.
        const showsTarget = config.targetPrefixes.some(prefix => prefix === context.origin || prefix.startsWith(context.origin + '/'));
        if (context.auxData?.frameId === info.id && context.auxData?.isDefault && showsTarget) target.topContexts.add(context.id);
        return;
      }
      if (message.method === 'Runtime.executionContextDestroyed') { target.topContexts.delete(message.params.executionContextId); return; }
      if (message.method === 'Runtime.executionContextsCleared') { target.topContexts.clear(); return; }
      if (message.method !== 'Runtime.bindingCalled' || message.params.name !== '__t3CustomReport') return;
      if (!target.topContexts.has(message.params.executionContextId)) {
        log('Ignored a report from outside the top frame', { target: info.id, executionContextId: message.params.executionContextId });
        return;
      }
      let report;
      try { report = JSON.parse(message.params.payload); } catch { return; }
      if (report.type === 'applied') {
        state.lastAppliedAt = new Date().toISOString();
        state.lastError = null;
        state.phase = 'connected';
        log('Customizations applied', { target: info.id, version: report.version });
      } else if (report.type === 'probes') {
        state.probes = report.probes;
        const missing = Object.entries(report.probes).filter(([, probe]) => probe.ok === false).map(([name]) => name);
        if (missing.length) log('Missing T3 hooks', { target: info.id, missing });
      } else if (report.type === 'script-error') problem('custom.js', report.error, { target: info.id });
      else if (report.type === 'reload-request') void reload(true);
      else if (report.type === 'request') void request(target, report, message.params.executionContextId);
    }, () => {
      targets.delete(info.id);
      log('Renderer disconnected', { target: info.id });
    });
    try {
      await target.cdp.call('Page.enable');
      await target.cdp.call('Runtime.enable');
      await target.cdp.call('Runtime.addBinding', { name: '__t3CustomReport' });
      target.ready = true;
      if (bundle) await apply(target);
      log('Attached to T3 renderer', { target: info.id, url: info.url });
    } catch (error) { problem('Attaching renderer', error, { target: info.id }); target.cdp.close(); }
  }
  async function stop() {
    if (stopping) return;
    stopping = true;
    watcher?.close(); clearTimeout(updateTimer); clearInterval(releaseTimer);
    const cleanupResults = await Promise.allSettled([...targets.values()].map(async target => {
      try {
        if (target.scriptId) await target.cdp.call('Page.removeScriptToEvaluateOnNewDocument', { identifier: target.scriptId });
        await target.cdp.call('Runtime.evaluate', { expression: 'globalThis.__t3Custom?.dispose()' });
        await target.cdp.call('Runtime.removeBinding', { name: '__t3CustomReport' });
      } finally { target.cdp.close(); }
    }));
    for (const result of cleanupResults) if (result.status === 'rejected') problem('Removing customizations', result.reason);
    await new Promise(resolve => server.close(resolve));
    log('Injector stopped', { cleanupComplete: cleanupResults.every(result => result.status === 'fulfilled') });
  }
  const server = net.createServer(socket => {
    let input = '';
    socket.setTimeout(3000, () => socket.destroy());
    socket.on('data', async chunk => {
      input += chunk;
      if (!input.includes('\n')) return;
      socket.removeAllListeners('data');
      try {
        const { action } = JSON.parse(input.split('\n')[0]);
        if (action === 'reload') await reload(true);
        else if (action !== 'status' && action !== 'stop') throw new Error(`Unknown control action: ${action}`);
        socket.end(`${JSON.stringify({ ...(await fullState()), targets: [...targets.values()].map(({ id, url }) => ({ id, url })), stopping: action === 'stop' })}\n`);
        if (action === 'stop') void stop();
      } catch (error) { socket.end(`${JSON.stringify({ error: error.message })}\n`); }
    });
    socket.on('error', error => log('Control client disconnected', { error: error.message }));
  });
  await new Promise((resolve, reject) => { server.once('error', reject); server.listen(controlPath, resolve); });
  if (process.platform !== 'win32') await fs.chmod(controlPath, 0o600);
  process.once('SIGTERM', () => void stop());
  process.once('SIGINT', () => void stop());
  await reload();
  watcher = watch(home, (_event, filename) => {
    if (!['custom.js', 'custom.css', 'config.json'].includes(String(filename))) return;
    clearTimeout(updateTimer); updateTimer = setTimeout(() => void reload(), 150);
  });
  log('Injector ready', { port: config.debugPort, targets: config.targetPrefixes });
  // A release check now and once a day; it installs the release only when settings.autoUpdate says so.
  void checkRelease();
  releaseTimer = setInterval(() => void checkRelease(), 24 * 60 * 60 * 1000);
  while (!stopping) {
    try {
      const response = await fetch(`http://127.0.0.1:${config.debugPort}/json/list`, { signal: AbortSignal.timeout(2000) });
      if (!response.ok) throw new Error(`HTTP ${response.status}`);
      const pages = await response.json();
      if (!Array.isArray(pages)) throw new Error('Endpoint did not return a CDP target list');
      discoveryError = null;
      for (const info of pages) {
        if (info.type === 'page' && config.targetPrefixes.some(prefix => info.url.startsWith(prefix)) && !targets.has(info.id)) await attach(info);
      }
      if (!targets.size) state.phase = 'waiting-for-t3-page';
    } catch (error) {
      state.phase = 'waiting-for-debug-port';
      const message = `Waiting for T3 debugging at 127.0.0.1:${config.debugPort}: ${error.cause?.message ?? error.message}`;
      if (message !== discoveryError) { log(message); discoveryError = message; }
    }
    if (!stopping) await delay(1000);
  }
}

async function startDaemon() {
  try { return await control('status'); } catch (error) {
    if (!['ENOENT', 'ECONNREFUSED'].includes(error.code)) throw error;
  }
  if (process.platform !== 'win32') await fs.rm(controlPath, { force: true });
  const logHandle = await fs.open(path.join(home, 'injector.log'), 'a', 0o600);
  const child = spawn(process.execPath, [entry, 'daemon', '--home', home], { detached: true, windowsHide: true, stdio: ['ignore', logHandle.fd, logHandle.fd] });
  child.unref(); await logHandle.close();
  for (let attempt = 0; attempt < 40; attempt++) {
    await delay(100);
    try { return await control('status'); } catch (error) { if (!['ENOENT', 'ECONNREFUSED'].includes(error.code)) throw error; }
  }
  throw new Error(`Injector failed to start. Read ${path.join(home, 'injector.log')}`);
}

async function launch() {
  const config = await loadConfig();
  await startDaemon();
  const debugAvailable = await debugPortOpen(config);
  if (debugAvailable) await control('reload');
  const notLaunched = await platform.launch(config, ['--remote-debugging-address=127.0.0.1', `--remote-debugging-port=${config.debugPort}`]);
  if (notLaunched && !debugAvailable) {
    const message = `T3 Code is already running without its debugging port (${notLaunched}). Quit T3 Code completely, then open T3 Custom again. Your current session has been left running.`;
    log(message);
    await platform.dialog('T3 Custom', message);
    return;
  }
  for (let attempt = 0; attempt < 60; attempt++) {
    const status = await control('status');
    if (status.lastError) throw new Error(status.lastError);
    if (status.targets.length && status.lastAppliedAt) return;
    await delay(500);
  }
  throw new Error(`T3 did not expose its renderer on port ${config.debugPort} within 30 seconds. Read ${path.join(home, 'injector.log')}`);
}

try {
  if (command === 'daemon') await daemon();
  else if (command === 'launch') await launch();
  else if (command === 'prepare') console.log(JSON.stringify(await startDaemon(), null, 2));
  else if (['status', 'stop', 'reload'].includes(command)) console.log(JSON.stringify(await control(command), null, 2));
  else if (command === 'update') {
    // update installs the newest release if it is newer; update --check only reports it.
    const apply = !process.argv.includes('--check');
    const result = await update(await loadConfig(), { apply, record: apply });
    if (result.installedAt) {
      // A running injector reloads the new files; one that is not running loads them when it starts.
      try { await control('reload'); result.reloaded = true; } catch (error) {
        if (!['ENOENT', 'ECONNREFUSED'].includes(error.code)) throw error;
        result.reloaded = false;
      }
    }
    console.log(JSON.stringify(result, null, 2));
    if (result.error) process.exitCode = 1;
  } else if (command === 'check') console.log(JSON.stringify({ release: await readRelease(), version: (await buildBundle(await loadConfig())).version, valid: true }));
  else throw new Error(`Unknown command ${command}. Use launch, prepare, status, reload, stop, update, or check.`);
} catch (error) {
  console.error(`T3 Custom ${command} failed: ${error.stack ?? error}`);
  if (command === 'launch') {
    await platform.dialog('T3 Custom failed', error.message).catch(() => {});
  }
  process.exitCode = 1;
}
