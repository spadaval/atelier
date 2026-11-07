---
created_at: "2026-11-07T03:35:59.047087245+00:00"
id: "atelier-secy"
issue_type: "bug"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "evidence"
    id: "atelier-htux"
    type: "discovered_from"
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Ticking an after-merge check on the base refuses land for reviewed items"
updated_at: "2026-11-07T03:35:59.047087245+00:00"
---

## Goal

The after-merge path atelier-n0qh built (ADR 0027) works for every merged item: ticking a `- [ ] After merge` line, or adding `carried to <id>`, in a records-only commit on the base passes `land`. Today it does only for items with no review findings. For an item that had a fixed `fix_now` finding, the edit to its done record makes `land` refuse with findings.disposed ("1 fix_now finding resolved after the current approval; it needs a fresh approval"), so the validation step the docs describe needs a fresh review, which defeats it.

## Done when

- A test: an item with a `fix_now` finding fixed and re-approved, merged, then its after-merge check ticked in a records-only commit on the base: `land` passes. Same for adding `carried to <existing id>`.
- Any other edit to the done record on the base is judged as today (a test shows a non-validation edit of the same record still refuses).
- The validation-only edit test reuses atelier-n0qh's rule (only the box and an added "carried to <id>" change; check text, reason and stand-in unchanged), not a second definition.
- The proof_timing scenario item has a fixed fix_now finding, so the scenario covers this case.

## Context

Found by the mission validation (receipt atelier-htux). The refusal is findings.disposed in crates/atelier-app/src/review_gate.rs (approval_accepted), reached from land in crates/atelier-cli/src/commands/session.rs. Reproduction: /home/supadava/.cache/atelier-tmp/n4mj-scripts/repro.sh atelier <dir> 1 (0 for the no-finding case that lands). The pre-mission binary refuses the same record edit, so the gate is not new; the after-merge path just never accounted for it. Strategy: docs/strategies/lighter-workflow.md (Shape "Proof has a time", "Show, don't enforce").
