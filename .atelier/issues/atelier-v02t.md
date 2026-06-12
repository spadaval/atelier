---
acceptance: []
created_at: "2026-06-12T00:59:08.810163494+00:00"
evidence_required: []
id: "atelier-v02t"
issue_type: "task"
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
status: "open"
title: "Specify root status, mission status, and mission show contracts"
updated_at: "2026-06-12T01:19:13.203406567+00:00"
---

Define the split between root status, mission status, and mission show. Acceptance: root status answers what the current checkout/operator should know before acting; mission status is a mission operational drill-down for advancement and closeout; mission show is the durable record/document view. The spec must explicitly account for the one-active-mission invariant and avoid overlapping half-document, half-status outputs.
