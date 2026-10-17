---
created_at: "2026-10-17T08:54:21.810949377+00:00"
id: "atelier-hueh"
issue_type: "bug"
labels:
- "orientation"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Judge ready work on the records start will check"
updated_at: "2026-10-17T08:54:21.810949377+00:00"
---

## Goal

When a work item's owner branch already exists, `start` checks that branch's records (`crates/atelier-cli/src/commands/start.rs`, about lines 52-66), while views classify from the current checkout (`crates/atelier-app/src/work_classification.rs`, about line 206). Repro from review of atelier-6wyv (finding atelier-kv1z): parent P with children C1 and C2, C2 blocked by B; `start C1` creates `task/P`; on main, supersede B and commit; `work mission`, `work parent` and `work ready` now list C2 as ready and offer `atelier start C2`, which refuses because B is still open on `task/P`.

## Done when

- Views either judge such rows on the owner branch's records or mark and skip them, as rows started on another branch already are.
- A session test reproduces the case above.
