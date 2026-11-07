---
created_at: "2026-11-07T03:35:05.237639709+00:00"
id: "atelier-htux"
evidence_type: "validation"
commit: "944e22c35dc76a8cd00d1b8108999357e3e7b3bb"
result: "pass"
captured_at: "2026-11-07T03:35:05.237500850+00:00"
agent_identity: "independent mission validator (claude-opus-5-5)"
target:
  kind: "issue"
  id: "atelier-n4mj"
  role: "validates"
criteria:
- slug: "gates-hold"
  result: "pass"
criteria_mission: "atelier-n4mj"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-n4mj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "gates-hold at 944e22c3: cargo nextest run -j 4 --workspace 986 passed; -p atelier-cli --features test-hooks --test merge_gates --test session --test cli_integration 643 passed; merge_gates+session+lifecycle_cost+proof_timing 232 passed. Every merge/land gate test removed or changed in git diff 05f01d83 HEAD -- crates/*/tests maps to a rule the mission removes:\n(1) merge --sync reruns (ADR 0026): removed sync_records_a_failing_rerun_and_refuses (sync_rerun_failed), sync_reports_a_rerun_that_errors_with_a_code_and_still_reports_the_approval (sync_rerun_error), sync_rerun_receipt_summary_is_the_old_summary_with_the_rerun_suffix, sync_reruns_a_legacy_nested_receipt_with_its_real_summary_and_one_suffix; sync_merges_the_base_reruns_current_receipts... renamed ..._records_no_receipts... and sync_stops_at_conflicts... now expect no rerun.\n(2) a later pass stops clearing an older fail once stale (ADR 0026): session land_names_uncommitted_files_that_make_a_newer_pass_stale -> uncommitted_code_does_not_revive_a_failure_a_later_pass_cleared, a_current_claim_whose_pass_a_later_commit_staled_* and a_retained_claim_* (land refused, now passes; land's 'Uncommitted changes make passing proof stale' note gone); verified_records newer_failure_blocks..._stale_pass_cannot_clear_it -> ..._until_any_later_pass and records_evidence evidence_list_verdict_agrees_with_the_close_gate (close refused, now allowed).\n(3) separate disposition event (ADR 0025): merge_check_names_each_finding... defer message now 'needs to be set by a non-author' (still refuses); session root_review_introductions..., a_disposition_without_findable_history..., retirement_cannot_hide..., a_changed_review_record_is_restored..., leaving_out_a_merge_that_rewrote_a_review... retarget the finding file or reword review_append_only (all still refuse); record_commit 2 files -> 1.\n(4) asking for proof again (ADR 0026 hints): merge_diverged, rebase-recovery and rewrite hints drop 'record fresh proof'; status/work views drop 'no proof at head' and Proof gaps (views, not gates).\n(5) statuses (atelier-bqbz, Outcome 'statuses and types'): request_review/blocked tests moved to the earlier-starter fixture policy and still pass; no gate removed.\nNo removed or changed gate test lacks a removing rule. The debrief lists these only in per-item outcomes (s66u, hwxm, bqbz) and does not name sync_rerun_failed/sync_rerun_error; this receipt is the consolidated list."
updated_at: "2026-11-07T03:35:05.238327702+00:00"
---

gates-hold at 944e22c3: cargo nextest run -j 4 --workspace 986 passed; -p atelier-cli --features test-hooks --test merge_gates --test session --test cli_integration 643 passed; merge_gates+session+lifecycle_cost+proof_timing 232 passed. Every merge/land gate test removed or changed in git diff 05f01d83 HEAD -- crates/*/tests maps to a rule the mission removes:
(1) merge --sync reruns (ADR 0026): removed sync_records_a_failing_rerun_and_refuses (sync_rerun_failed), sync_reports_a_rerun_that_errors_with_a_code_and_still_reports_the_approval (sync_rerun_error), sync_rerun_receipt_summary_is_the_old_summary_with_the_rerun_suffix, sync_reruns_a_legacy_nested_receipt_with_its_real_summary_and_one_suffix; sync_merges_the_base_reruns_current_receipts... renamed ..._records_no_receipts... and sync_stops_at_conflicts... now expect no rerun.
(2) a later pass stops clearing an older fail once stale (ADR 0026): session land_names_uncommitted_files_that_make_a_newer_pass_stale -> uncommitted_code_does_not_revive_a_failure_a_later_pass_cleared, a_current_claim_whose_pass_a_later_commit_staled_* and a_retained_claim_* (land refused, now passes; land's 'Uncommitted changes make passing proof stale' note gone); verified_records newer_failure_blocks..._stale_pass_cannot_clear_it -> ..._until_any_later_pass and records_evidence evidence_list_verdict_agrees_with_the_close_gate (close refused, now allowed).
(3) separate disposition event (ADR 0025): merge_check_names_each_finding... defer message now 'needs to be set by a non-author' (still refuses); session root_review_introductions..., a_disposition_without_findable_history..., retirement_cannot_hide..., a_changed_review_record_is_restored..., leaving_out_a_merge_that_rewrote_a_review... retarget the finding file or reword review_append_only (all still refuse); record_commit 2 files -> 1.
(4) asking for proof again (ADR 0026 hints): merge_diverged, rebase-recovery and rewrite hints drop 'record fresh proof'; status/work views drop 'no proof at head' and Proof gaps (views, not gates).
(5) statuses (atelier-bqbz, Outcome 'statuses and types'): request_review/blocked tests moved to the earlier-starter fixture policy and still pass; no gate removed.
No removed or changed gate test lacks a removing rule. The debrief lists these only in per-item outcomes (s66u, hwxm, bqbz) and does not name sync_rerun_failed/sync_rerun_error; this receipt is the consolidated list.
