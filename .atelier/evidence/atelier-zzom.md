---
created_at: "2026-06-15T19:01:03.847811912+00:00"
id: "atelier-zzom"
evidence_type: "validation"
captured_at: "2026-06-15T18:59:32.622762320+00:00"
command: "bash -lc 'set -e; target/debug/atelier mission status atelier-v5nb; cargo fmt -- --check; cargo nextest run; cargo nextest run --profile extended --run-ignored=only; target/debug/atelier lint; target/debug/atelier export --check; target/debug/atelier doctor; git diff --check; target/debug/atelier status; target/debug/atelier issue list --ready; target/debug/atelier evidence list --result pass >/tmp/fchz-evidence-pass.txt; wc -l /tmp/fchz-evidence-pass.txt'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-fchz"
  role: "validates"
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
status: "recorded"
title: "Final crate rewrite mission closeout validation: all epics closed and required gates pass"
updated_at: "2026-06-15T19:01:07.285460631+00:00"
---

## Summary

Final crate rewrite mission closeout validation: all epics closed and required gates pass

## Command

```console
bash -lc 'set -e; target/debug/atelier mission status atelier-v5nb; cargo fmt -- --check; cargo nextest run; cargo nextest run --profile extended --run-ignored=only; target/debug/atelier lint; target/debug/atelier export --check; target/debug/atelier doctor; git diff --check; target/debug/atelier status; target/debug/atelier issue list --ready; target/debug/atelier evidence list --result pass >/tmp/fchz-evidence-pass.txt; wc -l /tmp/fchz-evidence-pass.txt'
```

Exit status: 0

## Stdout

Bytes: 4395
Truncated: yes

```text
Lint passed.
Mission Status atelier-v5nb [ready] - Complete The Atelier Crate Migration
==========================================================================
Health:   ready
Tracker:  ok
Closeout: blocked

Work
----
Total: 1 ready, 44 done
  [epic] atelier-0fhv [done] high - Epic: Rewrite SQLite projection and runtime storage | 5 done
  [epic] atelier-2q5s [done] high - Epic: Split application layer and CLI shell | 6 done
  [epic] atelier-3kap [done] high - Epic: Root package deletion and warning-free closeout | 5 done
  [epic] atelier-4wor [done] high - Epic: Workspace scaffold and domain extraction | 3 done
  [epic] atelier-kjj1 [done] high - Epic: Extract RecordStore into atelier-records | 3 done
  [epic] atelier-lu10 [done] high - Remove active issue and legacy claim systems | 8 done
  [epic] atelier-qsbe [done] high - Epic: Architecture contracts and ADRs for crate rewrite | 3 done
  [epic] atelier-ycmp [done] high - Epic: Stratify tests and fuzz targets by crate boundary | 3 done
Other: 1 ready

Selectable Work
---------------
  atelier-fchz - Validate and close out crate rewrite mission | ready: no open blockers; mission-linked root; proof missing

Blocked Work
------------
(none)

Blockers
--------
(none)

Evidence
--------
Direct mission evidence: none

Reliability
-----------
Projection Freshness: current
Malformed Work: none
Missing Outcome Sections: none
Missing Evidence Sections: none
Graph Hygiene: clear
Attached Proof: missing - issue proof gaps: atelier-fchz
  Next: atelier evidence record --target issue/<id> --kind validation --result pass "..."
  Next: atelier evidence attach <evidence-id> issue <issue-id>
Docs/Help Drift: clear
Ignored Test Review: current
Open Blockers: none
Drill-downs:
  atelier mission audit atelier-v5nb
  atelier lint
  atelier doctor

Closeout Gates
--------------
Work: open - atelier-fchz
  Next: atelier issue close <issue-id> --reason "..."
Blockers: clear
Tracker State: current
Linked Issue Records: parseable
Validation Criteria: incomplete - workflow approval is still pending on linked validation/closeout work: atelier-fchz
  Next: atelier mission audit atelier-v5nb
Blocking Lints: clear
Docs/Help Drift: clear
Ignored Test Review: current
Worktree: clean

Active Work
-----------
(none)

Next Commands
-------------
  Inspect mission record (durable intent and linked work): atelier mission show atelier-v5nb
  Refresh mission status (current blockers and closeout gates): atelier mission status atelier-v5nb
  Inspect closeout gate detail: atelier mission status --closeout atelier-v5nb
  Start selectable mission work (1 selectable issue(s)): atelier start atelier-fchz
  Record validation proof (1 evidence gap(s)): atelier evidence record --target issue/<id> --kind validation --result pass "..."
  Check runtime health (tracker and projection state): atelier doctor
Lint passed.
Canonical export is current
State: /root/atelier/.atelier
Database: /root/atelier/.atelier/runtime/state.db
State: /root/atelier/.atelier
Install health:
  config: ok
  ignored_runtime_paths: ok
Projection rebuild:
  state_dir: ok
  rebuild_ready: ok
  projection_fresh: ok
  tables: issues, labels, dependencies, relations, records, record_links, projection_index_sources
Cache health:
  cache_dir: missing (optional)
  projection_metadata: ok
Runtime state:
  directory: ok
  database: ok
  local_tables: ok
  diagnostics: enabled
Compatibility:
  tables: 
Legacy health:
config: ok
database: ok
ignore_rules: ok
projection_fresh: ok
rebuild_ready: ok
runtime_state: ok
runtime_tables: ok
Atelier Status
==============
Tracker:       current
Ready work:    1
Current work:  none
Active mission: none (1 current)

Local State
-----------
Branch:   epic/atelier-lu10
Worktree: clean
Tracker:  current

Recent Activity
---------------
(no active mission)

Next Actions
------------
  Inspect mission choices (1 current mission(s), none active): atelier mission status
  Choose ready work (1 ready issue(s) available): atelier issue list --ready
  Start selected work (ready work exists): atelier start <issue-id>
  Check run
```

## Stderr

Bytes: 72962
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.02s
────────────
 Nextest run ID 720e5e78-956b-4070-910a-eb2c30d005f9 with nextest profile: default
    Starting 628 tests across 9 binaries (68 tests skipped)
        PASS [   0.008s] (  1/628) atelier-app command_storage::tests::access_modes_declare_projection_freshness_policy
        PASS [   0.017s] (  2/628) atelier-app workflow_policy::tests::rejects_invalid_evidence_validator_params
        PASS [   0.010s] (  3/628) atelier-app workflow_policy::tests::rejects_missing_issue_type_mapping
        PASS [   0.014s] (  4/628) atelier-app workflow_policy::tests::rejects_unknown_top_level_field
        PASS [   0.016s] (  5/628) atelier-app workflow_policy::tests::rejects_invalid_status_category
        PASS [   0.036s] (  6/628) atelier-app storage_layout::tests::canonical_dir_is_the_atelier_tree
        PASS [   0.016s] (  7/628) atelier-app workflow_policy::tests::parses_valid_policy
        PASS [   0.020s] (  8/628) atelier-app tests::app_entrypoint_returns_view_model_without_rendering
        PASS [   0.009s] (  9/628) atelier-cli command_surface::tests::expands_slash_command_references
        PASS [   0.009s] ( 10/628) atelier-cli command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.014s] ( 11/628) atelier-app storage_layout::tests::rebuild_temp_database_paths_are_local_atelier_paths
        PASS [   0.010s] ( 12/628) atelier-cli command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.016s] ( 13/628) atelier-app workflow_policy::tests::rejects_unknown_validator_reference
        PASS [   0.040s] ( 14/628) atelier-app workflow_policy::tests::rejects_unknown_template_variable
        PASS [   0.123s] ( 15/628) atelier-app export::tests::test_canonical_issue_type_is_explicit_not_label_derived
        PASS [   0.124s] ( 16/628) atelier-app rebuild::tests::rebuild_rejects_activity_for_missing_issue
        PASS [   0.123s] ( 17/628) atelier-app export::tests::test_canonical_noop_export_is_deterministic
        PASS [   0.128s] ( 18/628) atelier-app export::tests::test_canonical_check_reports_dangling_link
        PASS [   0.131s] ( 19/628) atelier-app export::tests::test_canonical_check_reports_invalid_duplicate_id
        PASS [   0.131s] ( 20/628) atelier-app export::tests::test_canonical_export_removes_stale_record_file
        PASS [   0.133s] ( 21/628) atelier-app rebuild::tests::rebuild_reports_invalid_relation_type
        PASS [   0.134s] ( 22/628) atelier-app rebuild::tests::record_table_rejects_non_canonical_record_kinds
        PASS [   0.135s] ( 23/628) atelier-app export::tests::test_canonical_export_preserves_issue_activity_sidecars
        PASS [   0.143s] ( 24/628) atelier-app export::tests::test_canonical_changed_record_export_rewrites_issue
        PASS [   0.144s] ( 25/628) atelier-app export::tests::test_canonical_check_ignores_sqlite_only_canonical_drift
        PASS [   0.144s] ( 26/628) atelier-app rebuild::tests::rebuild_reports_schema_mismatch
        PASS [   0.150s] ( 27/628) atelier-app rebuild::tests::rebuild_reports_malformed_front_matter
        PASS [   0.151s] ( 28/628) atelier-app export::tests::test_canonical_check_reports_stale_projection_metadata
        PASS [   0.008s] ( 29/628) atelier-cli commands::create::tests::test_get_template_exists
        PASS [   0.011s] ( 30/628) atelier-cli commands::comment::tests::test_validate_known_kinds
        PASS [   0.107s] ( 31/628) atelier-cli commands::comment::tests::test_add_comment_to_nonexistent_issue
        PASS [   0.135s] ( 32/628) atelier-app rebuild::tests::rebuild_reports_path_id_mismatch
        PASS [   0.013s] ( 33/628) atelier-cli commands::comment::tests::test_validate_unknown_kinds
        PASS [   0.010s] ( 34/628) atelier-cli commands::create::tests::test_get_template_not_found
        PASS [   0.010s] ( 35/628) atelier-cli commands::create::tests::test_invalid_priorities
```
