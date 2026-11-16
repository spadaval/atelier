// A Desk serving a repository the extension's tests own: its records are written here, with fixed
// IDs, titles and statuses, so no expectation depends on a live repository.
//
// As a module, `startFixtureDesk()` builds the repository in a temporary directory, starts
// `atelier-desk` on a free port and returns its URL. Run directly, it does the same and serves until
// interrupted, printing the URL to give the tests as DESK:
//
//   node fixture-desk.mjs            # then, elsewhere: DESK=<printed URL> node test-extension.mjs
//
// The Desk is `ATELIER_DESK`, else `target/debug/atelier-desk` of this checkout; its app is
// `DESK_APP`, else `desk/dist`. Build them with `cargo build -p atelier-desk` and
// `pnpm --dir desk build`.
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { spawn, execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const checkout = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

// The repository is served as `atelier`, the name the extension uses as its reference prefix, and
// its origin gives the identity T3's fixture project carries. The origin is never fetched: the Desk
// fetches nothing with `fetch_seconds = 0`, and the repository names no forge.
export const fixture = {
  repo: 'atelier',
  identity: 'example.invalid/fixture/atelier',
  origin: 'https://example.invalid/fixture/atelier.git',
  // Open, with one item started and threads T3 lists: GET /api/sidebar lists it first.
  mission: { id: 'atelier-fm01', title: 'Fixture mission' },
  // Open, with one needs-human item and no threads: the sidebar's "1 more mission".
  quietMission: { id: 'atelier-fm03', title: 'Quiet mission' },
  // Done: resolved by GET /api/records but not in the sidebar.
  closedMission: { id: 'atelier-fm02', title: 'Closed fixture mission' },
  items: {
    started: { id: 'atelier-fi01', title: 'First item' },
    waiting: { id: 'atelier-fi02', title: 'Second item' },
    done: { id: 'atelier-fi03', title: 'Third item' },
    quiet: { id: 'atelier-fi04', title: 'Quiet item' },
    // In no mission; the test's conversations name it.
    reference: { id: 'atelier-rf01', title: 'Fixture reference item' },
  },
  // Thread identities, as T3's fixture names its threads.
  threads: { lead: 'thread-b', reviewer: 'remote-thread-0123456789', implementer: 'thread-a' },
};

const created = '2026-11-01T09:00:00Z';

function mission({ id, title }, status, lead) {
  return `---\nschema: atelier.mission\nschema_version: 3\nid: ${id}\ntitle: ${title}\nstatus: ${status}\nlabels: []\nlead: ${lead ?? 'null'}\nplanners: []\ncreated_at: ${created}\n---\n\n## Why\n\nThe extension's tests need one.\n\n## Outcome\n\nThey pass.\n`;
}

function item({ id, title }, { status, mission = 'null', labels = '[]', implementers = '[]' }) {
  return `---\nschema: atelier.item\nschema_version: 3\nid: ${id}\ntitle: ${title}\nstatus: ${status}\nmission: ${mission}\nlabels: ${labels}\npriority: medium\nblocked_by: []\nimplementers: ${implementers}\nready: null\ntrivial: null\ncaused_by: null\ncreated_at: ${created}\n---\n\n## Goal\n\nBe ${title.toLowerCase()}.\n\n## Done when\n\n- [ ] It is there.\n`;
}

function review(reviewer) {
  return `---\nschema: atelier.review\nschema_version: 3\nround: 1\ncommit: 0000000000000000000000000000000000000000\nreviewers:\n- ${reviewer}\nmodel: fixture\nverdict: approve\ncreated_at: ${created}\n---\n\n## Summary\n\nThe fixture plan.\n`;
}

async function write(root, file, text) {
  await fs.mkdir(path.dirname(path.join(root, file)), { recursive: true });
  await fs.writeFile(path.join(root, file), text);
}

// The repository at `root`: on master, the mission open with its three items (one ready, one ready
// and needs-human, one done), the quiet mission open with one needs-human item, the closed mission
// and the reference item; on branch atelier-fi01, its item building, held by thread-a.
export async function writeFixtureRepository(root) {
  const { mission: open, quietMission, closedMission, items, threads } = fixture;
  const git = (...args) => execFileSync('git', ['-c', 'user.name=fixture', '-c', 'user.email=fixture@example.invalid', ...args], { cwd: root, stdio: 'pipe' });
  await fs.mkdir(root, { recursive: true });
  git('init', '-q', '-b', 'master');
  git('remote', 'add', 'origin', fixture.origin);
  await write(root, '.atelier/config.toml', 'schema_version = 3\nslug = "atelier"\nbase_branch = "master"\n');
  await write(root, `.atelier/missions/${open.id}/mission.md`, mission(open, 'open', threads.lead));
  await write(root, `.atelier/missions/${open.id}/reviews/1-fx01.md`, review(threads.reviewer));
  await write(root, `.atelier/missions/${quietMission.id}/mission.md`, mission(quietMission, 'open'));
  await write(root, `.atelier/missions/${closedMission.id}/mission.md`, mission(closedMission, 'done'));
  await write(root, `.atelier/items/${items.started.id}/item.md`, item(items.started, { status: 'ready', mission: open.id }));
  await write(root, `.atelier/items/${items.waiting.id}/item.md`, item(items.waiting, { status: 'ready', mission: open.id, labels: '[needs-human]' }));
  await write(root, `.atelier/items/${items.done.id}/item.md`, item(items.done, { status: 'done', mission: open.id }));
  await write(root, `.atelier/items/${items.quiet.id}/item.md`, item(items.quiet, { status: 'ready', mission: quietMission.id, labels: '[needs-human]' }));
  await write(root, `.atelier/items/${items.reference.id}/item.md`, item(items.reference, { status: 'ready' }));
  git('add', '-A');
  git('commit', '-qm', 'Fixture records');
  git('switch', '-qc', items.started.id);
  await write(root, `.atelier/items/${items.started.id}/item.md`, item(items.started, { status: 'building', mission: open.id, implementers: `[${threads.implementer}]` }));
  git('commit', '-qam', `Start ${items.started.id}`);
  git('switch', '-q', 'master');
}

// Builds the fixture repository under a new temporary directory and serves it with atelier-desk on
// a free loopback port. `stop()` ends the Desk and removes the directory; `log()` is its output.
export async function startFixtureDesk() {
  const binary = process.env.ATELIER_DESK ?? path.join(checkout, 'target', 'debug', process.platform === 'win32' ? 'atelier-desk.exe' : 'atelier-desk');
  const app = process.env.DESK_APP ?? path.join(checkout, 'desk', 'dist');
  for (const [what, file, how] of [['atelier-desk', binary, 'cargo build -p atelier-desk, or set ATELIER_DESK'], ["the Desk's app", path.join(app, 'index.html'), 'pnpm --dir desk install && pnpm --dir desk build, or set DESK_APP']]) {
    await fs.access(file).catch(() => { throw new Error(`No ${what} at ${file}: run ${how}.`); });
  }
  const temp = await fs.mkdtemp(path.join(os.tmpdir(), 'atelier-fixture-desk-'));
  await writeFixtureRepository(path.join(temp, 'repos', fixture.repo));
  const config = path.join(temp, 'desk.toml');
  const toml = value => JSON.stringify(value);
  await fs.writeFile(config, [`caller = "fixture"`, `roots = [${toml(path.join(temp, 'repos'))}]`, `state = ${toml(path.join(temp, 'state'))}`, `app = ${toml(app)}`, 'fetch_seconds = 0', `listen = ["127.0.0.1:0"]`, ''].join('\n'));
  const desk = spawn(binary, ['--config', config], { stdio: ['ignore', 'pipe', 'pipe'] });
  let output = '';
  const url = await new Promise((resolve, reject) => {
    const read = chunk => {
      output += chunk;
      const serving = output.match(/serving on (http:\/\/\S+)/);
      if (serving) resolve(serving[1]);
    };
    desk.stdout.on('data', read); desk.stderr.on('data', read);
    desk.on('error', reject);
    desk.on('exit', code => reject(new Error(`atelier-desk exited with ${code} before serving:\n${output}`)));
  });
  return {
    url,
    log: () => output,
    async stop() {
      if (desk.exitCode === null) { desk.kill(); await new Promise(resolve => desk.once('exit', resolve)); }
      await fs.rm(temp, { recursive: true, force: true });
    },
  };
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const desk = await startFixtureDesk();
  console.log(`The fixture Desk serves ${fixture.repo} at ${desk.url}. Interrupt to stop it.`);
  const stop = async () => { await desk.stop(); process.exit(0); };
  process.on('SIGINT', stop); process.on('SIGTERM', stop);
}
