---
acceptance: []
created_at: "2026-06-09T17:30:35.891065842+00:00"
evidence_required: []
id: "atelier-001k"
issue_type: "task"
labels:
- "beads:type:feature"
- "cli"
- "domain-model"
- "validator"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000l"
  - kind: "issue"
    id: "atelier-000u"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Add workflow validator evaluation command"
updated_at: "2026-06-10T14:51:43.085892951+00:00"
---

Add a first-class workflow validator evaluation command that can evaluate configured tracker and workflow transition conditions without mutating work state.

Acceptance:
A command such as atelier workflow validate exists with stable machine-readable output for configured validators such as durable state current, tests passed evidence present, evidence attached, validation criteria satisfied, review complete, and no blocking lints. Results identify the target record, transition, validator, pass/fail result, and actionable reason. Validator results can be used by workflow transitions and Mission Control projections.
