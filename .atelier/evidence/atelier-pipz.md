---
created_at: "2026-06-16T17:50:40.435063563+00:00"
id: "atelier-pipz"
evidence_type: "validation"
captured_at: "2026-06-16T17:50:40.107339003+00:00"
command: "bash -lc 'target/debug/atelier --help; for cmd in rebuild workflow diagnostics import-beads maintenance branch worktree; do echo \"### $cmd\"; target/debug/atelier $cmd --help 2>&1 | sed -n \"1,80p\"; done; rg -n \"Audited low-level surfaces|`rebuild`|`workflow check`|`diagnostics slow`|`import-beads`|`maintenance delete`|`branch`|`worktree`\" docs/product/command-audit/category-review.md'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-m1r7"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-m1r7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "adjacent command families have audited classifications"
updated_at: "2026-06-16T17:50:44.060013981+00:00"
---

## Summary

adjacent command families have audited classifications

## Command

```console
bash -lc 'target/debug/atelier --help; for cmd in rebuild workflow diagnostics import-beads maintenance branch worktree; do echo "### $cmd"; target/debug/atelier $cmd --help 2>&1 | sed -n "1,80p"; done; rg -n "Audited low-level surfaces|`rebuild`|`workflow check`|`diagnostics slow`|`import-beads`|`maintenance delete`|`branch`|`worktree`" docs/product/command-audit/category-review.md'
```

Exit status: 0

## Stdout

Bytes: 11644
Truncated: yes

```text
Mission and proof oriented work coordination for agents

Usage: atelier [OPTIONS] <COMMAND>



Setup:
  init          Initialize Atelier in the current repository

Orientation:
  man           Show role-specific operating guidance
  status        Show checkout, mission, work, and tracker signposts
  start         Start tracked work on an issue

Issues:
  issue         Create, list, show, update, close, and manage blockers
  search        Search issue text
  graph         Inspect mission and issue hierarchy and impact

Missions and planning:
  mission       Create, list, show, status, close, and update durable missions
  plan          Create, apply, revise, list, and link durable plans

Records:
  evidence      Capture validation evidence
  history       Inspect canonical repo, mission, issue, or epic activity

Advanced work:
  worktree      Create, inspect, merge, and remove mission or issue worktrees
  branch        Inspect and repair epic review branches

Maintenance:
  maintenance   Run explicit destructive maintenance commands
  lint          Validate tracker records
  doctor        Check runtime and derived-state health; use --fix for local repair

Common commands:
  atelier man
  atelier man worker
  atelier man reviewer
  atelier man manager
  atelier man admin
  atelier status
  atelier issue list
  atelier issue list --ready
  atelier issue list --blocked
  atelier issue show <id>
  atelier issue block <blocked-id> <blocker-id>
  atelier issue unblock <blocked-id> <blocker-id>
  atelier issue blocked [<id>]
  atelier mission list
  atelier mission show <id>
  atelier mission status
  atelier mission close <id> --reason "..."
  atelier history --mission <id>
  atelier history --issue <id>
  atelier start <issue-id>
  atelier issue transition <issue-id> --options
  atelier issue close <issue-id> --reason "..."
  atelier doctor
  atelier doctor --fix
  atelier help <command>

Options:
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]
  -h, --help                     Print help
  -V, --version                  Print version
### rebuild
Advanced projection diagnostic; normal local repair uses doctor --fix

Usage: atelier rebuild [OPTIONS]

Options:
  -i, --input <INPUT>            Canonical state directory to rebuild from
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]
  -h, --help                     Print help
### workflow
Advanced/debug workflow policy diagnostics

Usage: atelier workflow [OPTIONS] <COMMAND>

Commands:
  check  Run raw workflow-policy diagnostics; normal operator checks use lint and status surfaces
  help   Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]
  -h, --help                     Print help
### diagnostics
Advanced local command diagnostics; JSON is local-only telemetry, not workflow state

Usage: atelier diagnostics [OPTIONS] <COMMAND>

Commands:
  slow  Summarize slow command telemetry as stable local-only JSON for performance analysis
  help  Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default:
```

## Stderr

Bytes: 301
Truncated: no

```text
bash: line 1: rebuild: command not found
bash: line 1: workflow: command not found
bash: line 1: diagnostics: command not found
bash: line 1: import-beads: command not found
bash: line 1: maintenance: command not found
bash: line 1: branch: command not found
bash: line 1: worktree: command not found
```
