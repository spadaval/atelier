---
created_at: "2026-10-17T01:10:57.972926861+00:00"
id: "atelier-pxyz"
issue_type: "bug"
labels:
- "cli-output"
- "prune"
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
title: "Print `Next: atelier prune --apply` only when apply would do something"
updated_at: "2026-10-17T01:10:57.972926861+00:00"
---

## Goal

The hint is printed unconditionally (crates/atelier-cli/src/commands/prune.rs:118-121). On the real repository it follows `Candidates: 74` where all 74 are protected and 0 are eligible (`atelier -q prune` gives `mode=dry-run eligible=0 protected=74`). On a dirty checkout it appears even though apply then refuses: `Error: canonical prune requires a clean tracked checkout and no untracked .atelier records`.

## Done when

- With nothing eligible, the dry run ends `Nothing to prune`.
- On a dirty checkout it ends `⚠ apply needs a clean checkout: commit or stash N paths first`.
- Otherwise it ends `Next: atelier prune --apply`.
- Tests cover all three.
