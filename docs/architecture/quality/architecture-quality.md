# Architecture Quality Vocabulary

## Abstraction

Prefer abstractions that protect real domain invariants: canonical Markdown
records, projection-index freshness, workflow validator evaluation, typed links,
workflow transitions, evidence attachment, and SQLite rebuild. Avoid wrappers
whose only purpose is hiding current Chainlink names before the target behavior
is implemented.

## Coupling

CLI parsing, RecordStore persistence, ProjectionIndex rebuild, and workflow
validation should be coupled through explicit data structures rather than
command-output parsing. SQLite runtime state must stay decoupled from the
mergeable repository record store.

## Cohesion

Keep command handlers focused on user intent and argument handling. Keep
canonical Markdown parsing/rendering in RecordStore modules. Keep projection and
runtime schema transaction logic in database modules. Keep validation proof and
freshness checks in the boundary that owns them.

## Determinism

Canonical Markdown records must be deterministic across machines and runs. File
order, IDs, timestamps, JSON key ordering, and derived projection outputs need
explicit rules when they affect diffs or rebuild.

## Operability

Agent-facing commands should support stable JSON output when they are used for
coordination, automation, validation, or Mission Control. Errors should identify
the failed record, validator, transition, or file projection.

## Preservation

When replacing inherited Chainlink behavior, preserve useful CLI, SQLite,
session, locking, and test coverage until a bead or ADR explicitly permits
breakage or migration.
