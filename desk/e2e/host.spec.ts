import { expect, test, type Frame, type Page } from "@playwright/test";
import { createServer, type Server } from "node:http";
import type { AddressInfo } from "node:net";
import { live, tied, z4cr } from "./fixtures";
import { serve } from "./helpers";

// The host contract (docs/architecture/desk.md, "The host contract"): a
// fixture host page frames the embedded Desk and talks to it over
// postMessage. Each host is served on a loopback port of its own, so it has
// an origin of its own and the frame is cross-origin to it, as T3 Code's is.

type Received = { origin: string; data: { type: string; [field: string]: unknown } };

/** The fixture host: a page framing the Desk's path given in its query. */
function hostPage(desk: string, path: string): string {
  return `<!doctype html><title>Host</title>
<iframe id="desk" src="${desk}/embed${path}" style="width:420px;height:800px;border:0"></iframe>
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
        const path = new URL(request.url ?? "/", "http://host").searchParams.get("path") ?? "/inbox";
        response.writeHead(200, { "content-type": "text/html" }).end(hostPage(desk, path));
      });
      servers.push(server);
      server.listen(0, "127.0.0.1", () => resolve(`http://127.0.0.1:${(server.address() as AddressInfo).port}`));
    });
  [listed, stranger] = await Promise.all([start(), start()]);
});

test.afterAll(() => {
  for (const server of servers) server.close();
});

/** Opens the fixture host at `origin` framing the Desk's `path`, with `origins` listed, and returns the frame once the Desk has drawn. */
async function host(page: Page, origin: string, path: string, origins: string[]): Promise<Frame> {
  const world = await serve(page);
  world.embedOrigins = origins;
  await page.goto(`${origin}/?path=${encodeURIComponent(path)}`);
  const frame = page.frame({ url: /\/embed\// })!;
  await frame.waitForLoadState("networkidle");
  await expect(frame.getByRole("navigation", { name: "Where you are" })).not.toBeEmpty();
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

// Following the host's thread (`host:thread`): the Desk opens the page for
// what the thread holds now, and names the thread's part on it.
const follows = [
  { who: "an implementer", thread: tied.implementer, path: "/r/atelier/item/atelier-uwxy", line: "This thread implements this item." },
  { who: "a lead", thread: tied.lead, path: "/r/atelier/mission/atelier-z4cr", line: "This thread leads this mission." },
  { who: "a thread with no tie", thread: tied.none, path: "/inbox", line: "This thread holds no open work." },
  { who: "a thread that implemented a done item and reviews another", thread: tied.pastAndReview, path: "/r/atelier/item/atelier-u1ko", line: "This thread reviews this item." },
  { who: "a thread with only past ties", thread: tied.pastOnly, path: "/inbox", line: "This thread holds no open work." },
];

const at = (frame: Frame) => new URL(frame.url()).pathname;

for (const f of follows) {
  test(`the host's thread opens its page: ${f.who}`, async ({ page }) => {
    const frame = await host(page, listed, "/", [listed]);
    await expect.poll(async () => (await received(page)).map((m) => m.data.type)).toContain("desk:navigated");
    await send(page, { type: "host:thread", thread: f.thread });
    await expect.poll(() => at(frame)).toBe(`/embed${f.path}`);
    await expect(frame.locator(".thread-part")).toHaveText(f.line);
    await expect.poll(async () => (await received(page)).filter((m) => m.data.type === "desk:navigated").at(-1)?.data.path).toBe(f.path);
  });
}

test("the same thread again leaves the page alone, and another thread changes it", async ({ page }) => {
  const frame = await host(page, listed, "/", [listed]);
  await expect.poll(async () => (await received(page)).map((m) => m.data.type)).toContain("desk:navigated");
  await send(page, { type: "host:thread", thread: tied.implementer });
  await expect.poll(() => at(frame)).toBe("/embed/r/atelier/item/atelier-uwxy");

  // The human moves on; the host names the same thread again.
  await frame.getByRole("link", { name: "Inbox", exact: true }).click();
  await expect.poll(() => at(frame)).toBe("/embed/inbox");
  await expect(frame.locator(".thread-part")).toHaveCount(0);
  await send(page, { type: "host:thread", thread: tied.implementer });
  await page.waitForTimeout(300);
  expect(at(frame)).toBe("/embed/inbox");

  // Another thread opens its own page.
  await send(page, { type: "host:thread", thread: tied.lead });
  await expect.poll(() => at(frame)).toBe("/embed/r/atelier/mission/atelier-z4cr");
  await expect(frame.locator(".thread-part")).toHaveText("This thread leads this mission.");

  // A thread the host showed before is another thread again once it has moved off it.
  await send(page, { type: "host:thread", thread: tied.implementer });
  await expect.poll(() => at(frame)).toBe("/embed/r/atelier/item/atelier-uwxy");
});

test("a host whose origin is not listed cannot move the Desk", async ({ page }) => {
  const frame = await host(page, stranger, "/", [listed]);
  await send(page, { type: "host:thread", thread: tied.implementer });
  await page.waitForTimeout(300);
  expect(at(frame)).toBe("/embed/");
  await expect(frame.locator(".thread-part")).toHaveCount(0);
});
