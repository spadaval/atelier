---
created_at: "2026-06-29T18:20:56.771882055+00:00"
id: "atelier-gsor"
issue_type: "task"
labels:
- "agent-factory"
- "cli"
- "docs"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ubf2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Update CLI help, role guides, and docs for issue inventory versus work dashboards"
updated_at: "2026-06-29T18:20:56.771882055+00:00"
---

## Description

Update public help, role guidance, and product documentation so agents see the new command split consistently: `issue list` is generic issue inventory, `work ready` is the small top-level picker, `work blocked` is blocker triage, `work mission <id>` is the mission dashboard, and `issue show <id>` remains canonical record detail. `work queue` is not taught as the normal worker or manager entry point unless a separate implementation issue proves one explicit repo-wide job for it.

## Outcome

Help text, `atelier man` role surfaces, Agent Factory guidance where applicable, and command-audit/product docs no longer teach unsupported or contradictory commands such as `work queue --type`, `work queue --kind`, removed `issue list` assumptions, `work queue --ready` / `work queue --blocked` as normal role flow, or repo-wide `work queue --all` as the mission-dashboard drill-down. Documentation explains when to use `issue list`, `work ready`, `work blocked`, `work mission <id>`, and `issue show <id>` without creating compatibility aliases.
