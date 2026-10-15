---
created_at: "2026-10-15T21:00:45.354511214+00:00"
id: "atelier-ha6k"
issue_type: "feature"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-ha6k
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-15T23:21:11.807082820+00:00"
status: "done"
title: "Ship a someday state, and let new work be created in it"
updated_at: "2026-10-15T23:21:11.807082820+00:00"
---

## Goal

The default workflow has no state for work that is recorded but not near-term. P2/P3 items and children of draft epics crowd `work ready` (78 of 150 records in astromech). A custom `backlog` status (todo category, no direct start) works because classification treats it as backlog, but `issue create` has no `--status`, so new work can't be created there.

## Done when

`atelier init`'s default workflow includes a someday state with transitions in and out, `issue create` can create work in it, and `work ready` excludes it; tests cover all three.

## Context

astromech added `backlog` with `plan` (backlog→todo) and `defer` (todo→backlog) transitions to `.atelier/workflow.yaml`, and documented it in AGENTS.md.

Found 2026-10-15 while moving astromech's BACKLOG.md (about 135 items with research, references and decisions) into Atelier records. Sumeet's principle: Atelier records should hold the whole backlog; needing a side document means Atelier needs work.

## Close Reason

The starter workflow (and this repository's) has a backlog status (todo category, no start) with defer (todo/blocked -> backlog) and plan (backlog -> todo); supersede accepts backlog. issue create --status accepts the workflow's initial status or a todo-category status one unchecked transition from it (backlog via defer); an objective's todo behind ready is refused. work ready excludes backlog. Covered by test_someday_work_is_created_in_backlog_and_kept_out_of_ready. Named backlog, not someday, to match astromech's adopted policy.
