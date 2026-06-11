---
acceptance: []
created_at: "2026-06-11T02:45:01.078632625+00:00"
evidence_required: []
id: "atelier-kaei"
issue_type: "task"
labels:
- "assignee:root"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-vfqo"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Delete or simplify JSON result formatter code"
updated_at: "2026-06-11T04:31:01.560730327+00:00"
---

Remove output-format plumbing, JSON serialization branches, and command result renderers that exist only for command-result JSON mode. Acceptance: remaining formatter abstractions are human/quiet focused; dead code and unused dependencies are removed; compiler warnings stay clean.
