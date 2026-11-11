import { describe, expect, it } from "vitest";
import type { FindingRow } from "../api/FindingRow";
import type { Review } from "../api/Review";
import { findingFates, fixedInReview } from "./findings";

function finding(name: string, more: Partial<FindingRow> = {}): FindingRow {
  return { name, title: name, recommendation: "fix_now", was: null, now: null, decided: null, settlement: null, text: null, open: false, deferred: false, ...more };
}

function round(n: number, findings: FindingRow[]): Review {
  return { name: `r${n}`, round: n, commit: "abc", reviewers: [], model: null, verdict: "approve", created_at: "2026-11-11T00:00:00Z", findings, changed_since: null };
}

const fixed = (reviews: Review[]) => fixedInReview(reviews).map((f) => `${f.finding.name}@${f.fixedIn}`);
const fate = (reviews: Review[], name: string) => findingFates(reviews).get(name)?.words;

describe("fixedInReview", () => {
  it("lists a fix-now finding a later round confirms fixed, once", () => {
    const reviews = [round(1, [finding("1-aaaa.1")]), round(2, [finding("2-bbbb.1", { was: "1-aaaa.1", now: "fixed" })])];
    expect(fixed(reviews)).toEqual(["1-aaaa.1@2"]);
    expect(fate(reviews, "1-aaaa.1")).toBe("Fixed, confirmed in round 2");
    expect(fate(reviews, "2-bbbb.1")).toBe("Confirms 1-aaaa.1 fixed");
  });

  it("leaves out a finding a later round withdraws, though the core no longer counts it open", () => {
    const reviews = [round(1, [finding("1-aaaa.1")]), round(2, [finding("2-bbbb.1", { was: "1-aaaa.1", now: "withdrawn" })])];
    expect(fixed(reviews)).toEqual([]);
    expect(fate(reviews, "1-aaaa.1")).toBe("Withdrawn in round 2");
  });

  it("follows a finding kept in round 2 to the round that confirms it fixed", () => {
    const reviews = [
      round(1, [finding("1-aaaa.1")]),
      round(2, [finding("2-bbbb.1", { was: "1-aaaa.1", now: "kept" })]),
      round(3, [finding("3-cccc.1", { was: "2-bbbb.1", now: "fixed" })]),
    ];
    expect(fixed(reviews)).toEqual(["1-aaaa.1@3"]);
    expect(fate(reviews, "2-bbbb.1")).toBe("Fixed, confirmed in round 3");
  });

  it("shows a kept finding nobody has carried since as open, and not fixed", () => {
    const reviews = [round(1, [finding("1-aaaa.1")]), round(2, [finding("2-bbbb.1", { was: "1-aaaa.1", now: "kept", open: true })])];
    expect(fixed(reviews)).toEqual([]);
    expect(fate(reviews, "1-aaaa.1")).toBe("Open, kept in round 2");
    expect(fate(reviews, "2-bbbb.1")).toBe("Open, kept from 1-aaaa.1");
  });

  it("leaves out a finding a later round carries as decided", () => {
    const reviews = [round(1, [finding("1-aaaa.1")]), round(2, [finding("2-bbbb.1", { was: "1-aaaa.1", decided: "no_action" })])];
    expect(fixed(reviews)).toEqual([]);
    expect(fate(reviews, "1-aaaa.1")).toBe("No action, by decision in round 2");
  });

  it("does not take a closed finding nobody carried for a fixed one", () => {
    const reviews = [round(1, [finding("1-aaaa.1")])];
    expect(fixed(reviews)).toEqual([]);
    expect(fate(reviews, "1-aaaa.1")).toBe("Closed by a decision");
    expect(fate([round(1, [finding("1-aaaa.1", { open: true })])], "1-aaaa.1")).toBe("Open");
  });

  it("names a deferred finding's settlement", () => {
    const reviews = [round(1, [finding("1-aaaa.1", { recommendation: "defer", deferred: true, settlement: { kind: "taken", item: "atelier-e05n", by: "sumeet" } })])];
    expect(fate(reviews, "1-aaaa.1")).toBe("Deferred, taken as atelier-e05n");
    expect(fixed(reviews)).toEqual([]);
  });
});
