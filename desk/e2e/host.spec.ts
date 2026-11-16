import { expect, test, type Frame, type Page } from "@playwright/test";
import { createServer, type Server } from "node:http";
import type { AddressInfo } from "node:net";
import { live, tied, z4cr } from "./fixtures";
import { expectNoControlCutOff, serve } from "./helpers";

// The host contract (docs/architecture/desk.md, "The host contract"): a
// fixture host page frames the embedded Desk and talks to it over
// postMessage. Each host is served on a loopback port of its own, so it has
// an origin of its own and the frame is cross-origin to it, as T3 Code's is.

type Received = { origin: string; data: { type: string; [field: string]: unknown } };

/** The fixture host: a page framing the Desk's path given in its query, in a frame `width` pixels wide. */
function hostPage(desk: string, path: string, width: number): string {
  return `<!doctype html><title>Host</title><style>body{margin:0}</style>
<iframe id="desk" src="${desk}/embed${path}" style="width:${width}px;height:700px;border:0"></iframe>
<script>
  window.received = [];
  const frame = document.getElementById("desk");
  addEventListener("message", (e) => { if (e.source === frame.contentWindow) window.received.push({ origin: e.origin, data: e.data }); });
  window.send = (message) => frame.contentWindow.postMessage(message, ${JSON.stringify(desk)});
</script>`;
}

const servers: Server[] = [];
/** The origins of two fixture hosts: one the tests list, one they do not. */
let listed = "";
let stranger = "";

test.beforeAll(async () => {
  const desk = new URL(test.info().project.use.baseURL!).origin;
  const start = () =>
    new Promise<string>((resolve) => {
      const server = createServer((request, response) => {
        const query = new URL(request.url ?? "/", "http://host").searchParams;
        const path = query.get("path") ?? "/inbox";
        response.writeHead(200, { "content-type": "text/html" }).end(hostPage(desk, path, Number(query.get("width") ?? 420)));
      });
      servers.push(server);
      server.listen(0, "127.0.0.1", () => resolve(`http://127.0.0.1:${(server.address() as AddressInfo).port}`));
    });
  [listed, stranger] = await Promise.all([start(), start()]);
});

test.afterAll(() => {
  for (const server of servers) server.close();
});

/** Opens the fixture host at `origin` framing the Desk's `path` `width` pixels wide, with `origins` listed, and returns the frame once the Desk has drawn. */
async function host(page: Page, origin: string, path: string, origins: string[], width = 420): Promise<Frame> {
  const world = await serve(page);
  world.embedOrigins = origins;
  await page.setViewportSize({ width: Math.max(width, page.viewportSize()?.width ?? 0), height: 760 });
  await page.goto(`${origin}/?path=${encodeURIComponent(path)}&width=${width}`);
  const frame = page.frame({ url: /\/embed\// })!;
  await frame.waitForLoadState("networkidle");
  // Mission control draws its own top bar and keeps its breadcrumb hidden.
  if (!path.startsWith("/mission/")) await expect(frame.getByRole("navigation", { name: "Where you are" })).not.toBeEmpty();
  return frame;
}

const received = (page: Page) => page.evaluate(() => (window as unknown as { received: Received[] }).received);
const send = (page: Page, message: unknown) => page.evaluate((m) => (window as unknown as { send: (m: unknown) => void }).send(m), message);

/** The background and text colours the Desk draws its page in. */
const drawn = (frame: Frame) =>
  frame.evaluate(() => {
    const body = getComputedStyle(document.body);
    const main = getComputedStyle(document.querySelector(".main")!);
    return { background: body.backgroundColor, text: body.color, mainText: main.color };
  });

test("a listed host hears the Desk is ready and where it is, and themes it", async ({ page }) => {
  const frame = await host(page, listed, "/inbox", [listed]);
  await expect.poll(async () => (await received(page)).map((m) => m.data.type)).toContain("desk:navigated");
  const first = await received(page);
  expect(first[0].data).toEqual({ type: "desk:ready", version: 1 });
  expect(first.every((m) => m.origin === new URL(frame.url()).origin)).toBe(true);
  await expect.poll(async () => (await received(page)).at(-1)?.data).toEqual({ type: "desk:navigated", path: "/inbox", title: "Inbox · Desk" });

  // A navigation inside the Desk is told, with the new page's title.
  await frame.getByRole("region", { name: "atelier" }).getByRole("link", { name: z4cr.mission.title }).click();
  await expect.poll(() => frame.url()).toMatch(/\/embed\/r\/atelier\/mission\/atelier-z4cr$/);
  await expect
    .poll(async () => (await received(page)).filter((m) => m.data.type === "desk:navigated").at(-1)?.data)
    .toEqual({ type: "desk:navigated", path: "/r/atelier/mission/atelier-z4cr", title: `${z4cr.mission.title} · Desk` });
  const told = (await received(page)).at(-1)!.data;
  expect(told.title).toBe(await frame.title());

  // Light colours, then dark ones, each applied without a reload.
  await send(page, { type: "host:theme", scheme: "light", colors: { background: "#fafafa", foreground: "#202020" }, font: "Georgia, serif" });
  await expect.poll(() => drawn(frame)).toEqual({ background: "rgb(250, 250, 250)", text: "rgb(32, 32, 32)", mainText: "rgb(32, 32, 32)" });
  expect(await frame.evaluate(() => getComputedStyle(document.body).fontFamily)).toBe("Georgia, serif");
  await send(page, { type: "host:theme", colors: { background: "#121314", foreground: "#e0e1e2" } });
  await expect.poll(() => drawn(frame)).toMatchObject({ background: "rgb(18, 19, 20)", text: "rgb(224, 225, 226)" });
  expect(await frame.evaluate(() => document.documentElement.dataset.theme)).toBeUndefined();

  // A modern colour form sets the scheme from its luminance too, either way.
  const scheme = () => frame.evaluate(() => getComputedStyle(document.documentElement).colorScheme);
  await send(page, { type: "host:theme", colors: { background: "oklch(0.98 0 0)", foreground: "oklch(0.1 0 0)" } });
  await expect.poll(scheme).toBe("light");
  await send(page, { type: "host:theme", colors: { background: "color(srgb 0.07 0.075 0.08)", foreground: "#e0e1e2" } });
  await expect.poll(scheme).toBe("dark");
  await send(page, { type: "host:theme", colors: { background: "#121314" } });
  await expect.poll(() => drawn(frame)).toMatchObject({ background: "rgb(18, 19, 20)", text: "rgb(224, 225, 226)" });

  // A colour that does not parse is dropped, and the colour before it stays;
  // a message the Desk does not know is ignored.
  await send(page, { type: "host:theme", colors: { background: "not a colour", foreground: "var(--you)" } });
  await send(page, { type: "host:theme", colors: { background: "color-mix(in srgb, currentColor 50%, white)" } });
  await send(page, { type: "host:navigate", path: "https://example.com/" });
  await send(page, "host:theme");
  await page.waitForTimeout(200);
  expect(await drawn(frame)).toMatchObject({ background: "rgb(18, 19, 20)", text: "rgb(224, 225, 226)" });
  await expect.poll(() => frame.url()).toMatch(/\/embed\/r\/atelier\/mission\/atelier-z4cr$/);
});

test("a thread link asks a listed host to open the thread, and opens nothing itself", async ({ page, context }) => {
  const frame = await host(page, listed, "/agents", [listed]);
  await expect.poll(async () => (await received(page)).length).toBeGreaterThan(1);
  const before = (await received(page)).length;
  const url = frame.url();
  const opened: string[] = [];
  context.on("page", (p) => opened.push(p.url()));
  page.on("popup", (p) => opened.push(p.url()));

  const link = frame.getByRole("button", { name: live.uwxy.title });
  await link.click();
  await expect.poll(async () => (await received(page)).slice(before).map((m) => m.data)).toEqual([{ type: "desk:open-thread", thread: live.uwxy.id }]);
  await page.waitForTimeout(300);
  expect((await received(page)).length).toBe(before + 1);
  expect(opened).toEqual([]);
  expect(frame.url()).toBe(url);
  await expect(frame.getByRole("link", { name: live.uwxy.title })).toHaveCount(0);
});

test("a host whose origin is not listed hears nothing and changes nothing", async ({ page }) => {
  const frame = await host(page, stranger, "/agents", [listed]);
  const before = await drawn(frame);
  await send(page, { type: "host:theme", scheme: "light", colors: { background: "#fafafa", foreground: "#202020" } });
  await page.waitForTimeout(300);
  expect(await drawn(frame)).toEqual(before);
  expect(await received(page)).toEqual([]);

  // The thread link works as it does unframed: a link to T3 Code's address.
  await expect(frame.getByRole("link", { name: live.uwxy.title })).toHaveAttribute("href", live.uwxy.link);
});

test("with no origin listed, the framed Desk draws and sends and reads nothing", async ({ page }) => {
  const frame = await host(page, listed, "/inbox", []);
  const before = await drawn(frame);
  await send(page, { type: "host:theme", colors: { background: "#fafafa", foreground: "#202020" } });
  await page.waitForTimeout(300);
  expect(await drawn(frame)).toEqual(before);
  expect(await received(page)).toEqual([]);
  await expect(frame.getByRole("link", { name: "Inbox", exact: true })).toBeVisible();
});

// Following the host's thread (`host:thread`): each thread the host names
// opens "This thread", which reads what the thread holds, the threads it
// started and the records it names from the message.
const at = (frame: Frame) => new URL(frame.url()).pathname;
const navigated = async (page: Page) => (await received(page)).filter((m) => m.data.type === "desk:navigated").at(-1)?.data.path;

/** The fixture host framing the Desk's repositories page, once the Desk has told it where it is. */
async function connected(page: Page, path = "/", width = 420): Promise<Frame> {
  const frame = await host(page, listed, path, [listed], width);
  await expect.poll(async () => (await received(page)).map((m) => m.data.type)).toContain("desk:navigated");
  return frame;
}

test("the host's thread opens This thread, which shows what the thread holds", async ({ page }) => {
  const frame = await connected(page);
  await send(page, { type: "host:thread", thread: tied.implementer });
  await expect.poll(() => at(frame)).toBe("/embed/t");
  await expect.poll(() => navigated(page)).toBe("/t");
  const holds = frame.locator(".glance-block", { has: frame.getByRole("heading", { name: "Holds" }) });
  await expect(holds.locator(".hold-role")).toHaveText("This thread implements this item.");
  await expect(holds.getByRole("link", { name: /atelier-uwxy/ }).first()).toHaveAttribute("href", "/embed/r/atelier/item/atelier-uwxy");
  await expect(holds.getByText("1/2")).toBeVisible();
  await expect(holds.getByRole("link", { name: /#131 open/ })).toHaveAttribute("href", "https://github.com/spadaval/atelier/pull/131");

  // A lead's thread shows its mission, and the work under it with its threads.
  await send(page, { type: "host:thread", thread: tied.lead });
  await expect(frame.locator(".hold-role").first()).toHaveText("This thread leads this mission.");
  const work = frame.locator(".glance-block", { has: frame.getByRole("heading", { name: "Work under this thread" }) });
  await expect(work.getByRole("link", { name: /atelier-z4cr/ }).first()).toHaveAttribute("href", "/embed/mission/atelier/atelier-z4cr");
  await expect(work.locator(".chip")).not.toHaveCount(0);
});

for (const width of [420, 720]) {
  test(`This thread in a ${width} pixel panel fits and cuts off no control`, async ({ page }) => {
    const frame = await connected(page, "/", width);
    await send(page, { type: "host:thread", thread: tied.lead, children: ["thread:child-quiet"], mentioned: ["atelier-1ie1", "atelier-aab1"] });
    await expect(frame.getByRole("heading", { name: "Work under this thread" })).toBeVisible();
    await expect(frame.getByRole("heading", { name: "Mentioned" })).toBeVisible();
    expect(await frame.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth)).toBeLessThanOrEqual(0);
    await expectNoControlCutOff(frame);
  });
}

test("This thread lists the threads it started and the records it names", async ({ page }) => {
  const frame = await connected(page);
  const mentioned = ["atelier-uwxy", "atelier-1ie1", "atelier-z4cr", "atelier-1ie1", "nothing-0000"];
  await send(page, { type: "host:thread", thread: tied.implementer, children: ["thread:child-quiet"], mentioned });
  await expect.poll(() => at(frame)).toBe("/embed/t");
  const started = frame.locator(".glance-block", { has: frame.getByRole("heading", { name: "Other threads it started" }) });
  await expect(started.getByRole("button", { name: /Look into the flaky e2e test/ })).toBeVisible();
  // Quiet for two hours while it says it runs.
  await expect(started.locator(".child-row.stale")).toHaveCount(1);
  // Named records, each once, without the one it holds or one no repository has.
  const named = frame.locator(".glance-block", { has: frame.getByRole("heading", { name: "Mentioned" }) });
  await expect(named.locator(".glance-head .tally")).toHaveText("2");
  await expect(named.getByText("Done or dropped: 1 of 2")).toBeVisible();

  // Opening a thread it started asks the host.
  const before = (await received(page)).length;
  await started.getByRole("button", { name: /Look into the flaky e2e test/ }).click();
  await expect.poll(async () => (await received(page)).slice(before).map((m) => m.data)).toContainEqual({ type: "desk:open-thread", thread: "thread:child-quiet" });
});

test("a thread that holds, started and names nothing shows its project, or says so", async ({ page }) => {
  const frame = await connected(page);
  await send(page, { type: "host:thread", thread: tied.none, repository: "atelier" });
  await expect(frame.locator(".thread-part")).toHaveText("This thread holds nothing, started nothing and names no record. Here is atelier.");
  await expect(frame.getByRole("heading", { name: "Missions" })).toBeVisible();
  await expect(frame.getByRole("heading", { name: "In motion" })).toBeVisible();

  await send(page, { type: "host:thread", thread: tied.pastOnly });
  await expect(frame.locator(".glance-empty")).toHaveText("This thread holds nothing, started nothing and names no record.");
  await expect(frame.getByText(/Earlier it implemented atelier-1ie1 \(done\), reviewed atelier-1ie1 \(done\)/)).toBeVisible();
});

test("the same thread again leaves the page alone, and another thread opens This thread", async ({ page }) => {
  const frame = await connected(page);
  await send(page, { type: "host:thread", thread: tied.implementer });
  await expect.poll(() => at(frame)).toBe("/embed/t");

  // The human moves on; the host names the same thread again, with new children.
  await frame.getByRole("link", { name: "Inbox", exact: true }).click();
  await expect.poll(() => at(frame)).toBe("/embed/inbox");
  await send(page, { type: "host:thread", thread: tied.implementer });
  await send(page, { type: "host:thread", thread: tied.implementer, repository: "atelier", children: ["thread:child-quiet"] });
  await page.waitForTimeout(300);
  expect(at(frame)).toBe("/embed/inbox");

  // Back on This thread, the new children are shown where it is.
  await frame.evaluate(() => history.back());
  await send(page, { type: "host:thread", thread: tied.lead });
  await expect.poll(() => at(frame)).toBe("/embed/t");
  await send(page, { type: "host:thread", thread: tied.lead, children: ["thread:child-quiet"] });
  await expect(frame.getByRole("heading", { name: "Other threads it started" })).toBeVisible();

  // A thread the host showed before is another thread again once it has moved off it.
  await frame.getByRole("link", { name: "Inbox", exact: true }).click();
  await send(page, { type: "host:thread", thread: tied.implementer });
  await expect.poll(() => at(frame)).toBe("/embed/t");
});

test("a thread whose ties cannot be read still draws its page, and says why", async ({ page }) => {
  const frame = await connected(page);
  await send(page, { type: "host:thread", thread: tied.unreadable, mentioned: ["atelier-z4cr"] });
  await expect.poll(() => at(frame)).toBe("/embed/t");
  await expect(frame.getByRole("alert")).toHaveText("The records could not be read.");
  await expect(frame.getByRole("heading", { name: "Mentioned" })).toBeVisible();
  await expect(frame.locator(".thread-part, .glance-empty")).toHaveCount(0);
});

test("mission control tells the host its height, and the host hears Escape", async ({ page }) => {
  const frame = await connected(page, "/mission/atelier/atelier-z4cr", 1000);
  await expect(frame.getByText("Mission Control", { exact: true })).toBeVisible();
  await expect.poll(async () => (await received(page)).filter((m) => m.data.type === "desk:resized").at(-1)?.data.height).toBe(700);

  // Escape that nothing in the Desk handled, from wherever focus is.
  await frame.locator(".mcx-row.on").focus();
  await page.keyboard.press("Escape");
  await expect.poll(async () => (await received(page)).at(-1)?.data).toEqual({ type: "desk:key", key: "Escape" });
});

test("mission control's keys select items, open threads and pages, and page the log", async ({ page }) => {
  const frame = await connected(page, "/mission/atelier/atelier-z4cr", 1000);
  const items = frame.getByRole("listbox", { name: "Items" });
  const selected = () => items.locator('[aria-selected="true"] .id').textContent();
  // The item in motion is selected first.
  await expect.poll(selected).toBe("atelier-uwxy");
  // The keys are read wherever focus is in the frame.
  await frame.locator(".mcx-row.on").focus();
  const body = page.keyboard;
  await body.press("ArrowDown");
  await expect.poll(selected).toBe("atelier-1ie1");
  await expect.poll(() => new URL(frame.url()).searchParams.get("item")).toBe("atelier-1ie1");
  await body.press("k");
  await expect.poll(selected).toBe("atelier-uwxy");
  await body.press("j");
  await body.press("k");
  await expect.poll(selected).toBe("atelier-uwxy");

  // Enter opens the selected item's thread; L the mission's planner.
  const told = async () => (await received(page)).filter((m) => m.data.type === "desk:open-thread").map((m) => m.data.thread);
  await body.press("Enter");
  await expect.poll(told).toEqual([tied.implementer]);
  await body.press("L");
  await expect.poll(told).toEqual([tied.implementer, "planner-thread"]);

  // ] and [ page the progress log.
  const log = frame.locator(".mcx-pane.log .ptitle");
  const first = await log.textContent();
  expect(first).toMatch(/^Progress Log 1–\d+ of \d+$/);
  const [, shown, all] = /1–(\d+) of (\d+)/.exec(first!)!.map(Number);
  expect(shown, "the log fills more than its pane").toBeLessThan(all);
  await body.press("]");
  await expect(log).toHaveText(new RegExp(`^Progress Log ${shown + 1}–`));
  await body.press("[");
  await expect(log).toHaveText(first!);

  // I opens the item's page.
  await body.press("i");
  await expect.poll(() => at(frame)).toBe("/embed/r/atelier/item/atelier-uwxy");
});

test("a host whose origin is not listed cannot move the Desk", async ({ page }) => {
  const frame = await host(page, stranger, "/", [listed]);
  await send(page, { type: "host:thread", thread: tied.implementer });
  await page.waitForTimeout(300);
  expect(at(frame)).toBe("/embed/");
  await expect(frame.locator(".thread-part")).toHaveCount(0);
});
