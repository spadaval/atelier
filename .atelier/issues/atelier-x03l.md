---
created_at: "2026-06-16T15:46:48.954494041+00:00"
id: "atelier-x03l"
issue_type: "feature"
labels:
- "branch"
- "cli"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0nv2"
  - kind: "issue"
    id: "atelier-8jaf"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-16T16:51:14.162938659+00:00"
status: "done"
title: "Make start prepare the correct work branch automatically"
updated_at: "2026-06-16T16:51:14.162938659+00:00"
---

## Description

Make `atelier start <id>` the normal entrypoint that prepares the correct branch context automatically. Agents should not need to know or run branch helper commands before starting ordinary work.

## Outcome

- Starting a child issue under an epic creates or switches to the parent epic branch according to branch lifecycle policy, then transitions the issue into active workflow state.
- Starting a standalone issue creates or switches to the issue branch according to branch lifecycle policy, then transitions the issue into active workflow state.
- Starting an epic creates or switches to the epic branch according to branch lifecycle policy, then transitions the epic into active workflow state.
- Starting work from the configured base branch is allowed only through this lifecycle path; direct implementation on base is rejected or corrected with actionable output.
- Dirty worktree, missing parent epic, invalid branch owner, and branch checkout failures leave tracker state unchanged and print the next corrective command.
- Start output names the work item, branch owner, effective branch, base branch, and next proof/status commands without requiring manual branch setup.

## Evidence

- Failing-before/passing-after CLI tests or transcripts show child issue, standalone issue, and epic start all prepare the expected branch automatically.
- Failure-path tests prove dirty worktree and branch checkout failure do not transition the issue or epic to active state.
- Help transcript proves `atelier start` is the visible ordinary workflow entrypoint and does not require `atelier branch for-epic`.
- Focused tests, `cargo fmt -- --check`, `atelier lint atelier-x03l`, and `atelier export --check` pass.
