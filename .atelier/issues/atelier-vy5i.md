---
created_at: "2026-10-28T00:35:18.197859557+00:00"
id: "atelier-vy5i"
issue_type: "task"
labels:
- "needs-human"
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
status: "todo"
title: "Cut Atelier's first GitHub release"
updated_at: "2026-10-28T00:35:18.197859557+00:00"
---

## Goal

Choose the first release's version, bump the workspace version if needed, and push the `v<version>` tag so the release workflow publishes it. The human chooses the version and authorizes the tag push.

## Done when

The GitHub Release for the tag exists with all four archives, checksums and the shell installer, and its release workflow run passed.

## Context

Strategy Authority: pushing a release tag comes back to the human.
