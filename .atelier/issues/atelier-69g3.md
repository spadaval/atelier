---
created_at: "2026-06-19T03:58:49.566638519+00:00"
id: "atelier-69g3"
issue_type: "feature"
labels:
- "provider"
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-19T04:46:49.733969352+00:00"
status: "done"
title: "Rename pr command dispatch and help to review"
updated_at: "2026-06-19T04:46:49.733969352+00:00"
---

## Description

Rename the CLI parser, dispatch, and help surface from `pr` to `review` for
provider-backed review behavior.

## Outcome

- `atelier review open/link/status/show/merge/comments/comment` dispatches to
  the provider-backed implementation when review mode is provider.
- Existing review action language is updated so approve/request-changes are
  exposed as review commands rather than PR commands.
- Help output lists `review` commands with room/provider mode notes.
- Command output names review artifacts rather than PRs where the concept is
  mode-neutral.

## Evidence

- CLI parser/help tests prove `atelier review --help` and subcommand help expose
  the renamed command surface.
- Provider command tests prove dispatch reaches the existing Forgejo behavior.
- `atelier lint atelier-69g3` and focused CLI tests pass.
