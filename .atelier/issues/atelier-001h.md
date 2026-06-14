---
created_at: "2026-06-09T17:30:35.817157405+00:00"
id: "atelier-001h"
issue_type: "task"
labels:
- "assignee:root"
- "beads:type:feature"
- "cli"
- "domain-model"
- "mission"
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
closed_at: "2026-06-10T04:30:15.245386891+00:00"
status: "done"
title: "Add first-class mission commands"
updated_at: "2026-06-10T04:30:15.245386891+00:00"
---

## Description

Add first-class mission commands and persistence as part of the domain-model upgrade. A mission is the durable objective record for coordinated work: it owns intent, constraints, risks, validation expectations, and links to milestones, plans, evidence, epics, and issues without being represented as a generic issue or label.
`atelier mission create/show/list/update` exists with stable JSON; mission records export and rebuild deterministically using the single project-scoped random ID form; missions can link to milestones, plans, evidence, epics, and issues without being represented as generic issues; `mission show` exposes enough context for an agent to understand intent, constraints, active checkpoints, current risks, and linked work; compatibility with existing issue-shaped mission records is explicitly handled by the identity cutover and migration, not a long-lived dual implementation.
- `cargo fmt -- --check`

- `cargo test` or a named focused substitute

- `git diff --check`

- `atelier lint`

- `atelier export --check`

- `atelier doctor`
### Scope

- Add `atelier mission create/show/list/update` with stable human and JSON output.
- Persist mission records as first-class records with deterministic project-scoped random IDs and timestamps.
- Export missions to `.atelier-state/missions/<record-id>.md` and rebuild them into SQLite deterministically.
- Include mission fields needed by orchestrators: title, status, summary/body, constraints, current risks, validation expectations, milestone IDs, plan IDs, evidence IDs, and linked work IDs where available.
- Keep issue and epic work accountable as issues; missions describe objective context and gather links rather than becoming work queues.
- Do not introduce `MIS-*` IDs or numeric mission aliases.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
