---
created_at: "2026-06-29T20:15:16.029096137+00:00"
id: "atelier-ie31"
issue_type: "task"
labels:
- "cli"
- "complexity"
- "forgejo"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Hide routine Forgejo provider setup from normal workflow"
updated_at: "2026-06-29T20:15:16.029096137+00:00"
---

## Description

The command audit flags Forgejo/provider setup as useful plumbing that should not read like a normal operator workflow. Routine review commands should derive provider fields from issue context, and explicit provider setup should be reachable only from setup or recovery guidance.

## Outcome

Root help, role guides, and command audit no longer present Forgejo/provider setup as a routine workflow. If the command remains, it is hidden, admin-scoped, or referenced only from explicit setup/recovery paths, and tests or help snapshots prove the public surface stays bounded.
