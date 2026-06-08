# Architecture Quality Vocabulary

## Abstraction

Prefer abstractions that protect real domain invariants: canonical projections,
gate evaluation, typed links, workflow transitions, evidence attachment, and
SQLite rebuild. Avoid wrappers whose only purpose is hiding current Chainlink
names before the target behavior is implemented.

## Coupling

CLI parsing, persistence, export/rebuild, and workflow validation should be
coupled through explicit data structures rather than command-output parsing.
SQLite runtime state must stay decoupled from the mergeable repository
projection.

## Cohesion

Keep command handlers focused on user intent and argument handling. Keep schema
and transaction logic in database modules. Keep validation proof and stale-export
checks in the validation/export boundary that owns them.

## Determinism

Canonical exports must be deterministic across machines and runs. File order,
IDs, timestamps, JSON key ordering, and generated graph/projection outputs need
explicit rules when they affect diffs or rebuild.

## Operability

Agent-facing commands should support stable JSON output when they are used for
coordination, automation, validation, or Mission Control. Errors should identify
the failed record, gate, transition, or file projection.

## Preservation

When replacing inherited Chainlink behavior, preserve useful CLI, SQLite,
session, locking, and test coverage until a bead or ADR explicitly permits
breakage or migration.
