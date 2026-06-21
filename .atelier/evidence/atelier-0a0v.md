---
created_at: "2026-06-21T20:26:11.803240809+00:00"
id: "atelier-0a0v"
evidence_type: "validation"
captured_at: "2026-06-21T20:25:56.248977323+00:00"
command: "sh -c 'set -eu\nprintf \"== mission terminal status ==\\n\"\ntarget/debug/atelier issue status atelier-53bu --verbose\nprintf \"== mission table ==\\n\"\ntarget/debug/atelier issue table --kind mission\nprintf \"== mission history ==\\n\"\ntarget/debug/atelier history --mission atelier-53bu --limit 5\nprintf \"== lint ==\\n\"\ntarget/debug/atelier lint\nprintf \"== whitespace diff ==\\n\"\ngit diff --check\nprintf \"== format ==\\n\"\ncargo fmt -- --check\nprintf \"== focused nextest ==\\n\"\ncargo nextest run -p atelier-cli -E \"test(test_mission_list_human_overview_orders_and_summarizes) or test(test_mission_terminal_status_and_options_use_configured_objective_validators) or test(test_issue_show_surfaces_evidence_status) or test(test_evidence_capture_records_command_metadata_and_attaches_targets) or test(test_issue_closeout_uses_attached_pass_evidence_not_evidence_text) or test(test_mission_status_reports_terminal_checks_and_explicit_approval) or test(default_validators_are_target_and_transition_aware)\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-53bu"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-53bu"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Mission atelier-53bu final focused validation proof"
updated_at: "2026-06-21T20:26:16.619830570+00:00"
---

## Summary

Mission atelier-53bu final focused validation proof

## Command

```console
sh -c 'set -eu
printf "== mission terminal status ==\n"
target/debug/atelier issue status atelier-53bu --verbose
printf "== mission table ==\n"
target/debug/atelier issue table --kind mission
printf "== mission history ==\n"
target/debug/atelier history --mission atelier-53bu --limit 5
printf "== lint ==\n"
target/debug/atelier lint
printf "== whitespace diff ==\n"
git diff --check
printf "== format ==\n"
cargo fmt -- --check
printf "== focused nextest ==\n"
cargo nextest run -p atelier-cli -E "test(test_mission_list_human_overview_orders_and_summarizes) or test(test_mission_terminal_status_and_options_use_configured_objective_validators) or test(test_issue_show_surfaces_evidence_status) or test(test_evidence_capture_records_command_metadata_and_attaches_targets) or test(test_issue_closeout_uses_attached_pass_evidence_not_evidence_text) or test(test_mission_status_reports_terminal_checks_and_explicit_approval) or test(default_validators_are_target_and_transition_aware)"'
```

Exit status: 0

## Stdout

Bytes: 4783
Truncated: yes

```text
== mission terminal status ==
Mission Status atelier-53bu [ready] - Make workflow obligations explicit and minimal
====================================================================================
Health:   terminal
Tracker:  ok
Terminal: blocked

Work
----
Total: 23 done
  [epic] atelier-f9ci [done] high - Epic: Validate mission rework end to end | 2 done
  [epic] atelier-nbhp [done] high - Epic: Migrate mission records and projections | 2 done
  [epic] atelier-vays [done] high - Epic: Collapse mission command surface into issue commands | 4 done
  [epic] atelier-ncq9 [done] high - Epic: Move mission lifecycle into workflow policy | 3 done
  [epic] atelier-7jma [done] high - Epic: Define minimal mission contract | 2 done
  [epic] atelier-9n3r [done] medium - Epic: Make evidence requirements workflow-driven | 4 done

Selectable Work
---------------
(none)

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
Graph Hygiene: clear
Open Blockers: none
Drill-downs:
  atelier issue status atelier-53bu --verbose
  atelier lint

Terminal Checks
---------------
Work: closed
Blockers: clear
Tracker State: current
Linked Issue Records: parseable

Advanced Validator Detail
-------------------------
1 advanced terminal validator failure detected.
  fail  evidence.attached - no validating evidence link found

Branch Policy
-------------
Current branch: master
Base branch:    master
Owner branches:
  epic atelier-7jma (epic) -> epic/atelier-7jma | not current | merged
  epic atelier-9n3r (epic) -> epic/atelier-9n3r | not current | merged
  epic atelier-f9ci (epic) -> epic/atelier-f9ci | not current | merged
  epic atelier-nbhp (epic) -> epic/atelier-nbhp | not current | merged
  epic atelier-ncq9 (epic) -> epic/atelier-ncq9 | not current | merged
  epic atelier-vays (epic) -> epic/atelier-vays | not current | merged
Dirty state: clean
Branch mismatches: none

Active Work
-----------
(none)

Next Commands
-------------
  Inspect mission record (durable intent and linked work): atelier issue show atelier-53bu
  Refresh mission status (current blockers and terminal checks): atelier issue status atelier-53bu
  Inspect terminal check detail: atelier issue status atelier-53bu --verbose
== mission table ==
Issue Table: mission
====================
ID           Status       Health     Ready  Blocked  Done  Backlog  Title
atelier-24xn ready        blocked        3        6     0       0  Prune stale Atelier artifacts and branches
atelier-53bu ready        terminal       0        0    23       0  Make workflow obligations explicit and minimal
atelier-e3pk ready        steady         0        0     0       0  Superseded by 0v3f: Add session-aware Forgejo PR coordination

Next Commands
-------------
  Inspect one objective: atelier issue status <id>
  Open one objective record: atelier issue show <id>
  Browse grouped work: atelier issue list
== mission history ==
History
=======
Scope:          mission atelier-53bu - Make workflow obligations explicit and minimal
Source:         canonical .atelier issue activity, records, evidence, status roles, review artifacts, and record links; local runtime diagnostics excluded
Ordering:       newest first, timestamp then record/path
Limit:          5
Filters:        (none)
Showing:        5 of 379 matching events

Events
------
  2026-06-21 16:20 -04:00 | evidence_attached | (system) | issue/atelier-fyc9 | Normalize mission work and blocker relationships under issue commands | Attached evidence atelier-zpit to issue/atelier-fyc9 (validates)
  2026-06-21 16:20 -04:00 | evidence_attached | (system) | issue/atelier-2kfb | Delete mission root namespace and mission command shims | Attached evidence atelier-wrnt to issue/atelier-2kfb (validates)
  2026-06-21 16:20 -04:00 | evidence_attached | (system) | issue/atelier-76j0 | Run mission rework scenario validation | Attached evidence atelier-vxxj to issue/atelier-76j0 (validates)
  2026-06-21 16:20 -04:
```

## Stderr

Bytes: 1605
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
   Compiling atelier-records v0.2.0 (/root/atelier/crates/atelier-records)
   Compiling atelier-sqlite v0.2.0 (/root/atelier/crates/atelier-sqlite)
   Compiling atelier-app v0.2.0 (/root/atelier/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.41s
────────────
 Nextest run ID 303c5dac-b085-4c5d-a63d-399573b3f039 with nextest profile: default
    Starting 7 tests across 4 binaries (439 tests skipped)
        PASS [   0.009s] (1/7) atelier-cli commands::workflow::tests::default_validators_are_target_and_transition_aware
        PASS [   1.009s] (2/7) atelier-cli::cli_integration mission_projection_worktree::test_mission_terminal_status_and_options_use_configured_objective_validators
        PASS [   1.859s] (3/7) atelier-cli::cli_integration records_evidence::test_issue_closeout_uses_attached_pass_evidence_not_evidence_text
        PASS [   2.312s] (4/7) atelier-cli::cli_integration issues::test_issue_show_surfaces_evidence_status
        PASS [   2.561s] (5/7) atelier-cli::cli_integration records_evidence::test_evidence_capture_records_command_metadata_and_attaches_targets
        PASS [   6.083s] (6/7) atelier-cli::cli_integration records_evidence::test_mission_status_reports_terminal_checks_and_explicit_approval
        PASS [   6.276s] (7/7) atelier-cli::cli_integration mission_projection_worktree::test_mission_list_human_overview_orders_and_summarizes
────────────
     Summary [   6.277s] 7 tests run: 7 passed, 439 skipped
```

