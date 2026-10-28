---
created_at: "2026-10-28T04:31:55.316654389+00:00"
id: "atelier-gddg"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-gddg
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-vt8q"
    type: "advances"
  - kind: "issue"
    id: "atelier-vy5i"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-28T16:19:34.968214481+00:00"
status: "done"
title: "Push v0.3.0 on a green master commit and confirm the release published"
updated_at: "2026-10-28T16:19:34.968214481+00:00"
---

## Goal

Publish Atelier's first GitHub Release by pushing the `v0.3.0` tag that Sumeet authorized on 2026-10-28.

## Done when

Before the tag is pushed, the tagged commit is `master`'s merge of atelier-vy5i (or a later `master` commit) and its CI workflow run passed. After the push, the tag's Release workflow run passed, and the GitHub Release `v0.3.0` lists the four archives, their `.sha256` files, `sha256.sum`, `source.tar.gz` and `atelier-cli-installer.sh`, and is not a prerelease.

## Context

Strategy: `docs/strategies/release-distribution.md` (Authority: the tag push is the human's, now given). The release workflow publishes whether or not CI passed, so check CI first. Split out of atelier-vy5i on review.

## Close Reason

v0.3.0 is published: the tag on master 02ffeac8 went out after that commit's CI passed, and the release workflow published a stable GitHub Release with all four archives, checksums and the shell installer. The installer one-liner works in a clean HOME (evidence on this item).

### Deviations
None.

### Decisions
Tagged the merge commit of atelier-vy5i, which was master's head when its CI passed.

### Surprises
The first clean-HOME installer attempt hit a transient curl connection timeout; the retry succeeded with no change.

### Open items
- atelier-g6a1 validates S1–S4 independently, including an upgrade over an older install.
