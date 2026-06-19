---
created_at: "2026-06-19T03:57:20.400534757+00:00"
id: "atelier-qdgh"
issue_type: "epic"
labels:
- "provider"
- "review"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0jsk"
  - kind: "issue"
    id: "atelier-oe7c"
  - kind: "issue"
    id: "atelier-q199"
  - kind: "issue"
    id: "atelier-swxv"
  children:
  - kind: "issue"
    id: "atelier-69g3"
  - kind: "issue"
    id: "atelier-rb5b"
  - kind: "issue"
    id: "atelier-unwz"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Rename provider PR commands to review"
updated_at: "2026-06-19T03:58:54.855785909+00:00"
---

## Description

Move the current provider-backed PR functionality under `atelier review` and
remove the public `atelier pr` command surface. This epic owns provider command
rename and Forgejo parity, not native room behavior.

## Outcome

- Existing Forgejo PR open/link/status/show/comment/comments/review/merge
  behavior is reachable through `atelier review` commands.
- Provider-mode records use `review.kind: pull_request` and the configured
  provider, not the legacy `pull_request` field.
- `atelier pr ...` dispatch and help are removed rather than aliased.
- Provider validators, errors, and docs point users to `atelier review ...`.

## Evidence

- Provider-mode regression tests cover the renamed command surface and Forgejo
  parity.
- Negative CLI tests prove `atelier pr` is rejected with no compatibility
  dispatch.
- Search output proves active help/docs no longer instruct users to run
  `atelier pr`.
