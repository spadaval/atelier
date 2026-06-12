---
created_at: "2026-06-11T20:10:01.031144865+00:00"
id: "atelier-24sg"
issue_type: "epic"
labels:
- "cache"
- "migration"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-dnsx"
  - kind: "issue"
    id: "atelier-g9fd"
  - kind: "issue"
    id: "atelier-gye2"
  - kind: "issue"
    id: "atelier-unma"
  attachments:
  - kind: "evidence"
    id: "atelier-opbm"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Validate markdown-first migration, recovery, and cache behavior"
updated_at: "2026-06-12T00:30:14.129098182+00:00"
---

## Description

Prove the markdown-first system recovers from committed Markdown alone and fails safely when canonical records are invalid.

## Outcome

- Fixtures cover old .atelier-state plus .atelier/state.db migrating into single .atelier/ layout with ignored runtime files.
- Direct-edit tests manually add, edit, and delete Markdown records, then prove lint, show, ready list, search, and relationship commands reflect changes after auto-refresh.
- Cache tests delete state.db and simulate checkout-like clean state; commands rebuild or fail with clear guidance.
- Regression suite includes cargo fmt -- --check, cargo nextest run, extended ignored/property tests where storage invariants are touched, atelier lint, atelier doctor, and atelier export --check during migration.

## Evidence

Evidence was not specified in the legacy issue record.
