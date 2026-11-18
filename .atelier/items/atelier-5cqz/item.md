---
schema: atelier.item
schema_version: 3
id: atelier-5cqz
title: Let framed Desk pages connect in the T3 test instance, and give its done mission items
status: building
mission: atelier-osel
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A57e793d0-a6ba-4a2b-9999-f8cb06fe0316%3Adelegate-task%3Ae73c6040-f31e-45bc-921a-e869236f0428
ready:
  by: lead of atelier-osel, from the Reports of atelier-toee and atelier-ut89
  at: 2026-11-18T00:25:19.832037677Z
trivial: null
caused_by: null
created_at: 2026-11-18T00:25:03.508714447Z
---

## Goal

Let the T3 test instance show the pages this mission's remaining items must
be seen in. Three gaps in its fixture Desk made the screenshots of
atelier-ut89 and atelier-toee sparse or needed a second Desk by hand:

- The Desk allows no embedding origin, and T3 Code Nightly's page origin is
  `t3code://app`. `GET /api/embed` answers `{"origins":[]}`, so no framed
  Desk page in the instance ever gets `desk:ready`: not the panel, the inbox
  or an item's page. The mission's frames-kept criterion cannot be seen there.
- The Desk's configuration has no `[t3]` table, so the running mission page
  says T3's threads cannot be read.
- The fixture's done mission has no items and no Result, so the finished
  mission page is nearly empty.

## Done when

- [x] After `node t3-extension/t3-instance.mjs start`, an item's page and the inbox open framed in the instance and finish connecting, with no second Desk and no hand edit.
- [x] The running mission page in the instance lists the instance's thread among its threads, and does not say T3's threads cannot be read.
- [x] The finished mission page in the instance shows a Result, at least two lanes and at least two merged items with different numbers of review rounds.
- [x] The isolation check still passes: the fixture Desk reads the instance's T3 and nothing of the human's.
- [x] `test-contract.mjs` still passes with every hook against the instance.
- [x] The Report holds a screenshot from the instance of a framed item page that has connected, of the running mission page and of the finished one.

## Scope

In: `t3-extension/fixture-desk.mjs`, `t3-extension/t3-instance.mjs` and the
fixture's records. The first gap is one line in the instance's `desk.toml`,
`embed_origins = ["t3code://app"]`; atelier-toee's Report says how it was
found. atelier-ut89's Report describes the other two.

Out: the extension's own sources under `t3-extension/src`, which
atelier-40dc is rewriting at the same time, and the Desk's code. If the
`[t3]` table cannot point the fixture Desk at the instance's T3 without a
change to the Desk, say so in the Report and leave that line unticked for
the lead.

## Report

In the T3 test instance, a framed inbox and item page now connect, the
running mission page lists a thread at work, and the finished mission page
shows a Result, two lanes and two merged items. Nothing needs a second Desk or
a hand edit after `node t3-extension/t3-instance.mjs start`. The change is in
`t3-extension/fixture-desk.mjs`, `t3-extension/t3-instance.mjs` and the
README's "A test instance (Linux)".

**What changed.**

- `writeFixtureDesk(dir, { threads, embedOrigins, t3 })` writes
  `embed_origins` and, when given, a `[t3]` table. Plain `startFixtureDesk()`
  is unchanged: no origins, no `[t3]`.
- The instance passes `embed_origins = ["t3code://app"]` and a `[t3]` table
  whose `command` is the `t3` wrapper T3 writes into the instance's home
  (`home/.t3/bin/t3`, which pins `T3CODE_HOME` to the instance) and whose
  `url` is `http://127.0.0.1:<backend port>/mcp`. No change to the Desk was
  needed.
- The closed mission `atelier-fm02` has a Result and two done items,
  `atelier-fd01` (one review round) and `atelier-fd02` (two). Their steps are
  empty commits with Atelier's subjects at fixed dates, from the plan's merge
  to the close, since the Desk reads history from commit subjects.
- Thread identities in the fixture's records are quoted, since T3 names a
  thread started over MCP `mcp:<uuid>`.

**One addition the item did not name: a second, waiting thread.** The `[t3]`
table alone does not make the running page list the instance's thread. The
page lists only threads T3 says are live, and the first thread's run has
completed, so *At work* said "0 threads". `start` now also launches a second
thread through the instance's MCP server (`t3_thread_launch`,
`runtimeMode: "approval-required"`), asking it to run `ls`. It waits on that
approval until the instance stops. `start` waits until T3 reports
`pendingRequestCount > 0`. If the model answers without the tool, `start`
fails and names the thread and model. The fixture names this thread as the
implementer of `atelier-fi01`, and the first thread stays the lead. The token
is issued by the instance's `t3` command with `orchestration:operate`, lasts
15 minutes and is revoked after the call. `isolation` also looks for the
second thread's ID.

**For the lead: the mission page does not see a pending approval.** T3 lists
the waiting thread as `running` with `pendingRequestCount: 1`, and its own
sidebar marks it "Approval". The mission page counts a thread as waiting only
when its status is `waiting`, so it puts this thread under *At work*, not
*Needs you* (`t3-running.png`). Fixing it needs the Desk to carry T3's pending
requests (`T3Thread` in `crates/atelier-desk/src/t3.rs` drops the field) and
`liveWorkers` in `t3-extension/src/mission/model.ts` to read it. Both are
outside this item's scope, so I left them alone.

**Checks.** All were run with the instance in its own root
(`T3_TEST_ROOT=/var/tmp/atelier-t3-test-5cqz`) on ports 9337 and 47337, since
the default root is shared with other threads. The Desk was built from this
branch, in T3 Code Nightly `v0.1.11-nightly.20261117.4851`.

- `GET /api/embed` answers `{"origins":["t3code://app"]}`. The mission's read
  gives `t3.reachable: true`.
- The inbox (from the sidebar's Inbox row) and `atelier-fi02` (from *Open
  item* on the running mission page) each opened framed. Inside each frame,
  through its own DevTools target, the root carried the 16 host theme tokens.
  The Desk sets those only after it connects and sends `desk:ready`.
- The running page shows *At work*, 1 thread: "First item", `$ ls`. It does
  not say T3's threads cannot be read.
- The done page, opened with `__t3Custom.openMission("atelier",
  "atelier-fm02")`, shows the Result and lanes for "Reviewed once" (1 round,
  4h 0m) and "Reviewed twice" (2 rounds, 8h 30m). Both are under *Merged*,
  most rounds first.
- `node test-contract.mjs` against the instance exited 0 with all 35 probes
  ok.
- `node t3-instance.mjs isolation` passed. It ran with the Desk polling the
  instance's T3 and both threads. 15 entries under the guarded folders
  changed (the host's own T3), none naming or holding the instance, and
  OpenCode held no credentials.
- `node run-tests.mjs` passed all 5 tests.

**Screenshots** in `artifacts/`: `t3-item-framed.png` (a connected framed
item page), `t3-inbox-framed.png`, `t3-running.png` and `t3-done.png`.
