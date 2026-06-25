---
created_at: "2026-06-25T16:23:08.096908535+00:00"
id: "atelier-x2ad"
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
title: "Implement work mission dashboard"
updated_at: "2026-06-25T16:23:08.096908535+00:00"
---

## Description

Add `work mission <mission-id>` as the orchestrator dashboard for a live mission. It should show mission health, active/ready/blocked/backlog work grouped by epic, blockers, evidence/proof gaps, review/branch attention where available, close readiness, and ranked next actions using existing app/domain view data.

## Outcome

`work mission <mission-id>` gives an orchestrator one live mission control page: health, progress, active/ready/blocked/backlog work by epic, blockers, proof gaps, review/branch attention where available, close readiness, and next actions. It complements `issue show <mission-id>` record detail without replacing it.
