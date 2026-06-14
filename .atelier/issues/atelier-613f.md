---
created_at: "2026-06-14T02:52:40.095978862+00:00"
id: "atelier-613f"
issue_type: "task"
labels:
- "assignee:root"
- "reliability"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Filter runtime cache and temp files from canonical diagnostics"
updated_at: "2026-06-14T06:34:50.320542479+00:00"
---

## Description

Ensure runtime, cache, journal, lock, and temporary files do not appear as unsupported canonical projection files or canonical lint inputs.

## Outcome

Canonical diagnostics only report committed record/config problems, not rebuildable local artifacts.

## Evidence

- Tests or validation fixtures place representative ignored artifacts under
  `.atelier/runtime/`, `.atelier/cache/`, lock/journal paths, and temporary
  files near canonical records.
- `atelier lint`, projection refresh, and canonical diagnostics ignore
  rebuildable runtime/cache/temp artifacts while still reporting a deliberately
  malformed committed record.
- `git diff --check`, `atelier lint`, and the focused canonical-diagnostics
  tests pass.
