---
created_at: "2026-10-15T19:29:46.740869541+00:00"
id: "atelier-hlck"
evidence_type: "validation"
commit: "8f13d015eda0d0d702e77bf318fb686565e62955"
result: "pass"
captured_at: "2026-10-15T19:29:46.740328261+00:00"
agent_identity: "independent-validator-2026-10-15"
target:
  kind: "issue"
  id: "atelier-xz11"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-xz11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S3 PASS (independent-validator-2026-10-15, candidate 8f13d015). Tests read and run (PASS): verified_records::stale_code_and_acceptance_changes_require_new_proof_including_reverts (code, acceptance and policy edits, uncommitted, committed, and reverted, all stale proof), newer_failure_blocks_completion_without_presence_gate_and_stale_pass_cannot_clear_it, notes_status_and_new_evidence_records_do_not_stale_passing_proof, capture_refuses_uncommitted_code_and_head_changes_without_partial_evidence, side_branch_edits_reverts_and_merge_result_code_changes_stale_proof. Independent scenario /tmp/xz11/scen2.sh: captured evidence pins commit == HEAD with result pass/fail derived from exit status; close refused 'newer failing evidence <id>'; a manual pass followed by a code commit is stale and close stays refused; a fresh pass allows close; capture with dirty code refused. Real repo also shows the rule: at 8f13d015, atelier-8ud1 (done) fails evidence.no_newer_failure because its pass atelier-zqkt @ e30e10b7 predates code commit fdf8fd68 while fail atelier-3onc remains."
updated_at: "2026-10-15T19:29:46.742284231+00:00"
---

S3 PASS (independent-validator-2026-10-15, candidate 8f13d015). Tests read and run (PASS): verified_records::stale_code_and_acceptance_changes_require_new_proof_including_reverts (code, acceptance and policy edits, uncommitted, committed, and reverted, all stale proof), newer_failure_blocks_completion_without_presence_gate_and_stale_pass_cannot_clear_it, notes_status_and_new_evidence_records_do_not_stale_passing_proof, capture_refuses_uncommitted_code_and_head_changes_without_partial_evidence, side_branch_edits_reverts_and_merge_result_code_changes_stale_proof. Independent scenario /tmp/xz11/scen2.sh: captured evidence pins commit == HEAD with result pass/fail derived from exit status; close refused 'newer failing evidence <id>'; a manual pass followed by a code commit is stale and close stays refused; a fresh pass allows close; capture with dirty code refused. Real repo also shows the rule: at 8f13d015, atelier-8ud1 (done) fails evidence.no_newer_failure because its pass atelier-zqkt @ e30e10b7 predates code commit fdf8fd68 while fail atelier-3onc remains.
