---
created_at: "2026-06-14T21:43:25.014147831+00:00"
id: "atelier-rxpr"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-11gp"
    type: "advances"
  - kind: "issue"
    id: "atelier-9sni"
    type: "advances"
  - kind: "issue"
    id: "atelier-fb61"
    type: "advances"
  - kind: "issue"
    id: "atelier-l8r9"
    type: "advances"
  - kind: "issue"
    id: "atelier-ooyj"
    type: "advances"
  - kind: "issue"
    id: "atelier-pd77"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "ready"
title: "Adopt mission worktrees and epic review branches"
updated_at: "2026-06-15T01:16:24.849052905+00:00"
---

## Intent

Atelier should use missions as background-agent workspaces, epics as branch/review boundaries, and issues as lightweight implementation slices. A mission owns one shared worktree. Each epic owns one reviewable branch that can map to a PR. Issues under the epic no longer require independent review by default; they close with local proof while the epic carries review and validation gates for the coherent changeset.

## Constraints

- Mission worktrees replace the current default of one issue worktree per mutating subagent.
- Epic branches become the normal review and PR-equivalent boundary.
- Issues remain accountable implementation slices with local proof, but review/validation gates move to epics unless an issue explicitly represents validation or review work.
- Do not add compatibility aliases or old-command shims unless a human explicitly asks for them.
- Agent Factory portable guidance must be updated alongside Atelier repo-local guidance.

## Risks

- Existing issue worktrees and codex/atelier-* branches contain dirty or unmerged local state that must be classified before cleanup.
- Moving review gates from issues to epics can weaken proof unless epic closeout maps child issue proof to epic outcomes.
- Mission-level worktree sharing can reintroduce workspace contention unless Agent Factory assignments name the mission worktree, epic branch, and parallelism rules clearly.

## Validation

- Product docs and domain language define Mission = worktree, Epic = branch/review boundary, Issue = implementation slice.
- Workflow policy and tests prove ordinary issues can close without review while epics require review/validation before done.
- CLI commands create or locate a mission worktree and create/switch/status/merge epic branches without creating issue worktrees by default.
- Agent Factory skill and repo binding instruct orchestrators to share mission worktrees, use epic branches for review, and avoid per-issue worktrees unless isolation is justified.
- Migration evidence classifies existing issue worktrees and branches, removes clean contained worktrees, and preserves or folds unmerged/dirty state into the new model.
