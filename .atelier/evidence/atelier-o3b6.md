---
created_at: "2026-06-16T17:50:51.512081268+00:00"
id: "atelier-o3b6"
evidence_type: "validation"
captured_at: "2026-06-16T17:50:40.041009702+00:00"
command: "bash -lc 'target/debug/atelier --help | tee /tmp/atelier-m1r7-root-help.txt; ! grep -E \"^  export\\b|^  rebuild\\b|atelier export|atelier rebuild\" /tmp/atelier-m1r7-root-help.txt; for role in worker reviewer manager admin; do target/debug/atelier man \"$role\"; done | tee /tmp/atelier-m1r7-role-help.txt; ! grep -E \"atelier export|atelier rebuild\" /tmp/atelier-m1r7-role-help.txt'"
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
    bytes: 5779
    summary: "Mission and proof oriented work coordination for agents\n\nUsage: atelier [OPTIONS] <COMMAND>\n\n\n\nSetup:\n  init          Initialize Atelier in the current repository\n\nOrientation:\n  man           Show role-specific operating guidance\n  status        Show checkout, mission, work, and tracker signposts\n  start         Start tracked work on an issue\n\nIssues:\n  issue         Create, list, show, update, close, and manage blockers\n  search        Search issue text\n  graph         Inspect mission and issue hierarchy and impact\n\nMissions and planning:\n  mission       Create, list, show, status, close, and update durable missions\n  plan          Create, apply, revise, list, and link durable plans\n\nRecords:\n  evidence      Capture validation evidence\n  history       Inspect canonical repo, mission, issue, or epic activity\n\nAdvanced work:\n  worktree      Create, inspect, merge, and remove mission or issue worktrees\n  branch        Inspect and repair epic review branches\n\nMaintenance:\n  maintenance   Run explicit destructive maintenance commands\n  lint          Validate tracker records\n  doctor        Check runtime and derived-state health; use --fix for local repair\n\nCommon commands:\n  atelier man\n  atelier man worker\n  atelier man reviewer\n  atelier man manager\n  atelier man admin\n  atelier status\n  atelier issue list\n  atelier issue list --ready\n  atelier issue list --blocked\n  atelier issue show <id>\n  atelier issue block <blocked-id> <blocker-id>\n  atelier issue unblock <blocked-id> <blocker-id>\n  atelier issue blocked [<id>]\n  atelier mission list\n  atelier mission show <id>\n  atelier mission status\n  atelier mission close <id> --reason \"...\"\n  atelier history --mission <id>\n  atelier history --issue <id>\n  atelier start <issue-id>\n  atelier issue transition <issue-id> --options\n  atelier issue close <issue-id> --reason \"...\"\n  atelier doctor\n  atelier doctor --fix\n  atelier help <command>\n\nOptions:\n  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)\n      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]\n      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]\n  -h, --help                     Print help\n  -V, --version                  Print version\nAtelier Man: Worker\n===================\n\nCurrent State\n-------------\n  Repository: /root/atelier\n  Tracker:    current\n  Active mission: none\n  Current work:   1 issue(s)\n    atelier-m1r7 - Validate cleaned command surface and recovery paths\n  Ready work:     6\n\nMost Relevant Commands\n----------------------\n  1. atelier status - Review the checkout's current-work set.\n  2. atelier evidence record --target issue/<id> --kind test --result pass -- <command> - Attach proof.\n  3. atelier issue transition <id> --options - Inspect allowed next workflow steps.\n\nNormal Loop\n-----------\n  atelier status\n  atelier issue list --ready\n  atelier issue show <id>\n  atelier start <id>\n  atelier evidence record --target issue/<id> --kind test --result pass -- <command>\n  atelier issue close <id> --reason \"...\"\n\nNot Usually For This Role\n-------------------------\n  maintenance, diagnostics, raw workflow checks, bulk plan apply, branch merge\nAtelier Man: Reviewer\n=====================\n\nCurrent State\n-------------\n  Repository: /root/atelier\n  Tracker:    current\n  Active mission: none\n  Current work:   1 issue(s)\n    atelier-m1r7 - Validate cleaned command surface and recovery paths\n  Ready work:     6\n\nMost Relevant Commands\n----------------------\n  1. atelier issue transition <id> --options - Inspect workflow gates.\n  2. atelier evidence show <evidence-id> - Inspect attached proof.\n  3. atelier lint <id> - Validate focused tracker state.\n\nNormal Loop\n-----------\n  atelier mission status\n  atelier issue show <id>\n  atelier issue transition <id> --options\n  atelier evidence record --target issue/<id> --kind validation --result pass -- <command>\n  atelier history --issue <id>\n\nNot Usually For This Role\n-------------------------\n  init, maintenance del"
    truncated: true
  stderr:
    bytes: 122
    summary: "2026-06-16T17:50:47.034805Z  WARN Projection index was stale; rebuilt local SQLite projection from /root/atelier/.atelier\n"
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
title: "root and role help keep export rebuild out of normal workflow"
updated_at: "2026-06-16T17:50:55.745733055+00:00"
---

root and role help keep export rebuild out of normal workflow

Command: bash -lc 'target/debug/atelier --help | tee /tmp/atelier-m1r7-root-help.txt; ! grep -E "^  export\b|^  rebuild\b|atelier export|atelier rebuild" /tmp/atelier-m1r7-root-help.txt; for role in worker reviewer manager admin; do target/debug/atelier man "$role"; done | tee /tmp/atelier-m1r7-role-help.txt; ! grep -E "atelier export|atelier rebuild" /tmp/atelier-m1r7-role-help.txt'
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
Atelier Man: Worker
===================

Current State
-------------
  Repository: /root/atelier
  Tracker:    current
  Active mission: none
  Current work:   1 issue(s)
    atelier-m1r7 - Validate cleaned command surface and recovery paths
  Ready work:     6

Most Relevant Commands
----------------------
  1. atelier status - Review the checkout's current-work set.
  2. atelier evidence record --target issue/<id> --kind test --result pass -- <command> - Attach proof.
  3. atelier issue transition <id> --options - Inspect allowed next workflow steps.

Normal Loop
-----------
  atelier status
  atelier issue list --ready
  atelier issue show <id>
  atelier start <id>
  atelier evidence record --target issue/<id> --kind test --result pass -- <command>
  atelier issue close <id> --reason "..."

Not Usually For This Role
-------------------------
  maintenance, diagnostics, raw workflow checks, bulk plan apply, branch merge
Atelier Man: Reviewer
=====================

Current State
-------------
  Repository: /root/atelier
  Tracker:    current
  Active mission: none
  Current work:   1 issue(s)
    atelier-m1r7 - Validate cleaned command surface and recovery paths
  Ready work:     6

Most Relevant Commands
----------------------
  1. atelier issue transition <id> --options - Inspect workflow gates.
  2. atelier evidence show <evidence-id> - Inspect attached proof.
  3. atelier lint <id> - Validate focused tracker state.

Normal Loop
-----------
  atelier mission status
  atelier issue show <id>
  atelier issue transition <id> --options
  atelier evidence record --target issue/<id> --kind validation --result pass -- <command>
  atelier history --issue <id>

Not Usually For This Role
-------------------------
  init, maintenance del

Stderr summary:
2026-06-16T17:50:47.034805Z  WARN Projection index was stale; rebuilt local SQLite projection from /root/atelier/.atelier

