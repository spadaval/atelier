---
created_at: "2026-06-17T22:59:31.106600508+00:00"
id: "atelier-v35t"
evidence_type: "validation"
captured_at: "2026-06-17T22:59:18.189058170+00:00"
command: "bash -lc 'set -euo pipefail\nBIN=/root/atelier/target/debug/atelier\n$BIN --help | tee /tmp/atelier-help.txt\n! grep -q \"  plan          \" /tmp/atelier-help.txt\nif $BIN plan --help > /tmp/atelier-plan.out 2> /tmp/atelier-plan.err; then\n  echo \"atelier plan unexpectedly succeeded\" >&2\n  exit 1\nfi\ngrep -q \"unrecognized subcommand '\"'\"'plan'\"'\"'\" /tmp/atelier-plan.err\n! rg -n \"PlanRecord|PlanRecordData|PlanRevision|Record::Plan|create_plan|Commands::Plan|PlanCommands|planned_by|atelier plan show\" crates/atelier-core crates/atelier-records crates/atelier-app/src crates/atelier-cli/src crates/atelier-sqlite/src\n! rg -n \"MilestoneRecord|MilestoneRecordData|Record::Milestone|create_milestone|has_checkpoint|atelier milestone\" crates/atelier-core crates/atelier-records crates/atelier-app/src crates/atelier-cli/src crates/atelier-sqlite/src\ntmp=$(mktemp -d)\n(cd \"$tmp\" && $BIN init >/tmp/atelier-init.out)\ntest ! -e \"$tmp/.atelier/plans\"\ntest ! -e \"$tmp/.atelier/milestones\"\ncargo test -p atelier-records\ncargo test -p atelier-cli test_plan_apply_command_is_removed\ncargo test -p atelier-cli test_bundle_preview_rejects_plan_and_milestone_resources\ncargo test -p atelier-cli test_first_class_detail_views_read_payloads_from_record_store\ncargo test -p atelier-cli test_first_class_records_export_rebuild_and_validate\n$BIN lint atelier-a3e7\n$BIN lint atelier-v7d0\n$BIN lint\n$BIN export --check\n$BIN doctor\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-a3e7"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-a3e7"
    role: "validates"
  - kind: "issue"
    id: "atelier-v7d0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Plan and milestone record removal validation: plan command is absent, production code no longer exposes plan/milestone record APIs or link roles, init omits .atelier/plans and .atelier/milestones, focused tests and tracker checks pass."
updated_at: "2026-06-17T22:59:47.129119645+00:00"
---

## Summary

Plan and milestone record removal validation: plan command is absent, production code no longer exposes plan/milestone record APIs or link roles, init omits .atelier/plans and .atelier/milestones, focused tests and tracker checks pass.

## Command

```console
bash -lc 'set -euo pipefail
BIN=/root/atelier/target/debug/atelier
$BIN --help | tee /tmp/atelier-help.txt
! grep -q "  plan          " /tmp/atelier-help.txt
if $BIN plan --help > /tmp/atelier-plan.out 2> /tmp/atelier-plan.err; then
  echo "atelier plan unexpectedly succeeded" >&2
  exit 1
fi
grep -q "unrecognized subcommand '"'"'plan'"'"'" /tmp/atelier-plan.err
! rg -n "PlanRecord|PlanRecordData|PlanRevision|Record::Plan|create_plan|Commands::Plan|PlanCommands|planned_by|atelier plan show" crates/atelier-core crates/atelier-records crates/atelier-app/src crates/atelier-cli/src crates/atelier-sqlite/src
! rg -n "MilestoneRecord|MilestoneRecordData|Record::Milestone|create_milestone|has_checkpoint|atelier milestone" crates/atelier-core crates/atelier-records crates/atelier-app/src crates/atelier-cli/src crates/atelier-sqlite/src
tmp=$(mktemp -d)
(cd "$tmp" && $BIN init >/tmp/atelier-init.out)
test ! -e "$tmp/.atelier/plans"
test ! -e "$tmp/.atelier/milestones"
cargo test -p atelier-records
cargo test -p atelier-cli test_plan_apply_command_is_removed
cargo test -p atelier-cli test_bundle_preview_rejects_plan_and_milestone_resources
cargo test -p atelier-cli test_first_class_detail_views_read_payloads_from_record_store
cargo test -p atelier-cli test_first_class_records_export_rebuild_and_validate
$BIN lint atelier-a3e7
$BIN lint atelier-v7d0
$BIN lint
$BIN export --check
$BIN doctor
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 8440
Truncated: yes

```text
Mission and proof oriented work coordination for agents

Usage: atelier [OPTIONS] <COMMAND>



Setup:
  init          Initialize Atelier in the current repository

Orientation:
  man           Show role-specific operating guidance
  status        Show checkout, mission, work, and tracker signposts
  start         Start tracked work on an issue

Issues:
  issue         Create, list, show, update, close, and manage blockers
  search        Search issue text
  graph         Inspect mission and issue hierarchy and impact

Missions and planning:
  mission       Create, list, show, status, close, and update durable missions
  bundle        Preview and apply one-shot graph bundle files

Records:
  evidence      Capture validation evidence
  history       Inspect canonical repo, mission, issue, or epic activity

Advanced work:
  worktree      Create, inspect, merge, and remove mission or issue worktrees
  branch        Inspect and repair epic review branches

Maintenance:
  maintenance   Run explicit destructive maintenance commands
  lint          Validate tracker records
  doctor        Check runtime and derived-state health; use --fix for local repair

Common commands:
  atelier man
  atelier man worker
  atelier man reviewer
  atelier man manager
  atelier man admin
  atelier status
  atelier issue list
  atelier issue list --ready
  atelier issue list --blocked
  atelier issue show <id>
  atelier issue block <blocked-id> <blocker-id>
  atelier issue unblock <blocked-id> <blocker-id>
  atelier issue blocked [<id>]
  atelier mission list
  atelier mission show <id>
  atelier mission status
  atelier mission close <id> --reason "..."
  atelier history --mission <id>
  atelier history --issue <id>
  atelier start <issue-id>
  atelier issue transition <issue-id> --options
  atelier issue close <issue-id> --reason "..."
  atelier doctor
  atelier doctor --fix
  atelier help <command>

Options:
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]
  -h, --help                     Print help
  -V, --version                  Print version

running 41 tests
test activity::tests::issue_activity_sidecar_path_is_canonical ... ok
test activity::tests::front_matter_and_body_round_trip ... ok
test activity::tests::timestamp_activity_id_uses_utc_microseconds ... ok
test activity::tests::allocation_adds_deterministic_suffixes_for_same_timestamp_collisions ... ok
test activity::tests::rejects_invalid_schema ... ok
test record_id::tests::legacy_ids_are_project_scoped_base36 ... ok
test record_id::tests::validates_project_scoped_ids ... ok
test activity::tests::rejects_invalid_schema_version_subject_and_event_type ... ok
test activity::tests::lists_issue_activities_in_oldest_first_order ... ok
test tests::evidence_record_rejects_data_front_matter ... ok
test tests::issue_parser_contract_rejects_content_before_first_recognized_heading ... ok
test tests::issue_parser_contract_rejects_empty_present_sections ... ok
test tests::issue_parser_contract_rejects_duplicate_recognized_headings ... ok
test tests::issue_parser_contract_rejects_missing_required_sections ... ok
test activity::tests::create_allocates_collision_suffix_and_does_not_overwrite ... ok
test tests::issue_parser_reports_malformed_front_matter ... ok
test tests::issue_parser_contract_accepts_sectioned_body_without_legacy_arrays ... ok
test tests::issue_sections_report_shared_presence_state_and_search_text ... ok
test tests::issue_record_round_trips_explicit_closed_at_for_done_status ... ok
test tests::issue_record_renders_and_parses_deterministically ... ok
test activity::tests::write_refuses_to_overwrite_existing_activity ... ok
test tests::evidence_record_renders_and_parses_deterministically_without_data_blob ... ok
test tests::issue_parser_contract_rejects_legacy_acceptance_and_evidence_front_matter ... ok
test tests::mis
```

## Stderr

Bytes: 2081
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/atelier_records-63f63b856dfcc15b)
   Doc-tests atelier_records
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.72s
     Running unittests src/lib.rs (target/debug/deps/atelier-a4af03d4d4980b0c)
     Running unittests src/main.rs (target/debug/deps/atelier-f93dfb214ae0ef56)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-268a89a087d3e1b4)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-3ae4459e698a0c00)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.69s
     Running unittests src/lib.rs (target/debug/deps/atelier-a4af03d4d4980b0c)
     Running unittests src/main.rs (target/debug/deps/atelier-f93dfb214ae0ef56)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-268a89a087d3e1b4)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-3ae4459e698a0c00)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.68s
     Running unittests src/lib.rs (target/debug/deps/atelier-a4af03d4d4980b0c)
     Running unittests src/main.rs (target/debug/deps/atelier-f93dfb214ae0ef56)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-268a89a087d3e1b4)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-3ae4459e698a0c00)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.73s
     Running unittests src/lib.rs (target/debug/deps/atelier-a4af03d4d4980b0c)
     Running unittests src/main.rs (target/debug/deps/atelier-f93dfb214ae0ef56)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-268a89a087d3e1b4)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-3ae4459e698a0c00)
```

