---
created_at: "2026-11-06T23:39:07.679326401+00:00"
id: "atelier-bp3s"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-s66u"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Add a suite merge check to demiurge and cosplay-studio"
updated_at: "2026-11-06T23:39:07.679326401+00:00"
---

## Goal

Give demiurge and cosplay-studio a test of the merged tree. They have no CI and no `merge_checks`; until atelier-s66u ships, the only such test is `merge --sync` rerunning recorded suite receipts, which atelier-s66u removes.

## Done when

Each repository's `.atelier/workflow.yaml` has a `merge_checks` `shell.check` that runs its test suite on the merged tree (with `merge_strategy: merge_commit`), and one `atelier merge` there shows the check running and a failing suite refusing the merge.

## Context

Pick this up when each repository next adopts an Atelier release containing atelier-s66u (mission atelier-n4mj). It is outside the mission because it is work in other repositories. Strategy: ~/projects/atelier/docs/strategies/lighter-workflow.md (Scope, Risks).
