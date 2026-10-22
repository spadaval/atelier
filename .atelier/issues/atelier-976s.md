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
closed_at: "2026-10-22T22:51:22.223641986+00:00"
status: "done"
title: "Refuse land --push when the upstream does not resolve"
updated_at: "2026-10-22T22:51:22.223641986+00:00"
---

## Goal

`atelier land --push` can still publish unreviewed code from the base branch when the branch's upstream is set but does not resolve: the remote branch was deleted (git shows `[gone]`), or the remote is a URL or path. Land then skips the base-commit review check, prints "Base commits unchecked", passes and pushes. The reviewer reproduced both cases with a local bare remote. Found in review of atelier-yim7 (finding atelier-hwfm).

## Done when

Under `--push`, land refuses with a named code and fix when the upstream does not resolve to a remote-tracking ref it can check base commits against, and pushes nothing. Tests cover a gone upstream and a URL remote.

## Notes

- 2026-10-22 [decision] Check the base branch's upstream in the push step, not in the checks
  Reason: The checks' report must still print before the refusal; the refusal needs the base's configured branch.<base>.remote/merge to exist but base@{upstream} to fail to resolve. A base with no upstream at all keeps today's unchecked note, since the Goal names only a set-but-unresolved upstream.
  Rejected: Refusing in check_base_commits: it also runs without --push, where the note is correct

## Close Reason

`land --push` now refuses with `land_push_base_upstream_unresolved` (fix named, nothing pushed) when the base branch has an upstream configured that does not resolve to a remote-tracking ref: a gone remote branch or a URL/path remote.

### Deviations
None.

### Decisions
See the decision note on atelier-976s (check placed in the push step; a base with no upstream at all is unchanged).

### Surprises
The base-commit check that skips an unresolved upstream applies to the base's upstream, not the work branch's, so the refusal checks the base.

### Open items
None.
