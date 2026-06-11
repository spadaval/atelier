---
acceptance: []
blocks:
- "atelier-a4ps"
created_at: "2026-06-10T03:50:58.751960931+00:00"
depends_on:
- "atelier-po2n"
evidence_required: []
id: "atelier-tfn8"
issue_type: "task"
labels:
- "architecture"
- "doctor"
- "runtime-state"
- "sqlite"
links: []
parent: "atelier-zd4d"
priority: "P2"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Separate RuntimeState from canonical projection health"
updated_at: "2026-06-10T23:49:12.021247310+00:00"
---

Make local-only runtime state explicit and separate from canonical projection health.

Scope:
- Classify SQLite tables and .atelier files as canonical projection, local runtime, or compatibility.
- Update doctor/lint/rebuild reporting so missing or reset runtime data does not imply canonical record loss.
- Define how sessions, locks, timers, usage, agent identity, and UI/cache state reference canonical record IDs without becoming durable project records.
- Add migrations or docs for any tables that remain intentionally local-only.

Acceptance:
RuntimeState responsibilities are documented in code or architecture docs; doctor distinguishes projection readiness from runtime-state availability; rebuild from .atelier-state recreates canonical query behavior while local runtime tables are empty or migrated safely; tests cover clean rebuild, missing runtime directory, and runtime table preservation or reset behavior.

Validation:
- cargo fmt -- --check
- cargo test runtime or equivalent focused tests
- cargo test
- ./target/debug/atelier rebuild
- ./target/debug/atelier doctor
