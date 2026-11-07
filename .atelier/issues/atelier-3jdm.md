---
created_at: "2026-11-07T05:00:24.456556181+00:00"
id: "atelier-3jdm"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Judge an approval by the code it saw, not the records around it"
updated_at: "2026-11-07T05:00:24.456556181+00:00"
---

## Goal

Today an approval goes stale when almost any tracker record on the branch changes after it, unless the edit is on a short allowed list (status, timestamps, Close Reason, new review and evidence records). Agents hit this dozens of times: ticking Done when boxes at close, adding a note, linking a follow-up, recording dispositions. Each time a reviewer re-approved a records-only change and found nothing. Make an approval stale only when code changed after it, so record edits never cost a re-review, while any code change, including a fix after review, still does.

## Done when

- [ ] A test approves an item, then on the branch edits, one per commit: the item's Done when ticks, Notes, links, status and Close Reason; another issue's record; a finding's disposition; a new evidence receipt. After each, `atelier merge --check` reports the approval current.
- [ ] A test changes a source file after approval, and `merge --check` reports the approval stale and names the file. Changing `.atelier/workflow.yaml` or `.atelier/config.toml` after approval also makes it stale.
- [ ] A clean merge from the base keeps the approval current, and a conflicted merge whose resolution changes code makes it stale (existing tests kept or adjusted).
- [ ] The astromech-l5c6 sequence, closing and ticking Done when after approval, merges without a second approval in a scenario test.
- [ ] Approval freshness no longer reads record contents: `allowed_edge`, `issue_status_only` and their helpers in `crates/atelier-app/src/review_gate.rs` are gone or no longer used for freshness. The append-only check for review records keeps its own tests passing.
- [ ] docs/product and docs/architecture describe approval freshness as: stale only when code changed since the approved commit, not counting a clean merge from the base.

## Scope

Approval freshness in merge, land and status; their tests and docs. Must not change: merge refuses without a current approval; a fix commit after a `fix_now` finding makes the approval stale, so the fix is re-reviewed; a Done when edited after start is still shown to the reviewer (atelier-r9hb). The agent-factory skill text is updated later, by the item that rewrites the skill.

## Context

- Code means everything except `.atelier/issues`, `.atelier/reviews`, `.atelier/evidence` (and `.atelier/runtime`), the same split `scripts/merge-ci-passed.sh` uses.
- `crates/atelier-app/src/review_gate.rs` (`allowed_edge`, `issue_status_only`, `review_change_allowed`), `crates/atelier-app/src/git_freshness.rs`.
- One installed binary serves every repository on this host; this change reads no new record format, so no migration is needed.

## Design

docs/strategies/simpler-lifecycle.md, Shape: "Approvals cover code".
