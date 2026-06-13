---
created_at: "2026-06-12T21:53:13.836342388+00:00"
id: "atelier-eoaq"
evidence_type: "validation"
captured_at: "2026-06-12T21:53:13.733721104+00:00"
command: "target/debug/atelier workflow validate mission atelier-tcmr --validator ignored_tests_reviewed"
exit_status: "1"
path: null
uri: null
proof_scope: null
agent_identity: null
independence_level: null
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-l0yk"
    role: "validates"
  - kind: "mission"
    id: "atelier-tcmr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "blocked"
title: "ignored test inventory captured linked stale blockers"
updated_at: "2026-06-12T21:53:21.974039125+00:00"
---

ignored test inventory captured linked stale blockers

Command: target/debug/atelier workflow validate mission atelier-tcmr --validator ignored_tests_reviewed
Exit status: 1

Stdout summary (truncated):
Workflow Validation: mission atelier-tcmr
========================================
Transition: close
Validators: 1
Results
-------
  fail  ignored_tests_reviewed
      Reason: ignored test inventory found 59 blocker(s) across 63 ignored test(s)
  - tests/cli_integration.rs:1806 test_history_reads_activity_sidecars_with_filters_and_json reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored product-behavior test is still blocking closeout
  - tests/cli_integration.rs:1862 test_history_empty_and_invalid_limit reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored product-behavior test is still blocking closeout
  - tests/cli_integration.rs:1877 test_evidence_issue_link_creates_history_activity reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored product-behavior test is still blocking closeout
  - tests/cli_integration.rs:2775 test_session_start reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored product-behavior test is still blocking closeout
  - tests/cli_integration.rs:2787 test_session_status reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored product-behavior test is still blocking closeout
  - tests/cli_integration.rs:2800 test_session_work reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored product-behavior test is still blocking closeout
  - tests/cli_integration.rs:2814 test_session_end reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored product-behavior test is still blocking closeout
  - tests/cli_integration.rs:2941 test_archive_closed_issue reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored product-behavior test is still blocking closeout
  - tests/cli_integration.rs:2955 test_archive_open_issue_fails reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored product-behavior test is still blocking closeout
  - tests/cli_integration.rs:2974 test_archive_list reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored product-behavior test is still blocking closeout
  - tests/cli_integration.rs:2991 test_unarchive_issue reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored product-behavior test is still blocking closeout
  - tests/cli_integration.rs:3017 test_milestone_create reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored product-behavior test is still blocking closeout
  - tests/cli_integration.rs:3032 test_milestone_list reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored product-behavior test is still blocking closeout
  - tests/cli_integration.rs:3048 test_milestone_show reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored product-behavior test is still blocking closeout
  - tests/cli_integration.rs:3066 test_milestone_add_issues reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored product-behavior test is still blocking closeout
  - tests/cli_integration.rs:3085 test_milestone_close reason="obsolete legacy command surface removed" owner=(missing) issue="atelier-jqds" product=yes blocking=yes
      problem: ignored prod

Stderr summary:
Error: workflow validation failed: ignored_tests_reviewed

