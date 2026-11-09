import { cleanup, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import type { BriefResponse } from "../api/BriefResponse";
import type { DebriefResponse } from "../api/DebriefResponse";
import type { MissionResponse } from "../api/MissionResponse";
import type { ReposResponse } from "../api/ReposResponse";
import type { StrategyResponse } from "../api/StrategyResponse";
import { body, renderAt, serveFixtures } from "../test/api";

afterEach(() => cleanup());

async function heading(text: string | RegExp) {
  return screen.findByRole("heading", { level: 1, name: text });
}

describe("the repository list", () => {
  it("lists every repository with a link to each mission", async () => {
    serveFixtures();
    renderAt("/");
    await heading("Repositories");
    for (const name of ["alpha", "beta"]) expect(screen.getByRole("heading", { level: 2, name })).toBeInTheDocument();
    for (const [repo, id, title] of [
      ["alpha", "alpha-m1", "Read plans on the base branch"],
      ["alpha", "alpha-m2", "Ship the loader"],
      ["beta", "beta-m1", "Propose the board"],
      ["beta", "beta-m2", "Build the inbox"],
    ]) {
      const link = screen.getByRole("link", { name: new RegExp(title) });
      expect(link).toHaveAttribute("href", `/${repo}/mission/${id}`);
    }
  });

  it("shows a repository's error in place of its missions", async () => {
    const repos = body<ReposResponse>("/api/repos");
    repos.repos[1] = { ...repos.repos[1], error: "beta's records do not load", counts: null, missions: [] };
    serveFixtures({ "/api/repos": repos });
    renderAt("/");
    expect(await screen.findByRole("alert")).toHaveTextContent("beta's records do not load");
    expect(screen.getByRole("link", { name: /Ship the loader/ })).toBeInTheDocument();
  });
});

describe("a draft mission with a strategy", () => {
  it("opens on the brief the server names, linking all three pages", async () => {
    serveFixtures();
    renderAt("/alpha/mission/alpha-m1");
    await heading("Brief: Read plans on the base branch");
    const nav = screen.getByRole("navigation", { name: "Mission pages" });
    expect(within(nav).getByRole("link", { name: "Brief" })).toHaveAttribute("href", "/alpha/mission/alpha-m1/brief");
    expect(within(nav).getByRole("link", { name: "Debrief" })).toHaveAttribute("href", "/alpha/mission/alpha-m1/debrief");
    expect(within(nav).getByRole("link", { name: "Strategy" })).toHaveAttribute(
      "href",
      "/alpha/mission/alpha-m1/strategy",
    );
  });

  it("shows the brief's sections", async () => {
    serveFixtures();
    renderAt("/alpha/mission/alpha-m1/brief");
    await heading("Brief: Read plans on the base branch");
    for (const section of [
      "Outcome",
      "Sizing",
      "Derisking",
      "Success Criteria",
      "Scope",
      "Unknowns",
      "Open Human Decisions",
      "Risks",
      "Mission Reviews",
    ]) {
      expect(screen.getByRole("heading", { level: 2, name: section })).toBeInTheDocument();
    }
    expect(screen.getByText("Every plan is read on the base.")).toBeInTheDocument();
    expect(screen.getByText("The brief shows the plan.")).toBeInTheDocument();
    expect(screen.getByText("Load records from a commit")).toBeInTheDocument();
    expect(screen.getByText("size M")).toBeInTheDocument();
    expect(screen.getByText("Whether it is fast.")).toBeInTheDocument();
    expect(screen.getByText("No plan approval recorded.")).toBeInTheDocument();
    expect(screen.getByText("docs/strategies/one.md")).toBeInTheDocument();
  });

  it("shows the strategy as Markdown", async () => {
    serveFixtures();
    renderAt("/alpha/mission/alpha-m1/strategy");
    await heading("Strategy: docs/strategies/one.md");
    expect(screen.getByRole("heading", { level: 1, name: "Why" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { level: 1, name: "Risks and Unknowns" })).toBeInTheDocument();
    expect(screen.getByText("The base may move.").tagName).toBe("LI");
  });
});

describe("a closed mission without a strategy", () => {
  it("opens on the debrief and links no strategy", async () => {
    serveFixtures();
    renderAt("/alpha/mission/alpha-m2");
    await heading("Debrief: Ship the loader");
    const nav = screen.getByRole("navigation", { name: "Mission pages" });
    expect(within(nav).queryByRole("link", { name: "Strategy" })).toBeNull();
    expect(within(nav).getAllByRole("link")).toHaveLength(2);
  });

  it("shows the debrief's sections", async () => {
    serveFixtures();
    renderAt("/alpha/mission/alpha-m2/debrief");
    await heading("Debrief: Ship the loader");
    expect(screen.getByText(/^done \| 1 of 1 criteria proven/)).toBeInTheDocument();
    for (const section of ["What We Set Out To Do", "What We Did", "Results", "Validation"]) {
      expect(screen.getByRole("heading", { level: 2, name: section })).toBeInTheDocument();
    }
    expect(screen.getByText("Closed")).toBeInTheDocument();
    expect(screen.getByText("✓ proven")).toBeInTheDocument();
    expect(screen.getByText(/closed the mission/)).toBeInTheDocument();
    expect(screen.getAllByText(/delivered/).length).toBeGreaterThan(0);
  });

  it("says the strategy page has no strategy", async () => {
    serveFixtures();
    renderAt("/beta/mission/beta-m1/strategy");
    await heading("Not found");
    expect(screen.getByRole("alert")).toHaveTextContent("mission beta-m1 links no strategy");
  });
});

describe("an open mission past its plan", () => {
  it("shows what needs review and what is left", async () => {
    serveFixtures();
    renderAt("/beta/mission/beta-m2");
    await heading("Debrief: Build the inbox");
    expect(screen.getByRole("heading", { level: 2, name: "Needs Your Review" })).toBeInTheDocument();
    expect(screen.getByText(/2 criteria without passing proof: queued, linked/)).toBeInTheDocument();
    expect(screen.getByRole("heading", { level: 2, name: "Left To Do" })).toBeInTheDocument();
    const left = screen.getByRole("heading", { level: 2, name: "Left To Do" }).parentElement!;
    expect(within(left).getByText("beta-c1")).toBeInTheDocument();
  });
});

describe("not found", () => {
  it("shows the server's error for an unknown mission", async () => {
    serveFixtures();
    renderAt("/alpha/mission/alpha-zzzz");
    await heading("Not found");
    expect(screen.getByRole("alert")).toHaveTextContent("no mission alpha-zzzz in alpha");
  });

  it("shows the server's error for an unknown repository", async () => {
    serveFixtures();
    renderAt("/nowhere/mission/x");
    await heading("Not found");
    expect(screen.getByRole("alert")).toHaveTextContent("no Atelier repository named nowhere");
  });

  it("shows a not-found page for a path the app has no page for", async () => {
    serveFixtures();
    renderAt("/alpha/board/x/y");
    await heading("Not found");
  });
});

describe("a title holding a script", () => {
  const title = '<script>window.injected = 1</script><img src=x onerror="window.injected = 2">';

  function withTitle() {
    const repos = body<ReposResponse>("/api/repos");
    repos.repos[0].missions[1].title = title;
    const mission = body<MissionResponse>("/api/alpha/mission/alpha-m1");
    mission.mission.title = title;
    const brief = body<BriefResponse>("/api/alpha/mission/alpha-m1/brief");
    brief.brief.mission.title = title;
    brief.brief.items[0].issue.title = title;
    const debrief = body<DebriefResponse>("/api/alpha/mission/alpha-m1/debrief");
    debrief.debrief.mission.title = title;
    const strategy = body<StrategyResponse>("/api/alpha/mission/alpha-m1/strategy");
    strategy.path = title;
    serveFixtures({
      "/api/repos": repos,
      "/api/alpha/mission/alpha-m1": mission,
      "/api/alpha/mission/alpha-m1/brief": brief,
      "/api/alpha/mission/alpha-m1/debrief": debrief,
      "/api/alpha/mission/alpha-m1/strategy": strategy,
    });
  }

  for (const [path, prefix] of [
    ["/", null],
    ["/alpha/mission/alpha-m1", "Brief: "],
    ["/alpha/mission/alpha-m1/brief", "Brief: "],
    ["/alpha/mission/alpha-m1/debrief", "Debrief: "],
    ["/alpha/mission/alpha-m1/strategy", "Strategy: "],
  ] as const) {
    it(`appears as text on ${path}`, async () => {
      withTitle();
      const { container } = renderAt(path);
      if (prefix) await heading(prefix + title);
      else await screen.findAllByText(title);
      expect(container.querySelector("script")).toBeNull();
      expect(container.querySelector("img")).toBeNull();
      expect(container.textContent).toContain(title);
      expect((window as { injected?: number }).injected).toBeUndefined();
    });
  }
});
