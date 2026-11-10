---
created_at: "2026-11-10T00:56:10.707960610+00:00"
id: "atelier-8b0a"
issue_type: "task"
labels:
- "redesign"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-295u"
    type: "advances"
  - kind: "issue"
    id: "atelier-wd1i"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Compute the scorecard from the new records' status history and thread fields"
updated_at: "2026-11-10T01:14:42.577802653+00:00"
---

## Goal

Remove the thread-title matching the baseline needed, now that records say which thread held what.

## Done when

- [ ] Provisional: the mission criterion `from-records`. The event and transcript readers stay, and each number states its coverage.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-4-gardener.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- Which measures still need T3's events and the transcripts; expected: tool calls, tokens and interventions.
