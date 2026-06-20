---
created_at: "2026-06-20T16:47:54.924067578+00:00"
id: "atelier-oc4x"
issue_type: "epic"
labels:
- "command-surface"
- "cutting-pass"
- "review"
review:
  kind: pull_request
  number: 15
  provider: forgejo
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-0rx5"
  - kind: "issue"
    id: "atelier-3d81"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Epic: Simplify review and provider command surface"
updated_at: "2026-06-20T21:25:42.439747683+00:00"
---

## Description

Simplify review and provider command behavior so lifecycle/status output owns
when review artifacts are required, while provider setup remains explicit admin
work. This epic keeps review validation and Forgejo provisioning from leaking
into ordinary worker command loops.

## Outcome

- Review-link validation has one app-layer contract used by CLI commands and
  workflow validators.
- Forgejo provisioning keeps only the supported roles workflow and drops
  rejected flags or compatibility paths.
- Normal issue transition/status output routes review work without requiring
  operators to know provider internals.

## Evidence

- Focused review/provider tests prove the shared validation contract.
- Forgejo help and command audit document only supported setup commands.
- Removed provider flags fail without compatibility aliases.
