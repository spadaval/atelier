---
created_at: "2026-06-13T22:31:40.448328159+00:00"
id: "atelier-vu88"
issue_type: "task"
labels:
- "architecture"
- "cleanup"
- "lock-sync"
- "stabilization"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Remove inherited lock sync and daemon code"
updated_at: "2026-06-13T22:51:53.000432335+00:00"
---

## Description

Inherited Chainlink lock/sync and daemon code remains in the crate after the
public lock and sync command surfaces were removed. The current product model
uses workflow transitions, local work association, branch/worktree state,
clean-worktree checks, and evidence/closeout gates instead of remote lock sync.
Remove the defunct implementation paths unless a retained caller has an
explicit current product owner.

## Outcome

- `locks.rs`, `sync.rs`, `lock_check.rs`, and daemon remnants are deleted or
  reduced to a deliberately retained internal API with a documented owner.
- Normal ready/start/create/worktree/Agent Factory flows do not call inherited
  remote lock sync or emit removed `atelier locks` / `atelier sync` guidance.
- Local runtime ignore/layout rules retain only currently owned runtime paths;
  obsolete `.locks-cache` handling is removed or explicitly justified.
- Tests and docs describe work association as the coordination model, not
  inherited lock sync.

## Evidence

- `rg` command output for `lock_check`, `SyncManager`, `LocksFile`,
  `.locks-cache`, `atelier locks`, `atelier sync`, and `daemon` classifies all
  remaining references as deleted, docs-only provenance, or intentionally owned.
- Focused CLI tests or transcripts prove `atelier next`, `atelier start`,
  `atelier issue create --work`, and relevant Agent Factory start/claim flows
  work without remote lock state.
- Help or rejected-command transcript proves removed lock/sync/daemon command
  surfaces are absent or fail as unsupported.
- Deleted-code review or diff summary records any retained exception and its
  owning issue or architecture artifact.
- `cargo fmt -- --check`, relevant focused tests, `atelier lint`, and
  `atelier export --check` pass.

## Notes

Audit evidence already attached to parent `atelier-yqj6`: `src/commands/next.rs`
imports `lock_check` and skips issues locked by other agents;
`src/lock_check.rs` performs best-effort sync/fetch, reads
`.atelier/hook-config.json` auto-steal policy, and still tells users to run
removed `atelier locks check`; `src/command_surface.rs` classifies `locks` and
`sync` as removed roots; `src/sync.rs` still owns `.locks-cache` and
`atelier/locks`; `src/daemon.rs` appears orphaned.
