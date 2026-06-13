---
created_at: "2026-06-11T20:10:00.668923069+00:00"
id: "atelier-pjai"
issue_type: "epic"
labels:
- "lint"
- "markdown"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-24sg"
  children:
  - kind: "issue"
    id: "atelier-ca32"
  - kind: "issue"
    id: "atelier-iw2l"
  - kind: "issue"
    id: "atelier-jarw"
  - kind: "issue"
    id: "atelier-qrix"
  attachments:
  - kind: "evidence"
    id: "atelier-lm72"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T00:08:06.847192187+00:00"
status: "done"
title: "Strengthen canonical Markdown lint and direct-edit validation"
updated_at: "2026-06-12T00:08:06.847192187+00:00"
---

## Description

Make atelier lint the canonical validator for committed Markdown records and direct-edit workflows.
- atelier lint validates committed .atelier/ records directly from Markdown without depending on SQLite.
- atelier lint <id> validates a single record plus referenced relationship consistency.
- Hard failures cover ID/path mismatch, duplicate IDs, invalid schema/version, missing references, dependency cycles, invalid statuses/types/priorities, malformed activity sidecars, and unsupported committed files.
- atelier doctor remains runtime/install/cache health and does not replace canonical Markdown validation.
- Invalid Markdown test fixtures produce clear, actionable diagnostics.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
