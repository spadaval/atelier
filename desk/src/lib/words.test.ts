import { describe, expect, it } from "vitest";
import { countsLine, finished, statusTitle, total } from "./words";

describe("counts", () => {
  it("spells the counts as atelier show names the statuses", () => {
    const counts = [
      { status: "draft" as const, count: 5 },
      { status: "in_review" as const, count: 1 },
      { status: "done" as const, count: 2 },
    ];
    expect(countsLine(counts)).toBe("5 draft, 1 in review, 2 done");
    expect(total(counts)).toBe(8);
    expect(finished(counts)).toBe(2);
    expect(countsLine([])).toBe("No items");
  });

  it("titles a status for a pill", () => {
    expect(statusTitle("changes_requested")).toBe("Changes requested");
    expect(statusTitle("ready")).toBe("Ready");
  });
});
