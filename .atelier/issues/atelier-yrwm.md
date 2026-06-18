---
created_at: "2026-06-17T18:00:38.748336356+00:00"
id: "atelier-yrwm"
issue_type: "task"
labels:
- "cli"
- "pr"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T01:00:58.310277720+00:00"
status: "done"
title: "Add pr open, status, show, comments, comment, and review commands"
updated_at: "2026-06-18T01:00:58.310277720+00:00"
---

## Description

Add the public `atelier pr` command set for Forgejo pull request review work.

## Outcome

- `pr open`, `pr status`, `pr show`, `pr comments`, `pr comment`, and
  `pr review` are available with concise help.
- Commands infer the PR from active session, target issue or epic, or current
  owner branch where unambiguous.
- PR commands do not apply workflow transitions.

## Evidence

- Focused CLI tests prove command help, target inference, ambiguous target
  failure, and no workflow-transition side effects.
- Command transcript shows targeted `atelier pr` CLI tests pass.
