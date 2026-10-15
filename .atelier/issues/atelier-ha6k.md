---
created_at: "2026-10-15T21:00:45.354511214+00:00"
id: "atelier-ha6k"
issue_type: "feature"
labels:
- "adoption"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Ship a someday state, and let new work be created in it"
updated_at: "2026-10-15T21:00:45.354511214+00:00"
---

## Goal

The default workflow has no state for work that is recorded but not near-term. P2/P3 items and children of draft epics crowd `work ready` (78 of 150 records in astromech). A custom `backlog` status (todo category, no direct start) works because classification treats it as backlog, but `issue create` has no `--status`, so new work can't be created there.

## Done when

`atelier init`'s default workflow includes a someday state with transitions in and out, `issue create` can create work in it, and `work ready` excludes it; tests cover all three.

## Context

astromech added `backlog` with `plan` (backlog→todo) and `defer` (todo→backlog) transitions to `.atelier/workflow.yaml`, and documented it in AGENTS.md.

Found 2026-10-15 while moving astromech's BACKLOG.md (about 135 items with research, references and decisions) into Atelier records. Sumeet's principle: Atelier records should hold the whole backlog; needing a side document means Atelier needs work.
