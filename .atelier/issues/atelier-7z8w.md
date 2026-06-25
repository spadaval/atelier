---
created_at: "2026-06-25T16:22:57.069881874+00:00"
id: "atelier-7z8w"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "done"
title: "Define work dashboard command contract"
updated_at: "2026-06-25T16:22:57.069881874+00:00"
---

## Description

Document the `work` dashboard model in command-audit terms. `work` itself lists available dashboards. `work queue` is the repo-wide operational queue. `work mission <id>` answers what an orchestrator should do next inside one mission. `work epic <id>` answers execution/review-boundary state for one epic. Ready, active, blocked, and backlog are filters over queue-style views, not separate dashboard concepts.

## Outcome

Command-audit docs describe `work` as the dashboard namespace and `issue` as single-record detail and mutation. The documented personas, primary questions, and replacement paths make clear why `work queue`, `work mission <id>`, and `work epic <id>` exist, how filters apply, and why `issue list` is removed.
