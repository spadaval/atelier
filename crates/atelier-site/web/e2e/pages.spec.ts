import { expect, test, type Page } from "@playwright/test";

/** Every page of the fixture repositories, and the heading it shows. */
const pages: [string, string | RegExp][] = [["/", "Repositories"]];
for (const [repo, missions] of [
  ["alpha", ["alpha-m1", "alpha-m2"]],
  ["beta", ["beta-m1", "beta-m2"]],
] as const) {
  for (const id of missions) {
    pages.push([`/${repo}/mission/${id}`, /^(Brief|Debrief): /]);
    pages.push([`/${repo}/mission/${id}/brief`, /^Brief: /]);
    pages.push([`/${repo}/mission/${id}/debrief`, /^Debrief: /]);
  }
}
pages.push(["/alpha/mission/alpha-m1/strategy", "Strategy: docs/strategies/one.md"]);

/** Pages whose API answer is a 404, shown as a not-found page. */
const notFound = [
  "/alpha/mission/alpha-zzzz",
  "/alpha/mission/alpha-zzzz/brief",
  "/nowhere/mission/x/debrief",
  "/beta/mission/beta-m1/strategy",
  "/alpha/no/such/page",
];

/**
 * Loads `path` and checks it has no console error, beyond the one a browser
 * logs for an API 404 on a not-found page, and no horizontal scroll.
 */
async function load(page: Page, path: string, expected404: boolean) {
  const errors: string[] = [];
  page.on("console", (message) => {
    if (message.type() !== "error") return;
    if (expected404 && /status of 404/.test(message.text())) return;
    errors.push(message.text());
  });
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto(path);
  return errors;
}

async function scrollWidth(page: Page) {
  return page.evaluate(() => ({
    scroll: document.documentElement.scrollWidth,
    client: document.documentElement.clientWidth,
  }));
}

for (const [path, heading] of pages) {
  test(`${path} renders at 390 px`, async ({ page }) => {
    const errors = await load(page, path, false);
    await expect(page.getByRole("heading", { level: 1, name: heading })).toBeVisible();
    await expect(page.getByRole("alert")).toHaveCount(0);
    const { scroll, client } = await scrollWidth(page);
    expect(scroll).toBeLessThanOrEqual(client);
    expect(errors).toEqual([]);
  });
}

for (const path of notFound) {
  test(`${path} shows a not-found page`, async ({ page }) => {
    const errors = await load(page, path, true);
    await expect(page.getByRole("heading", { level: 1, name: "Not found" })).toBeVisible();
    const { scroll, client } = await scrollWidth(page);
    expect(scroll).toBeLessThanOrEqual(client);
    expect(errors).toEqual([]);
  });
}

test("the server sends the app's JavaScript and CSS with their content types", async ({ page, request }) => {
  const assets: Record<string, string> = {};
  page.on("response", (response) => {
    const url = new URL(response.url());
    if (url.pathname.startsWith("/assets/")) assets[url.pathname] = response.headers()["content-type"] ?? "";
  });
  for (const path of ["/", "/alpha/mission/alpha-m1/brief"]) {
    await page.goto(path);
    await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
  }
  const types = Object.entries(assets);
  expect(types.some(([path, type]) => path.endsWith(".js") && type.startsWith("text/javascript"))).toBe(true);
  expect(types.some(([path, type]) => path.endsWith(".css") && type.startsWith("text/css"))).toBe(true);

  const deep = await request.get("/alpha/mission/alpha-m1/debrief");
  expect(deep.status()).toBe(200);
  expect(deep.headers()["content-type"]).toContain("text/html");
  const missing = await request.get("/assets/index-missing.js");
  expect(missing.status()).toBe(404);
  expect(await missing.text()).not.toContain("<!doctype html>");
});

test("a mission link on the list opens its review", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("link", { name: /Ship the loader/ }).click();
  await expect(page).toHaveURL("/alpha/mission/alpha-m2");
  await expect(page.getByRole("heading", { level: 1, name: "Debrief: Ship the loader" })).toBeVisible();
});
