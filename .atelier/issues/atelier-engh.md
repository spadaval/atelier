---
created_at: "2026-11-10T00:56:06.737900196+00:00"
id: "atelier-engh"
issue_type: "task"
labels:
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-mal9"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Give `atelier-next` thread identity, the act table, and `start`, `submit` and `status`"
updated_at: "2026-11-10T01:29:38.619607290+00:00"
---

## Goal

Build the part of the loop that orients an implementer: the binary knows which thread is calling, finds started work by its branch, and `status` prints what the item needs and the next command.

## Done when

- [ ] `atelier-next` is a second binary target in `atelier-cli`. `cargo build` produces both binaries, and the old binary's tests pass unchanged.
- [ ] Scenario test: in a scratch repository with one ready item and `T3CODE_THREAD_ID=t1`, `atelier-next start <id>` sets the item `building` and appends `t1` to its `implementers` in one commit, then creates branch `<id>` pointing at that commit. `status` then names the role and the item, lists each unmet condition (each unticked Done when line, an empty Report) and ends with `Then: atelier submit`.
- [ ] `submit` with an unticked line or an empty Report changes nothing, exits non-zero, and prints the same list `status` prints. With both met it sets `in_review` in one commit that also holds the Report, and prints the reviewer's handoff line.
- [ ] `start` on a draft item changes nothing and names `ready` as the way out.
- [ ] Scenario test, take-over: with the item in each of `building`, `in_review`, `changes_requested` and `approved` (later statuses written by the fixture), `start <id>` by thread `t2` switches to the branch, appends `t2` to `implementers` so that it holds the item, leaves the status unchanged, and says `t1` held it.
- [ ] Scenario test, two workers: from two worktrees of one repository, `t1` and `t2` both run `start` on the same ready item. Exactly one creates the branch; the other changes nothing and is told `t1` holds the item and that running `start` again takes it over.
- [ ] Test, interrupted start: when `start` is stopped after its commit is written and before the branch is created, no branch `<id>` exists, the item is still ready on the base, and a later `start` succeeds.
- [ ] Scenario test, reading from the base: with the item started on its branch, `status` run on the base by a third thread shows the item as building and held by its implementer, and does not list it as ready. After the branch is merged, the base's record is used.
- [ ] Unit test: identity is `T3CODE_THREAD_ID`, else `CLAUDE_CODE_SESSION_ID`, else `CODEX_SESSION_ID`, else `GROK_SESSION_ID`, else `human`.
- [ ] Unit test over the act table: every act from every status gives the status the architecture specification lists, or a refusal. For each case, the conditions `status` prints equal the conditions the command reports.
- [ ] The Status, Threads and "Where records are read and written" sections of `docs/strategies/redesign-architecture.md` match what was built.

## Scope

`ready`, `new` and `start --new` belong to step B; fixtures write ready items as files. Missions do not exist yet, so `start` does not check a mission's status here. On the base, `status` lists ready and started items and nothing more yet. `review`, `verdict` and `merge` are the next two items.

## Context

Design: `docs/strategies/redesign-architecture.md`, Status, Threads and roles, Where records are read and written. Behaviour: `docs/strategies/redesign-product.md`, Lifecycle (including "When a thread dies") and Commands.

## Unknowns

- The wording of `status`. The first wording is a guess; the fresh-agent scenario at the end of step A is what tests it.
