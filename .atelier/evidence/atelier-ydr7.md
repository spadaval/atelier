---
created_at: "2026-06-15T18:24:12.544717148+00:00"
id: "atelier-ydr7"
evidence_type: "validation"
captured_at: "2026-06-15T18:24:12.544604939+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-wng0"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-wng0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "SQLite caller retarget validation: root CLI db and projection_index modules were deleted/moved into atelier-sqlite; rg over crates/atelier-cli/src and crates/atelier-sqlite/src found no crate::db, atelier::db, pub mod db, or crate::projection_index references. RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets passed; cargo test -p atelier-sqlite -- --nocapture passed 65 tests with 4 ignored extended tests; focused CLI nextest passed issue create/mutation durability, first-class records export/rebuild validation, and root status blocked-list guidance. Command transcripts in temp repos passed export --check, rebuild, doctor, status, and valid import-beads through the migrated sqlite boundary."
updated_at: "2026-06-15T18:24:15.940698759+00:00"
---

SQLite caller retarget validation: root CLI db and projection_index modules were deleted/moved into atelier-sqlite; rg over crates/atelier-cli/src and crates/atelier-sqlite/src found no crate::db, atelier::db, pub mod db, or crate::projection_index references. RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets passed; cargo test -p atelier-sqlite -- --nocapture passed 65 tests with 4 ignored extended tests; focused CLI nextest passed issue create/mutation durability, first-class records export/rebuild validation, and root status blocked-list guidance. Command transcripts in temp repos passed export --check, rebuild, doctor, status, and valid import-beads through the migrated sqlite boundary.
