import { cleanup, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import type { BoardResponse } from "../api/BoardResponse";
import { body, renderAt, serveFixtures } from "../test/api";

afterEach(() => cleanup());

async function heading(text: string | RegExp) {
  return screen.findByRole("heading", { level: 1, name: text });
}

/** The section under the `h2` named `title`. */
function section(title: string): HTMLElement {
  return screen.getByRole("heading", { level: 2, name: title }).closest("section")!;
}

/** The list row in `within` holding the item `id`. */
function row(container: HTMLElement, id: string): HTMLElement {
  return within(container).getByText(id, { selector: ".item > code" }).closest("li")!;
}

describe("the board", () => {
  it("shows each open mission with its progress, linking its page", async () => {
    serveFixtures();
    renderAt("/alpha");
    await heading("Board: alpha");
    const missions = section("Missions");
    const link = within(missions).getByRole("link", { name: /Read plans on the base branch/ });
    expect(link).toHaveAttribute("href", "/alpha/mission/alpha-m1");
    expect(link).toHaveTextContent("0 of 2 items done, 1 of 2 criteria proven");
    // alpha-m2 is done, so it is not an open mission.
    expect(within(missions).queryByText("alpha-m2")).toBeNull();
  });

  it("shows work in progress on its branch with its review state, findings and how far behind", async () => {
    serveFixtures();
    renderAt("/alpha");
    await heading("Board: alpha");
    const work = row(section("In Progress"), "alpha-a1");
    expect(within(work).getByRole("link")).toHaveAttribute("href", "/alpha/item/alpha-a1");
    for (const text of ["in_progress", "⚠ not approved", "⚠ 1 finding open", "behind 1"]) {
      expect(within(work).getByText(text)).toBeInTheDocument();
    }
    expect(within(work).getByText("alpha-a1", { selector: ".cue code" })).toBeInTheDocument();
  });

  it("words each review state as atelier status does", async () => {
    for (const [review, words] of [
      ["approved", "✓ approved"],
      ["approval_stale", "⚠ approval stale"],
      ["unreviewed", "⚠ unreviewed"],
    ] as const) {
      const board = body<BoardResponse>("/api/alpha/board");
      board.in_progress[0].on_branch!.review = review;
      serveFixtures({ "/api/alpha/board": board });
      renderAt("/alpha");
      await heading("Board: alpha");
      expect(within(row(section("In Progress"), "alpha-a1")).getByText(words)).toBeInTheDocument();
      cleanup();
    }
  });

  it("lists ready and needs-human work in the order the API gives", async () => {
    serveFixtures();
    renderAt("/alpha");
    await heading("Board: alpha");
    expect(within(section("Ready Work")).getByRole("link")).toHaveAttribute("href", "/alpha/item/alpha-a1");
    expect(within(section("Needs a Human")).getByRole("link")).toHaveAttribute("href", "/alpha/item/alpha-a2");
  });

  it("groups open work by mission, then by label, each with where it stands", async () => {
    serveFixtures();
    renderAt("/alpha");
    await heading("Board: alpha");
    const open = section("Open Work");
    const groups = within(open).getAllByRole("heading", { level: 3 });
    expect(groups.map((group) => group.textContent)).toEqual(["Read plans on the base branch", "Label ops"]);
    expect(within(groups[0]).getByRole("link")).toHaveAttribute("href", "/alpha/mission/alpha-m1");
    expect(within(row(open, "alpha-a1")).getByText("ready")).toBeInTheDocument();
    expect(within(row(open, "alpha-a2")).getByText("needs human")).toBeInTheDocument();
    const loose = row(open, "alpha-l1");
    expect(within(loose).getByText("blocked")).toBeInTheDocument();
    expect(within(loose).getByText("loose_end")).toBeInTheDocument();
  });

  it("lists work done in the last 14 days, newest first", async () => {
    serveFixtures();
    renderAt("/alpha");
    await heading("Board: alpha");
    const done = section("Done in the Last 14 Days");
    const ids = within(done)
      .getAllByRole("link")
      .map((link) => link.getAttribute("href"));
    expect(ids).toEqual(["/alpha/item/alpha-m2", "/alpha/item/alpha-b1"]);
  });

  it("says when a section is empty", async () => {
    serveFixtures();
    renderAt("/beta");
    await heading("Board: beta");
    expect(within(section("In Progress")).getByText("No work in progress.")).toBeInTheDocument();
    expect(within(section("Done in the Last 14 Days")).getByText("Nothing done.")).toBeInTheDocument();
  });

  it("is linked from the repository list, and links back from brief, debrief and item pages", async () => {
    serveFixtures();
    renderAt("/");
    await heading("Repositories");
    expect(screen.getByRole("link", { name: "alpha" })).toHaveAttribute("href", "/alpha");
    cleanup();
    for (const [path, title] of [
      ["/alpha/mission/alpha-m1/brief", "Brief: Read plans on the base branch"],
      ["/alpha/mission/alpha-m2/debrief", "Debrief: Ship the loader"],
      ["/alpha/item/alpha-b1", "Write the loader"],
    ]) {
      serveFixtures();
      renderAt(path);
      await heading(title);
      const crumbs = screen.getByRole("navigation", { name: "Breadcrumbs" });
      expect(within(crumbs).getByRole("link", { name: "alpha" })).toHaveAttribute("href", "/alpha");
      cleanup();
    }
  });
});

describe("an item page", () => {
  it("shows a task's record, links and review events", async () => {
    serveFixtures();
    renderAt("/alpha/item/alpha-b1");
    await heading("Write the loader");
    expect(screen.getByRole("heading", { level: 3, name: "Goal" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { level: 3, name: "Report" })).toBeInTheDocument();
    expect(screen.getByText("The loader reads every record.")).toBeInTheDocument();
    const links = section("Links");
    expect(within(links).getByText("advances")).toBeInTheDocument();
    expect(within(links).getByRole("link")).toHaveAttribute("href", "/alpha/mission/alpha-m2");
    const reviews = section("Reviews");
    expect(within(reviews).getByText("approval")).toBeInTheDocument();
    expect(within(reviews).getByText("alpha-r1")).toBeInTheDocument();
    expect(within(reviews).getByText("Looks right.")).toBeInTheDocument();
  });

  it("shows a decision's Question and Options", async () => {
    serveFixtures();
    renderAt("/alpha/item/alpha-a2");
    await heading("Choose the server");
    expect(screen.getByRole("heading", { level: 3, name: "Question" })).toBeInTheDocument();
    expect(screen.getByText("Which server?")).toBeInTheDocument();
    expect(screen.getByRole("heading", { level: 3, name: "Options" })).toBeInTheDocument();
    expect(screen.getByText("axum on tokio.").tagName).toBe("LI");
    expect(screen.getByText("needs human")).toBeInTheDocument();
  });

  it("shows a loose end with what it waits on", async () => {
    serveFixtures();
    renderAt("/alpha/item/alpha-l1");
    await heading("Check the loader on the server");
    expect(screen.getByText("loose_end")).toBeInTheDocument();
    expect(screen.getByText("ops")).toBeInTheDocument();
    expect(screen.getByRole("heading", { level: 3, name: "Notes" })).toBeInTheDocument();
    expect(screen.getByText("2026-01-01 The server is not set up yet.")).toBeInTheDocument();
    const links = section("Links");
    expect(within(links).getByText("blocked by")).toBeInTheDocument();
    expect(within(links).getByRole("link")).toHaveAttribute("href", "/alpha/item/alpha-a1");
    expect(within(section("Reviews")).getByText("No review events on this item.")).toBeInTheDocument();
  });

  it("shows a finding's recommendation and decision", async () => {
    const item = body<{ item: { reviews: unknown[] } }>("/api/alpha/item/alpha-b1");
    item.item.reviews.push({
      id: "alpha-r9",
      event: "finding",
      identity: "fixture-reviewer",
      commit: "0123456789abcdef",
      created_at: "2026-01-01T01:00:00Z",
      recommendation: "fix_now",
      disposition: {
        decision: "wont_fix",
        identity: "sumeet",
        recorded_at: "2026-01-02T00:00:00Z",
        reason: "Not worth it here.",
        follow_up: null,
      },
      summary: "R1 low: a nit.",
    });
    serveFixtures({ "/api/alpha/item/alpha-b1": item });
    renderAt("/alpha/item/alpha-b1");
    await heading("Write the loader");
    const finding = within(section("Reviews")).getByText("alpha-r9").closest("li")!;
    expect(finding).toHaveTextContent("recommends fix_now");
    expect(finding).toHaveTextContent("Decided wont_fix by sumeet");
    expect(finding).toHaveTextContent("Not worth it here.");
  });

  it("is a not-found page for an unknown item", async () => {
    serveFixtures();
    renderAt("/alpha/item/alpha-zzzz");
    await heading("Not found");
    expect(screen.getByRole("alert")).toHaveTextContent("no item alpha-zzzz in alpha");
  });
});
