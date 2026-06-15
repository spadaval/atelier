---
created_at: "2026-06-15T08:27:13.761839123+00:00"
id: "atelier-7aga"
evidence_type: "validation"
captured_at: "2026-06-15T08:27:13.761717735+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-fchz"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fchz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Mission closeout validation passed for atelier-v5nb on branch mission/atelier-v5nb. Commands: cargo fmt -- --check; cargo check --workspace (passed with existing dead-code/unused warnings); cargo nextest run --test cli_integration (206 passed, 1 skipped); cargo nextest run (636 passed, 24 skipped); cargo nextest run --profile extended --run-ignored=only (8 passed, 652 skipped); cargo check --manifest-path fuzz/Cargo.toml --bins; target/debug/atelier lint; target/debug/atelier export --check; target/debug/atelier doctor; git diff --check. Scenario coverage included stale smoke fixture migration to current CLI surfaces, runtime .gitignore behavior, rebuild fixture parsing, workflow closeout validators, missing/stale SQLite recovery, status-derived current work, records/evidence round trips, CLI help/docs parity, and fuzz crate buildability. Residual risk: cargo check emits pre-existing warning noise only."
updated_at: "2026-06-15T08:27:17.053120730+00:00"
---

Mission closeout validation passed for atelier-v5nb on branch mission/atelier-v5nb. Commands: cargo fmt -- --check; cargo check --workspace (passed with existing dead-code/unused warnings); cargo nextest run --test cli_integration (206 passed, 1 skipped); cargo nextest run (636 passed, 24 skipped); cargo nextest run --profile extended --run-ignored=only (8 passed, 652 skipped); cargo check --manifest-path fuzz/Cargo.toml --bins; target/debug/atelier lint; target/debug/atelier export --check; target/debug/atelier doctor; git diff --check. Scenario coverage included stale smoke fixture migration to current CLI surfaces, runtime .gitignore behavior, rebuild fixture parsing, workflow closeout validators, missing/stale SQLite recovery, status-derived current work, records/evidence round trips, CLI help/docs parity, and fuzz crate buildability. Residual risk: cargo check emits pre-existing warning noise only.
