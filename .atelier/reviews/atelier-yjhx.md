---
schema: atelier.review
schema_version: 1
id: atelier-yjhx
title: 'low: crates/atelier-cli/src/gate_report.rs shell.check line (Gap::FailedCommand fix) The fix for a failed shell.check, and for the merge.merged_tree failure folded into it, is the check command itself. Under merge_commit the check runs on the merged tree, so running it in the work checkout can pass while merge still refuses (repro: policy check ''test ! -f conflict.txt'', base adds conflict.txt after branching; printed ''Next: test ! -f conflict.txt'' passes locally, merge --check still refuses). Documented in human-cli-output.md as intended, but it does not move toward passing; consider saying ''merge the base into the branch, then rerun'' for merged_tree cases or  as the rerun.'
target: atelier-h1nc
identity: Independent reviewer (Claude Sonnet subagent)
commit: 342d93ff699a891b98de38b3ab5c4b932a1d303b
created_at: 2026-10-23T14:52:35.771498322Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/gate_report.rs shell.check line (Gap::FailedCommand fix) The fix for a failed shell.check, and for the merge.merged_tree failure folded into it, is the check command itself. Under merge_commit the check runs on the merged tree, so running it in the work checkout can pass while merge still refuses (repro: policy check 'test ! -f conflict.txt', base adds conflict.txt after branching; printed 'Next: test ! -f conflict.txt' passes locally, merge --check still refuses). Documented in human-cli-output.md as intended, but it does not move toward passing; consider saying 'merge the base into the branch, then rerun' for merged_tree cases or  as the rerun.
