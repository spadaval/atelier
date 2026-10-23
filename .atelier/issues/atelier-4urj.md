---
created_at: "2026-10-23T16:21:00.005772626+00:00"
id: "atelier-4urj"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-l82f"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Give evidence gates and evidence views one supersession rule"
updated_at: "2026-10-23T16:21:00.005772626+00:00"
---

## Goal

The rule for which receipt supersedes which (pass versus newer fail) exists three times: `evaluate_issue_gate`, `newest_failure`, and `EvidenceReadContext::item_proof` (crates/atelier-app/src/evidence_freshness.rs), which `evidence list` and `show` use. The gates do not call `item_proof`, so the views agree with the gates only because a test says so. `evidence list` also ignores a gate's kind filter, and `evidence show` judges only a receipt's first issue target.

## Done when

The gates and the evidence views call one function for supersession and freshness, gate decisions are unchanged (existing gate tests pass untouched), and `evidence show` reports a reused receipt's standing on each item it is attached to.

## Context

Review finding atelier-l82f on atelier-l7nz.
