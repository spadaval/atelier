---
created_at: "2026-11-08T05:51:18.943984031+00:00"
id: "atelier-g6rp"
issue_type: "mission"
labels:
- "agent-factory"
strategy:
  path: docs/strategies/agent-factory-in-atelier.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Ship Agent Factory from this repository, so a command and its guidance land in one change"
updated_at: "2026-11-08T05:53:09.141238042+00:00"
---

## Outcome

Agent Factory's only copy lives in this repository under `skills/agent-factory`, with its evals, and installs with Vercel's `skills` CLI. `atelier doctor` and `status` warn when the binary or skill is stale, `atelier setup --link` lets Sumeet edit the live skill from a checkout, the skill steps aside outside Atelier repositories, and spadaval/.agents keeps only skills that are still used. The strategy is [docs/strategies/agent-factory-in-atelier.md](../../docs/strategies/agent-factory-in-atelier.md).

## Notes

- 2026-11-08 [decision] file-pr stays a personal skill in ~/.claude/skills, not part of Agent Factory.
  Reason: It is about PR prose for any repository, Agent Factory targets Atelier repositories only, and its rule to open a real PR rather than a draft conflicts with Atelier repositories that keep a PR draft until review approves it.
