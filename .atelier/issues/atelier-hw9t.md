---
created_at: "2026-06-17T17:59:05.780483850+00:00"
id: "atelier-hw9t"
issue_type: "epic"
labels:
- "pr"
- "validator"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-4clo"
  - kind: "issue"
    id: "atelier-98mo"
  - kind: "issue"
    id: "atelier-cglp"
  - kind: "issue"
    id: "atelier-p7oa"
  children:
  - kind: "issue"
    id: "atelier-495r"
  - kind: "issue"
    id: "atelier-jhzk"
  - kind: "issue"
    id: "atelier-onie"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Gate workflow with linked merged PR"
updated_at: "2026-06-17T18:00:58.980383039+00:00"
---

## Description

Gate workflow closeout with linked merged Forgejo PR state while keeping PR
commands operational rather than lifecycle-driving.

## Outcome

- `linked_pr_merged` is available as a workflow validator.
- This repository can require `linked_pr_merged` for epic closeout.
- Validator failure output points operators to `atelier pr open` or
  `atelier pr status` as appropriate.

## Evidence

- Focused workflow tests cover linked merged PR pass/fail states and actionable
  failure output.
- Command transcript shows targeted validator tests plus `atelier lint` pass.
