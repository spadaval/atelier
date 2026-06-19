---
created_at: "2026-06-17T17:58:28.760509937+00:00"
id: "atelier-e3pk"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.mission"
schema_version: 1
status: "ready"
title: "Superseded by 0v3f: Add session-aware Forgejo PR coordination"
updated_at: "2026-06-19T23:29:57.319328552+00:00"
---

## Intent

Agents can start work, discuss and review PRs as distinct Forgejo role users, link the active PR to the owning epic or standalone issue, and close workflow-gated work only when the linked PR is merged.

## Constraints

- Workflow status remains Atelier-owned; PR commands do not drive transitions.
- Sessions are optional durable attribution and handoff records, not the current-work source of truth.
- Forgejo is the only PR provider in this mission; sudo-mode role users provide remote authorship.
- Typed custom fields live in .atelier/workflow.yaml schema version 2.

## Risks

- Reintroducing session language can conflict with recently removed legacy runtime current-work sessions.
- PR integration can become a second workflow state machine unless validators remain read-only gates.
- Durable command transcripts can grow or leak sensitive data unless bounded and redacted.

## Validation

- CONTEXT.md, product docs, and an ADR define Session, Typed field, Pull request artifact, and the workflow/PR boundary.
- atelier start auto-creates a durable session by default, fails on active mutating-session conflicts, and supports explicit reuse and disable flags.
- .atelier/workflow.yaml supports schema version 2 typed fields and issue records can store a forge_pr field.
- .atelier/config.toml supports Forgejo connection settings and role-to-sudo-user mapping.
- atelier pr supports open/status/show/comments/comment/review, PR inference, role sudo authorship, live unresolved comments, and active PR field persistence.
- linked_pr_merged blocks configured transitions until the linked Forgejo PR is merged.
- End-to-end validation covers worker/reviewer session identity, PR linking, sudo authorship, unresolved comments, merged-PR validation, lint/export/doctor health, and docs/help parity.
