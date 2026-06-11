---
acceptance: []
created_at: "2026-06-09T19:41:08.140632616+00:00"
evidence_required: []
id: "atelier-001l"
issue_type: "task"
labels:
- "assignee:root"
- "domain-model"
- "milestone"
- "task"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000u"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Define first-class milestone checkpoint records"
updated_at: "2026-06-09T20:20:08.248232780+00:00"
---

Define first-class milestone records as validated checkpoint states, not work containers or super-epics.

Acceptance:
Milestone records model desired_state, scope boundaries, validation_criteria, accepted evidence, completion_state, linked mission or missions, and contributing work links. Existing inherited atelier milestone behavior is classified as compatibility unless migrated. Docs and tests prove milestones are validated by evidence and workflow validators, not by milestone-attached transition checks.
