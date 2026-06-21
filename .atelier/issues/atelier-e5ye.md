---
created_at: "2026-06-21T17:40:57.628685239+00:00"
id: "atelier-e5ye"
issue_type: "feature"
labels:
- "evidence"
- "lint"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-lkz6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Stop parser and lint from requiring evidence prose by default"
updated_at: "2026-06-21T18:38:00.114849954+00:00"
---

## Description

Issue Markdown parsing and lint should not force every ordinary issue to carry an Evidence section or concrete proof wording. Evidence prose is required only when the issue contract, workflow policy, or issue type explicitly requires it.

## Outcome

- Ordinary open issues can parse and lint without an Evidence section when no explicit contract requires one.
- Issue types or validators that require evidence can still enforce it through configured policy.
- Existing tracker records are migrated or updated only where needed for the new policy.
- Help text and docs stop teaching Evidence prose as universal boilerplate.

## Evidence

- Parser and lint tests cover ordinary issues with and without Evidence sections.
- Tests or transcripts cover an explicit policy that still requires evidence.
- Command-surface search transcript over docs and help output shows universal Evidence boilerplate guidance removed.
- atelier lint and git diff --check pass.
