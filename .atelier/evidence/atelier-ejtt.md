---
created_at: "2026-06-11T23:28:48.933251763+00:00"
id: "atelier-ejtt"
data: "{\"captured_at\":\"2026-06-11T23:28:48.933197374+00:00\",\"kind\":\"validation\",\"path\":null,\"producer\":null,\"result\":\"pass\",\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-kxko"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Init and ignore scaffolding validated for tracked .atelier records: fresh init creates .atelier/config.toml plus issues/missions/milestones/plans/evidence directories, keeps .atelier/state.db local, writes precise root .gitignore entries for runtime/cache/lock/identity/rules/worktree paths, and no longer creates .atelier/.gitignore. Validation passed: cargo fmt -- --check; cargo test --no-run; cargo test commands::init::tests::test_run_fresh_init -- --nocapture; cargo test commands::init::tests::test_run_database_usable -- --nocapture; cargo test --test cli_integration test_init_force_update -- --nocapture; atelier export --check; atelier lint; atelier doctor; workflow validate issue atelier-kxko."
updated_at: "2026-06-11T23:28:54.448165692+00:00"
---

Init and ignore scaffolding validated for tracked .atelier records: fresh init creates .atelier/config.toml plus issues/missions/milestones/plans/evidence directories, keeps .atelier/state.db local, writes precise root .gitignore entries for runtime/cache/lock/identity/rules/worktree paths, and no longer creates .atelier/.gitignore. Validation passed: cargo fmt -- --check; cargo test --no-run; cargo test commands::init::tests::test_run_fresh_init -- --nocapture; cargo test commands::init::tests::test_run_database_usable -- --nocapture; cargo test --test cli_integration test_init_force_update -- --nocapture; atelier export --check; atelier lint; atelier doctor; workflow validate issue atelier-kxko.
