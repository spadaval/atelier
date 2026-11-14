import { expect, test, type Page } from "@playwright/test";
import { missions, z4cr } from "./fixtures";
import { expectFits, expectOnlyTheActs, load, phone, serve } from "./helpers";

test.beforeEach(async ({ page }) => {
  await serve(page);
});

/** Opens the first finding in `scope` and checks it is laid out as wide as its list, not squeezed into a column. */
async function expectReadableFinding(page: Page, scope: string) {
  const finding = page.locator(`${scope} .findings > li`).first();
  await finding.locator("summary").click();
  const widths = await finding.evaluate((li) => ({
    list: li.closest(".findings")!.getBoundingClientRect().width,
    details: li.querySelector("details")!.getBoundingClientRect().width,
    title: li.querySelector(".f-title")!.getBoundingClientRect().width,
  }));
  expect(widths.details).toBeGreaterThan(widths.list - 4);
  expect(widths.title).toBeGreaterThan(120);
  await expect(finding.locator("details > .md")).toBeVisible();
}

test("the repositories page shows each repository's counts and missions, and a repository's error", async ({ page }) => {
  await load(page, "/");
  const atelier = page.getByRole("region", { name: "atelier" });
  await expect(atelier.getByTestId("repo-counts")).toHaveText("2 in progress2 plans proposed1 item needs you");
  await expect(atelier.getByTestId("fetched")).toHaveText("Fetched from origin 2 minutes ago. The Desk fetches every 5 minutes.");
  await expect(atelier.getByRole("link", { name: z4cr.mission.title })).toHaveAttribute("href", "/r/atelier/mission/atelier-z4cr");
  await expect(page.getByRole("region", { name: "broken" })).toContainText("missing field `status`");
  await expectOnlyTheActs(page);
});

test("the sidebar's footer names the commit the server was built from", async ({ page }) => {
  await load(page, "/");
  if (phone(page)) await page.getByRole("button", { name: "Menu" }).click();
  const built = page.getByTestId("built-from");
  await expect(built).toHaveText("Built from 5e1f0c2d9");
  await expect(built).toHaveAttribute("title", /5e1f0c2d9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d$/);
});

test("the mission page shows the title, status and counts atelier show prints", async ({ page }) => {
  await load(page, "/r/atelier/mission/atelier-z4cr");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(z4cr.mission.title);
  await expect(page.getByTestId("status")).toHaveText("Open");
  await expect(page.locator(".paper-head").getByTestId("counts")).toHaveText("5 draft2 ready1 building1 done");
  // Overview: what waits on the human first, then the mission's sections.
  const overview = page.locator(".paper-body");
  for (const heading of ["Waiting on you", "Why", "Success criteria", "Scope", "Authority", "Result", "Plan reviews"]) {
    await expect(overview.getByRole("heading", { name: new RegExp(`^${heading}`) })).toBeVisible();
  }
  await expect(page.locator(".outcome")).toContainText("service reached over the tailnet");
  await expect(overview.getByTestId("round")).toHaveText("Round 2: approved (gpt-6.1-sol).");
  await expect(page.getByTestId("read-from")).toHaveText("Read from the base");
  await expect(page.getByRole("complementary", { name: "Mission details" }).getByTestId("fetched")).toContainText("Fetched from origin 2 minutes ago.");
  await expect(overview).toContainText("Find how atelier serve can read T3 thread status");
  await expect(overview).toContainText("Changed since: .atelier/missions/atelier-z4cr/design.md");
  await expect(overview).toContainText("this clone lacks the commit reviewed");
  await expect(overview).toContainText("The strategy does not say where the app's types come from");
  await expectReadableFinding(page, ".paper-body");
  await expectOnlyTheActs(page);
});

test("the Work tab groups items by status in lifecycle order, and a done item shows its Report and fixes", async ({ page }) => {
  await load(page, "/r/atelier/mission/atelier-z4cr?tab=work");
  await expect(page.locator(".group-title")).toHaveText(["Draft 5", "Ready 2", "Building 1", "Done 1"]);
  const building = page.getByRole("region", { name: "Building" });
  await expect(building).toContainText("Held by thread:delegated-");
  await expect(building).toContainText("atelier-uwxy needs its Report written");
  await expect(page.getByRole("region", { name: "Draft" })).toContainText("Waits on atelier-uwxy (building)");
  await expect(page.getByRole("region", { name: "Ready" })).toContainText("atelier-psjw is blocked by atelier-uwxy");
  await page.getByRole("button", { name: "Report and fixes" }).click();
  const report = page.locator(".report");
  await expect(report).toContainText("All Done when lines hold.");
  await expect(report).toContainText("The inbox route omits draft missions");
  await expect(report).not.toContainText("The history route has no owner");
  await expect(report).not.toContainText("The thread join splits identities on slashes");
  await expect(report.locator(".findings > li")).toHaveCount(1);
  await expectFits(page);
});

test("the Documents tab renders a document and follows its relative links", async ({ page }) => {
  await load(page, "/r/atelier/mission/atelier-z4cr?tab=docs");
  const doc = page.locator(".doc-body");
  await expect(doc.getByRole("heading", { name: "Sizing" })).toBeVisible();
  await expect(doc.locator(".front")).toContainText("2026-11-11: amended");
  await expect(doc.getByText("api.rs")).not.toHaveAttribute("href");
  await doc.getByRole("link", { name: "notes" }).click();
  await expect(doc.getByRole("heading", { name: "Notes of atelier-z4cr" })).toBeVisible();
  await doc.getByRole("link", { name: "the design" }).click();
  await doc.getByRole("link", { name: "the intent" }).click();
  await expect(page).toHaveURL(/doc=docs%2Fproduct%2Fdesk\.md#why-it-exists$/);
  await expect(doc.getByRole("heading", { name: "The Desk: product intent" })).toBeVisible();
  await expectFits(page);
  // A link in a mission's section opens in its Documents tab too.
  await page.getByRole("tab", { name: /Overview/ }).click();
  await page.getByRole("link", { name: "ADR 0032" }).click();
  await expect(doc.getByRole("heading", { name: "ADR 0032: The Desk is the human's product" })).toBeVisible();
});

test("every mission in the rail opens a page that renders, and the loose count links nowhere", async ({ page }) => {
  await load(page, "/r/atelier");
  const rail = page.getByRole("navigation", { name: "Missions of atelier" });
  await expect(rail.getByTestId("loose")).toHaveText("80 open items are in no mission.");
  await expect(rail.getByTestId("loose").locator("a")).toHaveCount(0);
  await expect(rail.getByRole("link", { name: "Agents" })).toHaveAttribute("href", "/r/atelier/agents");
  const hrefs = await rail.locator("a.mcard").evaluateAll((links) => links.map((a) => a.getAttribute("href")));
  expect(hrefs.sort()).toEqual(missions.missions.map((m) => `/r/atelier/mission/${m.id}`).sort());
  for (const href of hrefs) {
    await load(page, href!);
    const id = href!.split("/").pop()!;
    await expect(page.getByRole("heading", { level: 1 })).toHaveText(missions.missions.find((m) => m.id === id)!.title);
    await expect(page.getByRole("alert")).toHaveCount(0);
  }
});

test("a repository opens on the mission with work in progress, and on a phone on its rail", async ({ page }) => {
  await load(page, "/r/atelier");
  if (phone(page)) {
    await expect(page.getByRole("navigation", { name: "Missions of atelier" })).toBeVisible();
    await expect(page.locator(".paper")).toBeHidden();
  } else {
    await expect(page.getByRole("heading", { level: 1 })).toHaveText(z4cr.mission.title);
  }
});

test("the item page shows its goal, Done when ticks, report, reviews and blockers", async ({ page }) => {
  await load(page, "/r/atelier/item/atelier-1ie1");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Write the Desk's API as a document first");
  await expect(page.getByTestId("status")).toHaveText("Done");
  await expect(page.locator(".done-when li[data-ticked=true]")).toHaveCount(2);
  await expect(page.getByText("Read from the base")).toBeVisible();
  await expect(page.locator(".issue-main")).toContainText("All Done when lines hold.");
  await expect(page.locator(".issue-main")).toContainText("Round 1: changes asked");
  await expect(page.locator(".issue-main")).toContainText("Deferred, taken as atelier-e05n");
  const finding = (title: string) => page.locator(".findings > li").filter({ hasText: title });
  await expect(finding("The inbox route omits draft missions")).toContainText("Fixed, confirmed in round 2");
  await expect(finding("The thread join splits identities on slashes")).toContainText("Withdrawn in round 2");
  await expect(finding("Drafts are in the inbox now")).toContainText("Confirms 1-aaaa.1 fixed");
  await expect(page.locator(".issue-main")).toContainText("fixed in 3333cccc");
  await expectReadableFinding(page, ".issue-main");
  await expectOnlyTheActs(page);

  await load(page, "/r/atelier/item/atelier-uwxy");
  await expect(page.getByText("Read from its branch")).toBeVisible();
  await expect(page.locator(".done-when li[data-ticked=false]")).toHaveCount(1);
  await expect(page.locator(".decisions")).toContainText("Answered by");
  await expect(page.getByRole("complementary", { name: "Item details" })).toContainText("Build the Desk");
  await page.getByRole("link", { name: "the API" }).click();
  await expect(page).toHaveURL(/\/r\/atelier\/mission\/atelier-z4cr\?tab=docs&doc=docs%2Farchitecture%2Fdesk\.md#routes$/);

  await load(page, "/r/atelier/item/atelier-94pv");
  await expect(page.getByRole("complementary", { name: "Item details" }).getByRole("link", { name: /atelier-uwxy/ })).toHaveAttribute("href", "/r/atelier/item/atelier-uwxy");
});

test("the inbox lists every repository's needs-human items by mission, its proposed plans, and the plans proposed and merged lately", async ({ page }) => {
  await load(page, "/inbox");
  const atelier = page.getByRole("region", { name: "atelier" });
  await expect(atelier.getByTestId("proposed").first()).toHaveAttribute("href", "/r/atelier/mission/atelier-295u");
  await expect(atelier.getByRole("link", { name: z4cr.mission.title })).toHaveAttribute("href", "/r/atelier/mission/atelier-z4cr");
  await expect(atelier.getByRole("link", { name: /Find how atelier serve/ })).toHaveAttribute("href", "/r/atelier/item/atelier-aab1");
  await expect(atelier.getByRole("heading", { name: "In no mission" })).toBeVisible();
  await expect(atelier.getByRole("link", { name: /Read the fleet site/ })).toBeVisible();
  await expect(page.getByRole("region", { name: "broken" })).toContainText("missing field `status`");
  await expect(page.getByRole("link", { name: "Inbox" })).toContainText("5");
  const lately = page.getByRole("region", { name: "Lately" });
  await expect(lately.getByTestId("news")).toHaveCount(2);
  await expect(lately.getByTestId("news").first()).toContainText("Proposed: Give the gardener");
  await expect(lately.getByTestId("news").first()).toHaveAttribute("href", "/r/atelier/mission/atelier-295u");
  await expect(lately.getByTestId("news").nth(1)).toContainText("Merged: Build the Desk");
  await expectOnlyTheActs(page);
});

test("the proposed page lists each plan with its branch, pull request and round, and record changes by branch", async ({ page }) => {
  await load(page, "/proposed");
  const plans = page.getByTestId("plan");
  await expect(plans).toHaveCount(2);
  await expect(plans.first()).toContainText("origin/atelier-295u");
  await expect(plans.first()).toContainText("round 1: approved, changes asked");
  await expect(plans.first().getByTestId("pull-request")).toHaveAttribute("href", "https://github.com/spadaval/atelier/pull/412");
  await expect(plans.nth(1)).toContainText("no pull request");
  const change = page.getByTestId("change");
  await expect(change).toContainText("origin/backlog/file-bugs");
  await expect(change).toContainText("atelier-n3w1 added: Fix the fetch timer");
  await expect(change).toContainText("atelier-94pv dropped");
  await expect(page.getByTestId("fetched").first()).toContainText("Fetched from origin");
  await expectOnlyTheActs(page);
});

test("the in-motion page lists each started item with its branch and thread, and each mission in progress with its lead", async ({ page }) => {
  await load(page, "/motion");
  const items = page.getByTestId("motion-item");
  await expect(items).toHaveCount(2);
  await expect(items.first()).toContainText("origin/atelier-uwxy");
  await expect(items.first().getByRole("link", { name: /Implement atelier-uwxy/ })).toHaveAttribute("href", /app\.t3\.codes/);
  await expect(items.first()).toContainText("Running");
  await expect(items.nth(1)).toContainText("thread:gone");
  await expect(items.nth(1)).toContainText("not running in T3 Code");
  const missions = page.getByTestId("motion-mission");
  await expect(missions).toContainText("Build the Desk");
  await expect(missions).toContainText("Lead the LoR habits mission");
  await expectOnlyTheActs(page);
});

test("the disputed page lists each rebuttal and split plan round, with each side's model where known", async ({ page }) => {
  await load(page, "/disputed");
  const rebuttal = page.getByTestId("rebuttal");
  await expect(rebuttal).toContainText("atelier-uwxy/1-cccc.2: The cache key leaves out HEAD");
  await expect(rebuttal).toContainText("Rebutted: HEAD is in the key since 3f2a");
  await expect(rebuttal).toContainText("Raised by codex-reviewer (gpt-6.1-sol)");
  await expect(rebuttal).toContainText("(claude-opus-5-5)");
  await expect(rebuttal).toContainText("Withdrawn in 2-dddd.1");
  const split = page.getByTestId("split-round");
  await expect(split).toContainText("Plan of atelier-295u, round 1");
  await expect(split).toContainText("grok-reviewer (model unknown)");
  await expectOnlyTheActs(page);
});

test("a repository that cannot be read says so in the rail, where a phone shows it", async ({ page }) => {
  await load(page, "/r/broken");
  const rail = page.getByRole("navigation", { name: "Missions of broken" });
  await expect(rail.getByRole("alert")).toContainText("No repository broken");
  await expect(rail.getByRole("alert")).toBeVisible();
  await expect(page.locator("[aria-busy=true]")).toHaveCount(0);
});

test("an unknown mission says so", async ({ page }) => {
  await load(page, "/r/atelier/mission/atelier-zzzz");
  await expect(page.getByRole("alert")).toContainText("No mission atelier-zzzz");
});
