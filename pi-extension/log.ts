// A record of what a Pi session did, written beside the session and never
// shown to the model: one JSON line per event, one file per session. It
// exists so a long run can be measured afterward without replaying it.
//
// Nothing here may change a run. Every write is wrapped: a full disk or a
// missing directory loses log lines and nothing else.

import { appendFileSync, existsSync, mkdirSync, readFileSync } from "node:fs";
import { homedir } from "node:os";
import { basename, join } from "node:path";

export function logDir(): string {
  return process.env.ATELIER_PI_LOG_DIR?.trim() || join(homedir(), ".local", "state", "atelier-pi");
}

/** The log file for a Pi session, named for the session's own file. */
export function logFile(sessionFile: string | undefined): string {
  const name = sessionFile ? basename(sessionFile).replace(/\.jsonl$/, "") : `ephemeral-${process.pid}`;
  return join(logDir(), `${name}.jsonl`);
}

export function write(file: string, event: string, fields: Record<string, unknown>): void {
  try {
    mkdirSync(logDir(), { recursive: true });
    appendFileSync(file, `${JSON.stringify({ ts: new Date().toISOString(), event, ...fields })}\n`);
  } catch {
    // A log that cannot be written must not disturb the run.
  }
}

/** How large a value is once serialized, without keeping the serialization. */
export function size(value: unknown): number {
  try {
    return Buffer.byteLength(JSON.stringify(value) ?? "", "utf8");
  } catch {
    return -1;
  }
}

/** The `status:` of the item whose branch is checked out, or null when the branch is not an item's. */
export function itemStatus(cwd: string, branch: string): string | null {
  try {
    const path = join(cwd, ".atelier", "items", branch, "item.md");
    if (!branch || !existsSync(path)) return null;
    return /^status:\s*(\S+)/m.exec(readFileSync(path, "utf8"))?.[1] ?? null;
  } catch {
    return null;
  }
}
