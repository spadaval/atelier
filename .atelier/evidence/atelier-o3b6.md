---
created_at: "2026-06-16T17:50:51.512081268+00:00"
id: "atelier-o3b6"
evidence_type: "validation"
captured_at: "2026-06-16T17:50:40.041009702+00:00"
command: "bash -lc 'target/debug/atelier --help | tee /tmp/atelier-m1r7-root-help.txt; ! grep -E \"^  export\\b|^  rebuild\\b|atelier export|atelier rebuild\" /tmp/atelier-m1r7-root-help.txt; for role in worker reviewer manager admin; do target/debug/atelier man \"$role\"; done | tee /tmp/atelier-m1r7-role-help.txt; ! grep -E \"atelier export|atelier rebuild\" /tmp/atelier-m1r7-role-help.txt'"
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
title: "root and role help keep export rebuild out of normal workflow"
updated_at: "2026-06-16T17:50:55.745733055+00:00"
---

## Summary

root and role help keep export rebuild out of normal workflow

## Command

```console
bash -lc 'target/debug/atelier --help | tee /tmp/atelier-m1r7-root-help.txt; ! grep -E "^  export\b|^  rebuild\b|atelier export|atelier rebuild" /tmp/atelier-m1r7-root-help.txt; for role in worker reviewer manager admin; do target/debug/atelier man "$role"; done | tee /tmp/atelier-m1r7-role-help.txt; ! grep -E "atelier export|atelier rebuild" /tmp/atelier-m1r7-role-help.txt'
```

Exit status: 0

## Stdout

Bytes: 5779
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
```

## Stderr

Bytes: 122
Truncated: no

```text
2026-06-16T17:50:47.034805Z  WARN Projection index was stale; rebuilt local SQLite projection from /root/atelier/.atelier
```
