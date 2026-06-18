---
created_at: "2026-06-18T22:42:06.610418670+00:00"
id: "atelier-r0k7"
issue_type: "task"
labels:
- "forgejo"
- "pr"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T23:43:42.670083504+00:00"
status: "done"
title: "Preflight PR branch policy before remote creation"
updated_at: "2026-06-18T23:43:42.670083504+00:00"
---

## Description

`atelier pr open` currently calls Forgejo before local branch-policy validation
persists the `pull_request` field. A source or target branch mismatch can
therefore create a remote PR that Atelier refuses to link. Make the command
failure-atomic from Atelier's point of view by resolving the branch owner and
expected base/source branches before any remote create request is sent.

## Outcome

- `atelier pr open --issue <id>` rejects source or target branches that do not
  match the resolved branch owner before sending a Forgejo create request.
- Successful PR open behavior still stores the normalized `pull_request` number
  on the branch-owning issue or epic and records PR action attribution.
- Error text names the expected source branch, expected target branch, resolved
  owner, and corrective command or option.

## Evidence

- Focused test with a mocked Forgejo transport proves a branch mismatch sends
  zero remote requests and leaves canonical issue records unchanged.
- Focused passing test proves the valid path still opens the PR, persists the
  link, refreshes projection state, and records the PR action.
- `cargo test -p atelier-cli --lib commands::pr::tests` passes.
