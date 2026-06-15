---
created_at: "2026-06-15T18:58:22.953715710+00:00"
id: "atelier-l4m5"
evidence_type: "validation"
captured_at: "2026-06-15T18:58:16.413065778+00:00"
command: "bash -lc 'target/debug/atelier issue show atelier-3kap; test ! -d src; cargo metadata --no-deps --format-version 1 >/tmp/3kap-metadata.json; RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets; cargo fmt -- --check; target/debug/atelier export --check; git diff --check'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-3kap"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 3964
    summary: "atelier-3kap [epic] todo/todo - Epic: Root package deletion and warning-free closeout\n=====================================================================================\nStatus:   todo\nCategory: todo\nType:     epic\nPriority: high\nCreated:  2026-06-15 11:16 -04:00\nUpdated:  2026-06-15 11:17 -04:00\nLabels:   closeout, rewrite, workspace\nFile:     /root/atelier/.atelier/issues/atelier-3kap.md\n\nHierarchy\n---------\nParent: (none)\n\nTransition Readiness\n--------------------\n  block: allowed - to blocked\n    atelier issue transition atelier-3kap block\n  start: allowed - to in_progress\n    atelier issue transition atelier-3kap start\n  options: atelier issue transition atelier-3kap --options\n\nDescription\n-----------\nRemove the old root crate after the layered workspace is authoritative, and prove the repository has no compatibility imports, root modules, removed-command guidance, or warning debt at closeout.\n\nOutcome\n-------\n- The repository root is a virtual Cargo workspace with no root package, lib target, bin target, or root-owned integration test target.\n- The `atelier` executable is owned by `crates/atelier-cli` and still builds to `target/debug/atelier`.\n- Old root source modules and compatibility import paths are deleted rather than preserved as shims.\n- Closeout checks prove the workspace is warning-free and root-package regressions are guarded.\n\nEvidence\n--------\n- Child proof covers virtual workspace conversion, root source deletion, test/fuzz relocation, and compatibility-path validation.\n- `cargo metadata --no-deps --format-version 1` transcript shows no root package and lists the expected workspace members.\n- `RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets`, `cargo fmt -- --check`, `cargo nextest run`, `atelier lint`, `atelier export --check`, and `atelier doctor` pass before epic closeout.\n- Search transcript proves no `atelier::...` root compatibility imports, `crate::commands`, `crate::db`, or deleted root module references remain.\n\nNotes\n-----\n- Root package deletion closeout must inventory temporary adapters named by\n  implementation epics under `docs/architecture/source-layout.md` and either\n  remove them, link open removal work that blocks closeout, or classify them as\n  target-state code.\n\nBlocked by\n----------\n  atelier-0fhv [done/done] high - Epic: Rewrite SQLite projection and runtime storage\n  atelier-2q5s [done/done] high - Epic: Split application layer and CLI shell\n  atelier-4wor [done/done] high - Epic: Workspace scaffold and domain extraction\n  atelier-kjj1 [done/done] high - Epic: Extract RecordStore into atelier-records\n  atelier-lu10 [done/done] high - Remove active issue and legacy claim systems\n  atelier-qsbe [done/done] high - Epic: Architecture contracts and ADRs for crate rewrite\n  atelier-ycmp [done/done] high - Epic: Stratify tests and fuzz targets by crate boundary\n\nBlocking\n--------\n  atelier-fchz [todo/todo] high - Validate and close out crate rewrite mission\n\nSubissues\n---------\n5 total | status: done=5 | priority: high=5\n  atelier-4j3k [done] high - Convert root Cargo.toml to virtual workspace\n  atelier-cwgx [done] high - Move root integration smoke and fuzz imports into owning crates\n  atelier-epzs [done] high - Delete root crate source modules after migration\n  atelier-qsib [done] high - Add crate migration guardrails and warning-free closeout gate\n  atelier-vu2b [done] high - Prove no root compatibility paths or removed-command guidance remain\n\nRecent Activity\n---------------\n(none)\n\nNext Commands\n-------------\n  Edit issue Markdown: /root/atelier/.atelier/issues/atelier-3kap.md\n  Validate this issue: atelier lint atelier-3kap\n  Add a note: atelier issue note atelier-3kap \"...\"\n  Show full activity: atelier history --issue atelier-3kap\n  Show transition options: atelier issue transition atelier-3kap --options\n  Execute a transition: atelier issue transition atelier-3kap <transition>\nCanonical export is current\nState: /root/atelier/.atelier\n"
    truncated: false
  stderr:
    bytes: 139
    summary: "   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.13s\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3kap"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Root package deletion epic validated: child work closed, root source absent, virtual workspace and final compatibility validation complete"
updated_at: "2026-06-15T18:58:26.483914750+00:00"
---

Root package deletion epic validated: child work closed, root source absent, virtual workspace and final compatibility validation complete

Command: bash -lc 'target/debug/atelier issue show atelier-3kap; test ! -d src; cargo metadata --no-deps --format-version 1 >/tmp/3kap-metadata.json; RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets; cargo fmt -- --check; target/debug/atelier export --check; git diff --check'
Exit status: 0

Stdout summary:
atelier-3kap [epic] todo/todo - Epic: Root package deletion and warning-free closeout
=====================================================================================
Status:   todo
Category: todo
Type:     epic
Priority: high
Created:  2026-06-15 11:16 -04:00
Updated:  2026-06-15 11:17 -04:00
Labels:   closeout, rewrite, workspace
File:     /root/atelier/.atelier/issues/atelier-3kap.md

Hierarchy
---------
Parent: (none)

Transition Readiness
--------------------
  block: allowed - to blocked
    atelier issue transition atelier-3kap block
  start: allowed - to in_progress
    atelier issue transition atelier-3kap start
  options: atelier issue transition atelier-3kap --options

Description
-----------
Remove the old root crate after the layered workspace is authoritative, and prove the repository has no compatibility imports, root modules, removed-command guidance, or warning debt at closeout.

Outcome
-------
- The repository root is a virtual Cargo workspace with no root package, lib target, bin target, or root-owned integration test target.
- The `atelier` executable is owned by `crates/atelier-cli` and still builds to `target/debug/atelier`.
- Old root source modules and compatibility import paths are deleted rather than preserved as shims.
- Closeout checks prove the workspace is warning-free and root-package regressions are guarded.

Evidence
--------
- Child proof covers virtual workspace conversion, root source deletion, test/fuzz relocation, and compatibility-path validation.
- `cargo metadata --no-deps --format-version 1` transcript shows no root package and lists the expected workspace members.
- `RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets`, `cargo fmt -- --check`, `cargo nextest run`, `atelier lint`, `atelier export --check`, and `atelier doctor` pass before epic closeout.
- Search transcript proves no `atelier::...` root compatibility imports, `crate::commands`, `crate::db`, or deleted root module references remain.

Notes
-----
- Root package deletion closeout must inventory temporary adapters named by
  implementation epics under `docs/architecture/source-layout.md` and either
  remove them, link open removal work that blocks closeout, or classify them as
  target-state code.

Blocked by
----------
  atelier-0fhv [done/done] high - Epic: Rewrite SQLite projection and runtime storage
  atelier-2q5s [done/done] high - Epic: Split application layer and CLI shell
  atelier-4wor [done/done] high - Epic: Workspace scaffold and domain extraction
  atelier-kjj1 [done/done] high - Epic: Extract RecordStore into atelier-records
  atelier-lu10 [done/done] high - Remove active issue and legacy claim systems
  atelier-qsbe [done/done] high - Epic: Architecture contracts and ADRs for crate rewrite
  atelier-ycmp [done/done] high - Epic: Stratify tests and fuzz targets by crate boundary

Blocking
--------
  atelier-fchz [todo/todo] high - Validate and close out crate rewrite mission

Subissues
---------
5 total | status: done=5 | priority: high=5
  atelier-4j3k [done] high - Convert root Cargo.toml to virtual workspace
  atelier-cwgx [done] high - Move root integration smoke and fuzz imports into owning crates
  atelier-epzs [done] high - Delete root crate source modules after migration
  atelier-qsib [done] high - Add crate migration guardrails and warning-free closeout gate
  atelier-vu2b [done] high - Prove no root compatibility paths or removed-command guidance remain

Recent Activity
---------------
(none)

Next Commands
-------------
  Edit issue Markdown: /root/atelier/.atelier/issues/atelier-3kap.md
  Validate this issue: atelier lint atelier-3kap
  Add a note: atelier issue note atelier-3kap "..."
  Show full activity: atelier history --issue atelier-3kap
  Show transition options: atelier issue transition atelier-3kap --options
  Execute a transition: atelier issue transition atelier-3kap <transition>
Canonical export is current
State: /root/atelier/.atelier

Stderr summary:
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.13s

