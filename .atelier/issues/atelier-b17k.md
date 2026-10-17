---
created_at: "2026-10-17T08:54:22.045186811+00:00"
id: "atelier-b17k"
issue_type: "task"
labels:
- "orientation"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Remove or justify work ready's second readiness gate"
updated_at: "2026-10-17T08:54:22.045186811+00:00"
---

## Goal

`apply_ready_execution_gates` in `crates/atelier-cli/src/commands/work.rs` (about lines 128-160) runs `dependency_readiness_batch` (`objective_graph.rs`, about line 61), which applies the same transitive blocker rule `classify_issue` already applied, so its "readiness gate" blocker label can never appear and it costs a scan of every record. Shown in atelier-6wyv and its review (finding atelier-2ecb).

## Done when

- The gate is removed, or a comment and test record the case it catches that classification does not.
