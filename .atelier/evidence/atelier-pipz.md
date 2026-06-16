---
created_at: "2026-06-16T17:50:40.435063563+00:00"
id: "atelier-pipz"
evidence_type: "validation"
captured_at: "2026-06-16T17:50:40.107339003+00:00"
command: "bash -lc 'target/debug/atelier --help; for cmd in rebuild workflow diagnostics import-beads maintenance branch worktree; do echo \"### $cmd\"; target/debug/atelier $cmd --help 2>&1 | sed -n \"1,80p\"; done; rg -n \"Audited low-level surfaces|`rebuild`|`workflow check`|`diagnostics slow`|`import-beads`|`maintenance delete`|`branch`|`worktree`\" docs/product/command-audit/category-review.md'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-m1r7"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 11644
    summary: "Mission and proof oriented work coordination for agents\n\nUsage: atelier [OPTIONS] <COMMAND>\n\n\n\nSetup:\n  init          Initialize Atelier in the current repository\n\nOrientation:\n  man           Show role-specific operating guidance\n  status        Show checkout, mission, work, and tracker signposts\n  start         Start tracked work on an issue\n\nIssues:\n  issue         Create, list, show, update, close, and manage blockers\n  search        Search issue text\n  graph         Inspect mission and issue hierarchy and impact\n\nMissions and planning:\n  mission       Create, list, show, status, close, and update durable missions\n  plan          Create, apply, revise, list, and link durable plans\n\nRecords:\n  evidence      Capture validation evidence\n  history       Inspect canonical repo, mission, issue, or epic activity\n\nAdvanced work:\n  worktree      Create, inspect, merge, and remove mission or issue worktrees\n  branch        Inspect and repair epic review branches\n\nMaintenance:\n  maintenance   Run explicit destructive maintenance commands\n  lint          Validate tracker records\n  doctor        Check runtime and derived-state health; use --fix for local repair\n\nCommon commands:\n  atelier man\n  atelier man worker\n  atelier man reviewer\n  atelier man manager\n  atelier man admin\n  atelier status\n  atelier issue list\n  atelier issue list --ready\n  atelier issue list --blocked\n  atelier issue show <id>\n  atelier issue block <blocked-id> <blocker-id>\n  atelier issue unblock <blocked-id> <blocker-id>\n  atelier issue blocked [<id>]\n  atelier mission list\n  atelier mission show <id>\n  atelier mission status\n  atelier mission close <id> --reason \"...\"\n  atelier history --mission <id>\n  atelier history --issue <id>\n  atelier start <issue-id>\n  atelier issue transition <issue-id> --options\n  atelier issue close <issue-id> --reason \"...\"\n  atelier doctor\n  atelier doctor --fix\n  atelier help <command>\n\nOptions:\n  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)\n      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]\n      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]\n  -h, --help                     Print help\n  -V, --version                  Print version\n### rebuild\nAdvanced projection diagnostic; normal local repair uses doctor --fix\n\nUsage: atelier rebuild [OPTIONS]\n\nOptions:\n  -i, --input <INPUT>            Canonical state directory to rebuild from\n  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)\n      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]\n      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]\n  -h, --help                     Print help\n### workflow\nAdvanced/debug workflow policy diagnostics\n\nUsage: atelier workflow [OPTIONS] <COMMAND>\n\nCommands:\n  check  Run raw workflow-policy diagnostics; normal operator checks use lint and status surfaces\n  help   Print this message or the help of the given subcommand(s)\n\nOptions:\n  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)\n      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]\n      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]\n  -h, --help                     Print help\n### diagnostics\nAdvanced local command diagnostics; JSON is local-only telemetry, not workflow state\n\nUsage: atelier diagnostics [OPTIONS] <COMMAND>\n\nCommands:\n  slow  Summarize slow command telemetry as stable local-only JSON for performance analysis\n  help  Print this message or the help of the given subcommand(s)\n\nOptions:\n  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)\n      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default:"
    truncated: true
  stderr:
    bytes: 301
    summary: "bash: line 1: rebuild: command not found\nbash: line 1: workflow: command not found\nbash: line 1: diagnostics: command not found\nbash: line 1: import-beads: command not found\nbash: line 1: maintenance: command not found\nbash: line 1: branch: command not found\nbash: line 1: worktree: command not found\n"
    truncated: false
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
status: "pass"
title: "adjacent command families have audited classifications"
updated_at: "2026-06-16T17:50:44.060013981+00:00"
---

adjacent command families have audited classifications

Command: bash -lc 'target/debug/atelier --help; for cmd in rebuild workflow diagnostics import-beads maintenance branch worktree; do echo "### $cmd"; target/debug/atelier $cmd --help 2>&1 | sed -n "1,80p"; done; rg -n "Audited low-level surfaces|`rebuild`|`workflow check`|`diagnostics slow`|`import-beads`|`maintenance delete`|`branch`|`worktree`" docs/product/command-audit/category-review.md'
Exit status: 0

Stdout summary (truncated):
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

Stderr summary:
bash: line 1: rebuild: command not found
bash: line 1: workflow: command not found
bash: line 1: diagnostics: command not found
bash: line 1: import-beads: command not found
bash: line 1: maintenance: command not found
bash: line 1: branch: command not found
bash: line 1: worktree: command not found

