---
created_at: "2026-10-22T21:18:32.116340778+00:00"
id: "atelier-976s"
issue_type: "bug"
labels:
- "land"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Refuse land --push when the upstream does not resolve"
updated_at: "2026-10-22T22:48:09.493694038+00:00"
---

## Goal

`atelier land --push` can still publish unreviewed code from the base branch when the branch's upstream is set but does not resolve: the remote branch was deleted (git shows `[gone]`), or the remote is a URL or path. Land then skips the base-commit review check, prints "Base commits unchecked", passes and pushes. The reviewer reproduced both cases with a local bare remote. Found in review of atelier-yim7 (finding atelier-hwfm).

## Done when

Under `--push`, land refuses with a named code and fix when the upstream does not resolve to a remote-tracking ref it can check base commits against, and pushes nothing. Tests cover a gone upstream and a URL remote.
