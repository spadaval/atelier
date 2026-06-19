---
created_at: "2026-06-19T03:58:54.854990625+00:00"
id: "atelier-unwz"
issue_type: "task"
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
closed_at: "2026-06-19T04:47:10.465868168+00:00"
status: "done"
title: "Remove pr command surface and legacy guidance"
updated_at: "2026-06-19T04:47:10.465868168+00:00"
---

## Description

Remove the old `pr` public command surface and stale guidance after provider
behavior has moved to `review`.

## Outcome

- `atelier pr ...` is no longer accepted by the CLI.
- Root help, role guides, command docs, validation guidance, and error messages
  do not list `pr` as an active command.
- Old-command tests, docs, and fixtures are either removed or rewritten to the
  new `review` command surface.
- No compatibility alias, hidden shim, or staged deprecation remains.

## Evidence

- Negative CLI test output proves `atelier pr` is rejected.
- `rg -n 'atelier pr|\\bpr open\\b|pull_request:'` over active docs, source,
  tests, and fixtures is reviewed and any remaining hits are historical or
  migration-specific.
- `atelier lint atelier-unwz`, focused CLI tests, and `git diff --check` pass.
