import type { ItemRow } from "../api/ItemRow";
import type { Priority } from "../api/Priority";
import type { Status } from "../api/Status";
import { LIFECYCLE } from "./words";

export type StatusGroup = { status: Status; items: ItemRow[] };

/** Items under each status that has any, in lifecycle order, keeping their order within a status. */
export function groupByStatus(items: ItemRow[]): StatusGroup[] {
  return LIFECYCLE.map((status) => ({ status, items: items.filter((i) => i.status === status) })).filter((g) => g.items.length > 0);
}

/** What waits on the human in a mission: its open items labelled needs-human. */
export function waitingOnYou(items: ItemRow[]): ItemRow[] {
  const closed = new Set<Status>(["done", "dropped"]);
  return items.filter((i) => i.needs_human && !closed.has(i.status));
}

const URGENCY: Priority[] = ["critical", "high", "medium", "low"];

/** The open items among those in no mission, as the project home lists them: the most urgent first, then by ID. */
export function openByPriority(items: ItemRow[]): ItemRow[] {
  return items.filter((i) => i.status === "open").sort((a, b) => URGENCY.indexOf(a.priority) - URGENCY.indexOf(b.priority) || (a.id < b.id ? -1 : 1));
}
