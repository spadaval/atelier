---
created_at: "2026-06-12T04:58:59.488119009+00:00"
id: "atelier-nzy1"
issue_type: "epic"
labels:
- "evidence"
- "rework"
- "validators"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-a4sn"
  - kind: "issue"
    id: "atelier-pvuz"
  - kind: "issue"
    id: "atelier-pyre"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Harden mission closeout validators and evidence requirements"
updated_at: "2026-06-12T23:55:13.375529836+00:00"
---

## Description

Make mission and issue closeout validation prove real product outcomes instead
of only checking tracker freshness. The previous mission closed even though many
claimed command behaviors were absent.

## Outcome

- Mission closeout validators can require linked work to have valid sections,
  named evidence records, expected command transcripts, and clean tracker state.
- Mission status and closeout commands check mission-specific evidence
  requirements instead of relying on a raw workflow-validator command.
- Closeout fails when linked implementation issues are still open, malformed,
  missing Outcome/Evidence sections, or missing required evidence records.
- Evidence records distinguish broad validation runs from local notes and are
  attached to the records they validate.
- Mission status reports closeout blockers in user-facing language and does not
  require operators to understand a workflow-validator subsystem.
- Any remaining raw workflow validation command is hidden or scoped as an
  advanced/internal diagnostic; domain commands own normal closeout guidance.

## Evidence

- Tests prove a mission with missing command behavior/evidence cannot close.
- Tests prove section lint failures in linked work block mission closeout.
- Tests prove attached evidence can satisfy declared closeout requirements.
- Transcript coverage shows mission status and closeout commands agree on
  closeout blockers without recommending raw workflow validation.
- Run focused validator tests, `atelier export --check`, `atelier lint`, and
  `atelier doctor`.

## Notes

This epic should use the parsed issue-section integration work from
`atelier-40ou` instead of inventing separate closeout parsing.
