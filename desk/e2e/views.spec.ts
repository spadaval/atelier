import { expect, test, type Page } from "@playwright/test";
import { expectFits, expectNoControlCutOff, load, serve } from "./helpers";

// The views a host such as T3 Code shows over its main area, each at the
// widths it is shown at: mission control, the project home and the inbox.
// "This thread", which needs a host to name the thread, is in host.spec.ts.

test.beforeEach(async ({ page }) => {
  await serve(page);
});

const box = async (page: Page, selector: string) => (await page.locator(selector).first().boundingBox())!;

/** Opens `path` embedded in a window `width` by `height`. */
async function open(page: Page, path: string, width: number, height = 760) {
  await page.setViewportSize({ width, height });
  await load(page, `/embed${path}`);
}

const control = "/mission/atelier/atelier-z4cr";

for (const width of [1100, 860]) {
  test(`mission control at ${width} pixels keeps its four panes on one screen, clear of the host's close button`, async ({ page }) => {
    await open(page, control, width);
    await expect(page.getByText("Mission Control", { exact: true })).toBeVisible();
    const [active, items, log, workers] = await Promise.all([".mcx-pane.active", ".mcx-pane.items", ".mcx-pane.log", ".mcx-pane.workers"].map((s) => box(page, s)));
    // Active Item beside Items and the log, which are stacked; the threads along the bottom.
    expect(items.x).toBeGreaterThan(active.x + active.width - 1);
    expect(log.x).toBe(items.x);
    expect(log.y).toBeGreaterThan(items.y + items.height - 1);
    expect(workers.y).toBeGreaterThan(active.y + active.height - 1);
    // One screen: the page itself does not scroll.
    expect(await page.evaluate(() => document.documentElement.scrollHeight)).toBeLessThanOrEqual(760);
    // The top bar's content ends 44 pixels short of the right edge.
    const right = await page.locator(".mcx-top > *").evaluateAll((all) => Math.max(...all.map((e) => e.getBoundingClientRect().right)));
    expect(right).toBeLessThanOrEqual(width - 44);
    await expectNoControlCutOff(page);
  });
}

for (const width of [859, 420]) {
  test(`mission control at ${width} pixels stacks its panes and scrolls`, async ({ page }) => {
    await open(page, control, width);
    const [active, items, log] = await Promise.all([".mcx-pane.active", ".mcx-pane.items", ".mcx-pane.log"].map((s) => box(page, s)));
    expect(items.y).toBeGreaterThan(active.y + active.height - 1);
    expect(log.y).toBeGreaterThan(items.y + items.height - 1);
    expect(Math.round(items.x)).toBe(Math.round(active.x));
    await expectFits(page);
    await expectNoControlCutOff(page);
  });
}

test("mission control shows the active item, the items, the log and the threads", async ({ page }) => {
  await open(page, control, 1100);
  const active = page.locator(".mcx-pane.active");
  await expect(active.locator(".ptitle")).toHaveText("Active Item atelier-uwxy");
  await expect(active.getByText("Serve the Desk's reading routes from commits: the atelier-desk crate")).toBeVisible();
  await expect(active.getByText("Done when 1/2")).toBeVisible();
  await expect(active.getByRole("link", { name: "#131 open" })).toHaveAttribute("href", "https://github.com/spadaval/atelier/pull/131");
  await expect(active.getByText("· CI passing (2 passed, 1 skipped)")).toBeVisible();
  await expect(page.locator(".mcx-pane.items .ptitle")).toHaveText("Items 1/9");
  await expect(page.locator(".mcx-state")).toContainText("RUNNING");
  // The log reads the subjects, and marks what the human readied.
  const log = page.locator(".mcx-pane.log");
  await expect(log.getByText("Started atelier-uwxy")).toBeVisible();
  // The human readied every item five days ago, on the log's last page.
  for (let at = 0; at < 4; at++) await page.keyboard.press("]");
  await expect(log.locator(".mcx-log.waited").last()).toContainText("Ada readied 9 items");
  await expect(page.locator(".mcx-pane.workers .mcx-worker.live")).toHaveCount(2);
  // Choosing an item from the list makes it the active one.
  await page.locator(".mcx-row", { hasText: "atelier-1ie1" }).click();
  await expect(active.locator(".ptitle")).toHaveText("Active Item atelier-1ie1");
  await expect(active.getByText("merged")).toBeVisible();
});

test("mission control for a mission the Desk cannot find says so", async ({ page }) => {
  await open(page, "/mission/atelier/atelier-zzzz", 1100);
  await expect(page.locator(".mcx-loading")).toHaveText("No mission atelier-zzzz");
});

for (const width of [1300, 880]) {
  test(`the project home at ${width} pixels shows Now, Next, Later and the week`, async ({ page }) => {
    await open(page, "/home/atelier", width);
    const lane = (name: string) => page.locator(".rm-lane", { has: page.getByRole("heading", { name, exact: true }) });
    // Under way: missions with an item started or done, and started work in no mission.
    await expect(lane("Now").locator(".rm-title .t")).toHaveText([/Build the Desk/, /Move all five repositories/]);
    await expect(lane("Now").locator(".rm-loose .id")).toHaveText(["atelier-0019"]);
    await expect(lane("Next").locator(".rm-title .t")).toHaveText([/Ship Agent Factory/]);
    await expect(lane("Next").locator(".rm-loose .id")).toHaveText(["atelier-c1z3", "atelier-7w9s"]);
    await expect(lane("Next").getByText("blocked by atelier-52g7 (draft)")).toBeVisible();
    await expect(lane("Later").locator(".rm-title .t")).toHaveText([/Give the gardener/, /Try a plan whose stop is refused/, /A plan whose pull request will not close/]);
    await expect(lane("Later").locator(".rm-loose .id")).toHaveText(["atelier-52g7"]);
    // A mission's title opens its mission control.
    await expect(lane("Now").getByRole("link", { name: /Build the Desk/ })).toHaveAttribute("href", "/embed/mission/atelier/atelier-z4cr");
    const week = page.locator(".rm-week");
    await expect(week.locator(".rm-day")).toHaveCount(7);
    await expect(week.locator(".rm-day-head").last()).toContainText("Today");
    await expect(week.getByRole("link", { name: /Write the Desk's API as a document first/ })).toBeVisible();
    await expect(week).not.toContainText("atelier-old1");
    await expect(page.locator(".glance-foot")).toHaveText("Fetched 2m ago.");
    await expectNoControlCutOff(page);
  });
}

test("the project home draws its missions when the items in no mission cannot be read", async ({ page }) => {
  await page.route("**/api/roadmap**", (route) => route.fulfill({ status: 500, contentType: "application/json", body: JSON.stringify({ error: "The roadmap could not be read." }) }));
  await open(page, "/home/atelier", 1300);
  await expect(page.getByRole("alert")).toHaveText("The roadmap could not be read.");
  await expect(page.locator(".rm-lane")).toHaveCount(3);
  await expect(page.getByRole("link", { name: /Build the Desk/ }).first()).toBeVisible();
  await expect(page.locator(".rm-loose")).toHaveCount(0);
});

test("the inbox over a host's main area flows its sections into columns, and into one when narrow", async ({ page }) => {
  const lefts = () => page.locator(".inbox-repo").evaluateAll((all) => [...new Set(all.map((e) => Math.round(e.getBoundingClientRect().left)))]);
  await open(page, "/inbox", 1300);
  expect((await lefts()).length).toBeGreaterThan(1);
  await expectNoControlCutOff(page);
  await page.setViewportSize({ width: 720, height: 760 });
  await expect.poll(lefts).toHaveLength(1);
  await expectFits(page);
});
