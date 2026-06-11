---
acceptance: []
created_at: "2026-06-09T17:30:35.841097873+00:00"
evidence_required: []
id: "atelier-001i"
issue_type: "task"
labels:
- "assignee:root"
- "beads:type:feature"
- "cli"
- "domain-model"
- "plan"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000u"
  - kind: "issue"
    id: "atelier-001n"
  - kind: "issue"
    id: "atelier-001t"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Add first-class plan commands"
updated_at: "2026-06-10T04:30:15.278404729+00:00"
---

Add first-class durable plan commands for execution intent that must survive across sessions or agents.

## Acceptance

`atelier plan create/show/revise/link` exists with stable JSON, plan revisions preserve reasoned history, plans export and rebuild deterministically, and plans can link to missions, milestones, issues, and evidence.
## Validation

- `cargo fmt -- --check`
- `cargo test` or a named focused substitute
- `git diff --check`
- `atelier lint`
- `atelier export --check`
- `atelier doctor`
