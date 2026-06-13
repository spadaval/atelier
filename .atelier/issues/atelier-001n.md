---
created_at: "2026-06-09T19:46:28.279451320+00:00"
id: "atelier-001n"
issue_type: "task"
labels:
- "bulk"
- "domain-model"
- "feature"
- "json"
- "links"
- "plan"
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-001s"
  - kind: "issue"
    id: "atelier-001t"
  - kind: "issue"
    id: "atelier-001u"
  - kind: "issue"
    id: "atelier-001v"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T15:05:38.605006117+00:00"
status: "done"
title: "Add bulk graph apply from JSON plan files"
updated_at: "2026-06-10T15:05:38.605006117+00:00"
---

## Description

Add an agent-facing bulk creation/update command for complex plans, similar in spirit to `jira plan`, that can materialize many issues and first-class records from one JSON file while preserving hierarchy, typed links, dependency sequencing, and the single project-scoped random record ID model.
A command such as `atelier plan apply <file.json>` or `atelier bulk apply <file.json>` validates and applies a multi-item plan with internal references, parent-child relationships, typed links, and blockers. Dry-run output is deterministic. Failed validation creates no partial graph. Applied records export and rebuild deterministically, and docs include a compact JSON example using project-scoped random IDs.
- `cargo fmt -- --check`

- `cargo test` or a named focused substitute

- `git diff --check`

- `atelier lint`

- `atelier export --check`

- `atelier doctor`
### Scope

- Define a versioned JSON schema for bulk graph plans with client_ref fields for intra-file references.
- Allocate durable project-scoped random IDs during apply; do not allocate numeric or typed-prefix IDs.
- Support creating issues with parent-child hierarchy, dependencies, labels, priorities, issue types, descriptions, acceptance criteria, and notes.
- Support linking created records to missions, milestones, plans, and evidence when those first-class records are available.
- Provide `--dry-run`, stable JSON preview, validation errors with file paths/client_refs, and an apply summary mapping client_ref to durable IDs.
- Apply atomically where practical; if full transactionality is not possible, document recovery behavior and emit enough mapping data for repair.
- Keep import/rebuild separate: this is for authored work plans, not backup restore or explicit-ID migrations.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
