---
created_at: "2026-06-13T20:36:58.492825952+00:00"
id: "atelier-2rf7"
issue_type: "task"
labels:
- "artifact-update"
- "cli"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-5a73"
  - kind: "issue"
    id: "atelier-d7lw"
  - kind: "issue"
    id: "atelier-rgd1"
  - kind: "issue"
    id: "atelier-u08r"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Define command-purpose and output contract"
updated_at: "2026-06-13T22:41:56.094803697+00:00"
---

## Description

Write or update the durable CLI contract so each command group has one purpose, a default information budget, and named drill-down surfaces. The contract must resolve whether visible predecessor shapes such as evidence add/capture, root export/rebuild/lint/doctor placement, dep/link/graph/note boundaries, and hidden compatibility commands remain product surface.

## Outcome

- Product docs identify the job, default output, quiet output, and drill-down path for every visible command group.
- Predecessor or compatibility command shapes are classified as remove, hide temporarily with explicit reason, or keep with a distinct purpose.
- Dependent CLI implementation items have a clear contract to execute against.

## Evidence

- Documentation diff or ADR captures the command-purpose table and classification.
- Help transcript review confirms the documented surface matches current help or names the exact implementation follow-up.
- atelier lint and atelier export --check pass after the artifact update.
