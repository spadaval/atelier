---
created_at: "2026-06-08T19:40:03+00:00"
id: "atelier-0015"
issue_type: "task"
labels:
- "agent-factory"
- "dogfood"
- "migration"
- "mission"
- "task"
- "tracker"
- "validation"
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-0017"
  - kind: "issue"
    id: "atelier-0018"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Validate Agent Factory workflows on Atelier"
updated_at: "2026-06-08T22:33:11.061279585+00:00"
---

## Description

Run scenario-centered validation that Agent Factory can operate through Atelier after the repo and skill bindings are updated. Cover planning, ready-work discovery, issue creation, dependency edits, status updates, notes, closeout, validation evidence, sync/rebuild, and failure recovery.

This validation should use real repository work where possible rather than only synthetic fixtures.

## Outcome

Validation evidence includes command transcripts or tests for plan, ready, show, create, update, dependency add/remove, close, lint/check, export --check, rebuild, and sync; at least one real Agent Factory planning or docs task is completed using Atelier only; failures are classified into fixed, deferred with owner, or not applicable.

## Evidence

Evidence was not specified in the legacy issue record.
