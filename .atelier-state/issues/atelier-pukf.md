---
acceptance: []
blocks: []
created_at: "2026-06-11T02:01:40.065583473+00:00"
depends_on: []
evidence_required: []
id: "atelier-pukf"
issue_type: "task"
labels:
- "assignee:root"
- "cli-output"
links: []
parent: null
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Group queue and relationship list output"
updated_at: "2026-06-11T02:07:13.891383113+00:00"
---

Scope: Update issue list, issue ready, issue search, link list, and dep list human output to follow docs/architecture/human-cli-output.md queue/list grammar. Remove raw parent=/blocked_by= title tails where feasible and replace edge-syntax relationship dumps with grouped readable sections. Preserve JSON output.

Acceptance: queue summaries use readable labels; rows keep titles as titles; link list groups by relation/target kind; dep list includes useful issue context instead of quoted sentence dumps. Add or update focused tests.
