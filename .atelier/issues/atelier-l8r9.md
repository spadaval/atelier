---
created_at: "2026-06-14T21:43:51.444365451+00:00"
id: "atelier-l8r9"
issue_type: "epic"
labels:
- "branch"
- "worktree"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-hrmj"
  - kind: "issue"
    id: "atelier-lfwg"
  - kind: "issue"
    id: "atelier-ooyj"
  children:
  - kind: "issue"
    id: "atelier-3q31"
  - kind: "issue"
    id: "atelier-ccja"
  - kind: "issue"
    id: "atelier-noly"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T04:31:22.849066074+00:00"
status: "done"
title: "Epic: Add mission worktree and epic branch commands"
updated_at: "2026-06-15T04:31:22.849066074+00:00"
---

## Description

Replace issue-worktree defaults with mission worktree and epic branch helpers. Outcome: operators can create or locate a mission worktree, create/switch/status/merge epic branches inside that workspace, and run issue work without creating issue worktrees by default. Evidence: focused CLI tests and transcripts prove mission worktree creation, epic branch lifecycle, status output, and cleanup behavior.

## Outcome

- Mission worktree commands create and reuse one workspace per mission.
- Epic branch commands create, switch, inspect, and merge PR-scale review branches inside the mission workspace.
- Normal issue work no longer creates an issue worktree by default.

## Evidence

- Child issue proof from atelier-3q31, atelier-noly, and atelier-ccja maps to mission worktree, epic branch, and issue-worktree removal behavior.
- CLI transcript or tests prove the full mission-worktree plus epic-branch flow.
- Residue search transcript proves public workflow no longer teaches issue worktrees as the default.
