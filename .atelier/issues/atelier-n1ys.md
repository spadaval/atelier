---
created_at: "2026-06-12T04:51:52.623285724+00:00"
id: "atelier-n1ys"
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
status: "open"
title: "Validate sectioned issue workflow end to end"
updated_at: "2026-06-12T04:51:52.623285724+00:00"
---

## Description

Validate the sectioned issue workflow end to end after the contract,
parser/rendering, enforcement, and migration tasks land.

## Outcome

- A newly created issue can be edited into the required sectioned format and
  shown cleanly by `atelier issue show`.
- A malformed issue fails lint with actionable diagnostics.
- Agents cannot start work on malformed issues.
- Mission or issue closeout cannot pass while linked implementation work lacks
  required issue structure.
- Documentation, tests, and actual command behavior agree on section names and
  required sections.

## Evidence

- Capture command transcripts for successful sectioned issue show, failing lint,
  failing start, and closeout blocked by malformed linked work.
- Attach durable `atelier evidence` records for the final validation run.
- Run `cargo fmt -- --check`.
- Run focused CLI integration tests for section parsing, lint, start, and
  closeout.
- Run `atelier export --check`, `atelier lint`, and `atelier doctor`.

## Notes

This validation child should stay blocked until the implementation and migration
children are complete.
