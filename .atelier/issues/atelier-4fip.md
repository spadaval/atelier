---
created_at: "2026-07-06T17:20:25.691397620+00:00"
id: "atelier-4fip"
issue_type: "epic"
labels:
- "cli"
- "human-output"
- "mission-dashboard"
review:
  kind: pull_request
  number: 56
  provider: forgejo
fields:
  workflow_branch:
    branch_base: mission/atelier-c0mp
    integration_target: mission/atelier-c0mp
    merge_strategy: squash
    owner_issue_id: atelier-4fip
    owner_kind: epic
    review_target: mission/atelier-c0mp
    work_branch: epic/atelier-4fip
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-g5fl"
  children:
  - kind: "issue"
    id: "atelier-dy3u"
  - kind: "issue"
    id: "atelier-sjsz"
  - kind: "issue"
    id: "atelier-tdgs"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Epic: Build the formatted Mission Overview"
updated_at: "2026-07-07T06:22:01.837264732+00:00"
---

## Description

Evolve `atelier work missions` from a thin mission-type inventory alias into the high-level operational overview for all current missions. This epic owns the mission/epic projection, default filtering, collapsed summaries, formatting, semantic color, help, and focused coverage.

## Outcome

- `atelier work missions` answers which missions and epics constitute the current backlog without printing leaf subtasks by default.
- The default omits done missions, an explicit option includes them, and every displayed count and state is derived from the same mission `advances` and epic-child facts used by scoped dashboards.
- The Mission Overview is visually structured and consistently colored through shared output helpers, with complete colorless behavior.

## Evidence

- Child evidence must include focused projection tests for directed `advances` membership, descendant collapse, shared scope, exceptional work, ordering, limits, and cycle safety.
- Command transcript evidence must cover default and `--all` Mission Overview output, direct and unassigned work accounting, drill-down guidance, narrow output, interactive color, noninteractive output, and `NO_COLOR` behavior.
- `cargo nextest run -p atelier-app -p atelier-cli`, `cargo fmt -- --check`, `git diff --check`, and `atelier check atelier-4fip` must pass on the epic branch.
- An independent epic review and independent scenario validation must be attached before this epic is integrated into `mission/atelier-c0mp`.
