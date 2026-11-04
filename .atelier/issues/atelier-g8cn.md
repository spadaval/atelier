---
created_at: "2026-11-04T21:56:17.001274292+00:00"
id: "atelier-g8cn"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-g8cn
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-vt8q"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T22:08:19.372344919+00:00"
status: "done"
title: "Release v0.3.1 with static musl Linux builds"
updated_at: "2026-11-04T23:14:55.522547557+00:00"
---

## Goal

Publish v0.3.1 so the shell installer works on Linux hosts with glibc older than 2.35 (atelier-vo0z).

## Done when

- Workspace version is 0.3.1, Cargo.lock matches, and CHANGELOG has a 0.3.1 entry.
- A v0.3.1 tag on a green master commit publishes a GitHub Release with gnu and musl Linux archives, macOS archives, checksums and the shell installer.

## Notes

- 2026-11-04 [note] Tagged v0.3.1 on d14d2e5e (master CI green) on 2026-11-04; the tag had not been pushed after merge. Release run 41931151380 passed and published https://github.com/spadaval/atelier/releases/tag/v0.3.1 with gnu and musl Linux (x86_64, aarch64), both macOS archives, checksums and the shell installer.

## Close Reason

Bumped the workspace to 0.3.1 (Cargo.toml, Cargo.lock for all five crates) and added a CHANGELOG 0.3.1 entry for the static musl Linux builds from atelier-vo0z. `dist plan --tag v0.3.1` announces 0.3.1 with gnu and musl Linux archives, both macOS archives, checksums and the shell installer. PR #16 CI green on Linux and macOS, including the dist PR upload run for all six targets; nextest 947/947, test-hooks 953/953 (reviewer atelier-div0).

### Deviations
None.

### Decisions
Left the Agent Factory unvendoring (8b058ae2) out of the changelog; it is internal to the repository.

### Surprises
None.

### Open items
Tagging v0.3.1 happens after this merges and master's CI passes; the publish result is recorded as a note on this item.
