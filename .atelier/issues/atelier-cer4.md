---
created_at: "2026-06-18T16:45:49.884319364+00:00"
id: "atelier-cer4"
issue_type: "feature"
labels:
- "pr"
- "session-pr-overhaul"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T18:09:41.434957041+00:00"
status: "done"
title: "Add PR merge command behavior without workflow side effects"
updated_at: "2026-06-18T18:09:41.434957041+00:00"
---

## Description

Add `pr merge` as a remote PR action that remains separate from Atelier workflow transitions. It may update the issue-linked PR artifact and record issue-event attribution, but it must not close, start, validate, or otherwise transition Atelier issues as a side effect.

## Outcome

`atelier pr merge` merges or confirms merge state for the linked Forgejo PR, updates local `forge_pr` state when appropriate, records issue-event attribution, and points operators back to `issue transition --options` for workflow closeout. It rejects missing, ambiguous, unmergeable, or mismatched PR context with corrective guidance.

## Evidence

Mocked PR command tests prove successful merge, already-merged handling, missing or ambiguous target rejection, local field update or confirmation, issue-event attribution, and absence of Atelier workflow status changes.
