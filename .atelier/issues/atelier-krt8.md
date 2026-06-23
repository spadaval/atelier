---
created_at: "2026-06-23T20:17:15.737614196+00:00"
id: "atelier-krt8"
issue_type: "feature"
labels:
- "cli"
- "ux"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ih42"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove ambient expected-branch UX from normal status"
updated_at: "2026-06-23T20:17:15.737614196+00:00"
---

## Description

Stop presenting computed expected branches and branch owners as ambient lifecycle state in normal status/show/transition guidance. Branch names should appear when a branch or review action will create, switch, push, merge, or fail on a concrete branch; normal status should report current Git state without implying the user is on the wrong branch.

## Outcome

Human output talks about current branch and planned branch actions, not branch owner resolution. Commands that do not touch Git do not fail or warn because a computed branch context cannot be resolved.
