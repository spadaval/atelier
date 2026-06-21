---
created_at: '2026-06-18T20:33:37.872356885+00:00'
id: atelier-i4mw
issue_type: mission
labels:
- mission
priority: P2
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: issue
    id: atelier-918i
    type: advances
schema: atelier.issue
schema_version: 1
closed_at: '2026-06-18T21:21:15.857732961+00:00'
status: closed
title: Simplify workflow configuration schema v3
updated_at: '2026-06-18T21:21:15.857732961+00:00'
---

## Description

Simplify Atelier workflow configuration so workflows own issue type applicability, policy uses direct validators, branch policy is explicit, and PR links use a built-in pull_request field instead of typed forge_pr objects.

## Outcome

### Constraints

- No compatibility aliases, hidden defaults, migration command, or old-shape parser support.
- Committed workflow policy must be complete and inspectable.

### Risks

- This changes public workflow configuration, PR canonical state, docs, tests, and current tracker records together.

## Evidence

- Manual check: Docs, ADR, CONTEXT, starter config, parser tests, PR tests, migrated .atelier/workflow.yaml, lint, doctor, export check, and focused workflow/PR tests prove schema v3 readiness.

## Notes

### Terminal Notes

- Close reason: Schema v3 workflow configuration simplification is implemented, documented, validated, and committed.

Migrated from `.atelier/missions/atelier-i4mw.md` as a declared mission objective issue.
