---
acceptance: []
created_at: "2026-06-12T01:19:16.583054695+00:00"
evidence_required: []
id: "atelier-zj78"
issue_type: "spike"
labels:
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Measure automatic lint feasibility for command dispatch"
updated_at: "2026-06-12T03:37:45.918283493+00:00"
---

Investigate whether lint or targeted lint-like checks should run automatically during command dispatch. Acceptance: measure current lint cost on this repository and synthetic larger tracker states; classify cheap always-on checks versus full-project lint; recommend where automatic checks should run, where workflow validators should own enforcement, and where explicit atelier lint should remain the right command.
