---
created_at: "2026-10-15T19:31:01.984457690+00:00"
id: "atelier-749l"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-749l
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Name dirty files when land evidence freshness fails"
updated_at: "2026-10-16T19:03:26.703162487+00:00"
---

## Goal

When uncommitted non-record files make done items' passing evidence stale, `atelier land` names the dirty paths instead of only reporting an older failing evidence record.

## Done when

A test with a dirty code file and a done item with an old fail and a newer pass shows a message naming the dirty file.

## Context

Independent validation 2026-10-15: in a clean clone, `echo x > crates/scratch_dirty.rs; atelier land` reports "newer failing evidence atelier-3onc".
