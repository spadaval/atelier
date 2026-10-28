---
created_at: "2026-10-28T04:31:55.316654389+00:00"
id: "atelier-gddg"
issue_type: "task"
labels: []
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
status: "todo"
title: "Push v0.3.0 on a green master commit and confirm the release published"
updated_at: "2026-10-28T04:31:55.316654389+00:00"
---

## Goal

Publish Atelier's first GitHub Release by pushing the `v0.3.0` tag that Sumeet authorized on 2026-10-28.

## Done when

Before the tag is pushed, the tagged commit is `master`'s merge of atelier-vy5i (or a later `master` commit) and its CI workflow run passed. After the push, the tag's Release workflow run passed, and the GitHub Release `v0.3.0` lists the four archives, their `.sha256` files, `sha256.sum`, `source.tar.gz` and `atelier-cli-installer.sh`, and is not a prerelease.

## Context

Strategy: `docs/strategies/release-distribution.md` (Authority: the tag push is the human's, now given). The release workflow publishes whether or not CI passed, so check CI first. Split out of atelier-vy5i on review.
