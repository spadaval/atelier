---
created_at: "2026-06-08T19:39:56+00:00"
id: "atelier-0014"
issue_type: "task"
labels:
- "agent-factory"
- "assignee:root"
- "docs"
- "migration"
- "mission"
- "skill"
- "task"
- "tracker"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0015"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-08T22:28:18.592363471+00:00"
status: "done"
title: "Update Agent Factory skill to support Atelier tracker bindings"
updated_at: "2026-06-08T22:28:18.592363471+00:00"
---

## Description

Update the Agent Factory skill so Beads is no longer hard-coded as the default durable work queue. The skill should describe a tracker abstraction and support repositories that bind to Atelier, while preserving Beads instructions only as legacy/fallback guidance.

Scope includes SKILL.md wording, planning/orchestration procedures, tracker command standards, install/onboarding docs, and any examples that currently assume bd commands.

## Outcome

Agent Factory docs can route tracker operations through the repository's AGENTFACTORY.md binding; Atelier command examples exist for planning, ready, update, dependency, close, lint, sync/check; Beads-specific mechanics are isolated in a legacy section; new repositories can choose Atelier as their tracker during onboarding.

## Evidence

Evidence was not specified in the legacy issue record.
