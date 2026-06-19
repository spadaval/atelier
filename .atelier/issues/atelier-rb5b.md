---
created_at: "2026-06-19T03:58:52.184811180+00:00"
id: "atelier-rb5b"
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
status: "todo"
title: "Preserve Forgejo provider behavior through review commands"
updated_at: "2026-06-19T03:58:52.184811180+00:00"
---

## Description

Preserve current Forgejo behavior after the command rename and config migration.
This issue owns parity for provider-backed review actions.

## Outcome

- Forgejo open, link, status, show, comments, comment, approve,
  request-changes, and merge work through `atelier review` in provider mode.
- Provider review links write `review.kind: pull_request` with provider and ID
  fields.
- Provider-only `review link` hard-rejects in room mode.
- Room-only commands hard-reject in provider mode with direct guidance.

## Evidence

- Existing Forgejo integration tests are ported or extended to the `review`
  command surface.
- Wrong-mode tests prove provider-only and room-only rejections.
- Provider fixtures show the structured review field instead of legacy
  `pull_request`.
