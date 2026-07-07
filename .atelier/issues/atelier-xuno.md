---
created_at: "2026-07-07T05:25:40.335690417+00:00"
id: "atelier-xuno"
issue_type: "task"
labels:
- "lifecycle"
review:
  kind: pull_request
  number: 55
  provider: forgejo
fields:
  workflow_branch:
    branch_base: master
    integration_target: master
    merge_strategy: squash
    owner_issue_id: atelier-xuno
    owner_kind: issue
    review_target: master
    work_branch: task/atelier-xuno
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Reconcile post-publication mission attribution"
updated_at: "2026-07-07T05:27:22.487130869+00:00"
---

## Description

Integrate the post-publication activity sidecars from terminal atelier-mska and atelier-durs branches so their pushed branch tips become ancestors of Forgejo master and can later be cleaned safely. Evidence: atelier check, git diff --check, and ancestry checks for both mission branches.

## Outcome

The post-publication activity sidecars from terminal missions `atelier-mska`
and `atelier-durs` are integrated into Forgejo `master`, both pushed mission
branch tips are ancestors of `master`, and subsequent branch cleanup can
preserve their durable lifecycle history.

## Evidence

- Command transcript: `atelier check` passes on the reconciliation branch.
- Command transcript: `git diff --check origin/master...HEAD` reports no whitespace errors.
- Command transcript: `git merge-base --is-ancestor` succeeds for both authoritative mission
  branch tips against the reconciliation head.
- Command transcript: `git diff --name-status origin/master...HEAD` shows only this reconciliation
  record and the four post-publication activity sidecars.
