---
created_at: "2026-10-22T19:58:07.550441930+00:00"
id: "atelier-9g48"
issue_type: "feature"
labels:
- "merge"
- "orientation"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-ftqn"
  - kind: "issue"
    id: "atelier-tciw"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Show which waiting branches are ready to land"
updated_at: "2026-10-22T19:58:16.763000375+00:00"
---

## Goal

`status` lists each local work branch on one dense line. For each branch waiting to land it shows commits behind the base, whether it merges cleanly or which files conflict, approval state, and whether proof covers the branch head. It also names waiting branches that touch the same files, so an orchestrator brings the base in only for the branch that is next.

## Done when

- atelier-ftqn's and atelier-tciw's Done when hold, shown by session tests, with independent review.
- The view changes no Git state.
- Delivers atelier-k8h8 S1.

## Unknowns

atelier-tciw wants the cause of a stale approval from atelier-cisa (mission atelier-rc8j), which is not planned yet. Until it lands, show `approval stale` without the cause.

## Background

Both children change `crates/atelier-cli/src/commands/status.rs`; atelier-ftqn's row format comes first, and atelier-tciw adds the readiness facts. The branches block is already a `-----` section before Next (it renders as `Other Local Work Branches (committed records)`); the dense rows, the `Unowned:` line and approval marks are what remain of atelier-ftqn.
