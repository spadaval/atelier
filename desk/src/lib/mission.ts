/**
 * Where a mission stands, for mission control and the project home: an
 * item's stage and how long it has been in it, and the progress log, read
 * from the records' history (`GET /api/history`, from commit subjects, so
 * approximate); and whether a mission is under way.
 */
import type { HistoryEvent } from "../api/HistoryEvent";
import type { ItemPage } from "../api/ItemPage";
import type { ItemRow } from "../api/ItemRow";
import type { MissionRow } from "../api/MissionRow";

/** An item's stage in a word or two, with its review round. */
export function stage(row: ItemRow, events: HistoryEvent[]): string {
  const submits = events.filter((e) => e.id === row.id && e.kind === "submitted").length;
  switch (row.status) {
    case "in_review":
      return `in review ${Math.max(1, submits)}`;
    case "changes_requested":
      return "changes req";
    case "done":
      return events.some((e) => e.id === row.id && e.kind === "merged") ? "merged" : "done";
    default:
      return row.status.replaceAll("_", " ");
  }
}

/** When an item entered its stage: its latest recorded step, else when it was readied. */
export function stageAt(id: string, events: HistoryEvent[], page: ItemPage | null | undefined): string | null {
  return events.find((e) => e.id === id)?.at ?? page?.ready?.at ?? null;
}

/** `2h 14m`, `36m`, `48s`: a duration, short. */
export function span(ms: number): string {
  const s = Math.max(0, Math.round(ms / 1000));
  if (s < 60) return `${s}s`;
  const m = Math.round(s / 60);
  if (m < 60) return `${m}m`;
  const h = Math.floor(m / 60);
  if (h < 48) return `${h}h ${m % 60}m`;
  return `${Math.round(h / 24)}d`;
}

/** One line of the progress log. `waited` marks a step the human took or an item that needs a human. */
export type Entry = { at: string; text: string; id: string; waited: boolean; tone: string };

/**
 * The progress log of mission `missionId`, newest first: a line for each
 * step its commit subjects record, and one for each batch of items the
 * human readied, read from the items' ready marks, whose `by` starts with
 * `caller`. A step on an item that needs a human, and every step the human
 * took, waited on the human.
 */
export function logEntries(missionId: string, items: ItemRow[], events: HistoryEvent[], pages: Map<string, ItemPage>, caller: string): Entry[] {
  // With no caller known yet, no ready mark is the human's.
  const mine = (page: ItemPage) => caller !== "" && !!page.ready?.by.toLowerCase().startsWith(caller.toLowerCase());
  const needsYou = new Set(items.filter((i) => i.needs_human).map((i) => i.id));
  const recorded = new Set([...pages].filter(([, p]) => mine(p)).map(([id]) => id));
  const entries: Entry[] = [];
  for (const e of events) {
    if (e.kind === "readied" && recorded.has(e.id)) continue;
    const plan = e.id === missionId;
    const waited = needsYou.has(e.id);
    const one = (text: string, tone: string) => entries.push({ at: e.at, text, id: e.id, waited, tone });
    switch (e.kind) {
      case "planned":
        one("Mission planned", "plain");
        break;
      case "plan_review":
        one(`Plan review ${e.round}: ${e.verdict}`, e.verdict === "approved" ? "ok" : "bad");
        break;
      case "started":
        one(`Started ${e.id}`, "run");
        break;
      case "submitted":
        one(`Submitted ${e.id} for review`, "review");
        break;
      case "review":
        one(`Review ${e.round} of ${e.id}: ${e.verdict}`, e.verdict === "approved" ? "ok" : "bad");
        break;
      case "merged":
        one(plan ? "Plan merged: mission agreed" : `Merged ${e.id}`, "ok");
        break;
      case "closed":
        one(plan ? "Mission closed with its result" : `Closed ${e.id}`, "ok");
        break;
      case "dropped":
        one(plan ? "Mission dropped" : `Dropped ${e.id}`, "dim");
        break;
      case "readied":
        one(`Readied ${plan ? "the plan" : e.id}`, "plain");
        break;
      case "led":
        one("Lead took the mission", "plain");
        break;
    }
  }
  const readied = new Map<string, { by: string; at: string; ids: string[] }>();
  for (const [id, page] of pages) {
    if (!page.ready || !mine(page)) continue;
    const key = `${page.ready.by}\n${page.ready.at.slice(0, 16)}`;
    const group = readied.get(key) ?? { by: page.ready.by, at: page.ready.at, ids: [] };
    group.ids.push(id);
    readied.set(key, group);
  }
  for (const g of readied.values()) {
    const said = g.by.includes(":") ? `: "${g.by.split(":").slice(1).join(":").trim()}"` : "";
    entries.push({ at: g.at, text: `${named(caller)} readied ${g.ids.length === 1 ? g.ids[0] : `${g.ids.length} items`}${said}`, id: g.ids[0], waited: true, tone: "you" });
  }
  return entries.sort((a, b) => Date.parse(b.at) - Date.parse(a.at));
}

/** A caller's name as a person's: `ada` is `Ada`. */
const named = (caller: string) => caller.charAt(0).toUpperCase() + caller.slice(1);

/** Whether a mission is under way, for the project home's Now: an item of it is started or done. */
export function begun(m: Pick<MissionRow, "in_progress" | "counts">): boolean {
  return m.in_progress || m.counts.some((c) => c.status !== "open" && c.status !== "dropped" && c.count > 0);
}
