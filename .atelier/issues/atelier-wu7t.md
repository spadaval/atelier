---
created_at: "2026-10-23T18:02:22.121093790+00:00"
id: "atelier-wu7t"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1o56"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Give the right reason when evidence is stale because its item was uncommitted at the pin"
updated_at: "2026-10-23T18:02:22.121093790+00:00"
---

## Goal

Evidence freshness reports `stale (pin not in HEAD's history)` when the pin is in HEAD's history but the item's record was not committed at the pin. `staleness` in crates/atelier-app/src/evidence_freshness.rs maps every `first_mismatch` error to NotInHistory. The stale verdict is right; the reason is wrong.

## Done when

A test: start an item without committing records, commit code, capture a passing receipt, commit the records; `evidence record`, `list` and `show` give a reason naming the uncommitted record, not history.

## Context

Found by acceptance validation atelier-1o56.
