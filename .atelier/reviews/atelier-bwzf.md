---
schema: atelier.review
schema_version: 1
id: atelier-bwzf
title: 'crates/atelier-app/src/lint.rs:184 graph_errors stops at the first error that names no issue ID, so independent errors validated later are no longer reported (regression from d1cb7d90, breaks atelier-idae). Validation order is hierarchy, children cycle, blocks cycle, link types, policy; ''relationships.blocks contains a cycle'' names no ID, so named is empty and the loop breaks. Scenario: A blocks B, B blocks A, and unrelated C relates D with unconfigured type ''frobnicates''. ba7512b7 check reports 1 finding (cycle only); d1cb7d90 reports 2 (link type + cycle). A children cycle likewise masks a blocks cycle. The new test does not cover this. Suggested fix: make RecordIndex accumulate stage errors, or keep a cheap per-record pass for single-record checks (link types, non-issue targets) and add the scenario to check_reports_every_independent_graph_error_in_one_run.'
target: atelier-1ioo
identity: independent-reviewer-2-2026-10-15
commit: e5a1d2f46211ec26f4c6d5f78c7bb461f65bda0b
created_at: 2026-10-15T19:20:58.806683787Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/lint.rs:184 graph_errors stops at the first error that names no issue ID, so independent errors validated later are no longer reported (regression from d1cb7d90, breaks atelier-idae). Validation order is hierarchy, children cycle, blocks cycle, link types, policy; 'relationships.blocks contains a cycle' names no ID, so named is empty and the loop breaks. Scenario: A blocks B, B blocks A, and unrelated C relates D with unconfigured type 'frobnicates'. ba7512b7 check reports 1 finding (cycle only); d1cb7d90 reports 2 (link type + cycle). A children cycle likewise masks a blocks cycle. The new test does not cover this. Suggested fix: make RecordIndex accumulate stage errors, or keep a cheap per-record pass for single-record checks (link types, non-issue targets) and add the scenario to check_reports_every_independent_graph_error_in_one_run.
