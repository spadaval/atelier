---
created_at: "2026-06-25T16:23:13.436593102+00:00"
id: "atelier-lesf"
issue_type: "feature"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Implement work epic dashboard"
updated_at: "2026-06-25T16:23:13.436593102+00:00"
---

## Description

Add `work epic <epic-id>` as the focused dashboard for an epic branch/review boundary. It should show active/ready/blocked/backlog child work, proof gaps, review and branch state where available, transition readiness, and next actions without replacing `issue show <epic-id>` record detail.

## Outcome

`work epic <epic-id>` gives a focused execution dashboard for an epic: child work by state, blockers, proof gaps, review and branch state where available, transition readiness, and next actions. It keeps durable record text and relationship detail in `issue show <epic-id>`.
