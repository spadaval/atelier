---
created_at: "2026-10-17T01:10:57.981187320+00:00"
id: "atelier-6tmk"
issue_type: "bug"
labels:
- "cli-output"
- "setup"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2mky"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Handle repositories without Git or without commits"
updated_at: "2026-10-17T02:41:22.663991335+00:00"
---

## Goal

- Outside a Git repository, `atelier init` succeeds and writes `.gitignore`.
- In that same non-Git directory, `atelier status` exits 0 and prints `Checkout: unavailable - fatal: not a git repository`, and `atelier land` fails with a raw `git rev-parse` error.
- In a Git repository with no commits yet, right after `atelier init`, both `atelier prime` and `atelier land` fail with:
  ```
  Error: git rev-parse HEAD failed: fatal: ambiguous argument 'HEAD': unknown revision or path not in the working tree.
  Use '--' to separate paths from revisions, like this:
  ```
  Status shows git's text `Branch  : No commits yet on master`.

## Done when

- `atelier init` refuses outside Git with `not a Git repository; run git init first`, and writes nothing.
- Other commands outside Git say `not a Git repository` once and exit nonzero, instead of relaying `fatal:`.
- With an unborn HEAD, status, prime and land say `No commits yet: commit the Atelier install (git add .atelier .gitignore && git commit), then rerun`.
- Tests cover a non-Git directory and an unborn HEAD.

## Context

`git_freshness::head_commit` (crates/atelier-app/src/git_freshness.rs) is the common failure point. `git_state` in crates/atelier-cli/src/commands/status.rs:182 relays git's stderr.

## Notes

- 2026-10-17 [note] 2026-10-16: Review of atelier-sdb0 found status on an unborn HEAD prints 'Checkout: unavailable - git ls-tree -rz HEAD failed' (previously 'No commits yet'). Include status in this fix.
