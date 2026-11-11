---
created_at: "2026-11-07T03:36:54.344195431+00:00"
id: "atelier-8ghm"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "evidence"
    id: "atelier-htux"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:49.343023005+00:00"
status: "declined"
title: "Make the stale-pass land test assert that land passes"
updated_at: "2026-11-11T03:55:49.343023005+00:00"
---

## Goal

`a_retained_claim_whose_pass_later_commits_staled_keeps_its_failure_cleared` (crates/atelier-cli/tests/session.rs) lost its assertion that `land` passes when atelier-s66u changed the failure-clearing rule; it now only checks that a message is absent. Restore a positive assertion of the behavior it names.

## Done when

The test asserts the outcome land should have once a later pass cleared the failure (pass, or the exact unrelated refusal its fixture causes, such as the uncommitted file), and it fails if the failure is not cleared.

## Context

Noted by the atelier-n4mj validation (receipt atelier-htux). Low priority; test quality only.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It restores an assertion in a `land` test, and the What is removed table removes `land`.
