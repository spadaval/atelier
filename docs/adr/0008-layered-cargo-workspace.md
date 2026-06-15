# ADR 0008: Layered Cargo Workspace

## Status

Accepted.

## Context

Atelier is currently implemented as one Rust crate with command handlers,
canonical Markdown storage, workflow policy, SQLite projection/runtime code,
domain types, and CLI rendering in the same package. That shape was useful for
the Chainlink fork and early tracker replacement work, but it now hides the
product boundaries the crate rewrite depends on:

- canonical Markdown records must remain the durable source of truth;
- SQLite must be rebuildable projection and local runtime state, not a second
  canonical store;
- workflow policy must be a repository-owned contract rather than ad hoc command
  logic;
- CLI parsing and visible command behavior should stay stable while internal
  Rust APIs move; and
- active/current work must be derived from canonical workflow state and checkout
  context, not from runtime-only active-work or claim rows.

The current monolith makes those boundaries easy to collapse. It also encourages
large mechanical moves that preserve inherited module APIs instead of extracting
cohesive ownership.

## Decision

Atelier will move to a layered internal Cargo workspace. The `atelier` binary
remains the product executable, but the implementation is split into crates with
one-way dependency direction:

1. `atelier-core`
   owns record IDs, domain record data types, relationship values, status/value
   validation primitives, and shared constants. It must not depend on
   filesystem traversal, SQLite, Clap, telemetry, or command rendering.

2. `atelier-workflow`
   owns workflow policy parsing, status/category interpretation, transition
   validation, and workflow validator primitives on top of `atelier-core`.

3. `atelier-records`
   owns canonical Markdown record discovery, parsing, validation, deterministic
   rendering, ID allocation, atomic writes, relationship rendering, and activity
   sidecars. It depends on core domain types and may consume workflow types only
   through explicit contracts that do not make workflow policy a side effect of
   file parsing.

4. `atelier-sqlite`
   owns rebuildable `ProjectionIndex` tables and local `RuntimeState` tables.
   It may derive query rows from `atelier-records` and `atelier-workflow`, but it
   must not own canonical project facts that cannot be rebuilt from committed
   `.atelier/` records.

5. `atelier-app`
   owns use-case orchestration, command handlers, service ports, view models,
   and coordination between records, workflow, SQLite, runtime state, Git, and
   diagnostics. It is the boundary where product workflows are assembled.

6. `atelier-cli`
   owns Clap definitions, terminal setup, process exit mapping, and thin
   delegation into `atelier-app`. It must not become a second implementation of
   command behavior.

Dependency direction flows from outer layers to inner contracts:
`atelier-cli -> atelier-app -> {atelier-records, atelier-sqlite,
atelier-workflow} -> atelier-core`. Lower layers must not depend on
`atelier-app` or `atelier-cli`, and no crate may introduce a cycle to preserve an
old module path.

Rust crate APIs are internal to this repository. The rewrite does not promise
compatibility for old `atelier::...` Rust paths, inherited module names, or
temporary crate APIs. Repository tests and fuzz targets should move to the new
internal APIs that match each crate's ownership.

Temporary internal migration adapters are allowed only as tracked rewrite
scaffolding. They are not public compatibility layers. Each adapter must name an
owning issue, a removal condition, and a searchable marker or issue note that
lets mission closeout inventory remaining adapters. Once its removal condition
is met, cleanup should remove the adapter directly rather than stage a
deprecation window.

## Consequences

- Architecture, issue, and validation work can reason about ownership at crate
  boundaries instead of inherited file locations.
- The current single crate is implementation debt, not the target source layout.
- The workspace split should produce focused crate tests and fuzz targets for
  domain, workflow, records, SQLite, app, and CLI behavior.
- Visible CLI command intent and help/docs parity remain product contracts, but
  internal Rust APIs can break during extraction.
- SQLite rewrite work can replace local schemas freely because committed
  Markdown records are the durable recovery source.
- Runtime active-work and claim tables cannot justify preserving a public or
  internal compatibility surface. They are migration residue unless a later ADR
  defines a new durable current-work contract.

## Alternatives Considered

### Keep The Single Crate And Split Modules

This would reduce Cargo churn and avoid short-term workspace setup work. It was
rejected because the current crate already contains too many unrelated
responsibilities, and module-only cleanup does not create strong test,
dependency, or API pressure around storage and workflow boundaries.

### Extract A Records Crate Only

This would address the largest storage module first. It was rejected as the
target decision because the rewrite also needs workflow, SQLite runtime,
application orchestration, and CLI shell boundaries to avoid recreating the
monolith around `RecordStore`.

### Preserve Old Rust API Paths With Re-exports

This would make mechanical moves less disruptive. It was rejected because the
Rust APIs are internal and the mission explicitly favors direct removal over
compatibility shims unless a human asks for a compatibility window.
