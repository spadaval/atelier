# App/CLI Boundary Audit, 2026-06-17

Closed app-layer tracker work overstated the live boundary. The target
architecture in `docs/architecture/source-layout.md` says `atelier-app` owns
use-case orchestration and `atelier-cli` owns Clap parsing, telemetry, exit
mapping, and terminal rendering. Current live code has moved some shared
storage policy into `atelier-app::command_storage`, but CLI dispatch and command
modules still choose storage modes, open projections, construct `RecordStore`,
refresh projections, and coordinate mutations.

## Current Contradiction

Search transcript:

- `rg -n "command_storage\\(|projection_query_db\\(|canonical_mutation_db\\(|degraded_projection_query_db\\(|existing_projection_db\\(|RecordStore::new|Database::open|refresh_projection\\(" crates/atelier-cli/src crates/atelier-app/src`
- `rg -n "println!|eprintln!" crates/atelier-app/src`

The first search shows live CLI orchestration hotspots:

- `crates/atelier-cli/src/main.rs` selects storage access modes through
  `command_storage`, `projection_query_db`, `degraded_projection_query_db`,
  `canonical_mutation_db`, and `existing_projection_db`.
- `crates/atelier-cli/src/main.rs` directly constructs `RecordStore` for issue
  create/update flows.
- `crates/atelier-cli/src/commands/evidence.rs`,
  `crates/atelier-cli/src/commands/mission.rs`,
  `crates/atelier-cli/src/commands/workflow.rs`, and
  `crates/atelier-cli/src/commands/issue.rs` still open
  `Database`, construct `RecordStore`, and call projection refresh helpers as
  part of use-case orchestration.
- CLI unit tests also open `Database` directly; those test helpers are not the
  closeout blocker unless they mirror production command orchestration.

The second search currently returns no `println!` or `eprintln!` in
`crates/atelier-app/src`, so app-layer terminal rendering leakage is not the
dominant gap. The contradiction is that CLI code still decides orchestration
and storage access for workflows that the target architecture says should be
app APIs returning request, outcome, and view-model values.

## Required Closeout Proof

Future app-layer closeout claims must attach evidence that includes all of the
following searches and representative behavior tests:

- `crates/atelier-cli/src/main.rs` no longer calls `command_storage`,
  `projection_query_db`, `degraded_projection_query_db`,
  `canonical_mutation_db`, `existing_projection_db`, `RecordStore::new`, or
  `Database::open` for migrated workflows.
- Migrated CLI command modules no longer call `RecordStore::new`,
  `Database::open`, or local projection refresh helpers as part of use-case
  orchestration.
- `crates/atelier-app/src` remains free of `println!` and `eprintln!`; app APIs
  return typed outcomes or view models that CLI renderers print.
- Focused CLI tests or transcripts cover representative migrated status,
  mission, evidence, and workflow surfaces after the orchestration move.
- `atelier lint`, `atelier doctor`, and `git diff --check` pass.

## Follow-Up Owners

- `atelier-j75d` owns the epic-level app/CLI reconciliation.
- `atelier-uro5` owns moving command orchestration behind app APIs.
- `atelier-wpht` owns reducing CLI command modules to renderers and adapters
  after app APIs exist.
