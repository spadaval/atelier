// The parts of the extension that need no Pi: what counts as a handoff, how
// the guard is asked, and what an agent is reminded of. index.ts wires these
// to Pi's events; test.mjs runs them directly.

import { execFile } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

/** The delegation tools the guard reads, by suffix, as Codex's hook matches them. */
export const HANDOFF_TOOL =
  /(spawn_agent|send_message|send_input|followup_task|delegate_task|t3_thread_send|t3_thread_launch)$/;

/** Tools that change files directly. A reviewer reads a change; it does not make one. */
export const FILE_CHANGE_TOOLS = new Set(["edit", "write"]);

export interface Ran {
  code: number;
  stdout: string;
  stderr: string;
}

/** Run a command with text on stdin. Never rejects: a failure is a nonzero code. */
export function run(
  command: string,
  args: string[],
  options: { cwd?: string; input?: string; timeoutMs?: number; env?: NodeJS.ProcessEnv } = {},
): Promise<Ran> {
  return new Promise((resolve) => {
    const child = execFile(
      command,
      args,
      {
        cwd: options.cwd,
        env: options.env ?? process.env,
        timeout: options.timeoutMs ?? 30_000,
        maxBuffer: 8 * 1024 * 1024,
      },
      (error, stdout, stderr) => {
        const code = error ? (typeof error.code === "number" ? error.code : 1) : 0;
        resolve({ code, stdout: String(stdout), stderr: String(stderr) });
      },
    );
    child.stdin?.on("error", () => {});
    child.stdin?.end(options.input ?? "");
  });
}

/** The event the guard reads: the same shape Codex's PreToolUse hook sends it. */
export function guardEvent(toolName: string, input: unknown): string {
  return JSON.stringify({ hook_event_name: "PreToolUse", tool_name: toolName, tool_input: input });
}

export type Verdict = { deny: false } | { deny: true; reason: string };

/** Read the guard's answer. Anything that is not an explicit denial allows the call. */
export function readVerdict(stdout: string): Verdict {
  try {
    const out = JSON.parse(stdout)?.hookSpecificOutput;
    if (out?.permissionDecision === "deny") {
      return { deny: true, reason: String(out.permissionDecisionReason ?? "Handoff Guard blocked this message.") };
    }
  } catch {
    // Not JSON: the guard did not answer, so it did not deny.
  }
  return { deny: false };
}

export type Role = "reviewer" | "implementer" | null;

/** The role `atelier status` says this thread holds on an item, if it says one. */
export function roleFromStatus(status: string): Role {
  const match = /^You are the (reviewer|implementer) of \S+/m.exec(status);
  return match ? (match[1] as Role) : null;
}

/** The mission `atelier status` says this thread leads, if any. */
export function ledMissionFromStatus(status: string, threadId: string | undefined): string | null {
  if (!threadId) return null;
  for (const line of status.split("\n")) {
    const match = /^\s*(?:Mission\s+)?(\S+) \([^)]*\bled by ([^)\s]+)\)/.exec(line);
    if (match && match[2] === threadId) return match[1];
  }
  return null;
}

/** The body of one `## Heading` section of a Markdown record, without the heading. */
export function section(markdown: string, heading: string): string {
  const lines = markdown.split("\n");
  const start = lines.findIndex((line) => line.trim() === `## ${heading}`);
  if (start < 0) return "";
  const rest = lines.slice(start + 1);
  const end = rest.findIndex((line) => /^## /.test(line));
  return (end < 0 ? rest : rest.slice(0, end)).join("\n").trim();
}

/** The Done when lines not yet ticked. */
export function unticked(doneWhen: string): string[] {
  return doneWhen.split("\n").filter((line) => /^\s*- \[ \]/.test(line));
}

function title(markdown: string): string {
  const match = /^title:\s*(.+)$/m.exec(markdown);
  return match ? match[1].trim().replace(/^["']|["']$/g, "") : "";
}

function clip(text: string, limit: number): string {
  return text.length <= limit ? text : `${text.slice(0, limit)}\n[cut at ${limit} characters]`;
}

/**
 * What the thread is working toward, read from the records and nothing else:
 * the item of the branch it is on, or the mission it leads. Empty when the
 * checkout holds neither, so the extension stays silent outside atelier work.
 */
export function groundTruth(cwd: string, branch: string, ledMission: string | null, shown = ""): string {
  const itemPath = join(cwd, ".atelier", "items", branch, "item.md");
  if (branch && existsSync(itemPath)) {
    const item = readFileSync(itemPath, "utf8");
    const left = unticked(section(item, "Done when"));
    return clip(
      [
        `You hold item ${branch}: ${title(item)}`,
        `Goal:\n${section(item, "Goal") || "(not written yet)"}`,
        left.length > 0
          ? `Done when lines still unticked:\n${left.join("\n")}`
          : "Every Done when line is ticked, or none is written yet.",
      ].join("\n\n"),
      6000,
    );
  }
  const mission = ledMission ?? branch;
  const missionPath = join(cwd, ".atelier", "missions", mission, "mission.md");
  if (mission && existsSync(missionPath)) {
    const record = readFileSync(missionPath, "utf8");
    return clip(
      [
        `You lead mission ${mission}: ${title(record)}`,
        `Outcome:\n${section(record, "Outcome")}`,
        `Success criteria:\n${section(record, "Success criteria")}`,
        shown ? `Its items now (atelier show ${mission}):\n${shown.trim()}` : "",
      ]
        .filter(Boolean)
        .join("\n\n"),
      8000,
    );
  }
  return "";
}

/** A tmux session name for a background command, distinct per checkout. */
export function sessionName(cwd: string, name: string): string {
  const safe = name.replace(/[^A-Za-z0-9_-]/g, "-").slice(0, 40) || "job";
  return `atl-${createHash("sha256").update(cwd).digest("hex").slice(0, 6)}-${safe}`;
}
