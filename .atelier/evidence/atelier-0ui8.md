---
created_at: "2026-07-06T18:28:24.291325306+00:00"
id: "atelier-0ui8"
evidence_type: "validation"
captured_at: "2026-07-06T18:28:18.645540011+00:00"
command: "env HOME=/tmp/atelier-yysm-home CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup CARGO_TARGET_DIR=/root/atelier/target cargo nextest run -E 'test(test_issue_to_issue_validates_link_is_rejected) or test(test_evidence_relation_role_errors_are_corrective) or test(test_evidence_capture_records_command_metadata_and_attaches_targets) or test(test_evidence_list_elides_command_transcripts) or test(test_evidence_list_bounds_no_summary_command_capture) or test(test_evidence_list_bounds_default_output) or test(test_evidence_quiet_output_is_a_stable_id_composition_path) or test(test_history_repo_wide_is_bounded_and_routes_to_issue_drill_downs) or test(test_history_issue_scope_stays_on_one_record_and_includes_linked_evidence) or test(test_history_issue_scope_excludes_other_targets_of_reused_evidence) or test(test_history_rejects_removed_query_and_objective_scope_flags) or test(test_history_empty_states_and_invalid_limit)'"
exit_status: "0"
agent_identity: "independent-validator"
target:
  kind: "issue"
  id: "atelier-yysm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yysm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent epic validation PASS at 7e659fe7. PASS evidence jobs: record/show/list/attach remain distinct; attach is typed validates-only proof reuse and issue-to-issue validates is rejected. PASS evidence budget: default live list showed 20 of 796 with 776 omitted; command and long-title/transcript text is elided; the 5,200-character no-summary regression stays under 1 KiB without payload leakage; quiet list emitted all 796 IDs. PASS history: repository and one-issue modes default to 20, --limit is the only breadth control, and mission/epic/include-descendants/event-kind/actor/since all reject with exit 2. PASS isolation: evidence reused on A and B remains visible for A while B identity/private activity is absent from history --issue A. PASS docs/help/stale scan: help teaches record first, attach only for reuse, objective history routes to work mission/work epic/issue show; remaining removed forms occur only in explicit removal decision docs and negative tests. PASS hygiene: cargo fmt -- --check; both mission-base diff checks; atelier check and atelier check atelier-yysm. PASS suite: focused 12/12; broad isolated-HOME run reached 709/719 before ten ENOSPC/SQLite-open fixture failures, and every failed test passed on immediate serial rerun 10/10. Ignored inventory contains four explicitly nonblocking extended SQLite property tests unrelated to this epic. Child proof atelier-4oz4, atelier-lzdn, atelier-hhpx; superseded review fail atelier-4i9p; fix atelier-xj6a; passing review atelier-c5w2. Residual risk: host filesystem capacity prevents one all-parallel green exit, but no product failure remains."
updated_at: "2026-07-06T18:28:27.892973119+00:00"
---

## Summary

Independent epic validation PASS at 7e659fe7. PASS evidence jobs: record/show/list/attach remain distinct; attach is typed validates-only proof reuse and issue-to-issue validates is rejected. PASS evidence budget: default live list showed 20 of 796 with 776 omitted; command and long-title/transcript text is elided; the 5,200-character no-summary regression stays under 1 KiB without payload leakage; quiet list emitted all 796 IDs. PASS history: repository and one-issue modes default to 20, --limit is the only breadth control, and mission/epic/include-descendants/event-kind/actor/since all reject with exit 2. PASS isolation: evidence reused on A and B remains visible for A while B identity/private activity is absent from history --issue A. PASS docs/help/stale scan: help teaches record first, attach only for reuse, objective history routes to work mission/work epic/issue show; remaining removed forms occur only in explicit removal decision docs and negative tests. PASS hygiene: cargo fmt -- --check; both mission-base diff checks; atelier check and atelier check atelier-yysm. PASS suite: focused 12/12; broad isolated-HOME run reached 709/719 before ten ENOSPC/SQLite-open fixture failures, and every failed test passed on immediate serial rerun 10/10. Ignored inventory contains four explicitly nonblocking extended SQLite property tests unrelated to this epic. Child proof atelier-4oz4, atelier-lzdn, atelier-hhpx; superseded review fail atelier-4i9p; fix atelier-xj6a; passing review atelier-c5w2. Residual risk: host filesystem capacity prevents one all-parallel green exit, but no product failure remains.

## Command

```console
env HOME=/tmp/atelier-yysm-home CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup CARGO_TARGET_DIR=/root/atelier/target cargo nextest run -E 'test(test_issue_to_issue_validates_link_is_rejected) or test(test_evidence_relation_role_errors_are_corrective) or test(test_evidence_capture_records_command_metadata_and_attaches_targets) or test(test_evidence_list_elides_command_transcripts) or test(test_evidence_list_bounds_no_summary_command_capture) or test(test_evidence_list_bounds_default_output) or test(test_evidence_quiet_output_is_a_stable_id_composition_path) or test(test_history_repo_wide_is_bounded_and_routes_to_issue_drill_downs) or test(test_history_issue_scope_stays_on_one_record_and_includes_linked_evidence) or test(test_history_issue_scope_excludes_other_targets_of_reused_evidence) or test(test_history_rejects_removed_query_and_objective_scope_flags) or test(test_history_empty_states_and_invalid_limit)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 2018
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-yysm/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.20s
────────────
 Nextest run ID 83dfa1b8-1684-4b98-9770-ba1bd1bf13f7 with nextest profile: default
    Starting 12 tests across 9 binaries (711 tests skipped)
        PASS [   0.210s] ( 1/12) atelier-cli::cli_integration issues::test_history_rejects_removed_query_and_objective_scope_flags
        PASS [   0.370s] ( 2/12) atelier-cli::cli_integration records_evidence::test_evidence_list_elides_command_transcripts
        PASS [   0.377s] ( 3/12) atelier-cli::cli_integration records_evidence::test_evidence_list_bounds_no_summary_command_capture
        PASS [   0.395s] ( 4/12) atelier-cli::cli_integration issues::test_history_empty_states_and_invalid_limit
        PASS [   0.529s] ( 5/12) atelier-cli::cli_integration issues::test_history_repo_wide_is_bounded_and_routes_to_issue_drill_downs
        PASS [   0.533s] ( 6/12) atelier-cli::cli_integration issues::test_issue_to_issue_validates_link_is_rejected
        PASS [   0.801s] ( 7/12) atelier-cli::cli_integration records_evidence::test_evidence_quiet_output_is_a_stable_id_composition_path
        PASS [   0.840s] ( 8/12) atelier-cli::cli_integration issues::test_history_issue_scope_stays_on_one_record_and_includes_linked_evidence
        PASS [   0.983s] ( 9/12) atelier-cli::cli_integration records_evidence::test_evidence_relation_role_errors_are_corrective
        PASS [   0.984s] (10/12) atelier-cli::cli_integration issues::test_history_issue_scope_excludes_other_targets_of_reused_evidence
        PASS [   2.450s] (11/12) atelier-cli::cli_integration records_evidence::test_evidence_capture_records_command_metadata_and_attaches_targets
        PASS [   3.096s] (12/12) atelier-cli::cli_integration records_evidence::test_evidence_list_bounds_default_output
────────────
     Summary [   3.097s] 12 tests run: 12 passed, 711 skipped
```

