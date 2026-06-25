---
created_at: "2026-06-25T16:23:18.759236302+00:00"
id: "atelier-5ylh"
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
status: "todo"
title: "Remove issue list after work dashboard replacements land"
updated_at: "2026-06-25T16:23:18.759236302+00:00"
---

## Description

Stop teaching and dispatching `issue list` once `work queue` preserves its useful dashboard and filter behavior. Update help, man guidance, command audit, Agent Factory guidance, and rejection tests so multi-issue dashboards belong to `work`, while `issue` owns single-record detail and mutations.

## Outcome

`issue list` is removed from visible help, role guidance, docs, tests, and dispatch after `work queue` replacement behavior lands. Attempts to use `issue list` fail as an unsupported command, and guidance routes operators to `work queue` or `issue show <id>` depending on the job.
