---
created_at: "2026-10-17T17:51:42.072219114+00:00"
id: "atelier-s5x4"
issue_type: "bug"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Count done decision records as decided when the repository has no workflow policy"
updated_at: "2026-10-17T17:51:42.072219114+00:00"
---

## Goal

In a repository without .atelier/workflow.yaml, `decided()` in crates/atelier-app/src/decisions.rs returns false for every decision record, because it reads the status category from the policy. Done records then show under Open Decision Records, `work decisions -q` leaves them out, and a decided newer record never supersedes an older one. Found by review atelier-mws5 on atelier-fip8.

## Done when

With no workflow policy, a decision record with status `done` counts as decided (falling back to the status name), shown by a CLI test that removes workflow.yaml.
