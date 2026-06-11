---
acceptance: []
created_at: "2026-06-09T17:30:35.863516493+00:00"
evidence_required: []
id: "atelier-001j"
issue_type: "task"
labels:
- "beads:type:feature"
- "cli"
- "domain-model"
- "evidence"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000u"
  - kind: "issue"
    id: "atelier-001k"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Add first-class evidence commands"
updated_at: "2026-06-10T14:51:26.280656318+00:00"
---

Add first-class evidence commands for validation proof records.

## Acceptance

`atelier evidence add/show/list` exists with stable JSON; evidence records include kind, result, summary, path or URI, producer, and timestamp; records export and rebuild deterministically; artifact storage follows the evidence-backend decision.
## Validation

- `cargo fmt -- --check`
- `cargo test` or a named focused substitute
- `git diff --check`
- `atelier lint`
- `atelier export --check`
- `atelier doctor`
