---
created_at: "2026-06-13T02:35:59.064620959+00:00"
id: "atelier-xmss"
issue_type: "feature"
labels:
- "closeout"
- "evidence"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-rzsg"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Require claim-mapped evidence for parent closeout"
updated_at: "2026-06-13T02:35:59.064620959+00:00"
---

## Description

Update closeout policy so parent issues and missions require suitable proof coverage through linked accountable work, not merely any attached evidence.

## Outcome

- Parent closeout checks can identify which Outcome or Validation areas lack suitable proof coverage.
- Broad supporting checks cannot satisfy claim-specific proof requirements by themselves.
- Failure output names the missing coverage area and the evidence kind or validation work needed.

## Evidence

- Focused tests include a seeded weak-proof case that fails and a suitable coverage evidence case that passes.
- Transcript demonstrates the failure message is actionable.
