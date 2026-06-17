---
created_at: "2026-06-17T23:11:43.411860927+00:00"
id: "atelier-ipgc"
evidence_type: "validation"
captured_at: "2026-06-17T23:11:43.411852622+00:00"
target:
  kind: "issue"
  id: "atelier-2573"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2573"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Narrowed atelier-sqlite Database projection boundary. Database::conn is crate-private; normal add/get comment APIs were renamed to explicit legacy-import helpers; no-op session and work-association modules were deleted. Searches found no external direct Database::conn use, no old normal comment helper names, and no removed session/work APIs in production code. Validation passed: cargo check -p atelier-sqlite; cargo check -p atelier-cli; cargo test -p atelier-sqlite; cargo test -p atelier-app rebuild_recreates_canonical_projection_without_local_only_state; cargo test -p atelier-app refresh_projection_rebuilds_without_local_only_state; cargo test -p atelier-cli commands::comment::tests; cargo test -p atelier-cli commands::delete::tests; cargo fmt -- --check; target/debug/atelier lint atelier-2573; git diff --check; cargo build -p atelier-cli."
updated_at: "2026-06-17T23:11:47.308522473+00:00"
---

Narrowed atelier-sqlite Database projection boundary. Database::conn is crate-private; normal add/get comment APIs were renamed to explicit legacy-import helpers; no-op session and work-association modules were deleted. Searches found no external direct Database::conn use, no old normal comment helper names, and no removed session/work APIs in production code. Validation passed: cargo check -p atelier-sqlite; cargo check -p atelier-cli; cargo test -p atelier-sqlite; cargo test -p atelier-app rebuild_recreates_canonical_projection_without_local_only_state; cargo test -p atelier-app refresh_projection_rebuilds_without_local_only_state; cargo test -p atelier-cli commands::comment::tests; cargo test -p atelier-cli commands::delete::tests; cargo fmt -- --check; target/debug/atelier lint atelier-2573; git diff --check; cargo build -p atelier-cli.
