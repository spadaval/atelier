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

Stop approved garden runs from stalling unmerged on human sessions.

Today `skills/agent-factory/procedures/garden.md` requires an approved garden branch to wait for a synchronous human session before `atelier merge`. In practice, both `atelier-0019` (Nov 11) and `atelier-qb4f` (Nov 13) have sat unmerged in `approved` for over a week, leaving their reviewed upkeep fixes and deferred-finding settlements off `master` and causing newer runs to collide on the same deferred findings.

Decouple the two halves: merge reviewed upkeep and deferred-finding settlements immediately on approval, and hand proposals to the human asynchronously as a `needs-human` item. Also moots `atelier-y0ba`.

## Done when

- [ ] `skills/agent-factory/procedures/garden.md` is updated so that:
  - Step 7 merges the run branch (`atelier merge`) immediately once independent review approves it, landing its upkeep fixes and deferred-finding settlements.
  - The run's ≤ 3 proposals are filed as one item labelled `needs-human,garden` (linking to the merged run's Report and listing each proposal concisely for the human to agree, modify, or decline), instead of holding the run branch open for a `### Session` section.
  - Step 1's Done when checklist and Step 6's Report template match (removing the requirement to leave `### Session` empty and hold the merge).
- [ ] `docs/product.md` and `docs/design.md` (and `skills/agent-factory/SKILL.md` if it mentions holding a garden run for a session) match the decoupled flow.
- [ ] `atelier check` and `evals/agent-factory/test.sh` pass.

## Scope

In: `skills/agent-factory/procedures/garden.md`, `skills/agent-factory/SKILL.md` (only the gardener-with-human sentence if affected), `docs/product.md`, and `docs/design.md`.

Out: merging or settling the existing `atelier-0019` and `atelier-qb4f` branches themselves, changing the gardener's 8 focus areas or metrics calculations, and any Rust code changes.
