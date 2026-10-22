---
schema: atelier.review
schema_version: 1
id: atelier-grmb
title: 'medium: crates/atelier-cli/src/commands/merge.rs:342,518 With --json, a records-only base move during checks (atelier-p189 retry) prints the gate array at line 518 before the base check refuses with RetryOnMovedBase, then run() calls merge_once again, which prints a second array. stdout becomes two concatenated JSON documents, which a JSON consumer cannot parse, on exactly the path p189 adds (human output also repeats every gate line, which the merge_base_moved marker explains). Fix: emit the gate report only from the attempt that does not retry, for example by having merge_once return the nodes (or move the print after the base-moved check), and add a --json assertion to merge_commit_strategy_reruns_the_gates_when_the_base_moves_only_by_records_during_checks.'
target: atelier-c3kh
identity: Independent reviewer (Claude subagent)
commit: f5733347e44e5cbc88ce25b5680ea8e91d92ddfc
created_at: 2026-10-22T20:52:27.832682463Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-cli/src/commands/merge.rs:342,518 With --json, a records-only base move during checks (atelier-p189 retry) prints the gate array at line 518 before the base check refuses with RetryOnMovedBase, then run() calls merge_once again, which prints a second array. stdout becomes two concatenated JSON documents, which a JSON consumer cannot parse, on exactly the path p189 adds (human output also repeats every gate line, which the merge_base_moved marker explains). Fix: emit the gate report only from the attempt that does not retry, for example by having merge_once return the nodes (or move the print after the base-moved check), and add a --json assertion to merge_commit_strategy_reruns_the_gates_when_the_base_moves_only_by_records_during_checks.
