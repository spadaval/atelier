---
created_at: "2026-06-16T15:46:43.440770625+00:00"
id: "atelier-89by"
issue_type: "feature"
labels:
- "branch"
- "config"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0nv2"
  - kind: "issue"
    id: "atelier-bkw7"
  - kind: "issue"
    id: "atelier-mnwf"
  - kind: "issue"
    id: "atelier-x03l"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-16T16:39:57.219253564+00:00"
status: "done"
title: "Add branch lifecycle policy and merge strategy config"
updated_at: "2026-06-16T16:39:57.219253564+00:00"
---

## Description

Introduce a single branch lifecycle policy used by start, close, status, and validators. The policy should be derived from tracker structure and repository configuration instead of duplicated across CLI command handlers.

## Outcome

- Application code exposes one branch lifecycle policy that resolves branch owner, expected branch name, base branch, merge strategy, and whether the current item is merge-owned or nested under an epic.
- Default policy uses squash merge for branch integration.
- Repository configuration can change merge strategy, base branch, and branch naming templates without changing command behavior code.
- The policy treats child issues under an epic as nested work whose close is committed on the epic branch but not merged to base.
- The policy treats standalone issues and epics as branch owners whose close integrates their branch to base.
- Existing mission worktree and exceptional issue-worktree behavior either uses this policy or is clearly classified as repair/advanced behavior.

## Evidence

- Focused unit or integration tests cover policy resolution for child issue under epic, standalone issue, epic, invalid graph, missing base branch, and configured non-default merge strategy.
- CLI or app-level tests prove the configured squash-merge default is used when no override is present.
- Config fixture test proves alternate merge strategy and base branch are accepted and surfaced.
- `cargo fmt -- --check`, focused cargo tests, `atelier lint atelier-89by`, and `atelier export --check` pass.
