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
updated_at: "2026-11-10T00:56:06.737900196+00:00"
---

## Goal

Build the part of the loop that orients an implementer: the binary knows which thread is calling and what status the item is in, and `status` prints what the item needs and the next command.

## Done when

- [ ] `atelier-next` is a second binary target in `atelier-cli`. `cargo build` produces both binaries, and the old binary's tests pass unchanged.
- [ ] Scenario test: in a scratch repository with one ready item and `T3CODE_THREAD_ID=t1`, `atelier-next start <id>` creates branch `<id>` from the base, sets the item `building` and stores `t1` as its implementer. `status` then names the role and the item, lists each unmet condition (each unticked Done when line, an empty Report) and ends with `Then: atelier submit`.
- [ ] `submit` with an unticked line or an empty Report changes nothing, exits non-zero, and prints the same list `status` prints. With both met it sets `in_review` in one commit that also holds the Report, and prints the reviewer's handoff line.
- [ ] `start` on a draft item changes nothing and names `ready` as the way out.
- [ ] `start` by thread `t2` on an item `t1` is building passes the item to `t2` and says `t1` held it.
- [ ] Unit test: identity is `T3CODE_THREAD_ID`, else `CLAUDE_CODE_SESSION_ID`, else `CODEX_SESSION_ID`, else `GROK_SESSION_ID`, else `human`.
- [ ] Unit test over the act table: every act from every status gives the status the architecture specification lists, or a refusal. For each case, the conditions `status` prints equal the conditions the command reports.
- [ ] The Status and Threads sections of `docs/strategies/redesign-architecture.md` match what was built.

## Scope

`ready` and `new` belong to step B; fixtures write ready items as files. On the base, `status` lists ready items and nothing more yet. `review`, `verdict` and `merge` are the next two items.

## Context

Design: `docs/strategies/redesign-architecture.md`, Status, Threads and roles, Where records are read and written. Behaviour: `docs/strategies/redesign-product.md`, Lifecycle and Commands.

## Unknowns

- The wording of `status`. The first wording is a guess; the fresh-agent scenario at the end of step A is what tests it.
