---
created_at: "2026-06-14T21:43:51.435101860+00:00"
id: "atelier-11gp"
issue_type: "epic"
labels:
- "review"
- "workflow"
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-3vzm"
  - kind: "issue"
    id: "atelier-l543"
  - kind: "issue"
    id: "atelier-o2z8"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Move workflow review gates from issues to epics"
updated_at: "2026-06-14T21:44:54.240353912+00:00"
---

## Description

Change Atelier workflow policy so ordinary implementation issues close with local proof, while epics carry review and validation gates for coherent changesets. Outcome: issue workflows no longer require review by default; epic workflows require review/validation and child-proof mapping before done. Evidence: workflow docs, policy fixtures, and focused CLI tests prove issue close and epic close behavior.

## Outcome

- Ordinary implementation issues close with local proof instead of mandatory review and validation states.
- Epics carry review and validation gates for coherent changesets.
- Workflow help, policy, and tests consistently explain issue proof versus epic review.

## Evidence

- Child issue proof from atelier-l543, atelier-o2z8, and atelier-3vzm maps to workflow policy, validators, help, and tests.
- Focused test transcript proves issue close and epic close behavior differ as intended.
- Epic review evidence confirms the workflow model matches the product contract.
