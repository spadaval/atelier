---
created_at: "2026-06-15T21:31:14.386519884+00:00"
id: "atelier-jeyr"
issue_type: "feature"
labels:
- "cli"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-9p3t"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Remove closeout command vocabulary from mission surfaces"
updated_at: "2026-06-16T19:54:03.253577623+00:00"
---

## Description

Remove closeout from live mission CLI vocabulary and route terminal-state questions through ordinary status and transition guidance.

## Outcome

- `atelier mission status` no longer exposes `--closeout`.
- Mission help, status output, blocked-close output, next commands, and man/status guidance do not use `Closeout` headings or closeout wording.
- `atelier mission audit` is removed, folded into status, or reframed as a general report only if it still has a distinct job.
- Mission close or complete output describes terminal checks without creating a separate ceremony.

## Evidence

- `atelier mission --help`, `atelier mission status --help`, `atelier status`, and representative blocked mission output show no `--closeout` flag or `Closeout` headings.
- `rg` over `crates/atelier-cli/src` for user-facing closeout strings returns no live matches except explicit historical-test fixtures if retained.
- Focused CLI integration tests cover the replacement status and blocked terminal-check output.
