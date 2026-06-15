---
created_at: "2026-06-15T18:53:09.771127146+00:00"
id: "atelier-nucf"
evidence_type: "validation"
captured_at: "2026-06-15T18:53:07.507446910+00:00"
command: "bash -lc 'target/debug/atelier issue show atelier-2q5s; rg \"println!|eprintln!|clap::\" crates/atelier-app/src -n; test $? -eq 1; cargo fmt -- --check; target/debug/atelier export --check'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-2q5s"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 3664
    summary: "atelier-2q5s [epic] todo/todo - Epic: Split application layer and CLI shell\n===========================================================================\nStatus:   todo\nCategory: todo\nType:     epic\nPriority: high\nCreated:  2026-06-15 01:11 -04:00\nUpdated:  2026-06-15 11:17 -04:00\nLabels:   app-layer, cli, rewrite\nFile:     /root/atelier/.atelier/issues/atelier-2q5s.md\n\nHierarchy\n---------\nParent: (none)\n\nTransition Readiness\n--------------------\n  block: allowed - to blocked\n    atelier issue transition atelier-2q5s block\n  start: allowed - to in_progress\n    atelier issue transition atelier-2q5s start\n  options: atelier issue transition atelier-2q5s --options\n\nDescription\n-----------\nSeparate command orchestration from Clap parsing by introducing an application layer and thinning the CLI shell. Preserve visible command intent, help/docs alignment, and the human-first output contract.\n\nOutcome\n-------\n- `atelier-app` owns use-case orchestration through explicit request/outcome/view-model APIs and does not write directly to stdout or stderr.\n- `atelier-cli` owns Clap definitions, parsing, tracing setup, telemetry recording, command identity, terminal rendering, exit-code handling, and thin delegation into `atelier-app`.\n- Oversized command handlers are split by product job and view model ownership while keeping help-visible command jobs stable.\n- No old command aliases, fallback shims, or public compatibility facades are introduced.\n\nEvidence\n--------\n- Child issue proof shows application ports, request/outcome/view-model extraction, rendering ownership, and thin CLI shell wiring.\n- Search transcript proves `atelier-app` use-case code does not call `println!` or `eprintln!`.\n- CLI/help/docs parity checks cover visible workflow surfaces.\n- Representative command transcripts for status, issue, mission, evidence, lint, doctor, and export check remain behaviorally stable.\n\nNotes\n-----\n- Temporary adapters used while splitting `atelier-app` from `atelier-cli` must\n  follow `docs/architecture/source-layout.md`: name the adapter marker, removal\n  owner, removal condition, and proof that no public compatibility promise is\n  being created.\n\nBlocked by\n----------\n  atelier-qsbe [done/done] high - Epic: Architecture contracts and ADRs for crate rewrite\n\nBlocking\n--------\n  atelier-3kap [todo/todo] high - Epic: Root package deletion and warning-free closeout\n  atelier-4j3k [active/in_progress] high - Convert root Cargo.toml to virtual workspace\n  atelier-fchz [todo/todo] high - Validate and close out crate rewrite mission\n\nSubissues\n---------\n6 total | status: done=6 | priority: high=6\n  atelier-14z2 [done] high - Move command handlers and view models into atelier-app\n  atelier-4ren [done] high - Return request outcome and view model APIs from atelier-app\n  atelier-nyn0 [done] high - Migrate command workflows vertically into app CLI and storage crates\n  atelier-sclf [done] high - Own CLI parser dispatch tracing telemetry and rendering in atelier-cli\n  atelier-vv2i [done] high - Introduce application use-case layer and storage ports\n  atelier-zwna [done] high - Thin atelier-cli to Clap telemetry and exit-code shell\n\nRecent Activity\n---------------\n(none)\n\nNext Commands\n-------------\n  Edit issue Markdown: /root/atelier/.atelier/issues/atelier-2q5s.md\n  Validate this issue: atelier lint atelier-2q5s\n  Add a note: atelier issue note atelier-2q5s \"...\"\n  Show full activity: atelier history --issue atelier-2q5s\n  Show transition options: atelier issue transition atelier-2q5s --options\n  Execute a transition: atelier issue transition atelier-2q5s <transition>\nCanonical export is current\nState: /root/atelier/.atelier\n"
    truncated: false
  stderr:
    bytes: 0
    summary: ""
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2q5s"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "App/CLI split epic validated: all children closed, app has request/view APIs and no direct rendering, CLI owns parser telemetry rendering"
updated_at: "2026-06-15T18:53:13.406013965+00:00"
---

App/CLI split epic validated: all children closed, app has request/view APIs and no direct rendering, CLI owns parser telemetry rendering

Command: bash -lc 'target/debug/atelier issue show atelier-2q5s; rg "println!|eprintln!|clap::" crates/atelier-app/src -n; test $? -eq 1; cargo fmt -- --check; target/debug/atelier export --check'
Exit status: 0

Stdout summary:
atelier-2q5s [epic] todo/todo - Epic: Split application layer and CLI shell
===========================================================================
Status:   todo
Category: todo
Type:     epic
Priority: high
Created:  2026-06-15 01:11 -04:00
Updated:  2026-06-15 11:17 -04:00
Labels:   app-layer, cli, rewrite
File:     /root/atelier/.atelier/issues/atelier-2q5s.md

Hierarchy
---------
Parent: (none)

Transition Readiness
--------------------
  block: allowed - to blocked
    atelier issue transition atelier-2q5s block
  start: allowed - to in_progress
    atelier issue transition atelier-2q5s start
  options: atelier issue transition atelier-2q5s --options

Description
-----------
Separate command orchestration from Clap parsing by introducing an application layer and thinning the CLI shell. Preserve visible command intent, help/docs alignment, and the human-first output contract.

Outcome
-------
- `atelier-app` owns use-case orchestration through explicit request/outcome/view-model APIs and does not write directly to stdout or stderr.
- `atelier-cli` owns Clap definitions, parsing, tracing setup, telemetry recording, command identity, terminal rendering, exit-code handling, and thin delegation into `atelier-app`.
- Oversized command handlers are split by product job and view model ownership while keeping help-visible command jobs stable.
- No old command aliases, fallback shims, or public compatibility facades are introduced.

Evidence
--------
- Child issue proof shows application ports, request/outcome/view-model extraction, rendering ownership, and thin CLI shell wiring.
- Search transcript proves `atelier-app` use-case code does not call `println!` or `eprintln!`.
- CLI/help/docs parity checks cover visible workflow surfaces.
- Representative command transcripts for status, issue, mission, evidence, lint, doctor, and export check remain behaviorally stable.

Notes
-----
- Temporary adapters used while splitting `atelier-app` from `atelier-cli` must
  follow `docs/architecture/source-layout.md`: name the adapter marker, removal
  owner, removal condition, and proof that no public compatibility promise is
  being created.

Blocked by
----------
  atelier-qsbe [done/done] high - Epic: Architecture contracts and ADRs for crate rewrite

Blocking
--------
  atelier-3kap [todo/todo] high - Epic: Root package deletion and warning-free closeout
  atelier-4j3k [active/in_progress] high - Convert root Cargo.toml to virtual workspace
  atelier-fchz [todo/todo] high - Validate and close out crate rewrite mission

Subissues
---------
6 total | status: done=6 | priority: high=6
  atelier-14z2 [done] high - Move command handlers and view models into atelier-app
  atelier-4ren [done] high - Return request outcome and view model APIs from atelier-app
  atelier-nyn0 [done] high - Migrate command workflows vertically into app CLI and storage crates
  atelier-sclf [done] high - Own CLI parser dispatch tracing telemetry and rendering in atelier-cli
  atelier-vv2i [done] high - Introduce application use-case layer and storage ports
  atelier-zwna [done] high - Thin atelier-cli to Clap telemetry and exit-code shell

Recent Activity
---------------
(none)

Next Commands
-------------
  Edit issue Markdown: /root/atelier/.atelier/issues/atelier-2q5s.md
  Validate this issue: atelier lint atelier-2q5s
  Add a note: atelier issue note atelier-2q5s "..."
  Show full activity: atelier history --issue atelier-2q5s
  Show transition options: atelier issue transition atelier-2q5s --options
  Execute a transition: atelier issue transition atelier-2q5s <transition>
Canonical export is current
State: /root/atelier/.atelier

Stderr summary:
(none)

