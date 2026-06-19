---
created_at: "2026-06-18T16:45:36.922967557+00:00"
id: "atelier-ff55"
issue_type: "feature"
labels:
- "pr"
- "session-pr-overhaul"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-cer4"
  - kind: "issue"
    id: "atelier-cln0"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T17:55:11.827708787+00:00"
status: "done"
title: "Resolve PR target context and one-active enforcement"
updated_at: "2026-06-18T17:55:11.827708787+00:00"
---

## Description

Update PR target resolution and one-active-PR enforcement independently from PR action attribution. PR commands should infer issue context from an explicitly linked PR or owner branch before falling back to broader active-work guesses, and should reject ambiguous cases with corrective output.

## Outcome

`pr open`, `pr status`, `pr show`, `pr comments`, `pr comment`, `pr review`, and `pr merge` resolve their issue or epic target from linked `forge_pr` state or branch ownership first, reject ambiguous inference, and enforce one active PR per issue or epic without consulting live session records.

## Evidence

PR command tests prove linked-PR-first inference, branch-owner inference, ambiguity rejection, one-active-PR enforcement, and no dependency on `.atelier/sessions` as target context.
