---
created_at: "2026-06-17T18:01:03.841084494+00:00"
id: "atelier-4clo"
issue_type: "validation"
labels:
- "scenario"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Record scenario evidence for issue-event attempts, sudo PR authorship, and merged PR gating"
updated_at: "2026-06-18T16:49:17.642125611+00:00"
---

## Description

Record scenario validation evidence for the complete role-aware PR review flow
under the session-as-issue-events model.

## Outcome

- Scenario proves worker/reviewer/validator attempts are derived from issue
  events and create local attribution.
- Scenario proves a reviewer attempt can post or review through a configured
  Forgejo sudo role.
- Scenario proves unresolved comments are visible and merged PR state gates
  closeout.

## Evidence

- Evidence record contains the scenario transcript or mocked scenario output for
  issue-event attempt attribution, reviewer sudo authorship, unresolved comments,
  PR merge behavior, and merged PR validation.
- Independent validation note or review artifact maps the scenario to mission
  validation criteria.
