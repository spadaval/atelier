---
created_at: "2026-06-24T19:26:12.686012959+00:00"
id: "atelier-47cp"
issue_type: "epic"
labels: []
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-8c91"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-z0ll"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove redundant status, mission, blocker, table, and search surfaces"
updated_at: "2026-06-24T19:26:12.686012959+00:00"
---

## Description

After replacement read surfaces land, delete obsolete or redundant command paths instead of preserving aliases. Target removals include mission list/status, issue status, issue blocked, issue table, root search, scoped history variants that do not earn a focused owner, and stale next-action/help references. Preserve real capabilities through work views, issue show, issue transition, high-level history, and the centralized read pipeline.

Constraints:
- Start only after `atelier-fasv` proves replacement read surfaces preserve the surviving capabilities.
- Do not leave compatibility aliases, hidden normal-help entries, or stale guidance for removed commands unless a human explicitly approves a compatibility window.
- Do not replace root `search` with `issue list --query` during this cut.
- Keep failure output explicit enough to tell users the command was removed and which surviving command owns the capability.

## Outcome

Obsolete command paths are deleted after their capabilities have surviving owners. Mission list/status, issue status, issue blocked, issue table, root search, unnecessary scoped history variants, and stale help/next-action references no longer exist as public compatibility surfaces.

## Evidence

- Focused CLI tests prove removed commands fail without compatibility aliases after their replacement surfaces pass.
- Help/man/docs search proof shows removed command surfaces are not taught as normal workflow.
- Replacement transcripts prove objective rollup, blocker detail, table/list inventory, and high-level history still have surviving owners.
