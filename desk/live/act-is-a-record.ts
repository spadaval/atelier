// act-is-a-record: the mission's second success criterion, as one command.
//
//   pnpm act-is-a-record <converted clone> [--by <name>] [--atelier <binary>]
//       [--desk <atelier-desk binary>] [--app <built app>] [--differ] [--keep]
//
// It works only on copies under a new scratch directory, which it removes
// at the end (unless --keep): it clones <converted clone>, adds a draft
// mission there with the new `atelier new --mission` and commits it, and
// clones that commit twice. It serves the first with atelier-desk and the
// built app, on a loopback port of its own and with no T3 Code, and says go
// from the mission's page in a headless browser; in the second it runs the
// new `atelier ready <mission> --by <name>`. It checks that each has exactly
// one new commit, compares the two commits' messages and changed paths, and
// the files they changed with the time `ready` records set aside, and then
// checks that the page, loaded again, shows the mission ready. It prints
// what it compared and exits 1 on any difference.
//
// --by names who says go (default sumeet): the Desk's caller, and the CLI's
// --by and thread. --differ runs the CLI's `ready` under another name, so
// the two must differ and the command must fail: it shows the comparison
// can fail. The binaries default to this checkout's builds
// (target/debug/atelier, target/release/atelier-desk or else debug) and the app to desk/dist.

import { chromium } from "@playwright/test";
import { execFileSync, spawn, type ChildProcess } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync, mkdirSync } from "node:fs";
import os from "node:os";
import path from "node:path";

const usage =
  "usage: pnpm act-is-a-record <converted clone> [--by <name>] [--atelier <binary>] [--desk <atelier-desk binary>] [--app <built app>] [--differ] [--keep]";
const NO_PUSH = "/nonexistent/atelier-desk-scratch-clone";
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

/** Clones `from` into `to` and makes it unable to push. */
function cloneInto(from: string, to: string) {
  execFileSync("git", ["clone", "-q", "--no-hardlinks", from, to], { stdio: "ignore" });
  git(to, "remote", "set-url", "--push", "origin", NO_PUSH);
}

/** A record with the time `ready` writes (`at:` in its `ready:` block) set aside. */
function withoutReadyTime(text: string): string {
  return text.replace(/^(ready:\n(?: {2}.*\n)*? {2}at: ).*$/m, "$1<time>");
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
  const scratch = mkdtempSync(path.join(os.tmpdir(), "act-is-a-record-"));
  console.log(`Clone ${o.clone}; scratch ${scratch}; atelier ${o.atelier}; atelier-desk ${o.desk}; app ${o.app}; by ${o.by}`);
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
    // A draft mission in a copy of the clone, committed there.
    const source = path.join(scratch, "source");
    cloneInto(o.clone, source);
    const created = run(source, o.atelier, ["new", "--mission", "Check that saying go from the Desk is a record"], o.by);
    const id = created.match(/^Created draft mission (\S+?),/m)?.[1];
    if (!id) throw new Error(`atelier new printed no mission ID:\n${created}`);
    const record = path.join(source, ".atelier/missions", id, "mission.md");
    const filled = readFileSync(record, "utf8").replace(
      /^## (Why|Outcome|Success criteria|Scope|Authority)\n/gm,
      (heading) => `${heading}\nWritten by act-is-a-record, in a scratch copy.\n`,
    );
    writeFileSync(record, filled);
    git(source, "add", "--", path.relative(source, record));
    run(source, "git", ["-c", "user.name=act-is-a-record", "-c", "user.email=act@localhost.invalid", "commit", "-qm", `Plan mission ${id}`]);
    const base = git(source, "rev-parse", "HEAD");
    console.log(`Draft mission ${id}, committed as ${base.slice(0, 9)} in a copy of ${repo}`);

    // Two clones of that commit: one the Desk serves, one for the CLI.
    const served = path.join(scratch, "served", repo);
    const byCli = path.join(scratch, "by-cli");
    mkdirSync(path.dirname(served));
    cloneInto(source, served);
    cloneInto(source, byCli);

    // Say go from the mission's page.
    const config = path.join(scratch, "desk.toml");
    const toml = (s: string) => JSON.stringify(s);
    writeFileSync(config, `caller = ${toml(o.by)}\nroots = [${toml(path.dirname(served))}]\nlisten = ["127.0.0.1:0"]\napp = ${toml(o.app)}\n`);
    const started = await startDesk(o.desk, config);
    server = started.server;
    const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
    const errors: string[] = [];
    page.on("pageerror", (e) => errors.push(`page error: ${e.message}`));
    page.on("console", (m) => {
      if (m.type() === "error") errors.push(`console error: ${m.text()}`);
    });
    const url = `${started.address}/r/${encodeURIComponent(repo)}/mission/${encodeURIComponent(id)}`;
    await page.goto(url);
    await page.getByRole("button", { name: "Say go", exact: true }).click({ timeout: 60_000 });
    await page.getByRole("group", { name: "Confirm saying go" }).getByRole("button", { name: "Yes, say go" }).click();
    const done = page.getByTestId("done");
    await done.waitFor({ timeout: 60_000 });
    console.log(`Said go at ${url}: ${(await done.innerText()).split("\n")[0]}`);

    // The same act from the CLI.
    const by = o.differ ? `${o.by}-differs` : o.by;
    const said = run(byCli, o.atelier, ["ready", id, "--by", by], by);
    console.log(`Ran atelier ready ${id} --by ${by}: ${said.split("\n")[0]}`);

    // One new commit each, with the same message, paths and records.
    for (const [who, at] of [["Desk", served], ["CLI", byCli]] as const) {
      const count = git(at, "rev-list", "--count", `${base}..HEAD`);
      if (count !== "1") problems.push(`${who} commits`);
      console.log(`${count === "1" ? "same" : "DIFF"}  new commits by the ${who}: ${count}`);
    }
    compare("message", git(served, "log", "-1", "--format=%B"), git(byCli, "log", "-1", "--format=%B"));
    const paths = (at: string) => git(at, "diff-tree", "--no-commit-id", "--name-status", "-r", "HEAD");
    compare("changed paths", paths(served), paths(byCli));
    const changed = new Set([...paths(served).split("\n"), ...paths(byCli).split("\n")].map((l) => l.split("\t").pop()!).filter(Boolean));
    for (const file of changed) {
      const read = (at: string) => {
        try {
          return withoutReadyTime(git(at, "show", `HEAD:${file}`));
        } catch {
          return "(no such file)";
        }
      };
      compare(`${file}, with the ready time set aside`, read(served), read(byCli));
    }

    // The page, loaded again, shows the mission ready.
    await page.reload();
    await page.getByTestId("status").waitFor({ timeout: 60_000 });
    await page.waitForLoadState("networkidle");
    compare("the page's status", (await page.getByTestId("status").innerText()).trim(), "Ready");
    compare("Say go buttons on the page", String(await page.getByRole("button", { name: "Say go", exact: true }).count()), "0");
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
  console.log(problems.length === 0 ? "Saying go from the Desk left the record atelier ready leaves." : `${problems.length} differences: ${problems.join(", ")}.`);
  process.exit(problems.length === 0 ? 0 : 1);
}

main().catch((error: unknown) => {
  console.error(error instanceof Error ? error.message : error);
  process.exit(1);
});
