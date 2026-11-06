---
created_at: "2026-11-06T21:26:18.549920057+00:00"
id: "atelier-8s1m"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-hwxm"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "A fix re-check counts as the review of that fix"
updated_at: "2026-11-06T21:26:18.549920057+00:00"
---

## Goal

A reviewer confirming a fix_now disposition at its resolution commit has reviewed that fix. A fresh approval of the whole head is needed only for commits no finding resolution covers, or for a finding the reviewer marks as needing re-review.

## Done when

Provisional: in the lifecycle scenario a fixed finding needs no second approval of the head; a commit beyond the resolutions still makes approval stale; a finding marked for re-review still requires a fresh approval; approval-freshness docs state the rule in fewer words than today; Agent Factory review text in ~/projects/agents-skills says the re-check satisfies its "changes after review" rule.

## Unknowns

Whether scoping review to the fix lets problems through; judged from findings during the mission.
