---
created_at: "2026-06-15T03:55:14.603528547+00:00"
id: "atelier-q8a1"
issue_type: "task"
labels:
- "agent-factory"
- "cleanup"
- "docs"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-t35w"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Update Agent Factory workflow standards after active-work removal"
updated_at: "2026-06-15T03:55:14.603528547+00:00"
---

## Description

Remove obsolete Agent Factory guidance that still teaches hidden claim, finish, or runtime active-work association as the normal worker flow. The portable standards and this repository binding should route agents through the new Atelier-owned status-derived workflow instead.

## Outcome

- `AGENTFACTORY.md` no longer references claim, finish, or runtime active-work association as normal Atelier workflow.
- `/root/.agents/skills/agent-factory/standards/tracker.md` no longer includes worker-flow examples that call `atelier issue update <id> --claim` or `atelier finish <id>`.
- Agent Factory guidance points to `atelier man <role>`, `atelier status`, issue transitions, notes, evidence, and closeout surfaces that remain after the active issue system is removed.
- The docs distinguish portable orchestration guidance from Atelier-owned command contracts so obsolete command examples do not become hidden compatibility pressure.

## Evidence

- File change review shows the Agent Factory binding and portable tracker
  standard updated.
- Targeted search transcript over `AGENTFACTORY.md` and `/root/.agents/skills/agent-factory` shows no active guidance for `--claim`, `atelier finish`, or runtime active-work association.
- `atelier lint atelier-q8a1` and `atelier export --check` pass after the tracker updates.
- Review artifact or issue note maps the old worker-flow example to the replacement guidance.
