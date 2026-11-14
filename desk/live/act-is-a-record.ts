// act-is-a-record: the Desk's stop of a proposed plan leaves the record
// `atelier drop` leaves, and publishes it, as one command.
//
//   pnpm act-is-a-record <repository> [--by <name>] [--atelier <binary>]
//       [--desk <atelier-desk binary>] [--app <built app>] [--differ] [--keep]
//
// It works only on copies under a new scratch directory, which it removes
// at the end (unless --keep). It makes a bare copy of <repository>'s base
// branch to stand for the forge's remote, proposes a mission there on its
// own branch, and clones the remote twice. It serves the first with
// atelier-desk and the built app, on a loopback port of its own, with no T3
// Code and with a `gh` stand-in, and stops the plan from the mission's page
// in a headless browser; in the second it runs `atelier drop <mission>
// --reason <reason>` on the plan's branch and commits it. It checks that
// the Desk pushed exactly one commit to the plan's branch and closed its
// pull request, compares that commit with the CLI's by message, changed
// paths and files, and checks that the page, loaded again, shows the plan
// stopped. It prints what it compared and exits 1 on any difference.
//
// <repository> must name a forge in its `.atelier/config.toml`, since a
// plan's stop is published by a push. --by names who stops it (default
// sumeet): the Desk's caller, and the CLI's thread. --differ gives the
// CLI's drop another reason, so the two must differ and the command must
// fail: it shows the comparison can fail. The binaries default to this
// checkout's builds (target/debug/atelier, target/release/atelier-desk or
// else debug) and the app to desk/dist.

import { chromium } from "@playwright/test";
import { execFileSync, spawn, type ChildProcess } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

const usage =
  "usage: pnpm act-is-a-record <repository> [--by <name>] [--atelier <binary>] [--desk <atelier-desk binary>] [--app <built app>] [--differ] [--keep]";
const REASON = "Stopped by act-is-a-record, in a scratch copy";
const IDENTITIES = ["T3CODE_THREAD_ID", "CLAUDE_CODE_SESSION_ID", "CODEX_SESSION_ID", "CODEX_THREAD_ID", "GROK_SESSION_ID"];

function fail(message: string): never {
  console.error(message);
  process.exit(2);
}

function parseArgs(argv: string[]) {
  const top = path.resolve(import.meta.dirname, "../..");
  const built = (...candidates: string[]) => candidates.map((c) => path.join(top, c)).find(existsSync);
  const o = {
    clone: "",
    by: "sumeet",
    atelier: built("target/debug/atelier"),
    desk: built("target/release/atelier-desk", "target/debug/atelier-desk"),
    app: path.join(top, "desk/dist"),
    differ: false,
    keep: false,
  };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    const value = () => argv[++i] ?? fail(usage);
    if (a === "--by") o.by = value();
    else if (a === "--atelier") o.atelier = value();
    else if (a === "--desk") o.desk = value();
    else if (a === "--app") o.app = value();
    else if (a === "--differ") o.differ = true;
    else if (a === "--keep") o.keep = true;
    else if (!o.clone && !a.startsWith("--")) o.clone = a;
    else fail(usage);
  }
  if (!o.clone) fail(usage);
  if (!o.atelier || !existsSync(o.atelier)) fail("No new atelier binary: build it, or give --atelier <binary>");
  if (!o.desk || !existsSync(o.desk)) fail("No atelier-desk binary: build it, or give --desk <binary>");
  if (!existsSync(path.join(o.app, "index.html"))) fail(`No built app at ${o.app}: run pnpm build, or give --app <dir>`);
  return { ...o, clone: path.resolve(o.clone), atelier: path.resolve(o.atelier), desk: path.resolve(o.desk), app: path.resolve(o.app) };
}

/** The environment a command runs in: no harness's identity, and `thread` as T3 Code's thread when given. */
function environment(thread?: string): NodeJS.ProcessEnv {
  const env = { ...process.env };
  for (const name of IDENTITIES) delete env[name];
  if (thread) env.T3CODE_THREAD_ID = thread;
  return env;
}

function run(cwd: string, command: string, args: string[], thread?: string): string {
  return execFileSync(command, args, { cwd, env: environment(thread), encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] }).trim();
}

const git = (cwd: string, ...args: string[]) => run(cwd, "git", args);

/** The `gh` stand-in: one pull request for `branch`, open until `pr close`, which it records in `log`. */
function ghStandIn(dir: string, branch: string): { gh: string; log: string } {
  const gh = path.join(dir, "gh");
  const state = path.join(dir, "pr-state");
  const log = path.join(dir, "gh.log");
  writeFileSync(state, "OPEN");
  writeFileSync(
    gh,
    `#!/bin/sh\ncase "$1 $2" in\n'pr list') printf '[{"number":1,"url":"https://forge.invalid/pull/1","state":"%s","headRefName":"${branch}"}]' "$(cat ${state})" ;;\n'pr close') printf '%s\\n' "$*" >> ${log}; printf CLOSED > ${state} ;;\n*) exit 1 ;;\nesac\n`,
    { mode: 0o755 },
  );
  return { gh, log };
}

/** A proposed mission's record, open, with every section of a plan written. */
function missionRecord(id: string): string {
  const sections = ["Why", "Outcome", "Success criteria", "Scope", "Authority"].map((h) => `## ${h}\n\nWritten by act-is-a-record, in a scratch copy, as Sumeet asked.\n`).join("\n");
  return `---\nschema: atelier.mission\nschema_version: 3\nid: ${id}\ntitle: Check that a stop from the Desk is a record\nstatus: open\nlabels: []\nlead: null\nplanners:\n- act-is-a-record\ncreated_at: 2026-11-13T00:00:00Z\n---\n\n${sections}\n## Result\n`;
}

/** Starts atelier-desk and gives it with the address it serves on. */
async function startDesk(desk: string, config: string): Promise<{ server: ChildProcess; address: string }> {
  const server = spawn(desk, ["--config", config], { env: environment(), stdio: ["ignore", "ignore", "pipe"] });
  let said = "";
  const address = await new Promise<string>((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error(`atelier-desk did not start: ${said}`)), 30_000);
    server.stderr!.on("data", (chunk: Buffer) => {
      said += chunk.toString();
      const m = said.match(/serving on (http:\/\/\S+)\n/);
      if (m) {
        clearTimeout(timer);
        resolve(m[1]);
      }
    });
    server.on("exit", (code) => reject(new Error(`atelier-desk exited with ${code}: ${said}`)));
  });
  return { server, address };
}

async function main() {
  const o = parseArgs(process.argv.slice(2));
  const repo = path.basename(o.clone);
  const config = git(o.clone, "show", "HEAD:.atelier/config.toml");
  const base = config.match(/^base_branch = "(.+)"$/m)?.[1] ?? fail(`${o.clone} names no base branch`);
  const slug = config.match(/^slug = "(.+)"$/m)?.[1] ?? fail(`${o.clone} names no slug`);
  if (!/^\[forge\]/m.test(config)) fail(`${o.clone} names no forge, so the Desk publishes no stop there`);
  const scratch = mkdtempSync(path.join(process.env.XDG_CACHE_HOME ?? path.join(os.homedir(), ".cache"), "act-is-a-record-"));
  console.log(`Repository ${o.clone}; scratch ${scratch}; atelier ${o.atelier}; atelier-desk ${o.desk}; app ${o.app}; by ${o.by}`);
  const problems: string[] = [];
  // One line per comparison: a short value whole, a long one by its line
  // count, and a difference by its first differing line.
  const compare = (what: string, desk: string, cli: string) => {
    const same = desk === cli;
    if (!same) problems.push(what);
    const a = desk.split("\n");
    const b = cli.split("\n");
    const at = a.findIndex((line, i) => line !== b[i]);
    const line = at === -1 ? a.length : at;
    const told = same
      ? a.length > 1 ? `${a.length} lines, equal` : JSON.stringify(desk)
      : a.length > 1 || b.length > 1
        ? `line ${line + 1}: Desk ${JSON.stringify(a[line] ?? "(none)")}, CLI ${JSON.stringify(b[line] ?? "(none)")}`
        : `Desk ${JSON.stringify(desk)}, CLI ${JSON.stringify(cli)}`;
    console.log(`${same ? "same" : "DIFF"}  ${what}: ${told}`);
  };
  let server: ChildProcess | undefined;
  const browser = await chromium.launch();
  try {
    // The forge's remote: a bare copy of the base branch.
    const origin = path.join(scratch, "origin.git");
    execFileSync("git", ["clone", "-q", "--bare", "--no-hardlinks", "--single-branch", "--branch", base, o.clone, origin], { stdio: "ignore" });
    // A plan, proposed on its own branch and pushed.
    const id = `${slug}-${Math.random().toString(36).slice(2, 6).replace(/[^a-z0-9]/g, "x").padEnd(4, "x")}`;
    const planner = path.join(scratch, "planner");
    execFileSync("git", ["clone", "-q", origin, planner], { stdio: "ignore" });
    git(planner, "switch", "-qc", id);
    const record = `.atelier/missions/${id}/mission.md`;
    mkdirSync(path.join(planner, path.dirname(record)), { recursive: true });
    writeFileSync(path.join(planner, record), missionRecord(id));
    git(planner, "add", "--", record);
    run(planner, "git", ["-c", "user.name=act-is-a-record", "-c", "user.email=act@localhost.invalid", "commit", "-qm", `Plan ${id}`]);
    git(planner, "push", "-q", "origin", id);
    const plan = git(planner, "rev-parse", "HEAD");
    console.log(`Proposed mission ${id} on branch ${id}, at ${plan.slice(0, 9)}`);

    // Two clones of the remote: one the Desk serves, one for the CLI.
    const served = path.join(scratch, "served", repo);
    const byCli = path.join(scratch, "by-cli");
    mkdirSync(path.dirname(served));
    execFileSync("git", ["clone", "-q", origin, served], { stdio: "ignore" });
    execFileSync("git", ["clone", "-q", origin, byCli], { stdio: "ignore" });
    for (const at of [served, byCli]) {
      git(at, "config", "user.name", "act-is-a-record");
      git(at, "config", "user.email", "act@localhost.invalid");
    }

    // Stop it from the mission's page.
    const { gh, log } = ghStandIn(scratch, id);
    const deskConfig = path.join(scratch, "desk.toml");
    const toml = (s: string) => JSON.stringify(s);
    writeFileSync(
      deskConfig,
      `caller = ${toml(o.by)}\nrepos = [${toml(served)}]\nstate = ${toml(path.join(scratch, "state"))}\ngh = ${toml(gh)}\nlisten = ["127.0.0.1:0"]\napp = ${toml(o.app)}\n`,
    );
    const started = await startDesk(o.desk, deskConfig);
    server = started.server;
    const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
    const errors: string[] = [];
    page.on("pageerror", (e) => errors.push(`page error: ${e.message}`));
    page.on("console", (m) => {
      if (m.type() === "error") errors.push(`console error: ${m.text()}`);
    });
    const url = `${started.address}/r/${encodeURIComponent(repo)}/mission/${encodeURIComponent(id)}`;
    await page.goto(url);
    await page.getByRole("button", { name: "Stop this plan", exact: true }).click({ timeout: 60_000 });
    const form = page.getByRole("form", { name: "Stop this plan" });
    await form.getByRole("textbox").fill(REASON);
    await form.getByRole("button", { name: "Yes, stop it" }).click();
    const done = page.getByTestId("done");
    await done.waitFor({ timeout: 60_000 });
    console.log(`Stopped from ${url}: ${(await done.innerText()).split("\n")[0]}`);

    // The same act from the CLI, on the plan's branch.
    git(byCli, "switch", "-q", id);
    const reason = o.differ ? `${REASON}, differently` : REASON;
    const said = run(byCli, o.atelier, ["drop", id, "--reason", reason], o.by);
    console.log(`Ran atelier drop ${id}: ${said.split("\n")[0]}`);
    git(byCli, "commit", "-qm", `Drop ${id}`, "--", record);

    // One new commit each, with the same message, paths and records.
    const pushed = git(origin, "rev-parse", `refs/heads/${id}`);
    for (const [who, at, tip] of [["Desk, pushed", origin, pushed], ["CLI", byCli, "HEAD"]] as const) {
      const count = git(at, "rev-list", "--count", `${plan}..${tip}`);
      if (count !== "1") problems.push(`${who} commits`);
      console.log(`${count === "1" ? "same" : "DIFF"}  new commits by the ${who}: ${count}`);
    }
    compare("message", git(origin, "log", "-1", "--format=%B", pushed), git(byCli, "log", "-1", "--format=%B"));
    const paths = (at: string, tip: string) => git(at, "diff-tree", "--no-commit-id", "--name-status", "-r", tip);
    compare("changed paths", paths(origin, pushed), paths(byCli, "HEAD"));
    const changed = new Set([...paths(origin, pushed).split("\n"), ...paths(byCli, "HEAD").split("\n")].map((l) => l.split("\t").pop()!).filter(Boolean));
    for (const file of changed) {
      const read = (at: string, tip: string) => {
        try {
          return git(at, "show", `${tip}:${file}`);
        } catch {
          return "(no such file)";
        }
      };
      compare(file, read(origin, pushed), read(byCli, "HEAD"));
    }
    compare("pull requests closed", readFileSync(log, "utf8").trim(), `pr close ${id} --comment Stopped from the Desk by ${o.by}: ${REASON}`);

    // The page, loaded again, shows the plan stopped.
    await page.reload();
    await page.getByTestId("status").waitFor({ timeout: 60_000 });
    await page.waitForLoadState("networkidle");
    compare("the page's status", (await page.getByTestId("status").innerText()).trim(), "Stopped");
    compare("Stop buttons on the page", String(await page.getByRole("button", { name: "Stop this plan", exact: true }).count()), "0");
    compare("page and console errors", errors.join("; "), "");
  } finally {
    await browser.close();
    if (server && server.exitCode === null) {
      const exited = new Promise((resolve) => server!.on("exit", resolve));
      server.kill("SIGTERM");
      await exited;
    }
    if (o.keep) console.log(`Kept ${scratch}`);
    else rmSync(scratch, { recursive: true, force: true });
  }
  console.log(problems.length === 0 ? "The stop from the Desk left the record atelier drop leaves, and published it." : `${problems.length} differences: ${problems.join(", ")}.`);
  process.exit(problems.length === 0 ? 0 : 1);
}

main().catch((error: unknown) => {
  console.error(error instanceof Error ? error.message : error);
  process.exit(1);
});
