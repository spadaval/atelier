---
created_at: "2026-11-10T00:56:08.329209632+00:00"
id: "atelier-pz1y"
issue_type: "task"
labels:
- "redesign"
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
    id: "atelier-yptz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Add `land`, `check` and `setup codex`, and move the session hooks to the new `status`"
updated_at: "2026-11-10T00:56:08.329209632+00:00"
---

## Goal

Finish the handoff commands and orient Codex sessions as Claude sessions are oriented today.

## Done when

- [ ] Provisional: `land` on the base refuses an unpushed commit that is not an Atelier merge and touches a path outside `.atelier/`; `check` validates every record; a Codex session in a fixture repository starts with `status` output.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-2-core.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- Which Codex hook event runs at session start. If none fits, `AGENTS.md` says to run `atelier status` first.
