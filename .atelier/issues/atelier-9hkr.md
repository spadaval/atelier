---
created_at: "2026-11-08T23:08:23.312691158+00:00"
id: "atelier-9hkr"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-0cjk"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "skills/agent-factory/SKILL.md:8 The opening names only success, \"Not an Atelier repository\" and atelier not found. Insi…"
updated_at: "2026-11-08T23:08:23.312691158+00:00"
---

## Goal

Deferred from finding atelier-0cjk on atelier-nz6a @dd46c40d, by `atelier review defer`.

low: skills/agent-factory/SKILL.md:8 The opening names only success, "Not an Atelier repository" and atelier not found. Inside an Atelier repository, atelier status can fail another way: a broken .atelier/config.toml gives project_config_parse_error (reproduced, exit 1), and a binary older than the repository's policy refuses too. The text leaves that case to the agent's guess, and a literal reader could step aside in a repository that uses Atelier. The error text names .atelier/, so most agents will carry on. Suggested fix when a line is free: say that any other failure means the repository uses Atelier, so diagnose it and continue.

## Done when

Done when was not specified.
