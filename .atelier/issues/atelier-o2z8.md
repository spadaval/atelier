---
created_at: "2026-06-14T21:44:52.241320329+00:00"
id: "atelier-o2z8"
issue_type: "task"
labels:
- "validation"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3vzm"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T22:32:19.766108752+00:00"
status: "done"
title: "Move closeout validators to epic-scale review"
updated_at: "2026-06-14T22:32:19.766108752+00:00"
---

## Description

Adjust validators so epic closeout checks child issue completion, mapped proof, clean branch/worktree state, review completion, and validation completion. Outcome: issue close remains lightweight while epic close protects coherent changesets. Evidence: tests or transcripts show issue close succeeds with local proof, epic close fails without child mapping/review/validation, and epic close passes when requirements are satisfied.

## Outcome

- Epic closeout checks child issue completion, mapped proof, clean workspace state, review completion, and validation completion.
- Ordinary issue close remains scoped to local proof and does not claim parent or epic completion.
- Validator failure messages identify the missing epic-scale review or proof mapping.

## Evidence

- Focused tests or transcripts show issue close succeeds with local proof.
- Focused tests or transcripts show epic close fails without child-proof mapping, review, or validation.
- Focused tests or transcripts show epic close passes after required review, validation, and proof mapping exist.
