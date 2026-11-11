---
created_at: "2026-11-11T00:57:39.599582995+00:00"
id: "atelier-efsl"
issue_type: "bug"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-efsl
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-nafc"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Show only startable items under Ready to start, and have `merge` say its checks ran: two fumbles the rehearsal found"
updated_at: "2026-11-11T01:00:18.872210913+00:00"
---

## Goal

Fix two places where the new binary's output misled on real data. The rehearsal on five converted repositories (atelier-nafc) found them:

- `status` on the base lists every ready item under "Ready to start", each with its unmet conditions, whether or not `start` would accept it. In one converted repository 29 items stood under that heading and one could start. An agent told to run `status` and do what it says reads that heading as work it can pick up.
- `merge` prints nothing about the base's merge checks when they pass, so an agent cannot tell from a successful merge that any check ran.

## Done when

- [ ] `status` on the base lists under "Ready to start" only the items `start` would accept now. Ready items that cannot start yet are not listed one by one: one line counts them by cause (a blocker that is not done, a mission that is not ready) and names `atelier list` as where to see them. A test builds one item that can start, one that waits on a blocker and one in a draft mission, and checks the heading holds the first alone and the line counts the other two.
- [ ] For a caller that holds no item, the `Then:` line that follows names an item that can start; when no ready item can start, `status` says so and offers no `start` on a ready item. What `status` prints for a caller that holds an item, including the `start` that returns it to that item's branch, is unchanged.
- [ ] A `merge` that runs the base's merge checks and makes the merge commit says in one line how many checks from the base's `config.toml` ran on the merged tree and that they passed, before the line that reports the merge. With no check configured it says so in that line's place. The line claims only what happened in that run: when `ATELIER_SKIP` passed a failing check, the skip's own line stands and no line says the checks passed; and a run that finds the item already merged, and only pushes, prints nothing about checks. A test covers each of the four.
- [ ] The fresh-agent scenario's fixture still reads as before where one item can start: the `next-loop` grader's tests pass, and the Report shows `status` on that fixture's base.
- [ ] `docs/strategies/redesign-architecture.md` and `docs/strategies/redesign-product.md` match what was built where they describe these two outputs.

## Scope

What `status` on the base and a successful `merge` print, in `next/`. No flag, no config option, no change to what `start` or `merge` accept, and no change to `list`. No file under `crates/` is changed.

## Context

The rehearsal's Report (`atelier issue show atelier-nafc --full`) has both findings. The base's `status` is built in `atelier-app`'s `view.rs` and printed from `atelier-cli`'s `output.rs`; `start`'s conditions are in the act table, `atelier-app`'s `acts.rs`, which is what decides whether an item can start. `merge` is `atelier-cli`'s `merge.rs`. Behaviour: `docs/strategies/redesign-product.md`, Commands ("What `status` prints") and Merge.
