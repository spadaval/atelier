---
schema: atelier.review
schema_version: 2
id: atelier-yjhx
title: 'low: crates/atelier-cli/src/gate_report.rs shell.check line (Gap::FailedCommand fix) The fix for a failed shell.check, and for the merge.merged_tree failure folded into it, is the check command itself. Under merge_commit the check runs on the merged tree, so running it in the work checkout can pass while merge still refuses (repro: policy check ''test ! -f conflict.txt'', base adds conflict.txt after branching; printed ''Next: test ! -f conflict.txt'' passes locally, merge --check still refuses). Documented in human-cli-output.md as intended, but it does not move toward passing; consider saying ''merge the base into the branch, then rerun'' for merged_tree cases or  as the rerun.'
target: atelier-h1nc
identity: Independent reviewer (Claude Sonnet subagent)
commit: 342d93ff699a891b98de38b3ab5c4b932a1d303b
created_at: 2026-10-23T14:52:35.771498322Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude Opus 5.5, rc8j)
    commit: d4a52ac942571049b35accfa2c5e9b1e928682dd
    recorded_at: 2026-10-23T14:58:30.186770547Z
    resolution_commit: 06272b9d4b6401c8d7fe6c7561257da0b2c9446d
    reason: Under merge_commit the check runs on the merged tree; the fix must bring the base in first.
    migrated_from: atelier-9ihw
---

low: crates/atelier-cli/src/gate_report.rs shell.check line (Gap::FailedCommand fix) The fix for a failed shell.check, and for the merge.merged_tree failure folded into it, is the check command itself. Under merge_commit the check runs on the merged tree, so running it in the work checkout can pass while merge still refuses (repro: policy check 'test ! -f conflict.txt', base adds conflict.txt after branching; printed 'Next: test ! -f conflict.txt' passes locally, merge --check still refuses). Documented in human-cli-output.md as intended, but it does not move toward passing; consider saying 'merge the base into the branch, then rerun' for merged_tree cases or  as the rerun.
