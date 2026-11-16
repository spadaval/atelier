/**
 * The embedded mode, for a host such as T3 Code that shows the Desk in a
 * panel and draws the frame around it. A path under `/embed` turns it on:
 * the app's router takes `/embed` as its base, so every link inside the Desk
 * keeps it, and a reload stays in it. Without the prefix nothing changes.
 */
export const EMBED_BASE = "/embed";

/** Whether `pathname` is under the embedded base. */
export function isEmbedded(pathname: string): boolean {
  return pathname === EMBED_BASE || pathname.startsWith(`${EMBED_BASE}/`);
}

/** Whether this page was opened in the embedded mode. */
export const embedded = typeof location !== "undefined" && isEmbedded(location.pathname);
