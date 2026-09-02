---
created_at: "2026-09-02T15:03:48.926268217+00:00"
id: "atelier-y6id"
evidence_type: "test"
captured_at: "2026-09-02T15:03:18.846061484+00:00"
command: "cargo nextest run"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-pnuk"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-pnuk"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run"
updated_at: "2026-09-02T15:03:48.929391500+00:00"
---

## Summary

cargo nextest run

## Command

```console
cargo nextest run
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 94985
Truncated: yes

```text
 Downloading crates ...
  Downloaded bit-set v0.8.0
  Downloaded tempfile v3.27.0
  Downloaded wait-timeout v0.2.1
  Downloaded fnv v1.0.7
  Downloaded quick-error v1.2.3
  Downloaded fastrand v2.3.0
  Downloaded rand_xorshift v0.4.0
  Downloaded errno v0.3.14
  Downloaded unarray v0.1.4
  Downloaded rand_core v0.9.3
  Downloaded getrandom v0.3.4
  Downloaded arbitrary v1.4.2
  Downloaded rand_chacha v0.9.0
  Downloaded rand v0.9.4
  Downloaded bit-vec v0.8.0
  Downloaded rusty-fork v0.3.1
  Downloaded ppv-lite86 v0.2.21
  Downloaded proptest v1.9.0
  Downloaded zerocopy v0.8.31
  Downloaded rustix v1.1.4
  Downloaded linux-raw-sys v0.12.1
   Compiling getrandom v0.3.4
   Compiling num-traits v0.2.19
   Compiling bitflags v2.10.0
   Compiling rustix v1.1.4
   Compiling linux-raw-sys v0.12.1
   Compiling fastrand v2.3.0
   Compiling regex-syntax v0.8.8
   Compiling zerocopy v0.8.31
   Compiling wait-timeout v0.2.1
   Compiling quick-error v1.2.3
   Compiling bit-vec v0.8.0
   Compiling fnv v1.0.7
   Compiling unarray v0.1.4
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
   Compiling arbitrary v1.4.2
   Compiling rusqlite v0.40.1
   Compiling bit-set v0.8.0
   Compiling rand_core v0.9.3
   Compiling rand v0.9.4
   Compiling rand_xorshift v0.4.0
   Compiling chrono v0.4.45
   Compiling regex-automata v0.4.14
   Compiling tempfile v3.27.0
   Compiling atelier-core v0.2.0 (/root/atelier/crates/atelier-core)
   Compiling ppv-lite86 v0.2.21
   Compiling rusty-fork v0.3.1
   Compiling rand_chacha v0.9.0
   Compiling proptest v1.9.0
   Compiling atelier-workflow v0.2.0 (/root/atelier/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/root/atelier/crates/atelier-records)
   Compiling matchers v0.2.0
   Compiling tracing-subscriber v0.3.23
   Compiling atelier-sqlite v0.2.0 (/root/atelier/crates/atelier-sqlite)
   Compiling atelier-app v0.2.0 (/root/atelier/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 18.75s
────────────
 Nextest run ID 8be89bd6-a1bb-4780-b74b-287cac2a7f35 with nextest profile: default
    Starting 762 tests across 9 binaries
        PASS [   0.009s] (  1/762) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.009s] (  2/762) atelier-app health::tests::review_backend_health_reports_provider_auth_and_missing_repo
        PASS [   0.009s] (  3/762) atelier-app health::tests::review_backend_health_reports_unreachable_provider
        PASS [   0.010s] (  4/762) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.010s] (  5/762) atelier-app health::tests::review_backend_health_reports_role_author_readiness_failures
        PASS [   0.011s] (  6/762) atelier-app forgejo::tests::provider_contract_lists_reviews_then_comments_by_review_id
        PASS [   0.013s] (  7/762) atelier-app forgejo::tests::provisions_role_users_and_repository_permissions
        PASS [   0.008s] (  8/762) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
        PASS [   0.009s] (  9/762) atelier-app issue_inventory::tests::default_and_explicit_positive_limits_apply_after_selection_and_ordering
        PASS [   0.008s] ( 10/762) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.010s] ( 11/762) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.011s] ( 12/762) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.009s] ( 13/762) atelier-app issue_inventory::tests::zero_limit_is_rejected
        PASS [   0.026s] ( 14/762) atelier-app forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship
        PASS [   0.028s] ( 15/762) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.027s] ( 16/762) atelier-app forgejo::tests::opens_pull_with_role_sudo_header_and_payload
```
