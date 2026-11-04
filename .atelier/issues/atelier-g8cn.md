---
created_at: "2026-11-04T21:56:17.001274292+00:00"
id: "atelier-g8cn"
issue_type: "task"
labels: []
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
status: "todo"
title: "Release v0.3.1 with static musl Linux builds"
updated_at: "2026-11-04T21:56:17.001274292+00:00"
---

## Goal

Publish v0.3.1 so the shell installer works on Linux hosts with glibc older than 2.35 (atelier-vo0z).

## Done when

- Workspace version is 0.3.1, Cargo.lock matches, and CHANGELOG has a 0.3.1 entry.
- A v0.3.1 tag on a green master commit publishes a GitHub Release with gnu and musl Linux archives, macOS archives, checksums and the shell installer.
