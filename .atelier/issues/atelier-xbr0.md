---
created_at: "2026-06-14T03:51:21.428697688+00:00"
id: "atelier-xbr0"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T07:24:44.406896361+00:00"
status: "done"
title: "Clarify mission create section semantics in help"
updated_at: "2026-06-14T07:24:44.406896361+00:00"
---

## Description

mission create exposes --body, --constraint, --risk, and --validation, but help does not say that --body populates Intent rather than the whole Markdown body. This caused real issue-creation confusion during the operability mission setup. Update command help and examples so section ownership is explicit. Validation: atelier mission create --help names the generated sections and states that --body maps to Intent; docs or examples show how to create a mission with constraints, risks, and validation without post-hoc Markdown repair.

## Outcome

`atelier mission create --help` clearly states which generated mission section
each flag populates, especially that `--body` maps to Intent. Mission creation
examples show constraints, risks, and validation criteria without requiring
post-creation canonical Markdown repair.

## Evidence

- `atelier mission create --help` transcript names the generated mission
  sections and states that `--body` maps to Intent.
- File diff or docs example shows mission creation with constraints, risks, and
  validation without requiring post-hoc canonical Markdown repair.
- `git diff --check` and `atelier lint` pass after
  the help/docs changes.
