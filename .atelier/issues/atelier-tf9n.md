---
created_at: "2026-10-17T01:10:57.984299499+00:00"
id: "atelier-tf9n"
issue_type: "bug"
labels:
- "cli-output"
- "work-views"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show direct blockers on work rows, mark inherited ones, drop the count"
updated_at: "2026-10-17T01:10:57.984299499+00:00"
---

## Goal

Work-view rows show the whole transitive blocker closure plus blockers inherited from the parent, sorted by ID: `open_blockers` is `dependency_closure(...).blocking_ids()` (crates/atelier-app/src/work_classification.rs:350-361). So `5hp9 … | blockers: atelier-fhep, atelier-il31, atelier-ubti | 3 blockers` while `issue show atelier-5hp9` "Blocked by" lists only fhep; `1gc5 | blockers: atelier-9vdd, atelier-spvd` hides that spvd blocks its parent; active row `s47h [in_progress] … | blockers: atelier-spvd` gives no hint it is inherited. Repo-wide `work blocked` ("Repo-wide blocker triage") prints only `| 3 blockers`, no IDs. Also `apply_ready_execution_gates` pushes the literal string `"readiness gate"` into `open_blockers` (crates/atelier-cli/src/commands/work.rs:192), which inflates the count and would print as a fake blocker ID once IDs are shown.

## Done when

- Rows in `work mission`, `work parent`, `work blocked` and `work all` list direct blockers, and mark inherited ones: `⚠ blocked by atelier-fhep; via parent atelier-qhnh: atelier-spvd`.
- A row whose chain is longer than 1 names its far end: `(chain of 3, ends at atelier-ubti)`.
- The `| N blocker(s)` suffix is gone (human_output.rs:363-367).
- A gate failure is carried as a separate reason, not as a blocker ID; no row ever prints "readiness gate" as a blocker.
- A test pins `work blocked` and `issue show` "Blocked by" to the same direct blocker IDs for the same issue.

## Context

Reworks the same row rendering as atelier-deyy. Fixture: scratch clone with mission atelier-j76w; parent atelier-qhnh (children s47h active, 9t9q blocked_by s47h, 1gc5 blocked_by 9vdd); qhnh blocked_by decision atelier-spvd (needs a human); chain 5hp9 <- fhep <- il31 <- ubti (ubti ready, low, no mission); 9vdd advances superseded mission atelier-ba3h.
