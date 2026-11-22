---
schema: atelier.item
schema_version: 3
id: atelier-7637
title: Decouple garden upkeep merges from human proposal sessions
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
created_at: 2026-11-22T05:30:53.799635070Z
---

## Goal

Stop approved garden runs from stalling unmerged on human sessions, while cleanly reassigning every job the old garden session handled.

Today `skills/agent-factory/procedures/garden.md` requires an approved garden branch to wait for a synchronous human session before `atelier merge`. In practice, both `atelier-0019` (Nov 11) and `atelier-qb4f` (Nov 13) have sat unmerged (approved locally, still `in_review` on `origin`) for over a week, leaving their reviewed upkeep fixes and deferred-finding settlements off `master` and causing newer runs to collide on the same deferred findings.

Decouple the two halves: merge reviewed upkeep and deferred-finding settlements immediately on approval, and hand proposals and newly found bugs to the human asynchronously via `needs-human` items on a backlog branch. Also moots `atelier-y0ba`.

## Done when

- [ ] `skills/agent-factory/procedures/garden.md` (opening paragraph and Steps 1, 4, 5, 6, and 7) is updated so that:
  - Step 7 merges the run branch (`atelier merge`) immediately once independent review approves it, landing its upkeep fixes and deferred-finding settlements.
  - Immediately after `atelier merge` (using today's action table where `atelier new` belongs on a backlog branch), the gardener creates `backlog/garden-proposals-<YYYY-MM-DD>` (`atelier start --backlog ...`), files — or updates if an open `needs-human,garden` proposals item already exists in the repository, so daily runs (`atelier-jy8k`) do not spam the Inbox — one item labelled `needs-human,garden` referencing the merged run's Report and listing its ≤ 3 proposals, commits, submits, and merges it. The run is complete only once that backlog branch merges.
  - Bugs found during the run (Step 4/5) are filed on that same `backlog/*` branch labelled `bug,needs-human` (with `--caused-by`), instead of waiting as drafts for a session.
  - Carried proposals (Step 4) are read from open and recently closed `needs-human,garden` items (and their `## Report` decisions) rather than an unmerged `### Session` section.
  - Applying answers: once the human answers the `needs-human,garden` item, a **planner** files agreed proposals as items on a backlog branch and closes the `needs-human,garden` item with `atelier close <id> "<decisions>"`.
  - Step 1's Done when checklist and Step 6's Report template match (removing `### Session` and the hold-before-merge requirement).
- [ ] `skills/agent-factory/SKILL.md` ("Who Works With the Human"), `docs/product.md`, and `docs/design.md` match the decoupled flow.
- [ ] `atelier check` and `evals/agent-factory/test.sh` pass.

## Scope

In: `skills/agent-factory/procedures/garden.md`, `skills/agent-factory/SKILL.md`, `docs/product.md`, and `docs/design.md`.

Out: merging or settling the existing `atelier-0019` and `atelier-qb4f` branches themselves, changing the gardener's 8 focus areas or metrics calculations, and any Rust code changes.
