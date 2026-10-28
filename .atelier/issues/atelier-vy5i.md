---
created_at: "2026-10-28T00:35:18.197859557+00:00"
id: "atelier-vy5i"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-vy5i
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-5tl2"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-vt8q"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Cut Atelier's first GitHub release"
updated_at: "2026-10-28T04:26:06.877807755+00:00"
---

## Goal

Choose the first release's version, bump the workspace version if needed, and push the `v<version>` tag so the release workflow publishes it. The human chooses the version and authorizes the tag push.

## Done when

Before the tag is pushed, the tagged commit is on `master` and its CI run passed, since the release workflow publishes whether or not CI passed. Afterwards, the GitHub Release for the tag exists with all four archives, checksums and the shell installer, and its release workflow run passed.

## Context

Strategy Authority: pushing a release tag comes back to the human. The tag must equal the workspace version (`dist plan --tag=v<x>` refuses a mismatch). The release-workflow CI-gate gap was found in review of atelier-7v9q (finding R1).

## Notes

- 2026-10-28 [note] 2026-10-28 [decision] Sumeet chose v0.3.0 for the first release and authorized pushing the tag: 0.2.0 has been the version since before the Markdown-record rebuild, so older source builds also report it.
