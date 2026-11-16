/**
 * Following the host's thread: which page the embedded Desk opens for the
 * thread its host is showing (`host:thread`, docs/architecture/desk.md, "The
 * host contract"). Only a thread's current ties count, as `GET /api/ties`
 * marks them. A thread with none, including one tied only to finished work,
 * gets its project's page when the host names a repository the Desk serves,
 * and the inbox otherwise.
 */
import type { ReposResponse } from "../api/ReposResponse";
import type { Tie } from "../api/Tie";
import { to } from "./api";

/** The page for a thread, and the line that names the thread's part on it. */
export type Place = { path: string; line: string };

/** The current ties in the order a page is chosen by: the item it implements, the item it reviews, then the mission it leads, plans or reviews the plan of. */
const ORDER: { kind: Tie["kind"]; role: Tie["role"]; line: string }[] = [
  { kind: "item", role: "implementer", line: "This thread implements this item." },
  { kind: "item", role: "reviewer", line: "This thread reviews this item." },
  { kind: "mission", role: "lead", line: "This thread leads this mission." },
  { kind: "mission", role: "planner", line: "This thread plans this mission." },
  { kind: "mission", role: "reviewer", line: "This thread reviews this mission's plan." },
];

/** The inbox, for a thread that holds no current work. */
export const INBOX: Place = { path: "/inbox", line: "This thread holds no open work." };

/** A repository's page, for a thread that holds no current work in that repository. */
export const project = (repo: string): Place => ({ path: to.repo(repo), line: "This thread holds no open work in this project." });

/** The thread's first current tie in `ORDER`, as its page, or null with none. */
export function tied(ties: Tie[]): Place | null {
  for (const want of ORDER) {
    const tie = ties.find((t) => t.current && t.kind === want.kind && t.role === want.role);
    if (tie) return { path: tie.kind === "item" ? to.item(tie.repo, tie.id) : to.mission(tie.repo, tie.id), line: want.line };
  }
  return null;
}

/** Whether `repo` names a repository the Desk serves: one `GET /api/repos` lists and can read. */
export function serves(repos: ReposResponse, repo: string): boolean {
  return repos.repos.some((r) => r.name === repo && r.error === null);
}

/**
 * The page the embedded Desk opens for a thread with `ties`, whose host's
 * project maps to `repository`, given as the name of a repository the Desk
 * serves, or null when the host named none or one the Desk does not serve.
 */
export function place(ties: Tie[], repository: string | null = null): Place {
  return tied(ties) ?? (repository === null ? INBOX : project(repository));
}
