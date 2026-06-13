---
created_at: "2026-06-13T20:55:09.170535879+00:00"
id: "atelier-i9ob"
issue_type: "feature"
labels:
- "agent-readiness"
- "assignee:root"
- "cli"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Move work selection guidance into Atelier status surfaces"
updated_at: "2026-06-13T23:51:48.858292322+00:00"
---

## Description

Make Atelier answer the recurring fresh-agent question of what work is selectable next from the active mission or epic graph, with blockers and proof gaps explained by focused status output.

## Outcome

- `atelier status`, `atelier mission status`, or a documented focused command identifies selectable work in the active mission graph without requiring agents to manually infer it from broad issue lists.
- Output explains why candidates are ready or blocked using concise blocker, parent, and proof-gap information.
- AGENTFACTORY.md no longer needs to teach mission or epic graph filtering beyond invoking the Atelier-owned status surface.

## Evidence

- Command transcript shows a fresh-agent work-selection flow from repository entry to next selectable issue.
- Help or docs identify the focused work-selection command and expected default output.
- `atelier lint`, `atelier export --check`, `atelier doctor`, and focused CLI tests or transcripts pass.
