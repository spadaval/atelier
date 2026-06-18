---
created_at: "2026-06-18T16:20:40.354238188+00:00"
id: "atelier-jdvz"
issue_type: "epic"
labels:
- "pr"
- "session-pr-overhaul"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-lvgo"
  children:
  - kind: "issue"
    id: "atelier-cer4"
  - kind: "issue"
    id: "atelier-cln0"
  - kind: "issue"
    id: "atelier-ff55"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Update PR behavior for issue-event attribution"
updated_at: "2026-06-18T16:45:49.885129841+00:00"
---

## Description

Coordinate Forgejo PR behavior changes for the session-as-issue-events model. This epic delegates target inference/enforcement, PR action attribution, and merge behavior to focused children.

## Outcome

`pr open`, `pr comment`, `pr review`, and `pr merge` resolve issue context from linked PR or branch state first, reject ambiguous inference, enforce one active PR per issue or epic, and record local issue-event attribution activity for PR actions without making PR commands drive Atelier workflow transitions.

## Evidence

Child proof demonstrates one-active-PR enforcement, linked-PR-first inference, issue-event attribution activity, and `pr merge` behavior that does not bypass workflow transition policy.
