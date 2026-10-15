---
created_at: "2026-10-15T19:31:01.940323187+00:00"
id: "atelier-83xi"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Reduce land and merge --check cost on long Git histories"
updated_at: "2026-10-15T19:31:01.940323187+00:00"
---

## Goal

Keep `atelier land` and `merge --check` practical on repositories with long histories, and make the benchmark show it.

## Done when

The read-command benchmark includes a long-history fixture; the cause of the per-commit cost is diagnosed; land/merge --check on this repository are measured before and after any change.

## Context

Independent validation 2026-10-15 (S4 evidence atelier-ggv8): land and merge --check take 7.5-8.2 s on this repository (1,943 commits), with about 4,600 git processes per land, while the benchmark fixture shows about 0.3 s. Still under the 30 s Claude hook timeout. No latency budget applies (human direction 2026-10-04); this is a practical-impact observation.
