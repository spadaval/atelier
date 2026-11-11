import { expect, test, type Page } from "@playwright/test";
import { answers, codexChild, live, type World } from "./fixtures";
import { expectFits, expectOnlyTheActs, load, serve } from "./helpers";

let world: World;

test.beforeEach(async ({ page }) => {
  world = await serve(page);
});

test.afterEach(async () => {
  // Every act and session the app sent carried the app's header, to the two routes that take them.
  for (const sent of world.sent) {
    expect(sent.header).toBe("1");
    expect(["/api/act", "/api/sessions"]).toContain(sent.path);
  }
});

const details = (page: Page, name: "Mission details" | "Item details") => page.getByRole("complementary", { name });
const agents = (page: Page, name: "Mission details" | "Item details") => details(page, name).locator("section.card").filter({ has: page.getByRole("heading", { name: "Agents" }) });

test("a mission's page shows its agents: live threads link to T3 Code, others are named whole", async ({ page }) => {
  await load(page, "/r/atelier/mission/atelier-z4cr");
  const card = agents(page, "Mission details");
  const running = card.getByTestId("agent").filter({ hasText: live.uwxy.title });
  await expect(running.getByRole("link", { name: live.uwxy.title })).toHaveAttribute("href", live.uwxy.link);
  await expect(running).toContainText("Implementer");
  await expect(running).toContainText("Running");
  await expect(running).toContainText("claude-opus-5-5");
  await expect(running.getByRole("link", { name: "atelier-uwxy", exact: true })).toHaveAttribute("href", "/r/atelier/item/atelier-uwxy");
  await expect(card.getByTestId("agent").filter({ hasText: live.asking.title })).toContainText("Waiting on a person");
  const reviewer = card.getByTestId("agent").filter({ hasText: codexChild });
  await expect(reviewer).toContainText("Reviewer");
  await expect(reviewer).toContainText("not running in T3 Code");
  // The identity is whole on the page, and in no link.
  await expect(reviewer.locator(".ident")).toHaveText(codexChild);
  await expect(reviewer.locator(".ident")).toHaveAttribute("title", codexChild);
  const hrefs = await page.locator("a").evaluateAll((links) => links.map((a) => a.getAttribute("href") ?? ""));
  for (const href of hrefs) expect(href).not.toContain("review-child");
  await expect(card.getByTestId("t3-note")).toHaveCount(0);
  await expectOnlyTheActs(page);
});

test("when T3 Code cannot be reached a mission's and an item's pages render and say threads are unavailable", async ({ page }) => {
  for (const path of ["/r/atelier/mission/atelier-qr5s", "/r/atelier/item/atelier-u1ko"]) {
    await load(page, path);
    const card = agents(page, path.includes("mission") ? "Mission details" : "Item details");
    await expect(card.getByTestId("t3-note")).toContainText("Threads are unavailable: T3 Code cannot be reached.");
    await expect(card.getByTestId("t3-note")).toContainText("connection refused");
    // Not known to be stopped either: no claim about T3 Code beyond the note.
    await expect(card.getByTestId("agent")).toContainText(codexChild);
    await expect(card.getByTestId("agent")).not.toContainText("not running");
    await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
  }
});

test("an item's page shows its agents, and a ready item with none says no thread has started it", async ({ page }) => {
  await load(page, "/r/atelier/item/atelier-uwxy");
  const card = agents(page, "Item details");
  await expect(card.getByRole("link", { name: live.uwxy.title })).toHaveAttribute("href", live.uwxy.link);
  await expect(card.getByTestId("agent").filter({ hasText: codexChild })).toContainText("not running in T3 Code");
  await load(page, "/r/atelier/item/atelier-psjw");
  await expect(agents(page, "Item details")).toContainText("No thread has started this item.");
  await expect(agents(page, "Item details").getByTestId("agent")).toHaveCount(0);
});

test("the live-threads page lists T3 Code's threads with what each holds, and a repository's only its own", async ({ page }) => {
  await load(page, "/agents");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("3 threads are working, and 1 waits on a person.");
  const working = page.getByRole("region", { name: "Working on missions and items" });
  const uwxy = working.getByTestId("thread").filter({ hasText: live.uwxy.title });
  await expect(uwxy.getByRole("link", { name: live.uwxy.title })).toHaveAttribute("href", live.uwxy.link);
  await expect(uwxy.getByRole("link", { name: "atelier-uwxy", exact: true })).toHaveAttribute("href", "/r/atelier/item/atelier-uwxy");
  await expect(uwxy).toContainText("Implementer on");
  await expect(uwxy.getByRole("link", { name: "#131 open" })).toHaveAttribute("href", "https://github.com/spadaval/atelier/pull/131");
  const lor = working.getByTestId("thread").filter({ hasText: live.lor.title });
  await expect(lor.getByRole("link", { name: "lor-h4bt", exact: true })).toHaveAttribute("href", "/r/lor/mission/lor-h4bt");
  await expect(page.getByRole("region", { name: "Holding nothing in the records" })).toContainText(live.free.title);
  await expect(page.locator(".page")).not.toContainText(/effort|context|token/i);
  await expectOnlyTheActs(page);

  await page.getByRole("navigation", { name: "Repositories" }).getByRole("link", { name: "atelier" }).click();
  await expect(page).toHaveURL(/\/r\/atelier\/agents$/);
  await expect(page.getByTestId("thread")).toHaveCount(2);
  await expect(page.locator(".page")).not.toContainText(live.lor.title);
  await expect(page.locator(".page")).not.toContainText(live.free.title);
  await expectFits(page);
});

test("the inbox lists the threads waiting on a person, each linking to T3 Code", async ({ page }) => {
  await load(page, "/inbox");
  const atelier = page.getByRole("region", { name: "atelier" });
  const waiting = atelier.getByTestId("thread");
  await expect(waiting).toHaveCount(1);
  await expect(waiting.getByRole("link", { name: live.asking.title })).toHaveAttribute("href", live.asking.link);
  await expect(waiting).toContainText("Waiting on a person");
  await expect(waiting.getByRole("link", { name: "atelier-aab1", exact: true })).toHaveAttribute("href", "/r/atelier/item/atelier-aab1");
});

test("Say go asks once, sends ready with the app's header, and the page then shows the mission ready", async ({ page }) => {
  await load(page, "/r/atelier/mission/atelier-295u");
  await expect(page.getByTestId("status")).toHaveText("Draft");
  await expect(page.getByRole("link", { name: "Inbox" })).toContainText("6");
  await page.getByRole("button", { name: "Say go" }).click();
  const confirm = page.getByRole("group", { name: "Confirm saying go" });
  await expect(confirm).toBeVisible();
  expect(world.sent).toEqual([]);
  await confirm.getByRole("button", { name: "Yes, say go" }).click();
  await expect(page.getByTestId("status")).toHaveText("Ready");
  expect(world.sent).toEqual([{ path: "/api/act", header: "1", body: { act: "ready", repo: "atelier", ids: ["atelier-295u"] } }]);
  const done = page.getByTestId("done");
  await expect(done).toContainText("You said go, committed as 76cbb6c0.");
  await expect(done).toContainText("Made ready, as sumeet said: atelier-295u.");
  await expect(page.getByRole("button", { name: "Say go" })).toHaveCount(0);
  await expect(page.locator(".paper-head")).not.toContainText("This mission is a draft");
  // The rest of the app is read again too: the sidebar's inbox count drops the draft.
  await expect(page.getByRole("link", { name: "Inbox" })).toContainText("5");
});

test("Say go is offered only where go would be accepted, and Not yet sends nothing", async ({ page }) => {
  await load(page, "/r/atelier/mission/atelier-z4cr");
  await expect(page.getByRole("button", { name: "Say go" })).toHaveCount(0);
  await load(page, "/r/atelier/mission/atelier-295u");
  await page.getByRole("button", { name: "Say go" }).click();
  await page.getByRole("button", { name: "Not yet" }).click();
  await expect(page.getByRole("button", { name: "Say go" })).toBeVisible();
  expect(world.sent).toEqual([]);
});

test("a refused go shows the refusal's text and next step as given, and nothing else changes", async ({ page }) => {
  await load(page, "/r/atelier/mission/atelier-k9rf");
  await page.getByRole("button", { name: "Say go" }).click();
  await page.getByRole("button", { name: "Yes, say go" }).click();
  const refusal = page.getByRole("alert");
  await expect(refusal).toContainText(answers.goRefused.text);
  await expect(refusal).toContainText(answers.goRefused.next);
  await expect(refusal).toContainText(answers.goRefused.rules[0]);
  await expect(page.getByTestId("status")).toHaveText("Draft");
  await expect(page.getByRole("button", { name: "Say go" })).toBeVisible();
  await expectFits(page);
});

test("an answer on an item's page is sent with its settlement and then shows as not yet committed", async ({ page }) => {
  await load(page, "/r/atelier/item/atelier-uwxy");
  const decision = page.getByTestId("decision").first();
  await expect(decision).toContainText("Not answered yet.");
  await decision.getByRole("button", { name: "Answer the question on atelier-uwxy" }).click();
  const form = page.getByRole("form", { name: "Answer on atelier-uwxy" });
  await expect(form.getByRole("button", { name: "Send the answer" })).toBeDisabled();
  await form.getByLabel("Your answer, in one line").fill("Yes, key it by HEAD too.");
  await form.getByLabel("Defer it").check();
  await form.getByRole("button", { name: "Send the answer" }).click();
  await expect(decision.getByTestId("pending")).toHaveText("Answered from the Desk, not committed yet");
  expect(world.sent).toEqual([
    { path: "/api/act", header: "1", body: { act: "decide", repo: "atelier", id: "atelier-uwxy", answer: "Yes, key it by HEAD too.", settle: "defer" } },
  ]);
  await expect(decision).toContainText("Yes, key it by HEAD too.");
  await expect(decision).toContainText("deferred");
  await expect(decision).not.toContainText("Not answered yet.");
  await expect(page.getByRole("form")).toHaveCount(0);
  await expectFits(page);

  // The mission's page shows it pending too, and no longer waiting on you.
  await load(page, "/r/atelier/mission/atelier-z4cr");
  const decided = page.locator(".prose-sect").filter({ has: page.getByRole("heading", { name: /^Decided on the way/ }) });
  await expect(decided).toContainText("Yes, key it by HEAD too.");
  await expect(decided.getByTestId("pending")).toBeVisible();
  await expect(page.getByTestId("question")).toHaveCount(0);
});

test("an answer from the inbox shows as answered and not committed, and a refused one as given", async ({ page }) => {
  await load(page, "/inbox");
  const atelier = page.getByRole("region", { name: "atelier" });
  await atelier.getByRole("button", { name: "Answer the question on atelier-u1ko" }).click();
  let form = atelier.getByRole("form", { name: "Answer on atelier-u1ko" });
  await expect(form.getByRole("radio")).toHaveCount(0);
  await form.getByLabel("Your answer, in one line").fill("Not yet.");
  await form.getByRole("button", { name: "Send the answer" }).click();
  await expect(form.getByRole("alert")).toContainText(answers.answerRefused.text);
  await expect(form.getByRole("alert")).toContainText(answers.answerRefused.next);
  await expect(atelier.getByTestId("question")).toHaveCount(2);

  await atelier.getByRole("button", { name: "Answer the question on atelier-uwxy" }).click();
  form = atelier.getByRole("form", { name: "Answer on atelier-uwxy" });
  await form.getByLabel("Your answer, in one line").fill("Yes.");
  await form.getByRole("button", { name: "Send the answer" }).click();
  const answered = atelier.getByTestId("answered");
  await expect(answered.getByTestId("pending")).toHaveText("Answered from the Desk, not committed yet");
  await expect(answered.getByRole("link", { name: /Should the cache key/ })).toHaveAttribute("href", "/r/atelier/item/atelier-uwxy");
  await expect(atelier.getByTestId("question")).toHaveCount(1);
  expect(world.sent.at(-1)).toEqual({ path: "/api/act", header: "1", body: { act: "decide", repo: "atelier", id: "atelier-uwxy", answer: "Yes.", settle: null } });
  await expectFits(page);
});

test("a mission's page starts a planning session and links to its thread, or shows the refusal as given", async ({ page }) => {
  await load(page, "/r/atelier/mission/atelier-z4cr");
  await page.getByRole("button", { name: "Start a planning session" }).click();
  expect(world.sent).toEqual([]);
  await page.getByRole("group", { name: "Confirm starting a planning session" }).getByRole("button", { name: "Yes, start it" }).click();
  await expect(page.getByTestId("started").getByRole("link", { name: "Open the thread" })).toHaveAttribute("href", answers.started.link);
  expect(world.sent).toEqual([{ path: "/api/sessions", header: "1", body: { repo: "atelier", mission: "atelier-z4cr" } }]);
  await expect(page.getByRole("button", { name: "Start a planning session" })).toHaveCount(0);

  await load(page, "/r/atelier/mission/atelier-qr5s");
  await page.getByRole("button", { name: "Start a planning session" }).click();
  await page.getByRole("button", { name: "Yes, start it" }).click();
  await expect(page.getByRole("alert")).toContainText(answers.sessionRefused.text);
  await expect(page.getByRole("button", { name: "Start a planning session" })).toBeVisible();
  await expectFits(page);
});
