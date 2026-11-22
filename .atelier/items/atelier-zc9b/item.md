---
schema: atelier.item
schema_version: 3
id: atelier-zc9b
title: Fix recurring CLI friction in submit handoffs, status on base, pre-push recovery, and Responses parsing
status: ready
mission: atelier-ww7w
labels: []
priority: medium
blocked_by: []
implementers: []
ready:
  by: Sumeet, in conversation on 2026-11-22
  at: 2026-11-22T05:33:31.467134267Z
trivial: null
caused_by: null
created_at: 2026-11-22T05:30:53.741617559Z
---

## Goal

Eliminate five small, recurring CLI friction bugs already observed in real agent runs and evals:
1. **`atelier-u49o`:** `atelier submit` tells the reviewer `Run atelier review <id> and follow what it prints.` without naming the checkout path, causing reviewers started from another directory (often the main checkout) to hit a refusal on their first call.
2. **`atelier-o2vv`:** On `base` when a thread holds nothing, `atelier status` ends with `Then: atelier start <top-ready-item>`, which can steer a delegated thread toward an unrelated item.
3. **`atelier-yhaf`:** In the base's own checkout, `atelier status` for a thread holding a started item prints `Then: atelier start <id> (switches to its branch)`, even though `start` does not switch the base checkout — causing agents that follow `Then:` lines to loop.
4. **`atelier-jfos`:** `pre-push`'s refusal tells the agent to run `git branch -f <base> <old>`, which Git refuses whenever `<base>` is checked out in the current worktree.
5. **`atelier-o91z`:** Any malformed line under `### Responses` makes the whole `item.md` fail to parse (exit 2), wedging `status`, `show`, and `review` on the item and preventing `ATELIER_SKIP`.

Covers `atelier-u49o`, `atelier-o2vv`, `atelier-yhaf`, `atelier-jfos`, and `atelier-o91z`.

## Done when

- [ ] `submit`'s reviewer handoff task line names the item checkout's path (e.g., `In <path>, run atelier review <id> and follow what it prints.`), with no backticks; a test covers a reviewer following that task from the main checkout.
- [ ] Away from an item branch, when the caller holds nothing, `atelier status` lists startable items without printing `Then: atelier start <id>`. `skills/agent-factory/SKILL.md` and `AGENTS.md` add one line noting that if a session starts in a different repository from its task, run `atelier status` again in the task's checkout.
- [ ] In the base checkout, when `atelier status` shows a started item the caller holds, it names the existing worktree path (via `git::checkout_of`) or prints `git worktree add <path> <branch>`, never `Then: atelier start <id> (switches to its branch)`. Elsewhere, `status` still suggests `atelier start <id>`.
- [ ] `pre-push`'s refusal recovery instructions work when `<base>` is checked out in the current worktree (e.g., detaching or resetting cleanly instead of failing `git branch -f <base> <old>`), verified by a hook scenario test on a checked-out base.
- [ ] Malformed lines under `### Responses` do not cause `ItemDir` loading to fail with exit 2: valid responses still parse, while malformed lines are reported as validation problems by `atelier check`, `atelier status`, and `atelier submit` (naming the line and expected syntax), and `status`, `show`, and `review` continue to work on the item.
- [ ] `cargo nextest run --locked` and `evals/agent-factory/test.sh` pass (update any fixture test assertions that only matched the old buggy `status` or `pre-push` output, keeping every scenario's contract intact).

## Scope

In: `crates/atelier-core/src/{records/body.rs,records/validate.rs,say.rs,work.rs}`, `crates/atelier-cli/src/{commands.rs,output.rs,check.rs,review.rs}`, `skills/agent-factory/SKILL.md`, `AGENTS.md`, `evals/agent-factory/` (only if a scenario's unit test asserts the old buggy wording), and Rust tests.

Out: `skills/agent-factory/roles/reviewer.md`, `t3-extension/`, and removing `draft`/`ready` (`atelier-dh7a`).
