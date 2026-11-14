import { expect, type Page } from "@playwright/test";
import { World } from "./fixtures";

/** Answers every /api request of `page` from a fresh fixture server, and returns it. */
export async function serve(page: Page): Promise<World> {
  const world = new World();
  await page.route("**/api/**", (route) => {
    const request = route.request();
    const { status, body } = world.respond(request.method(), new URL(request.url()), request.headers()["x-atelier-desk"], request.postData());
    return route.fulfill({ status, contentType: "application/json", body: JSON.stringify(body) });
  });
  return world;
}

export const phone = (page: Page) => (page.viewportSize()?.width ?? 1000) < 600;

export async function expectFits(page: Page) {
  const wider = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  expect(wider, "the page is wider than the screen").toBeLessThanOrEqual(0);
}

/** Loads a page and fails on an uncaught error or a page wider than the screen. */
export async function load(page: Page, path: string) {
  const errors: string[] = [];
  page.on("pageerror", (e) => errors.push(String(e)));
  await page.goto(path);
  await page.waitForLoadState("networkidle");
  await expectFits(page);
  expect(errors).toEqual([]);
}

/**
 * The Desk's only act controls are a plan's stop and Start a planning
 * session: no other button names an act, and no form but the stop's is on
 * the page.
 */
export async function expectOnlyTheActs(page: Page) {
  // A button that only opens or folds part of the page is no act.
  const names = await page
    .getByRole("button")
    .evaluateAll((buttons) => buttons.filter((b) => !b.hasAttribute("aria-expanded")).map((b) => (b.getAttribute("aria-label") ?? b.textContent ?? "").trim()));
  const acts = names.filter((n) => /go|answer|decide|ready|start|session|close|drop|merge|approve|submit|review|note|transition|history|activity|stop/i.test(n));
  for (const name of acts) expect(name).toMatch(/^(Stop this plan|Yes, stop it|Start a planning session|Yes, start it)$/);
  const forms = await page.locator("form").evaluateAll((all) => all.map((f) => f.getAttribute("aria-label") ?? ""));
  for (const form of forms) expect(form).toBe("Stop this plan");
}
