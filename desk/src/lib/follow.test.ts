import { describe, expect, it } from "vitest";
import type { Tie } from "../api/Tie";
import type { RepoSummary } from "../api/RepoSummary";
import { INBOX, place, serves } from "./follow";

const tie = (kind: Tie["kind"], id: string, role: Tie["role"], current: boolean, repo = "atelier"): Tie => ({
  repo,
  kind,
  id,
  title: id,
  status: kind === "item" ? (current ? "building" : "done") : current ? "open" : "done",
  role,
  current,
});

describe("place", () => {
  it("opens the item a thread implements", () => {
    expect(place([tie("item", "atelier-a1", "implementer", true)])).toEqual({ path: "/r/atelier/item/atelier-a1", line: "This thread implements this item." });
  });
  it("opens the item a thread reviews", () => {
    expect(place([tie("item", "atelier-a1", "reviewer", true)])).toEqual({ path: "/r/atelier/item/atelier-a1", line: "This thread reviews this item." });
  });
  it("opens the mission a thread leads, plans or reviews the plan of", () => {
    expect(place([tie("mission", "atelier-m1", "lead", true)]).line).toBe("This thread leads this mission.");
    expect(place([tie("mission", "atelier-m1", "planner", true)])).toEqual({ path: "/r/atelier/mission/atelier-m1", line: "This thread plans this mission." });
    expect(place([tie("mission", "atelier-m1", "reviewer", true, "lor")])).toEqual({ path: "/r/lor/mission/atelier-m1", line: "This thread reviews this mission's plan." });
  });
  it("prefers the item it implements, then the item it reviews, then the mission", () => {
    const lead = tie("mission", "atelier-m1", "lead", true);
    const reviews = tie("item", "atelier-r1", "reviewer", true);
    const builds = tie("item", "atelier-b1", "implementer", true);
    expect(place([lead, reviews, builds]).path).toBe("/r/atelier/item/atelier-b1");
    expect(place([lead, reviews]).path).toBe("/r/atelier/item/atelier-r1");
  });
  it("passes over ties that are not current", () => {
    expect(place([tie("item", "atelier-d1", "implementer", false), tie("item", "atelier-r1", "reviewer", true)]).path).toBe("/r/atelier/item/atelier-r1");
    expect(place([tie("item", "atelier-d1", "implementer", false), tie("mission", "atelier-m1", "lead", false)])).toEqual(INBOX);
  });
  it("opens the inbox for a thread with no ties, or only a decider's", () => {
    expect(place([])).toEqual({ path: "/inbox", line: "This thread holds no open work." });
    expect(place([tie("item", "atelier-a1", "decider", true)])).toEqual(INBOX);
  });

  it("opens the project's page for a thread with no current tie when the host names a repository", () => {
    expect(place([], "lor")).toEqual({ path: "/r/lor", line: "This thread holds no open work in this project." });
    expect(place([tie("item", "atelier-d1", "implementer", false)], "atelier").path).toBe("/r/atelier");
  });
  it("keeps a current tie's page whatever repository the host names", () => {
    expect(place([tie("item", "atelier-a1", "implementer", true)], "lor").path).toBe("/r/atelier/item/atelier-a1");
  });
});

describe("serves", () => {
  const repo = (name: string, error: string | null = null) => ({ name, error }) as RepoSummary;
  const repos = { repos: [repo("atelier"), repo("broken", "does not read")], built_from: "x" };
  it("counts a repository the Desk lists and can read", () => {
    expect(serves(repos, "atelier")).toBe(true);
    expect(serves(repos, "broken")).toBe(false);
    expect(serves(repos, "elsewhere")).toBe(false);
  });
});
