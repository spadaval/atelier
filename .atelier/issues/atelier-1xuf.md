---
created_at: "2026-06-13T20:44:51.132496135+00:00"
id: "atelier-1xuf"
issue_type: "task"
labels:
- "readiness"
- "repo-hygiene"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Decide repository ownership and contribution templates"
updated_at: "2026-06-13T23:10:46.710213507+00:00"
---

## Description

The repo currently lacks .github ownership and contribution scaffolding. Decide whether CODEOWNERS, issue templates, PR template, and dependency update automation are useful for Atelier now, and add only the pieces that reduce agent or human coordination mistakes.

## Outcome

- CODEOWNERS, PR template, issue templates, and dependency update automation are each classified as add, defer, or not applicable.
- Any added templates reinforce Atelier tracker usage and do not create a competing GitHub issue workflow.
- Dependency update automation is added or explicitly deferred with a documented reason.

## Evidence

- Review artifact or docs diff records the classification for each .github hygiene item.
- File changes for .github templates or automation are inspected and shown in `git diff --check` output.
- `atelier lint` and `atelier export --check` command transcripts pass after tracker updates.
