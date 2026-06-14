---
created_at: "2026-06-14T02:52:40.095978862+00:00"
id: "atelier-613f"
issue_type: "task"
labels:
- "reliability"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Filter runtime cache and temp files from canonical diagnostics"
updated_at: "2026-06-14T02:52:40.095978862+00:00"
---

## Description

Ensure runtime, cache, journal, lock, and temporary files do not appear as unsupported canonical projection files or canonical lint inputs.

## Outcome

Canonical diagnostics only report committed record/config problems, not rebuildable local artifacts.

## Evidence

Tests or validation fixtures cover representative runtime/cache/temp files.
