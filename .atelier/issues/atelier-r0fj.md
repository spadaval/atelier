---
created_at: "2026-11-04T23:12:48.493454620+00:00"
id: "atelier-r0fj"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-0a52"
  - kind: "issue"
    id: "atelier-b3qt"
  - kind: "issue"
    id: "atelier-jnec"
  - kind: "issue"
    id: "atelier-mg7w"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Make the debrief and brief report history and next steps accurately"
updated_at: "2026-11-04T23:12:49.810169493+00:00"
---

## Goal

Fix the mission-review follow-ups from atelier-r6bt's reviews and validation: a debrief from a work branch that merged the base (atelier-jnec), one clock for the timeline (atelier-b3qt), the brief's approval footer (atelier-mg7w) and which outcome the brief shows (atelier-0a52). One branch and one review; jnec before b3qt because both depend on the first-parent walk in record_history.rs.

## Done when

Every child is terminal with its own Done when met and proven, and one independent review of the branch is approved here.
