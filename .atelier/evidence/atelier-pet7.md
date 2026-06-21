---
created_at: "2026-06-21T19:02:33.550699425+00:00"
id: "atelier-pet7"
evidence_type: "validation"
captured_at: "2026-06-21T19:02:24.646786225+00:00"
command: "bash -lc 'cargo fmt -- --check && cargo build -p atelier-cli --bin atelier && target/debug/atelier lint && git diff --check && cargo nextest run -p atelier-cli test_root_status_reports_current_mission_counts_without_active_focus test_mission_terminal_status_and_options_use_configured_objective_validators test_mission_close_uses_configured_objective_validators && rg -n \"commands::mission::active_mission|pub fn active_mission|is_active_mission|mission_focus_label\" crates/atelier-cli/src || true'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-kivn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-kivn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'cargo fmt -- --check && cargo build -p atelier-cli --bin atelier && target/debug/atelier lint && git diff --check && cargo nextest run -p atelier-cli test_root_status_reports_current_mission_counts_without_active_focus test_mission_terminal_status_and_options_use_configured_objective_validators test_mission_close_uses_configured_objective_validators && rg -n \"commands::mission::active_mission|pub fn active_mission|is_active_mission|mission_focus_label\" crates/atelier-cli/src || true'"
updated_at: "2026-06-21T19:02:38.641246822+00:00"
---

## Summary

bash -lc 'cargo fmt -- --check && cargo build -p atelier-cli --bin atelier && target/debug/atelier lint && git diff --check && cargo nextest run -p atelier-cli test_root_status_reports_current_mission_counts_without_active_focus test_mission_terminal_status_and_options_use_configured_objective_validators test_mission_close_uses_configured_objective_validators && rg -n "commands::mission::active_mission|pub fn active_mission|is_active_mission|mission_focus_label" crates/atelier-cli/src || true'

## Command

```console
bash -lc 'cargo fmt -- --check && cargo build -p atelier-cli --bin atelier && target/debug/atelier lint && git diff --check && cargo nextest run -p atelier-cli test_root_status_reports_current_mission_counts_without_active_focus test_mission_terminal_status_and_options_use_configured_objective_validators test_mission_close_uses_configured_objective_validators && rg -n "commands::mission::active_mission|pub fn active_mission|is_active_mission|mission_focus_label" crates/atelier-cli/src || true'
```

Exit status: 0

## Stdout

Bytes: 13
Truncated: no

```text
Lint passed.
```

## Stderr

Bytes: 1034
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-ncq9/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.88s
   Compiling atelier-cli v0.2.0 (/root/atelier-ncq9/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.96s
────────────
 Nextest run ID afddb36f-addf-4d9d-8bb0-c6d15a326442 with nextest profile: default
    Starting 3 tests across 4 binaries (443 tests skipped)
        PASS [   0.734s] (1/3) atelier-cli::cli_integration mission_projection_worktree::test_root_status_reports_current_mission_counts_without_active_focus
        PASS [   0.793s] (2/3) atelier-cli::cli_integration mission_projection_worktree::test_mission_terminal_status_and_options_use_configured_objective_validators
        PASS [   2.473s] (3/3) atelier-cli::cli_integration mission_projection_worktree::test_mission_close_uses_configured_objective_validators
────────────
     Summary [   2.474s] 3 tests run: 3 passed, 443 skipped
```

