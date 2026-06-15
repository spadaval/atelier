---
created_at: "2026-06-15T05:13:38.313691561+00:00"
id: "atelier-rxgn"
issue_type: "validation"
labels:
- "markdown"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T08:02:57.482106263+00:00"
status: "done"
title: "Validate Markdown record round trips after records extraction"
updated_at: "2026-06-15T08:02:57.482106263+00:00"
---

## Description

Independently validate canonical Markdown round trips after `atelier-records` extraction.

## Outcome

- Issues, missions, plans, evidence, relationships, and activity sidecars parse and render deterministically.
- Direct Markdown edits continue to refresh projection-backed reads correctly.
- No generic escaped payload or duplicate relationship surface is introduced by the extraction.

## Evidence

- Round-trip test transcript or evidence record covers all supported record kinds and sidecars.
- `atelier export --check` passes after the extraction.
- Scenario transcript proves direct Markdown edits are reflected in list, show, search, or graph commands after refresh.
