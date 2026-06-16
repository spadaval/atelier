---
created_at: "2026-06-16T15:47:00.847232247+00:00"
id: "atelier-bkw7"
issue_type: "feature"
labels:
- "branch"
- "cli"
- "status"
priority: "P2"
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
status: "in_progress"
title: "Surface branch lifecycle state in status and transition output"
updated_at: "2026-06-16T17:05:41.493036989+00:00"
---

## Description

Surface branch lifecycle information where agents already look for next actions. Branch state should be visible enough to prevent wrong-branch work without making explicit branch commands the main path.

## Outcome

- `atelier status` shows the current branch, configured base branch, branch owner when known, and mismatches between current branch and active issue or epic work.
- `atelier issue show <id>` shows expected branch owner, expected branch, base branch, and whether the item is nested under an epic or owns its own merge branch.
- `atelier issue transition <id> --options` reports branch-context blockers before start or close transitions fail.
- `atelier mission status <id>` summarizes active epic branches, dirty branch state, unmerged owner branches, and active child issues running outside the expected branch.
- Output gives corrective lifecycle commands such as `atelier start <id>` or `atelier issue close <id> --reason ...`, not `atelier branch for-epic` as the normal fix.

## Evidence

- CLI transcript or tests cover status output on base branch, expected issue branch, expected epic branch, and wrong branch.
- CLI transcript or tests cover `issue show` and `issue transition --options` for child issue, standalone issue, and epic branch ownership.
- CLI transcript or tests cover mission status with at least one active epic branch and one branch mismatch.
- Help or transcript output proves corrective next actions use lifecycle commands rather than branch setup commands.
- Focused tests, `cargo fmt -- --check`, `atelier lint atelier-bkw7`, and `atelier export --check` pass.
