---
created_at: "2026-06-17T18:00:36.206985183+00:00"
id: "atelier-mpah"
issue_type: "task"
labels:
- "client"
- "forgejo"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Implement Forgejo PR client with sudo-mode role authorship"
updated_at: "2026-06-17T18:00:36.206985183+00:00"
---

## Description

Implement the Forgejo API client used by `atelier pr` and PR validators,
including sudo-mode role authorship.

## Outcome

- The client can find, open, show, comment on, review, and inspect comments for
  Forgejo PRs.
- Remote write requests include the configured `Sudo:` user for the active
  session role.
- API failures are rendered as actionable operator errors without silently
  falling back to admin authorship.

## Evidence

- Mocked HTTP tests prove request paths, payloads, `Sudo:` headers, merged/open
  PR states, and failure handling.
- Command transcript shows targeted Forgejo client tests pass.
