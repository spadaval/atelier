---
created_at: "2026-06-21T18:53:57.382217525+00:00"
id: "atelier-00d1"
evidence_type: "validation"
captured_at: "2026-06-21T18:53:36.956057181+00:00"
command: "bash -lc 'set -euo pipefail\ntarget/debug/atelier issue show atelier-53bu | rg \"Mission atelier-53bu|File:     .*/.atelier/missions/atelier-53bu.md|Intent|Constraints|Risks|Validation|Linked Work|Evidence Gaps|atelier history --mission atelier-53bu\"\ntarget/debug/atelier issue status atelier-53bu | rg \"Mission Status atelier-53bu|Selectable Work|Blocked Work|Reliability|Terminal Checks|Next Commands\"\ntarget/debug/atelier issue status --verbose atelier-53bu | rg \"Advanced Validator Detail|Branch Policy|Active Work\"\ntarget/debug/atelier --quiet issue status atelier-53bu | rg \"atelier-53bu health=.*tracker=ok\"\ntarget/debug/atelier issue status atelier-vays | rg \"Issue Status atelier-vays|Ready Work|Blocked Work|Terminal Checks\"\ncargo nextest run -p atelier-cli test_issue_create_mission_type_writes_mission_sections_and_issue_show_reads_them test_mission_status_cli_reports_control_state test_issue_status_renders_objective_work_health\n'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-62po"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-62po"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Mission objective show/status surfaces expose rich show, compact status, verbose detail, and quiet mode"
updated_at: "2026-06-21T18:54:02.388937759+00:00"
---

## Summary

Mission objective show/status surfaces expose rich show, compact status, verbose detail, and quiet mode

## Command

```console
bash -lc 'set -euo pipefail
target/debug/atelier issue show atelier-53bu | rg "Mission atelier-53bu|File:     .*/.atelier/missions/atelier-53bu.md|Intent|Constraints|Risks|Validation|Linked Work|Evidence Gaps|atelier history --mission atelier-53bu"
target/debug/atelier issue status atelier-53bu | rg "Mission Status atelier-53bu|Selectable Work|Blocked Work|Reliability|Terminal Checks|Next Commands"
target/debug/atelier issue status --verbose atelier-53bu | rg "Advanced Validator Detail|Branch Policy|Active Work"
target/debug/atelier --quiet issue status atelier-53bu | rg "atelier-53bu health=.*tracker=ok"
target/debug/atelier issue status atelier-vays | rg "Issue Status atelier-vays|Ready Work|Blocked Work|Terminal Checks"
cargo nextest run -p atelier-cli test_issue_create_mission_type_writes_mission_sections_and_issue_show_reads_them test_mission_status_cli_reports_control_state test_issue_status_renders_objective_work_health
'
```

Exit status: 0

## Stdout

Bytes: 721
Truncated: no

```text
Mission atelier-53bu [ready] - Make workflow obligations explicit and minimal
File:     /root/atelier-vays/.atelier/missions/atelier-53bu.md
Intent
Constraints
Risks
Validation
Linked Work
Evidence Gaps
  atelier history --mission atelier-53bu
Mission Status atelier-53bu [ready] - Make workflow obligations explicit and minimal
Selectable Work
Blocked Work
Reliability
Terminal Checks
Next Commands
Advanced Validator Detail
Branch Policy
Active Work
atelier-53bu health=blocked ready=6 blocked=12 done=4 backlog=1 blockers=0 evidence_gaps=19 validator_failures=5 tracker=ok terminal_ready=no
Issue Status atelier-vays - Epic: Collapse mission command surface into issue commands
Ready Work
Blocked Work
Terminal Checks
```

## Stderr

Bytes: 829
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-vays/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.93s
────────────
 Nextest run ID 86e6a38e-11e2-4749-88bc-ecfbfbef0bab with nextest profile: default
    Starting 3 tests across 4 binaries (439 tests skipped)
        PASS [   0.556s] (1/3) atelier-cli::cli_integration issues::test_issue_create_mission_type_writes_mission_sections_and_issue_show_reads_them
        PASS [   1.255s] (2/3) atelier-cli::cli_integration setup_guidance::test_issue_status_renders_objective_work_health
        PASS [   4.556s] (3/3) atelier-cli::cli_integration mission_projection_worktree::test_mission_status_cli_reports_control_state
────────────
     Summary [   4.557s] 3 tests run: 3 passed, 439 skipped
```

