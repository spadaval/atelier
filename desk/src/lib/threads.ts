import type { LiveThread } from "../api/LiveThread";

/** The live threads a page lists, with the holds it shows for each: on a repository's page, only that repository's. */
export function threadsFor(threads: LiveThread[], repo: string | undefined): { holding: LiveThread[]; free: LiveThread[] } {
  if (repo === undefined) return { holding: threads.filter((t) => t.holds.length > 0), free: threads.filter((t) => t.holds.length === 0) };
  const holding = threads.flatMap((t) => {
    const holds = t.holds.filter((h) => h.repo === repo);
    return holds.length > 0 ? [{ ...t, holds }] : [];
  });
  return { holding, free: [] };
}
