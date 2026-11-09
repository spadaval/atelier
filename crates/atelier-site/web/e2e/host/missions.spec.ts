import { expect, test } from "@playwright/test";
import type { ReposResponse } from "../../src/api/ReposResponse";

// Every mission of every repository the server reads: its brief and debrief
// render with no console error, no error alert and no horizontal scroll.
test("every mission's brief and debrief renders", async ({ page, request }) => {
  const repos = (await (await request.get("/api/repos")).json()) as ReposResponse;
  const paths = repos.repos.flatMap((repo) =>
    repo.missions.flatMap((mission) => [
      `/${repo.repo}/mission/${mission.id}/brief`,
      `/${repo.repo}/mission/${mission.id}/debrief`,
    ]),
  );
  const failures: string[] = [];
  let errors: string[] = [];
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });
  page.on("pageerror", (error) => errors.push(error.message));
  for (const path of paths) {
    errors = [];
    await page.goto(path);
    const heading = page.getByRole("heading", { level: 1, name: /^(Brief|Debrief): / });
    const alert = page.getByRole("alert");
    await expect(heading.or(alert)).toBeVisible({ timeout: 60_000 });
    if (await alert.count()) errors.push(`alert: ${await alert.first().textContent()}`);
    const wide = await page.evaluate(
      () => document.documentElement.scrollWidth > document.documentElement.clientWidth,
    );
    if (wide) errors.push("scrolls horizontally");
    if (errors.length > 0) failures.push(`${path}: ${errors.join("; ")}`);
  }
  console.log(
    `${repos.repos.length} repositories, ${paths.length / 2} missions, ${paths.length} pages, ${failures.length} failed`,
  );
  expect(failures).toEqual([]);
});
