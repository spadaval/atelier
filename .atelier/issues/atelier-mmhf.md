---
created_at: "2026-06-21T17:40:37.513320749+00:00"
id: "atelier-mmhf"
issue_type: "feature"
labels:
- "evidence"
- "status"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-lkz6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Replace hardcoded evidence proof gaps with validator readiness"
updated_at: "2026-06-21T19:00:13.563293979+00:00"
---

## Description

Remove bespoke proof-gap calculations from status, issue status, issue show, and transition readiness surfaces where they claim evidence is missing independently of configured workflow validators. Those surfaces should report evidence requirements only when the active workflow transition or readiness check includes a configured validator that fails.

## Outcome

- status no longer reports evidence/proof gaps as a universal hardcoded requirement.
- issue status and issue show surface failed configured validators with concise reason and help hint.
- transition options and blocked transition output use the same configured validator result path.
- Existing non-evidence readiness checks remain visible where they are actual workflow or tracker-safety requirements.

## Evidence

- Focused CLI transcripts show no evidence warning for an ordinary issue whose workflow does not configure evidence.attached.
- Focused CLI transcripts show evidence help when evidence.attached is configured and fails.
- Search or test proof shows old bespoke evidence/proof-gap status paths are removed or no longer active.
- atelier lint and git diff --check pass.
