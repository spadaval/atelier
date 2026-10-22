---
created_at: "2026-10-16T21:55:56.381402258+00:00"
id: "atelier-7b6e"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-as7i"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Batch the per-review history walks in land with exact answers"
updated_at: "2026-10-16T23:53:46.527253791+00:00"
---

## Goal

After atelier-v5tt, land on this repository spends about 2.2 s (of 5 s) in roughly 260 path-limited git log walks, two or three per retained review event: introduced_query and the two scope_queries in review_gate.rs. Answer them from fewer walks.

## Done when

The walk count does not grow with review records, answers are identical to the per-path walks (parity tests over merges, criss-cross, deleted and re-added paths, cherry-picked review files, grafts and shallow clones), and land is measured before and after on this repository and the benchmark history fixture. The time between Git calls (in-process record reads and tree diffs, about 1.3 s of proof checks in the atelier-v5tt profile) is attributed and reduced or recorded as accepted. `ancestry_path` in git_freshness.rs walks forward from the pin over a parent-to-children map instead of repeating passes, which are quadratic when a long side branch is dated before its parents (review finding atelier-pew5), with a date-skewed parity case.

## Context

Split from atelier-v5tt on 2026-10-16.

## Unknowns

Path-limited history simplification and date ordering depend on the start commit and pathspec. A candidate: one --full-history walk per directory, used only when a path has exactly one adding commit (then the per-path walk must find it), falling back to the per-path walk otherwise. Ordering-dependent queries (first add among scope paths up to an event, first owner update with branch metadata) need a separate argument.

## Notes

- 2026-10-16 [note] 2026-10-16: atelier-37uq made land check only claims a change since a trusted point can affect, so land on this repository dropped from about 5.8 s to 0.33 s. These walks now cost mainly atelier doctor (about 4.5 s), which runs on demand, so this is no longer urgent.
