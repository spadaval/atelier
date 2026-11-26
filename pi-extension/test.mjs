// Runs the extension against a stand-in for Pi: no model, no Pi install.
// `node --test test.mjs` (Node 22.18 or later, which runs TypeScript directly).
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { chmodSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import {
  groundTruth,
  guardEvent,
  HANDOFF_TOOL,
  ledMissionFromStatus,
  readVerdict,
  roleFromStatus,
  section,
  sessionName,
  unticked,
} from "./lib.ts";

const ITEM = `---
id: demo-1
title: Print the merge line
status: building
---

## Goal

Show the merge line of the first item.

## Done when

- [x] The line prints for a later item.
- [ ] The line prints for the first item.
- [ ] A test covers a repository with one merge.

## Report
`;

const MISSION = `---
id: demo-m
title: "Edit home from the web"
---

## Outcome

Home is edited on a big screen.

## Success criteria

- round-trip: one save shows on the phone.

## Scope

Not this.
`;

/** A checkout with one item and one mission, on the branch given. */
function checkout(branch) {
  const dir = mkdtempSync(join(tmpdir(), "pi-ext-"));
  mkdirSync(join(dir, ".atelier/items/demo-1"), { recursive: true });
  mkdirSync(join(dir, ".atelier/missions/demo-m"), { recursive: true });
  writeFileSync(join(dir, ".atelier/items/demo-1/item.md"), ITEM);
  writeFileSync(join(dir, ".atelier/missions/demo-m/mission.md"), MISSION);
  const git = (...args) => execFileSync("git", args, { cwd: dir, stdio: "ignore" });
  git("init", "-q", "-b", branch);
  return dir;
}

/** Executables named `atelier` and a guard, first on PATH for the extension under test. */
function fakeBin({ status, guard }) {
  const bin = mkdtempSync(join(tmpdir(), "pi-ext-bin-"));
  writeFileSync(join(bin, "atelier"), `#!/bin/sh\ncat <<'EOT'\n${status}\nEOT\n`);
  writeFileSync(join(bin, "guard"), `#!/bin/sh\ncat > "${bin}/seen"\nprintf '%s' '${guard}'\n`);
  chmodSync(join(bin, "atelier"), 0o755);
  chmodSync(join(bin, "guard"), 0o755);
  return bin;
}

/** Load the extension into a stand-in for Pi and return its handlers and what it sent. */
async function load() {
  const handlers = {};
  const sent = [];
  const tools = {};
  const pi = {
    on: (name, handler) => ((handlers[name] ??= []).push(handler)),
    sendMessage: (message, options) => sent.push({ message, options }),
    appendEntry: () => {},
    registerTool: (tool) => (tools[tool.name] = tool),
  };
  const { default: factory } = await import(`./index.ts?${Math.random()}`);
  factory(pi);
  const fire = async (name, event, ctx) => {
    let last;
    for (const handler of handlers[name] ?? []) last = (await handler(event, ctx)) ?? last;
    return last;
  };
  return { fire, sent, tools };
}

test("a handoff tool is matched by suffix, as Codex's hook matches it", () => {
  for (const name of ["mcp__t3-code__delegate_task", "mcp__t3_code__t3_thread_send", "spawn_agent"]) {
    assert.ok(HANDOFF_TOOL.test(name), name);
  }
  for (const name of ["bash", "edit", "mcp__t3-code__t3_thread_read", "background_start"]) {
    assert.ok(!HANDOFF_TOOL.test(name), name);
  }
});

test("the guard is sent Codex's hook event and only an explicit deny blocks", () => {
  assert.deepEqual(JSON.parse(guardEvent("delegate_task", { task: "hi" })), {
    hook_event_name: "PreToolUse",
    tool_name: "delegate_task",
    tool_input: { task: "hi" },
  });
  assert.deepEqual(readVerdict("{}"), { deny: false });
  assert.deepEqual(readVerdict("not json"), { deny: false });
  assert.deepEqual(
    readVerdict('{"hookSpecificOutput":{"permissionDecision":"deny","permissionDecisionReason":"fused"}}'),
    { deny: true, reason: "fused" },
  );
});

test("role and led mission are read from atelier status as it prints them", () => {
  assert.equal(roleFromStatus("You are the reviewer of demo-1 (thread t1), round 2.\nReview file: x"), "reviewer");
  assert.equal(roleFromStatus("You are the implementer of demo-1 (thread t1).\nItem: x"), "implementer");
  assert.equal(roleFromStatus("You are thread t1, on branch master."), null);
  const status =
    "Missions in progress:\n  demo-m (in progress, led by t1): Edit home\n  other-m (agreed, led by t2): Else";
  assert.equal(ledMissionFromStatus(status, "t1"), "demo-m");
  assert.equal(ledMissionFromStatus(status, "t3"), null);
  assert.equal(ledMissionFromStatus("Mission demo-m (in progress, led by t1): Edit home", "t1"), "demo-m");
  assert.equal(ledMissionFromStatus(status, undefined), null);
});

test("a record's section ends at the next heading, and only unticked lines are left", () => {
  assert.equal(section(ITEM, "Goal"), "Show the merge line of the first item.");
  assert.equal(section(ITEM, "Missing"), "");
  assert.deepEqual(unticked(section(ITEM, "Done when")), [
    "- [ ] The line prints for the first item.",
    "- [ ] A test covers a repository with one merge.",
  ]);
});

test("ground truth is the branch's item, or the led mission, or nothing", () => {
  const dir = checkout("demo-1");
  const item = groundTruth(dir, "demo-1", null);
  assert.match(item, /You hold item demo-1: Print the merge line/);
  assert.match(item, /first item\./);
  assert.doesNotMatch(item, /later item/);
  const mission = groundTruth(dir, "backlog/x", "demo-m", "demo-1 building");
  assert.match(mission, /You lead mission demo-m: Edit home from the web/);
  assert.match(mission, /round-trip/);
  assert.match(mission, /demo-1 building/);
  assert.doesNotMatch(mission, /Not this/);
  assert.equal(groundTruth(dir, "backlog/x", null), "");
});

test("background jobs are named per checkout", () => {
  assert.notEqual(sessionName("/a", "dev"), sessionName("/b", "dev"));
  assert.match(sessionName("/a", "dev server!"), /^atl-[0-9a-f]{6}-dev-server-$/);
});

test("the first run of a session gets atelier status and the item's unticked lines", async () => {
  const dir = checkout("demo-1");
  const bin = fakeBin({ status: "You are the implementer of demo-1 (thread t1).", guard: "{}" });
  process.env.PATH = `${bin}:${process.env.PATH}`;
  const pi = await load();
  const ctx = { cwd: dir };
  await pi.fire("session_start", { reason: "startup" }, ctx);
  const first = await pi.fire("before_agent_start", {}, ctx);
  assert.match(first.message.content, /You are the implementer of demo-1/);
  assert.match(first.message.content, /A test covers a repository with one merge/);
  assert.equal(await pi.fire("before_agent_start", {}, ctx), undefined);
});

test("nothing is said in a checkout atelier does not track", async () => {
  const dir = mkdtempSync(join(tmpdir(), "pi-ext-plain-"));
  const pi = await load();
  assert.equal(await pi.fire("before_agent_start", {}, { cwd: dir }), undefined);
  await pi.fire("turn_end", {}, { cwd: dir });
  assert.equal(pi.sent.length, 0);
});

test("a denied handoff is blocked with the guard's reason, also from inside a script", async () => {
  const dir = checkout("demo-1");
  const deny = '{"hookSpecificOutput":{"permissionDecision":"deny","permissionDecisionReason":"fused words"}}';
  const bin = fakeBin({ status: "You are the implementer of demo-1 (thread t1).", guard: deny });
  process.env.PATH = `${bin}:${process.env.PATH}`;
  process.env.ATELIER_PI_GUARD = join(bin, "guard");
  const pi = await load();
  const ctx = { cwd: dir };
  const blocked = await pi.fire(
    "tool_call",
    { toolName: "mcp__t3-code__delegate_task", input: { task: "x" }, parentToolCallId: "codemode-1" },
    ctx,
  );
  assert.deepEqual(blocked, { block: true, reason: "fused words" });
  assert.equal(await pi.fire("tool_call", { toolName: "bash", input: { command: "ls" } }, ctx), undefined);
  process.env.ATELIER_PI_GUARD = "off";
  assert.equal(await pi.fire("tool_call", { toolName: "delegate_task", input: { task: "x" } }, ctx), undefined);
  delete process.env.ATELIER_PI_GUARD;
});

test("a reviewer cannot edit, an implementer can, and the role follows atelier commands", async () => {
  const dir = checkout("demo-1");
  const bin = fakeBin({ status: "You are the implementer of demo-1 (thread t1).", guard: "{}" });
  process.env.PATH = `${bin}:${process.env.PATH}`;
  const pi = await load();
  const ctx = { cwd: dir };
  await pi.fire("before_agent_start", {}, ctx);
  assert.equal(await pi.fire("tool_call", { toolName: "edit", input: {} }, ctx), undefined);
  writeFileSync(join(bin, "atelier"), "#!/bin/sh\necho 'You are the reviewer of demo-1 (thread t1), round 1.'\n");
  await pi.fire("tool_result", { toolName: "bash", input: { command: "atelier review demo-1" } }, ctx);
  const blocked = await pi.fire("tool_call", { toolName: "edit", input: {} }, ctx);
  assert.equal(blocked.block, true);
  assert.equal(await pi.fire("tool_call", { toolName: "read", input: {} }, ctx), undefined);
});

test("the records are sent again every 25 turns and after a compaction", async () => {
  const dir = checkout("demo-1");
  const bin = fakeBin({ status: "You are the implementer of demo-1 (thread t1).", guard: "{}" });
  process.env.PATH = `${bin}:${process.env.PATH}`;
  const pi = await load();
  const ctx = { cwd: dir };
  for (let turn = 0; turn < 24; turn++) await pi.fire("turn_end", {}, ctx);
  assert.equal(pi.sent.length, 0);
  await pi.fire("turn_end", {}, ctx);
  assert.equal(pi.sent.length, 1);
  assert.equal(pi.sent[0].options.deliverAs, "steer");
  assert.match(pi.sent[0].message.content, /The line prints for the first item/);
  await pi.fire("session_compact", { reason: "threshold" }, ctx);
  assert.equal(pi.sent.length, 2);
  assert.match(pi.sent[1].message.content, /the records are right/);
});

test("a background command can be started, read and stopped", async () => {
  const dir = checkout("demo-1");
  const pi = await load();
  const ctx = { cwd: dir };
  const call = (name, params) => pi.tools[name].execute("id", params, undefined, undefined, ctx);
  const started = await call("background_start", { name: "echo", command: "echo hello-from-job; sleep 30" });
  assert.match(started.content[0].text, /Started echo/);
  await new Promise((resolve) => setTimeout(resolve, 500));
  const read = await call("background_read", { name: "echo" });
  assert.match(read.content[0].text, /still running/);
  assert.match(read.content[0].text, /hello-from-job/);
  assert.match((await call("background_stop", { name: "echo" })).content[0].text, /Stopped echo/);
  assert.equal((await call("background_read", { name: "echo" })).isError, true);
});
