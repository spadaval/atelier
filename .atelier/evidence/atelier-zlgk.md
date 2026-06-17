---
created_at: "2026-06-17T23:03:00.849981673+00:00"
id: "atelier-zlgk"
evidence_type: "validation"
captured_at: "2026-06-17T23:02:19.691243516+00:00"
command: "bash -lc 'set -euo pipefail\nBIN=/root/atelier/target/debug/atelier\ncargo fmt -- --check\ncargo check -p atelier-cli\ncargo test -p atelier-records\ncargo test -p atelier-cli\n$BIN --help | tee /tmp/atelier-help-aqqc.txt\n! grep -q \"  plan          \" /tmp/atelier-help-aqqc.txt\n! grep -q \"  milestone     \" /tmp/atelier-help-aqqc.txt\nif $BIN plan --help > /tmp/atelier-plan-aqqc.out 2> /tmp/atelier-plan-aqqc.err; then exit 1; fi\ngrep -q \"unrecognized subcommand '\"'\"'plan'\"'\"'\" /tmp/atelier-plan-aqqc.err\nif $BIN milestone --help > /tmp/atelier-milestone-aqqc.out 2> /tmp/atelier-milestone-aqqc.err; then exit 1; fi\ngrep -q \"unrecognized subcommand '\"'\"'milestone'\"'\"'\" /tmp/atelier-milestone-aqqc.err\n! rg -n \"PlanRecord|PlanRecordData|PlanRevision|Record::Plan|create_plan|Commands::Plan|PlanCommands|planned_by|atelier plan show\" crates/atelier-core crates/atelier-records crates/atelier-app/src crates/atelier-cli/src crates/atelier-sqlite/src\n! rg -n \"MilestoneRecord|MilestoneRecordData|Record::Milestone|create_milestone|has_checkpoint|atelier milestone\" crates/atelier-core crates/atelier-records crates/atelier-app/src crates/atelier-cli/src crates/atelier-sqlite/src\nrg -n \"first-class plan|first-class milestone|\\\\.atelier/plans|\\\\.atelier/milestones|atelier plan|atelier milestone|plan create|plan show|milestone create|plan records|milestone records|plans,|milestones,|plans\\\\[|milestones\\\\[|plan_ids|milestone_ids|plan_drift|linked milestones|linked plans|plans and milestones|milestones and plans\" SPEC.md CONTEXT.md docs .atelier/issues || true\n$BIN lint atelier-aqqc\n$BIN lint\n$BIN export --check\n$BIN doctor\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-aqqc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7qsr"
    role: "validates"
  - kind: "issue"
    id: "atelier-aqqc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent closeout validation for plan/milestone removal: docs and tracker references are deferred or historical, public help omits plan/milestone commands, production searches show no record APIs or link roles, full CLI tests and tracker health checks pass."
updated_at: "2026-06-17T23:04:12.420976100+00:00"
---

## Summary

Independent closeout validation for plan/milestone removal: docs and tracker references are deferred or historical, public help omits plan/milestone commands, production searches show no record APIs or link roles, full CLI tests and tracker health checks pass.

## Command

```console
bash -lc 'set -euo pipefail
BIN=/root/atelier/target/debug/atelier
cargo fmt -- --check
cargo check -p atelier-cli
cargo test -p atelier-records
cargo test -p atelier-cli
$BIN --help | tee /tmp/atelier-help-aqqc.txt
! grep -q "  plan          " /tmp/atelier-help-aqqc.txt
! grep -q "  milestone     " /tmp/atelier-help-aqqc.txt
if $BIN plan --help > /tmp/atelier-plan-aqqc.out 2> /tmp/atelier-plan-aqqc.err; then exit 1; fi
grep -q "unrecognized subcommand '"'"'plan'"'"'" /tmp/atelier-plan-aqqc.err
if $BIN milestone --help > /tmp/atelier-milestone-aqqc.out 2> /tmp/atelier-milestone-aqqc.err; then exit 1; fi
grep -q "unrecognized subcommand '"'"'milestone'"'"'" /tmp/atelier-milestone-aqqc.err
! rg -n "PlanRecord|PlanRecordData|PlanRevision|Record::Plan|create_plan|Commands::Plan|PlanCommands|planned_by|atelier plan show" crates/atelier-core crates/atelier-records crates/atelier-app/src crates/atelier-cli/src crates/atelier-sqlite/src
! rg -n "MilestoneRecord|MilestoneRecordData|Record::Milestone|create_milestone|has_checkpoint|atelier milestone" crates/atelier-core crates/atelier-records crates/atelier-app/src crates/atelier-cli/src crates/atelier-sqlite/src
rg -n "first-class plan|first-class milestone|\\.atelier/plans|\\.atelier/milestones|atelier plan|atelier milestone|plan create|plan show|milestone create|plan records|milestone records|plans,|milestones,|plans\\[|milestones\\[|plan_ids|milestone_ids|plan_drift|linked milestones|linked plans|plans and milestones|milestones and plans" SPEC.md CONTEXT.md docs .atelier/issues || true
$BIN lint atelier-aqqc
$BIN lint
$BIN export --check
$BIN doctor
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 79775
Truncated: yes

```text

running 41 tests
test activity::tests::front_matter_and_body_round_trip ... ok
test activity::tests::allocation_adds_deterministic_suffixes_for_same_timestamp_collisions ... ok
test activity::tests::issue_activity_sidecar_path_is_canonical ... ok
test activity::tests::create_allocates_collision_suffix_and_does_not_overwrite ... ok
test activity::tests::rejects_invalid_schema ... ok
test activity::tests::timestamp_activity_id_uses_utc_microseconds ... ok
test record_id::tests::legacy_ids_are_project_scoped_base36 ... ok
test record_id::tests::validates_project_scoped_ids ... ok
test activity::tests::rejects_invalid_schema_version_subject_and_event_type ... ok
test activity::tests::write_refuses_to_overwrite_existing_activity ... ok
test tests::issue_parser_contract_rejects_content_before_first_recognized_heading ... ok
test tests::issue_parser_contract_accepts_sectioned_body_without_legacy_arrays ... ok
test tests::issue_parser_contract_rejects_empty_present_sections ... ok
test tests::issue_parser_contract_rejects_duplicate_recognized_headings ... ok
test tests::issue_parser_contract_rejects_legacy_acceptance_and_evidence_front_matter ... ok
test tests::issue_parser_contract_rejects_missing_required_sections ... ok
test tests::issue_parser_reports_malformed_front_matter ... ok
test tests::evidence_record_rejects_data_front_matter ... ok
test tests::issue_parser_contract_rejects_unknown_top_level_sections ... ok
test activity::tests::lists_issue_activities_in_oldest_first_order ... ok
test tests::issue_record_renders_and_parses_deterministically ... ok
test tests::issue_parser_reports_schema_and_path_mismatch ... ok
test tests::issue_record_round_trips_explicit_closed_at_for_done_status ... ok
test tests::issue_sections_report_shared_presence_state_and_search_text ... ok
test tests::mission_record_rejects_data_front_matter ... ok
test tests::plan_and_milestone_record_kinds_are_deferred ... ok
test tests::mission_render_normalizes_legacy_evidence_attachments ... ok
test tests::evidence_record_renders_and_parses_deterministically_without_data_blob ... ok
test tests::record_store_allocates_ids_across_canonical_dirs ... ok
test tests::registered_first_class_record_kinds_have_canonical_contracts ... ok
test tests::workflow_validator_kind_is_registered_but_not_canonical_yet ... ok
test tests::mission_record_renders_and_parses_deterministically_without_data_blob ... ok
test tests::write_issue_atomic_rejects_path_traversal_ids_before_writing ... ok
test tests::record_store_mutates_generic_issue_and_domain_relationships ... ok
test tests::write_issue_atomic_ignores_stale_fixed_temp_artifact ... ok
test tests::record_store_discovers_and_rejects_noncanonical_issue_paths ... ok
test tests::record_store_label_unlabel_mutates_issue_front_matter ... ok
test tests::record_store_block_unblock_mutates_blocker_relationships ... ok
test tests::record_store_relate_unrelate_mutates_both_issue_records ... ok
test tests::record_store_block_rejects_cycles_and_self_blocks ... ok
test tests::record_store_mutates_issue_child_relationships_in_canonical_markdown ... ok

test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 170 tests
test command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections ... ok
test command_surface::tests::expands_slash_command_references ... ok
test command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent ... ok
test commands::comment::tests::test_validate_known_kinds ... ok
test commands::comment::tests::test_validate_unknown_kinds ... ok
test commands::create::tests::test_invalid_priorities_never_validate ... ok
test commands::create::tests::test_list_templates ... ok
test commands::create::tests::test_get_template_exists ... ok
test commands::create::tests::test_get_template_not_found ... ok
test commands::create::tests::test_template_bug_description_prefix ... ok
test commands::create::tests::test_template_featu
```

## Stderr

Bytes: 1087
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.79s
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/atelier_records-63f63b856dfcc15b)
   Doc-tests atelier_records
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.05s
     Running unittests src/lib.rs (target/debug/deps/atelier-a4af03d4d4980b0c)
     Running unittests src/main.rs (target/debug/deps/atelier-f93dfb214ae0ef56)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-268a89a087d3e1b4)
Switched to branch 'main'
Switched to branch 'codex/atelier-4iyn'
Preparing worktree (new branch 'codex/atelier-n14q')
Switched to branch 'main'
Switched to branch 'main'
Switched to branch 'mission/atelier-nt00'
Switched to branch 'main'
Switched to branch 'main'
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-3ae4459e698a0c00)
   Doc-tests atelier
```

