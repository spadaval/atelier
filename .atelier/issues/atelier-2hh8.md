---
created_at: "2026-10-17T09:02:27.670226390+00:00"
id: "atelier-2hh8"
issue_type: "bug"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-2hh8
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Set SO_NOSIGPIPE on the cat-file socket on Apple"
updated_at: "2026-10-17T09:11:02.486259098+00:00"
---

## Goal

atelier-bsyn writes `git cat-file --batch` requests with `libc::send(..., MSG_NOSIGNAL)` so a dead child is an error rather than SIGPIPE. libc defines `MSG_NOSIGNAL` for Apple targets, but whether XNU honours it on `send` was not verified on macOS; Rust's standard library sets `SO_NOSIGPIPE` on Apple sockets instead. If the flag is ignored, macOS behaves as before bsyn (atelier ends on SIGPIPE when the child dies).

## Done when

- On Apple targets the socket also has `SO_NOSIGPIPE` set, or a macOS run of the bsyn test shows `MSG_NOSIGNAL` is honoured.
- The choice is recorded in a comment next to the socket setup.
