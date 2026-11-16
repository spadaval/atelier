// Spike: hand-written types for the reads in crates/atelier-desk/src/server/spike.rs.
import type { ItemRow } from "./ItemRow";
import type { Priority } from "./Priority";
import type { Role } from "./Role";
import type { Status } from "./Status";

export type SideThread = { identity: string; role: Role };
export type SideItem = { id: string; title: string; status: Status; threads: SideThread[] };
export type SideMission = {
  id: string;
  title: string;
  state: "in_progress" | "agreed" | "proposed";
  done: number;
  total: number;
  waiting: number;
  threads: SideThread[];
  items: SideItem[];
};
export type Sidebar = { repos: { repo: string; missions: SideMission[]; error?: string }[] };

export type Launch = { ready: { id: string; title: string; priority: Priority }[]; waiting: number; threads_waiting: { identity: string; title: string }[] };

export type EventKind = "planned" | "plan_review" | "started" | "submitted" | "review" | "merged" | "readied" | "closed" | "dropped" | "led";
export type HistoryEvent = {
  commit: string;
  at: string;
  kind: EventKind;
  id: string;
  record: "item" | "mission" | null;
  title: string | null;
  round: number | null;
  verdict: string | null;
  subject: string;
};
export type History = { repo: string; events: HistoryEvent[] };

export type Checks = { state: "passing" | "failing" | "pending" | "none"; checks: { name: string; state: string }[] };

export type Roadmap = { repo: string; loose: ItemRow[] };
