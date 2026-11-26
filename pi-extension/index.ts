// Atelier's extension for the Pi coding agent. It gives a Pi thread what a
// Codex or Claude Code thread already gets from hooks, and adds the parts we
// want to own: see README.md. It does nothing in a checkout with no .atelier.

import type { ExtensionAPI, ExtensionContext } from "@earendil-works/pi-coding-agent";
import { existsSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";
import {
  FILE_CHANGE_TOOLS,
  HANDOFF_TOOL,
  groundTruth,
  guardEvent,
  ledMissionFromStatus,
  readVerdict,
  roleFromStatus,
  run,
  sessionName,
  type Role,
} from "./lib.ts";

const REMIND_EVERY = Number(process.env.ATELIER_PI_REMIND_EVERY ?? 25);

/** The guard command: ATELIER_PI_GUARD, or the logging wrapper, or the guard itself. */
function guardCommand(): string[] | null {
  const configured = process.env.ATELIER_PI_GUARD?.trim();
  if (configured === "off") return null;
  if (configured) return configured.split(/\s+/);
  const bin = join(homedir(), ".local", "bin");
  if (existsSync(join(bin, "handoff-guard-logged"))) return [join(bin, "handoff-guard-logged"), "pi"];
  if (existsSync(join(bin, "handoff-guard"))) return [join(bin, "handoff-guard"), "hook"];
  return null;
}

const text = (value: string) => ({ content: [{ type: "text" as const, text: value }], details: {} });

export default function (pi: ExtensionAPI) {
  let status = "";
  let role: Role = null;
  let greeted = false;
  let turns = 0;

  const tracked = (ctx: ExtensionContext) => existsSync(join(ctx.cwd, ".atelier"));

  async function readStatus(ctx: ExtensionContext) {
    const result = await run("atelier", ["status"], { cwd: ctx.cwd });
    status = result.code === 0 ? result.stdout.trim() : "";
    role = roleFromStatus(status);
  }

  async function truth(ctx: ExtensionContext): Promise<string> {
    const branch = (await run("git", ["branch", "--show-current"], { cwd: ctx.cwd })).stdout.trim();
    const led = ledMissionFromStatus(status, process.env.T3CODE_THREAD_ID);
    const shown = led ? (await run("atelier", ["show", led], { cwd: ctx.cwd })).stdout : "";
    return groundTruth(ctx.cwd, branch, led, shown);
  }

  async function remind(ctx: ExtensionContext, why: string) {
    await readStatus(ctx);
    const block = await truth(ctx);
    if (!block) return;
    pi.sendMessage(
      { customType: "atelier-ground-truth", content: `${why}\n\n${block}`, display: true },
      { deliverAs: "steer" },
    );
  }

  pi.on("session_start", async () => {
    greeted = false;
    turns = 0;
  });

  // Codex and Claude Code run `atelier status` from a session-start hook. Pi
  // reads neither hook file, so the first run of a session gets it here.
  pi.on("before_agent_start", async (_event, ctx) => {
    if (greeted || !tracked(ctx)) return;
    greeted = true;
    await readStatus(ctx);
    if (!status) return;
    const block = await truth(ctx);
    return {
      message: {
        customType: "atelier-status",
        content: block ? `${status}\n\n${block}` : status,
        display: true,
      },
    };
  });

  pi.on("tool_call", async (event, ctx) => {
    if (tracked(ctx) && role === "reviewer" && FILE_CHANGE_TOOLS.has(event.toolName)) {
      return {
        block: true,
        reason:
          "You are this item's reviewer, and a reviewer does not change the files under review. " +
          "Write findings with `atelier review` and `atelier verdict`.",
      };
    }
    if (!HANDOFF_TOOL.test(event.toolName)) return;
    const guard = guardCommand();
    if (!guard) return;
    const result = await run(guard[0], guard.slice(1), {
      cwd: ctx.cwd,
      input: guardEvent(event.toolName, event.input),
      timeoutMs: 5000,
    });
    const verdict = readVerdict(result.stdout);
    // Like Codex's hook, a guard that fails to answer does not stop the call.
    // The failure is written to the session so a silent guard is not mistaken for a clean run.
    if (result.code !== 0) pi.appendEntry("atelier-guard-error", { tool: event.toolName, stderr: result.stderr });
    if (verdict.deny) return { block: true, reason: verdict.reason };
  });

  // `atelier review` and `atelier start` change the role mid-session, so read it again after them.
  pi.on("tool_result", async (event, ctx) => {
    const command = (event.input as { command?: unknown } | undefined)?.command;
    if (tracked(ctx) && event.toolName === "bash" && typeof command === "string" && /\batelier\b/.test(command)) {
      await readStatus(ctx);
    }
  });

  // Every REMIND_EVERY turns, put the goal and what is left at the tail of the
  // conversation. The system prompt is left alone so the cached prefix holds.
  pi.on("turn_end", async (_event, ctx) => {
    if (!tracked(ctx) || !(REMIND_EVERY > 0)) return;
    turns += 1;
    if (turns % REMIND_EVERY === 0) await remind(ctx, "A reminder of what this thread is for, from atelier's records:");
  });

  // Pi writes the summary. The records say what is still true, so they follow it.
  pi.on("session_compact", async (_event, ctx) => {
    if (!tracked(ctx)) return;
    turns = 0;
    await remind(
      ctx,
      "Earlier messages were just summarized. Where the summary and these records disagree, the records are right:",
    );
  });

  // Long-running commands. Pi's bash runs a command to its end; a dev server,
  // an emulator or a watch build needs to keep running while the agent works.
  pi.registerTool({
    name: "background_start",
    label: "Start background command",
    description:
      "Start a long-running shell command (a dev server, an emulator, a watch build) that keeps running after this call returns. " +
      "Read its output with background_read and stop it with background_stop.",
    parameters: {
      type: "object",
      properties: {
        name: { type: "string", description: "A short name for the job, used to read and stop it" },
        command: { type: "string", description: "The shell command to run" },
      },
      required: ["name", "command"],
    },
    async execute(_id, params: { name: string; command: string }, _signal, _onUpdate, ctx) {
      const session = sessionName(ctx.cwd, params.name);
      const made = await run("tmux", ["new-session", "-d", "-s", session, "-x", "200", "-y", "50", "-c", ctx.cwd, params.command]);
      if (made.code !== 0) return { ...text(`Could not start ${params.name}: ${made.stderr.trim()}`), isError: true };
      // Keep the pane after the command exits so its last output can still be read.
      await run("tmux", ["set-option", "-t", session, "remain-on-exit", "on"]);
      return text(`Started ${params.name}. Read its output with background_read.`);
    },
  });

  pi.registerTool({
    name: "background_read",
    label: "Read background command",
    description: "Read the latest output of a command started with background_start, and whether it is still running.",
    parameters: {
      type: "object",
      properties: {
        name: { type: "string", description: "The job's name" },
        lines: { type: "number", description: "How many trailing lines to read (default 80)" },
      },
      required: ["name"],
    },
    async execute(_id, params: { name: string; lines?: number }, _signal, _onUpdate, ctx) {
      const session = sessionName(ctx.cwd, params.name);
      const lines = Math.max(1, Math.min(2000, Math.floor(params.lines ?? 80)));
      const pane = await run("tmux", ["capture-pane", "-p", "-J", "-t", session, "-S", `-${lines}`]);
      if (pane.code !== 0) return { ...text(`No background command named ${params.name}.`), isError: true };
      const dead = await run("tmux", ["display-message", "-p", "-t", session, "#{pane_dead} #{pane_dead_status}"]);
      const [isDead, exit] = dead.stdout.trim().split(" ");
      const state = isDead === "1" ? `exited with status ${exit || "unknown"}` : "still running";
      return text(`${params.name} is ${state}.\n\n${pane.stdout.trimEnd()}`);
    },
  });

  pi.registerTool({
    name: "background_stop",
    label: "Stop background command",
    description: "Stop a command started with background_start.",
    parameters: {
      type: "object",
      properties: { name: { type: "string", description: "The job's name" } },
      required: ["name"],
    },
    async execute(_id, params: { name: string }, _signal, _onUpdate, ctx) {
      const killed = await run("tmux", ["kill-session", "-t", sessionName(ctx.cwd, params.name)]);
      return killed.code === 0
        ? text(`Stopped ${params.name}.`)
        : { ...text(`No background command named ${params.name}.`), isError: true };
    },
  });
}
