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
  - kind: "review"
    id: "atelier-pew5"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-83w3"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Batch the per-review history walks in land with exact answers"
updated_at: "2026-10-22T20:57:28.440000489+00:00"
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
- 2026-10-22 [note] From atelier-as7i: two history queries share the merge blind spot's neighbours. (1) introduced_query in crates/atelier-app/src/review_gate.rs uses --diff-filter=A, so a review record added in a commit that deletes a similar file is shown as a rename (R), not an add, and gets no introducing commit; adding --no-renames would fix it. (2) append_only_history's first-review query (same file, 'log --root --reverse --diff-filter=A ... -- .atelier/reviews/') has no --diff-merges=first-parent, so a repo whose first review record appears in a conflict-resolving merge would find no first commit, or a later one; it needs the same --diff-merges=first-parent --no-patch (and --no-renames) as introduced_query. Cover both in the merge parity tests.
- 2026-10-22 [note] From review of atelier-as7i (finding atelier-83w3): the as7i decision note and the comment at review_gate.rs:1084 wrongly say a clean merge stays simplified away under --diff-merges=first-parent; git lists both the side commit and the merge. Callers rely on --reverse putting the true introducer first; correct the note and document that contract when rewriting the queries.
