---
created_at: "2026-06-13T02:36:02.617739676+00:00"
id: "atelier-b7wl"
issue_type: "feature"
labels:
- "cli"
- "closeout"
- "mission"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ezvf"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Fold mission audit into contextual status or closeout"
updated_at: "2026-06-13T02:36:02.617739676+00:00"
---

## Description

Implement the decided command model so mission audit information is available without making operators choose between competing normal commands.

## Outcome

- Operators can see audit gaps, missing proof, and pass summaries through the chosen mission status or closeout surface.
- Full line-by-line audit remains available through an explicit verbose or diagnostic path.
- Deprecated or hidden audit behavior has clear help text or replacement guidance if applicable.

## Evidence

- CLI tests cover compact audit/status output and verbose line-by-line output.
- Negative tests prove obsolete or confusing normal paths are absent or redirected according to the command contract.
