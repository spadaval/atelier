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
const homeIndex = process.argv.indexOf('--home');
const home = homeIndex < 0 ? path.join(os.homedir(), 'Library/Application Support/T3 Custom') : path.resolve(process.argv[homeIndex + 1]);
const command = process.argv[2] ?? 'launch';
const socketPath = path.join(home, 'injector.sock');
const cleanEnvironment = () => {
  const env = { ...process.env };
  delete env.ELECTRON_RUN_AS_NODE;
  delete env.NODE_OPTIONS;
  return env;
};
const loadConfig = async () => JSON.parse(await fs.readFile(path.join(home, 'config.json'), 'utf8'));
const log = (message, extra = {}) => console.log(JSON.stringify({ time: new Date().toISOString(), message, ...extra }));

function control(action) {
  return new Promise((resolve, reject) => {
    const socket = net.createConnection(socketPath);
    let text = '';
    socket.setTimeout(3000, () => socket.destroy(new Error(`Injector control timed out: ${socketPath}`)));
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
  const [js, css, adapter] = await Promise.all([fs.readFile(jsPath, 'utf8'), fs.readFile(cssPath, 'utf8'), fs.readFile(path.join(home, 'host-adapter.js'), 'utf8')]);
  const version = createHash('sha256').update(js).update(css).update(adapter).update(JSON.stringify(config)).digest('hex').slice(0, 12);
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
        config: ${JSON.stringify(config.desk ?? {})},
        home: ${JSON.stringify(home)},
        request: (action, payload = {}) => new Promise((resolve, reject) => {
          const id = crypto.randomUUID();
          const timer = setTimeout(() => { requests.delete(id); reject(new Error(action + ': no reply from injector after 20 seconds')); }, 20000);
          requests.set(id, { resolve, reject, timer }); report('request', { id, action, payload });
        }),
        reportError: error => { state.error = String(error); report('script-error', { error: state.error }); },
        version: state.version
      };
      try {
        await (async function(api) {\n${adapter}\n${js}\n})(api);
        if (!controller.signal.aborted) report('applied');
      } catch (error) { api.reportError(error?.stack ?? error); }
    };
    if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', run, { once: true, signal: controller.signal });
    else void run();
  })();\n//# sourceURL=t3-custom-loader.js`;
  new vm.Script(source, { filename: jsPath });
  return { source, version };
}

async function daemon() {
  let config = await loadConfig();
  const targets = new Map();
  const state = { pid: process.pid, startedAt: new Date().toISOString(), port: config.debugPort, phase: 'waiting', version: null, lastAppliedAt: null, lastError: null, targets: [], home };
  let bundle;
  let stopping = false;
  let watcher;
  let updateTimer;
  let discoveryError;
  let reloadQueue = Promise.resolve();

  function problem(operation, error, extra = {}) {
    state.lastError = `${operation}: ${error.message ?? error}`;
    log(state.lastError, extra);
  }
  async function apply(target, refreshDocument = false) {
    if (target.scriptId) await target.cdp.call('Page.removeScriptToEvaluateOnNewDocument', { identifier: target.scriptId });
    const result = await target.cdp.call('Page.addScriptToEvaluateOnNewDocument', { source: bundle.source });
    target.scriptId = result.identifier;
    // Chromium chooses CSP enforcement when it initializes a document.
    if (refreshDocument) { await target.cdp.call('Page.reload'); return; }
    const evaluated = await target.cdp.call('Runtime.evaluate', { expression: bundle.source, returnByValue: true });
    if (evaluated.exceptionDetails) throw new Error(evaluated.exceptionDetails.exception?.description ?? evaluated.exceptionDetails.text);
  }
  function reload(force = false) {
    reloadQueue = reloadQueue.then(() => reloadFiles(force));
    return reloadQueue;
  }
  async function reloadFiles(force) {
    try {
      config = await loadConfig();
      const next = await buildBundle(config);
      if (!force && bundle?.version === next.version) return;
      bundle = next;
      state.version = next.version;
      state.lastError = null;
      for (const target of targets.values()) {
        if (target.ready) await apply(target);
      }
      log('Customization files loaded', { version: next.version });
    } catch (error) { problem('Loading custom.js/custom.css', error); }
  }
  async function request(target, report, executionContextId) {
    let response;
    try {
      let data;
      if (report.action === 'status') data = { ...state, home, targets: targets.size };
      else if (report.action === 'open-folder') {
        await execute('/usr/bin/open', [home], { env: cleanEnvironment() }); data = { opened: true };
      } else if (report.action === 'desk-get') {
        const requested = report.payload.path;
        const match = /^\/api\/repos(?:\/([^/]+)\/(?:items|missions)\/([a-z0-9-]+))?$/.exec(requested);
        if (!match || (match[1] && !config.desk.repositories.some(repo => repo.repo === decodeURIComponent(match[1])))) throw new Error(`Desk read is not allowed: ${requested}`);
        const url = config.desk.baseUrl.replace(/\/$/, '') + requested;
        const result = await fetch(url, { signal: AbortSignal.timeout(15000), headers: { accept: 'application/json' }, redirect: 'error' });
        const body = await result.json();
        if (!result.ok) { const error = new Error(`GET ${url}: HTTP ${result.status}: ${body.error ?? result.statusText}`); error.status = result.status; throw error; }
        data = body;
      } else throw new Error(`Unsupported injector request: ${report.action}`);
      response = { data };
    } catch (error) { response = { error: { message: error.message, status: error.status ?? null } }; }
    await target.cdp.call('Runtime.evaluate', { expression: `globalThis.__t3Custom?.answer(${JSON.stringify(report.id)}, ${JSON.stringify(response)})`, contextId: executionContextId }).catch(error => log('Request recipient disappeared', { action: report.action, error: error.message }));
  }
  async function attach(info) {
    const target = { id: info.id, url: info.url, ready: false, scriptId: null };
    targets.set(info.id, target);
    target.cdp = new CDP(info.webSocketDebuggerUrl, message => {
      if (message.method !== 'Runtime.bindingCalled' || message.params.name !== '__t3CustomReport') return;
      let report;
      try { report = JSON.parse(message.params.payload); } catch { return; }
      if (report.type === 'applied') {
        state.lastAppliedAt = new Date().toISOString();
        state.lastError = null;
        state.phase = 'connected';
        log('Customizations applied', { target: info.id, version: report.version });
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
      if (config.bypassCSP) await target.cdp.call('Page.setBypassCSP', { enabled: true });
      target.ready = true;
      if (bundle) await apply(target, config.bypassCSP);
      log('Attached to T3 renderer', { target: info.id, url: info.url, bypassCSP: config.bypassCSP });
    } catch (error) { problem('Attaching renderer', error, { target: info.id }); target.cdp.close(); }
  }
  async function stop() {
    if (stopping) return;
    stopping = true;
    watcher?.close(); clearTimeout(updateTimer);
    const cleanupResults = await Promise.allSettled([...targets.values()].map(async target => {
      try {
        if (target.scriptId) await target.cdp.call('Page.removeScriptToEvaluateOnNewDocument', { identifier: target.scriptId });
        await target.cdp.call('Runtime.evaluate', { expression: 'globalThis.__t3Custom?.dispose()' });
        if (config.bypassCSP) {
          await target.cdp.call('Page.setBypassCSP', { enabled: false });
          await target.cdp.call('Page.reload');
        }
        await target.cdp.call('Runtime.removeBinding', { name: '__t3CustomReport' });
      } finally { target.cdp.close(); }
    }));
    for (const result of cleanupResults) if (result.status === 'rejected') problem('Removing customizations', result.reason);
    await new Promise(resolve => server.close(resolve));
    await fs.rm(socketPath, { force: true });
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
        socket.end(`${JSON.stringify({ ...state, targets: [...targets.values()].map(({ id, url }) => ({ id, url })), stopping: action === 'stop' })}\n`);
        if (action === 'stop') void stop();
      } catch (error) { socket.end(`${JSON.stringify({ error: error.message })}\n`); }
    });
    socket.on('error', error => log('Control client disconnected', { error: error.message }));
  });
  await new Promise((resolve, reject) => { server.once('error', reject); server.listen(socketPath, resolve); });
  await fs.chmod(socketPath, 0o600);
  process.once('SIGTERM', () => void stop());
  process.once('SIGINT', () => void stop());
  await reload();
  watcher = watch(home, (_event, filename) => {
    if (!['custom.js', 'custom.css', 'host-adapter.js', 'config.json'].includes(String(filename))) return;
    clearTimeout(updateTimer); updateTimer = setTimeout(() => void reload(), 150);
  });
  log('Injector ready', { port: config.debugPort, targets: config.targetPrefixes });
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
  await fs.rm(socketPath, { force: true });
  const logHandle = await fs.open(path.join(home, 'injector.log'), 'a', 0o600);
  const child = spawn(process.execPath, [entry, 'daemon', '--home', home], { detached: true, stdio: ['ignore', logHandle.fd, logHandle.fd] });
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
  let debugAvailable = false;
  try {
    const response = await fetch(`http://127.0.0.1:${config.debugPort}/json/version`, { signal: AbortSignal.timeout(1000) });
    debugAvailable = Boolean((await response.json()).webSocketDebuggerUrl);
  } catch {}
  const executable = path.join(config.appPath, 'Contents/MacOS', path.basename(config.appPath, '.app'));
  const { stdout } = await execute('/bin/ps', ['-axo', 'args=']);
  const appRunning = stdout.split('\n').some(line => line.trim() === executable || line.trim().startsWith(`${executable} --remote-debugging-`) || line.trim().startsWith(`${executable} -psn_`));
  if (!debugAvailable && appRunning) {
    const message = 'T3 Code is already running without its debugging port. Quit T3 Code completely (Command-Q), then open T3 Custom. Your current session has been left running.';
    await execute('/usr/bin/osascript', ['-e', `display dialog ${JSON.stringify(message)} with title "T3 Custom" buttons {"OK"} default button "OK"`]);
    return;
  }
  if (debugAvailable) await control('reload');
  await execute('/usr/bin/open', ['-a', config.appPath, '--args', '--remote-debugging-address=127.0.0.1', `--remote-debugging-port=${config.debugPort}`], { env: cleanEnvironment() });
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
  else if (command === 'check') console.log(JSON.stringify({ version: (await buildBundle(await loadConfig())).version, valid: true }));
  else throw new Error(`Unknown command ${command}. Use launch, prepare, status, reload, stop, or check.`);
} catch (error) {
  console.error(`T3 Custom ${command} failed: ${error.stack ?? error}`);
  if (command === 'launch') {
    await execute('/usr/bin/osascript', ['-e', `display dialog ${JSON.stringify(error.message)} with title "T3 Custom failed" buttons {"OK"} default button "OK" with icon stop`]).catch(() => {});
  }
  process.exitCode = 1;
}
