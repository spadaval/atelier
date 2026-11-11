// desk-matches-cli: the mission's first success criterion, as one command.
//
//   pnpm desk-matches-cli <desk address> <clones directory> [--atelier <binary>]
//
// For every open mission in every clone under <clones directory>, it loads
// the mission's page from the running Desk at <desk address> in a headless
// browser, fails on a page error or a console error, and compares the page's
// title, status and per-status item counts with what the new
// `atelier show <mission>` prints in that clone. For every clone it compares
// the inbox's needs-human items with `atelier list --needs-human` there. It
// prints one line per mission and per inbox and exits 1 on any difference.
// The binary defaults to target/debug/atelier in this checkout.
//
// It reads only: the Desk's pages, and `show` and `list` in the clones.

import { chromium, type Browser, type Page } from "@playwright/test";
import { execFileSync } from "node:child_process";
import { existsSync, readdirSync } from "node:fs";
import path from "node:path";

const usage = "usage: pnpm desk-matches-cli <desk address> <clones directory> [--atelier <binary>]";

type Shown = { title: string; status: string; inProgress: boolean; counts: Record<string, number> };

function fail(message: string): never {
  console.error(message);
  process.exit(2);
}

function parseArgs(argv: string[]) {
  const positional: string[] = [];
  let atelier: string | undefined;
  for (let i = 0; i < argv.length; i++) {
    if (argv[i] === "--atelier") atelier = argv[++i] ?? fail(usage);
    else positional.push(argv[i]);
  }
  if (positional.length !== 2) fail(usage);
  const top = path.resolve(import.meta.dirname, "../..");
  atelier ??= [path.join(top, "target/debug/atelier")].find(existsSync);
  if (!atelier || !existsSync(atelier)) fail(`No new atelier binary${atelier ? ` at ${atelier}` : ""}: build it, or give --atelier <binary>`);
  return { address: positional[0].replace(/\/+$/, ""), clones: path.resolve(positional[1]), atelier: path.resolve(atelier) };
}

/** Runs a command in `cwd` with no harness's identity, and gives its output. */
function run(cwd: string, command: string, args: string[]): string {
  const env = { ...process.env };
  for (const name of ["T3CODE_THREAD_ID", "CLAUDE_CODE_SESSION_ID", "CODEX_SESSION_ID", "CODEX_THREAD_ID", "GROK_SESSION_ID"]) delete env[name];
  return execFileSync(command, args, { cwd, env, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] });
}

/** The clones: each directory under `clones` whose last commit holds an Atelier configuration. */
function clonesIn(clones: string): string[] {
  return readdirSync(clones, { withFileTypes: true })
    .filter((d) => d.isDirectory())
    .map((d) => d.name)
    .filter((name) => {
      try {
        run(path.join(clones, name), "git", ["cat-file", "-e", "HEAD:.atelier/config.toml"]);
        return true;
      } catch {
        return false;
      }
    })
    .sort();
}

/** `atelier show <mission>`: its title, status, and how many items it lists under each status. */
function show(atelier: string, clone: string, id: string): Shown {
  const text = run(clone, atelier, ["show", id]);
  const lines = text.split("\n");
  const title = lines[0].match(/^Mission \S+: (.*)$/)?.[1];
  const status = text.match(/^Status: ([a-z ]+?)(, in progress)?$/m);
  if (title === undefined || !status) throw new Error(`atelier show ${id} printed no title or status line`);
  const counts: Record<string, number> = {};
  const items = lines.slice(lines.indexOf("Items:") + 1, lines.indexOf("Questions:"));
  let heading: string | null = null;
  for (const line of items) {
    const h = line.match(/^ {2}([A-Z][a-z ]*):$/);
    if (h) heading = h[1].toLowerCase().replaceAll(" ", "_");
    else if (heading && /^ {4}\S+ \(/.test(line)) counts[heading] = (counts[heading] ?? 0) + 1;
  }
  return { title, status: status[1].replaceAll(" ", "_"), inProgress: status[2] !== undefined, counts };
}

/** The missions in the clone's last commit. */
function missionsIn(clone: string): string[] {
  const listed = run(clone, "git", ["ls-tree", "--name-only", "HEAD", ".atelier/missions/"]);
  return listed.split("\n").filter(Boolean).map((p) => path.basename(p)).sort();
}

/** `atelier list --needs-human`: the IDs of the items it lists. */
function needsHuman(atelier: string, clone: string): string[] {
  const text = run(clone, atelier, ["list", "--needs-human"]);
  return [...text.matchAll(/^ +([A-Za-z0-9][\w.-]*) \(/gm)].map((m) => m[1]).sort();
}

/** Opens `url` in a new page and fails on a page error, a console error or a failed load. */
async function load(browser: Browser, url: string, ready: (page: Page) => Promise<void>): Promise<{ page: Page; errors: string[] }> {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  const errors: string[] = [];
  page.on("pageerror", (e) => errors.push(`page error: ${e.message}`));
  page.on("console", (m) => {
    if (m.type() === "error") errors.push(`console error: ${m.text()}`);
  });
  const response = await page.goto(url);
  if (!response?.ok()) errors.push(`${url} answered ${response?.status() ?? "nothing"}`);
  await ready(page);
  await page.waitForLoadState("networkidle");
  return { page, errors };
}

function countsText(counts: Record<string, number>): string {
  const text = Object.entries(counts)
    .map(([status, n]) => `${n} ${status}`)
    .join(", ");
  return text || "no items";
}

async function main() {
  const { address, clones, atelier } = parseArgs(process.argv.slice(2));
  const repos = clonesIn(clones);
  if (repos.length === 0) fail(`No clone with an Atelier configuration is under ${clones}`);
  console.log(`Desk at ${address}; clones in ${clones}: ${repos.join(", ")}; atelier is ${atelier}`);
  const browser = await chromium.launch();
  let differ = 0;
  let missions = 0;
  const report = (ok: boolean, line: string) => {
    if (!ok) differ++;
    console.log(`${ok ? "same" : "DIFF"}  ${line}`);
  };
  try {
    for (const repo of repos) {
      const clone = path.join(clones, repo);
      for (const id of missionsIn(clone)) {
        const cli = show(atelier, clone, id);
        if (cli.status !== "draft" && cli.status !== "ready") continue;
        missions++;
        const url = `${address}/r/${encodeURIComponent(repo)}/mission/${encodeURIComponent(id)}`;
        const { page, errors } = await load(browser, url, (p) => p.getByRole("heading", { level: 1 }).waitFor({ timeout: 60_000 }));
        const head = page.locator(".paper-head");
        const shown: Shown = {
          title: (await page.getByRole("heading", { level: 1 }).innerText()).trim(),
          status: (await page.getByTestId("status").innerText()).trim().toLowerCase().replaceAll(" ", "_"),
          inProgress: (await head.getByText("Work in progress", { exact: true }).count()) > 0,
          counts: {},
        };
        for (const span of await head.locator("[data-testid=counts] > [data-status]").all()) {
          shown.counts[(await span.getAttribute("data-status")) ?? "?"] = Number(await span.locator(".num").innerText());
        }
        await page.close();
        const problems = [...errors];
        if (shown.title !== cli.title) problems.push(`title: page "${shown.title}", show "${cli.title}"`);
        if (shown.status !== cli.status) problems.push(`status: page ${shown.status}, show ${cli.status}`);
        if (shown.inProgress !== cli.inProgress) problems.push(`in progress: page ${shown.inProgress}, show ${cli.inProgress}`);
        if (countsText(shown.counts) !== countsText(cli.counts)) problems.push(`counts: page ${countsText(shown.counts)}, show ${countsText(cli.counts)}`);
        const said = `${cli.status}${cli.inProgress ? ", in progress" : ""}; ${countsText(cli.counts)}`;
        report(problems.length === 0, `${repo} ${id} (${said}): ${problems.length ? problems.join("; ") : cli.title}`);
      }
    }
    const { page, errors } = await load(browser, `${address}/inbox`, (p) => p.getByRole("heading", { level: 1 }).waitFor({ timeout: 60_000 }));
    await page.getByText("Reading every repository").waitFor({ state: "detached", timeout: 60_000 });
    if (errors.length) report(false, `inbox: ${errors.join("; ")}`);
    for (const repo of repos) {
      const cli = needsHuman(atelier, path.join(clones, repo));
      const region = page.getByRole("region", { name: repo, exact: true });
      const shown = (await region.locator("[data-testid=needs-human] .iline .id").allInnerTexts()).map((t) => t.trim()).sort();
      const same = shown.join(" ") === cli.join(" ");
      const listed = (ids: string[]) => (ids.length ? ids.join(" ") : "none");
      report(same, `${repo} inbox needs-human: ${same ? listed(cli) : `page ${listed(shown)}; list --needs-human ${listed(cli)}`}`);
    }
    await page.close();
  } finally {
    await browser.close();
  }
  console.log(differ === 0 ? `All ${missions} open missions and ${repos.length} inboxes match the CLI.` : `${differ} differences.`);
  process.exit(differ === 0 ? 0 : 1);
}

main().catch((error: unknown) => {
  console.error(error instanceof Error ? error.message : error);
  process.exit(1);
});
