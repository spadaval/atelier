import { expect, test, type Page } from "@playwright/test";
import { z4cr } from "./fixtures";
import { expectNoControlCutOff, load, serve } from "./helpers";

// The embedded mode, for a host such as T3 Code that shows the Desk in a
// panel 400 to 750 pixels wide: a path under /embed turns it on.

test.beforeEach(async ({ page }) => {
  await serve(page);
});

const pages = {
  inbox: "/inbox",
  repositories: "/",
  mission: "/r/atelier/mission/atelier-z4cr",
  item: "/r/atelier/item/atelier-1ie1",
  proposed: "/proposed",
  "in motion": "/motion",
  disputed: "/disputed",
};

/** Checks that the page draws no chrome of its own, keeps its breadcrumb, and links to the inbox. */
async function expectNoChrome(page: Page) {
  await expect(page.getByRole("complementary", { name: "Pages" })).toHaveCount(0);
  await expect(page.locator(".brand")).toHaveCount(0);
  await expect(page.getByTestId("built-from")).toHaveCount(0);
  await expect(page.getByRole("button", { name: /^Switch to/ })).toHaveCount(0);
  await expect(page.getByRole("button", { name: "Menu" })).toHaveCount(0);
  await expect(page.getByRole("navigation", { name: "Where you are" })).not.toBeEmpty();
  await expect(page.getByRole("link", { name: "Inbox", exact: true })).toHaveAttribute("href", "/embed/inbox");
}

for (const width of [420, 720]) {
  for (const [name, path] of Object.entries(pages)) {
    test(`embedded at ${width} pixels, the ${name} page draws no chrome, fits, and cuts off no control`, async ({ page }) => {
      await page.setViewportSize({ width, height: 800 });
      await load(page, `/embed${path}`);
      await expectNoChrome(page);
      await expectNoControlCutOff(page);
    });
  }
}

test("the embedded mode stays on as the user follows links inside the Desk", async ({ page }) => {
  await page.setViewportSize({ width: 420, height: 800 });
  await load(page, "/embed/inbox");
  const atelier = page.getByRole("region", { name: "atelier" });
  await expect(atelier.getByRole("link", { name: z4cr.mission.title })).toHaveAttribute("href", "/embed/r/atelier/mission/atelier-z4cr");
  await atelier.getByRole("link", { name: z4cr.mission.title }).click();
  await expect(page).toHaveURL(/\/embed\/r\/atelier\/mission\/atelier-z4cr$/);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(z4cr.mission.title);
  await expectNoChrome(page);
  // Up the breadcrumb to the repository, then back to the inbox from the top bar.
  await page.getByRole("navigation", { name: "Where you are" }).getByRole("link").first().click();
  await expect(page).toHaveURL(/\/embed\/r\/atelier$/);
  await expectNoChrome(page);
  await page.getByRole("link", { name: "Inbox", exact: true }).click();
  await expect(page).toHaveURL(/\/embed\/inbox$/);
  // A reload keeps the mode, since it lives in the path.
  await page.reload();
  await expectNoChrome(page);
});

test("without /embed the Desk keeps its sidebar, brand and theme toggle", async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await load(page, "/inbox");
  await expect(page.locator(".side .brand")).toBeVisible();
  await expect(page.getByRole("button", { name: /^Switch to/ })).toBeVisible();
  await expect(page.getByTestId("built-from")).toBeVisible();
  await expect(page.locator(".inbox-btn")).toHaveCount(0);
});
