---
created_at: "2026-10-17T15:51:59.572033649+00:00"
id: "atelier-isyf"
issue_type: "task"
labels:
- "adoption"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Pin merge checks to the merged checkout's own target directory"
updated_at: "2026-10-22T19:58:19.144250530+00:00"
---

## Goal

`shell.check` passes the caller environment through, so an exported `CARGO_TARGET_DIR` makes the merge-check builds share a target outside the merged checkout. Shared targets have run another branch's binary in tests here. Found in the atelier-qymu review (atelier-oi1z).

## Done when

- The nextest merge checks in `.atelier/workflow.yaml` set `CARGO_TARGET_DIR=target`, and the comment says why `.atelier` is excluded from the whitespace check.
