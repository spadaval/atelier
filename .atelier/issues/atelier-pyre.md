---
created_at: "2026-06-12T20:29:57.173649956+00:00"
id: "atelier-pyre"
issue_type: "validation"
labels:
- "closeout"
- "evidence"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-nzy1"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T23:53:41.853912369+00:00"
status: "done"
title: "Validate closeout proof gates through domain commands"
updated_at: "2026-06-12T23:53:41.853912369+00:00"
---

## Description

Validate closeout proof gates through the domain commands that operators and
agents are expected to use.
- Mission closeout fails when linked work is open, malformed, missing required
  proof, or contradicted by parent outcomes.
- Issue closeout fails when required proof is missing or unrelated.
- Attached evidence can satisfy declared proof requirements when it validates
  the correct target.
- Domain status and closeout commands report the same blocker classes and
  recovery actions.
- Raw workflow validation is not required as the normal operator path.
- Positive and negative transcripts for issue closeout and mission closeout.
- Tests proving missing evidence, unrelated evidence, malformed sections, open
  work, and dirty/stale state block closeout with actionable diagnostics.
- Evidence record attached to this validation issue with the scenario matrix.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
