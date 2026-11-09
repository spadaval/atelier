import { expect, test } from "@playwright/test";
import type { InboxResponse } from "../../src/api/InboxResponse";

// Every link on the inbox of the repositories the server reads: each item
// page and mission page renders with no console error, no error alert and
// no horizontal scroll.
test("every inbox link opens a page that renders", async ({ page, request }) => {
  const inbox = (await (await request.get("/api/inbox")).json()) as InboxResponse;
  const paths = new Set<string>();
  for (const repo of inbox.repos) {
    for (const { item, missions } of repo.items) {
      paths.add(`/${repo.repo}/item/${item.id}`);
      for (const mission of missions) paths.add(`/${repo.repo}/mission/${mission.id}`);
    }
    for (const mission of repo.drafts) paths.add(`/${repo.repo}/mission/${mission.id}`);
  }
  const failures: string[] = [];
  let errors: string[] = [];
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });
  page.on("pageerror", (error) => errors.push(error.message));

  // The inbox itself shows every link the API names.
  await page.goto("/inbox");
  await expect(page.getByRole("heading", { level: 1, name: "Inbox" })).toBeVisible({ timeout: 60_000 });
  const shown = new Set(
    await page.locator("main a").evaluateAll((links) => links.map((link) => link.getAttribute("href") ?? "")),
  );
  for (const path of paths) if (!shown.has(path)) failures.push(`/inbox does not link ${path}`);

  for (const path of paths) {
    errors = [];
    await page.goto(path);
    const heading = page.getByRole("heading", { level: 1 });
    await expect(heading).toBeVisible({ timeout: 60_000 });
    await expect(heading).not.toHaveText("Loading…", { timeout: 60_000 });
    const alert = page.getByRole("alert");
    if (await alert.count()) errors.push(`alert: ${await alert.first().textContent()}`);
    const wide = await page.evaluate(
      () => document.documentElement.scrollWidth > document.documentElement.clientWidth,
    );
    if (wide) errors.push("scrolls horizontally");
    if (errors.length > 0) failures.push(`${path}: ${errors.join("; ")}`);
  }
  const items = inbox.repos.reduce((sum, repo) => sum + repo.items.length, 0);
  const drafts = inbox.repos.reduce((sum, repo) => sum + repo.drafts.length, 0);
  console.log(
    `${inbox.repos.length} repositories, ${items} items, ${drafts} draft missions, ${paths.size} pages, ${failures.length} failed`,
  );
  expect(failures).toEqual([]);
});
