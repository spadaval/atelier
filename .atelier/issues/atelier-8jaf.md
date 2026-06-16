---
created_at: "2026-06-16T15:47:07.032397268+00:00"
id: "atelier-8jaf"
issue_type: "task"
labels:
- "branch"
- "cli"
- "docs"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0nv2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-16T17:17:55.955823235+00:00"
status: "done"
title: "Demote explicit branch commands from normal workflow docs and help"
updated_at: "2026-06-16T17:17:55.955823235+00:00"
---

## Description

Update public command docs, help, command audits, and Agent Factory guidance so explicit branch commands are not taught as the routine way to work. Branch setup belongs to `atelier start`, and branch integration belongs to close.

## Outcome

- Root help, role guides, product docs, command audits, `AGENTFACTORY.md`, and `/root/.agents/skills/agent-factory` describe lifecycle-owned branch setup and close-time squash merge.
- `atelier branch for-epic` is removed, hidden, or documented only as an advanced repair/diagnostic command; it is not a normal workflow step in help or next actions.
- Docs describe squash merge as the default integration strategy and point to the configuration field for alternate strategies.
- Docs preserve the product concept that operators start and close work, while Git details appear only when they affect recovery or audit.
- Historical docs that retain old branch command examples are explicitly marked historical or excluded from target-state guidance.

## Evidence

- Search command transcript using `rg` over product docs, command audits, `AGENTFACTORY.md`, `/root/.agents/skills/agent-factory`, and CLI help fixtures shows no target-state routine workflow telling agents to run `atelier branch for-epic`.
- Help transcript shows `atelier start` and `atelier issue close` as the normal lifecycle path.
- File diff for the product docs or workflow config docs shows the squash-merge default and customization point.
- `atelier lint atelier-8jaf`, `atelier export --check`, `git diff --check`, and relevant docs/help tests pass.
