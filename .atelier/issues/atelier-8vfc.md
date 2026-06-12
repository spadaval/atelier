---
acceptance: []
created_at: "2026-06-12T03:08:50.805471803+00:00"
evidence_required: []
id: "atelier-8vfc"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Enrich command and command-group help text"
updated_at: "2026-06-12T03:35:22.221929929+00:00"
---

Improve help text so command groups and common commands explain what they are for, when to use them, and how they relate to neighboring commands. Scope: top-level help, major groups such as issue, mission, start/finish, worktree, evidence, status, setup/config/state/maintenance, and representative subcommands. Acceptance: help output includes useful short descriptions, examples or common workflows where appropriate, boundaries between similar commands, and compatibility/deprecation notes for moved or advanced surfaces. Help remains concise enough to scan and does not reintroduce private-context assumptions or command-result JSON guidance.
