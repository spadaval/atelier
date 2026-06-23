---
created_at: "2026-06-23T16:21:20.075184044+00:00"
id: "atelier-sdqy"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-xa9s"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Unbundle generic record usage from domain services"
updated_at: "2026-06-23T16:21:20.075184044+00:00"
---

## Description

Move behavior toward concrete Issue, Evidence, and Review domain types and confine mixed-kind enums to storage, rebuild, import/export, or watcher boundaries.

## Outcome

- Domain services no longer depend on a generic Record enum where the concrete domain type is known.

## Evidence

Evidence was not specified in the bundle.
