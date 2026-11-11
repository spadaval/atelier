import type { FindingRow } from "../api/FindingRow";
import type { Review } from "../api/Review";
import { recommendationWords } from "./words";

/** What became of a finding, in words, with the tone it shows in. */
export type Fate = { words: string; tone: "" | "ok" | "bad" | "review" };

type Placed = { finding: FindingRow; round: number };

function placed(reviews: Review[]): Placed[] {
  return reviews.flatMap((r) => r.findings.map((finding) => ({ finding, round: r.round })));
}

/**
 * The finding of a later round that carries `name` (names it in `Was:`),
 * as the core reads it: the latest such round's.
 */
function carrierOf(all: Placed[], name: string, round: number): Placed | undefined {
  return all.filter((p) => p.finding.was === name && p.round > round).sort((a, b) => b.round - a.round)[0];
}

/**
 * Follows a finding through the rounds that carry it, to the last one. A
 * carrying finding's `now` says what became of the one it carries: fixed,
 * withdrawn, or kept, in which case the carrying finding stands in its
 * place and may be carried in turn.
 */
function lastCarrier(all: Placed[], start: Placed): Placed | undefined {
  let last: Placed | undefined;
  let at = start;
  const seen = new Set<string>([start.finding.name]);
  for (;;) {
    const next = carrierOf(all, at.finding.name, at.round);
    if (!next || seen.has(next.finding.name)) return last;
    seen.add(next.finding.name);
    last = next;
    at = next;
  }
}

function settled(f: FindingRow): Fate | null {
  if (f.settlement?.kind === "taken") return { words: `Deferred, taken as ${f.settlement.item}`, tone: "" };
  if (f.settlement?.kind === "dropped") return { words: `Deferred, then dropped: ${f.settlement.reason}`, tone: "" };
  return null;
}

/** What a carrying finding says became of the earlier one, at round `round`. */
function carried(end: Placed): Fate {
  const f = end.finding;
  if (f.open) return { words: `Open, kept in round ${end.round}`, tone: "bad" };
  if (f.decided === "defer") return settled(f) ?? { words: `Deferred by decision, round ${end.round}`, tone: "review" };
  if (f.decided === "no_action") return { words: `No action, by decision in round ${end.round}`, tone: "" };
  if (f.now === "fixed") return { words: `Fixed, confirmed in round ${end.round}`, tone: "ok" };
  if (f.now === "withdrawn") return { words: `Withdrawn in round ${end.round}`, tone: "" };
  return { words: `Kept in round ${end.round}, then closed by a decision`, tone: "" };
}

/** A first-raised finding no later round carries. */
function first(f: FindingRow): Fate {
  if (f.open) return { words: "Open", tone: "bad" };
  const s = settled(f);
  if (s) return s;
  if (f.deferred) return { words: "Deferred", tone: "review" };
  if (f.recommendation === "fix_now") return { words: "Closed by a decision", tone: "" };
  return { words: recommendationWords[f.recommendation], tone: "" };
}

/** A finding that carries `was` and is not carried further: it says what became of `was`. */
function carrying(f: FindingRow, was: string): Fate {
  if (f.open) return { words: `Open, kept from ${was}`, tone: "bad" };
  if (f.decided === "defer") return settled(f) ?? { words: `Defers ${was}`, tone: "review" };
  if (f.decided === "no_action") return { words: `No action on ${was}`, tone: "" };
  if (f.now === "fixed") return { words: `Confirms ${was} fixed`, tone: "ok" };
  if (f.now === "withdrawn") return { words: `Withdraws ${was}`, tone: "" };
  return { words: `Keeps ${was}, closed by a decision`, tone: "" };
}

/**
 * The fate of every finding in an item's or a plan's reviews, by name.
 * A finding a later round carries takes its fate from the end of that
 * chain; one that carries another says so.
 */
export function findingFates(reviews: Review[]): Map<string, Fate> {
  const all = placed(reviews);
  const fates = new Map<string, Fate>();
  for (const p of all) {
    const f = p.finding;
    const end = lastCarrier(all, p);
    fates.set(f.name, end ? carried(end) : f.was ? carrying(f, f.was) : first(f));
  }
  return fates;
}

/**
 * The fix-now findings a later round confirmed fixed: each first-raised
 * finding whose chain of carrying findings ends in one that says fixed and
 * was not decided otherwise. Each is listed once, with the round that
 * confirmed it.
 */
export function fixedInReview(reviews: Review[]): { finding: FindingRow; round: number; fixedIn: number }[] {
  const all = placed(reviews);
  return all.flatMap((p) => {
    if (p.finding.was || p.finding.recommendation !== "fix_now") return [];
    const end = lastCarrier(all, p);
    if (!end || end.finding.now !== "fixed" || end.finding.decided !== null || end.finding.open) return [];
    return [{ finding: p.finding, round: p.round, fixedIn: end.round }];
  });
}
