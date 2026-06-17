---
created_at: "2026-06-17T18:00:56.462015759+00:00"
id: "atelier-jhzk"
issue_type: "task"
labels:
- "config"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Update repo workflow policy to require linked_pr_merged for epic closeout"
updated_at: "2026-06-17T18:00:56.462015759+00:00"
---

## Description

Update this repository's workflow policy so epic closeout can require the new
linked merged PR validator.

## Outcome

- `.atelier/workflow.yaml` uses schema version 2.
- The epic workflow close transition includes `linked_pr_merged`.
- Existing non-epic workflows remain valid and documented.

## Evidence

- Manual check of `.atelier/workflow.yaml` file content shows the repository
  workflow policy declares `linked_pr_merged` for epic closeout.
- Command transcript shows `atelier lint` passes after the workflow policy
  update.
