---
created_at: "2026-06-16T15:46:29.285889651+00:00"
id: "atelier-5sjm"
issue_type: "epic"
labels:
- "branch"
- "git"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-0nv2"
  - kind: "issue"
    id: "atelier-168m"
  - kind: "issue"
    id: "atelier-89by"
  - kind: "issue"
    id: "atelier-8jaf"
  - kind: "issue"
    id: "atelier-bkw7"
  - kind: "issue"
    id: "atelier-mnwf"
  - kind: "issue"
    id: "atelier-x03l"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Make branch lifecycle automatic and squash-merge by default"
updated_at: "2026-06-16T15:47:26.736815316+00:00"
---

## Description

Make branch handling an implementation detail of the work lifecycle. Operators and agents should start and close work; Atelier should derive the correct branch from the issue graph, enforce that work happens there, and merge completed branch-owned work back to the configured base branch.

## Outcome

- `atelier start <id>` derives the branch owner from the tracker graph: the nearest parent epic owns ordinary child issue work, while standalone issues own their own issue branch.
- Normal work happens on the derived owner branch; agents do not need to run `atelier branch for-epic` as a routine workflow step.
- Closing a child issue under an epic commits the tracker close/evidence state onto the parent epic branch and does not merge the epic branch to the base branch.
- Closing a standalone issue or epic commits the terminal tracker state and merges the owner branch back to the configured base branch as one operator action.
- Branch merges use squash merge by default, with repository configuration for alternate merge strategy, base branch, and branch naming policy.
- If close-time commit or merge fails, the issue or epic is not left closed without the corresponding branch integration.
- Status, transition, docs, and Agent Factory guidance surface branch lifecycle state in product terms rather than making branch commands the normal user path.

## Evidence

- Product contract file diff documents branch owner derivation, child issue close behavior, standalone issue and epic merge behavior, squash-merge default, and configuration escape hatches.
- CLI transcripts or integration tests prove `atelier start` creates or switches to the derived branch for a child issue, a standalone issue, and an epic.
- CLI transcripts or integration tests prove child issue close commits tracker state on the epic branch without merging to base.
- CLI transcripts or integration tests prove standalone issue and epic close commit terminal tracker state and squash-merge the owner branch to the configured base branch.
- Failure-path transcript proves a merge or commit failure leaves the target issue or epic open and prints recovery guidance.
- `rg` over docs, help, tests, `AGENTFACTORY.md`, and `/root/.agents/skills/agent-factory` shows `atelier branch for-epic` is no longer taught as the normal workflow path.
- `atelier lint`, `atelier export --check`, `cargo fmt -- --check`, and relevant focused cargo tests pass.

## Notes

The core invariant is branch-owned work, not branch commands. A child issue under an epic closes locally on the epic branch because the epic branch may still contain unfinished sibling work. The merge to base belongs to the epic close. Standalone issues have no parent epic, so their own issue branch is the merge boundary.
