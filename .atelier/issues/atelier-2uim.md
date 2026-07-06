---
created_at: "2026-06-29T17:40:30.194241997+00:00"
id: "atelier-2uim"
issue_type: "validation"
labels:
- "agent-factory"
- "validation"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1mga"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Validate simplified mission planning and lifecycle model"
updated_at: "2026-06-29T17:40:30.194241997+00:00"
---

## Description

An independent validator verifies the simplified process after implementation. The validator starts from the mission Outcome and linked work, then chooses proof for each claim instead of following planner-authored validation paperwork.

## Outcome

The validation result is a claim map over the mission and linked epic outcomes. For each claim, the validator records pass, fail, blocked, or deferred; cites the issue IDs being validated; and includes the command transcript, artifact, evidence record, or file reference used to judge it. The claim map covers planning guidance, lifecycle workflow configuration, closeout behavior, reliability gates, and validator-selected proof from the mission Outcome.
