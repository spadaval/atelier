---
created_at: "2026-06-13T23:07:17.800352002+00:00"
id: "atelier-796u"
data: "{\"agent_identity\":null,\"captured_at\":\"2026-06-13T23:07:17.800282239+00:00\",\"follow_up_ids\":[],\"independence_level\":\"unspecified\",\"kind\":\"validation\",\"path\":null,\"producer\":null,\"proof_scope\":\"scoped to the attached target or summary\",\"residual_risks\":[],\"result\":\"pass\",\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-u08r"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "CLI surface audit: A simple, lean issue tracker CLI\n\nUsage: atelier [OPTIONS] <COMMAND>\n\n\n\nSetup:\n  init          Initialize Atelier in the current repository\n\nOrientation:\n  prime         Show repository operating guidance for recovery and onboarding\n  status        Show checkout, mission, work, and tracker signposts\n  start         Start tracked work on an issue\n  abandon       Clear active local work without changing issue status\n\nIssues:\n  issue         Create, list, show, update, and close issues\n  dep           Manage issue blockers with add, remove, and list\n  search        Search issue text\n  link          Manage typed issue links\n  graph         Inspect issue hierarchy and impact\n  note          Add issue activity notes\n\nMissions and planning:\n  mission       Create, list, show, status, and update durable missions\n  plan          Create, apply, revise, list, and link durable plans\n\nRecords:\n  evidence      Capture validation evidence\n  history       Inspect canonical repo, mission, issue, or epic activity\n\nAdvanced work:\n  worktree      Create, inspect, merge, and remove issue worktrees\n\nState management:\n  export        Write or check canonical tracker records\n  rebuild       Rebuild local SQLite state from canonical tracker records\n  import-beads  Import an external Beads JSONL backup\n\nIntegrations:\n  integrations  Install optional integrations such as Claude hooks\n\nMaintenance:\n  maintenance   Run explicit destructive maintenance commands\n  diagnostics   Inspect local command diagnostics\n  lint          Validate tracker records\n  doctor        Check runtime and exported-state health\n\nCommon commands:\n  atelier prime\n  atelier status\n  atelier issue list\n  atelier issue list --ready\n  atelier issue show <id>\n  atelier mission list\n  atelier mission show <id>\n  atelier mission status\n  atelier history --mission <id>\n  atelier history --issue <id>\n  atelier start <issue-id>\n  atelier abandon [issue-id] --reason \"...\"\n  atelier issue transition <issue-id> --options\n  atelier issue close <issue-id> --reason \"...\"\n  atelier doctor\n  atelier help <command>\n\nOptions:\n  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)\n      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]\n      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]\n  -h, --help                     Print help\n  -V, --version                  Print version omits removed roots like work; First-class evidence records\n\nUsage: atelier evidence [OPTIONS] <COMMAND>\n\nCommands:\n  record   Record proof manually or by capturing a command transcript\n  add      Add validation evidence\n  capture  Capture a command transcript as validation evidence\n  show     Show an evidence record\n  attach   Attach evidence to a target record\n  list     List evidence records\n  help     Print this message or the help of the given subcommand(s)\n\nOptions:\n  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)\n      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]\n      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]\n  -h, --help                     Print help shows record/show/attach/list only;  and Work Status\n===========\nActive: no\nNext Commands\n-------------\n  atelier issue list --ready\n  atelier start <issue-id> now fail as unknown root commands. Residue search confirms issue_compat_guidance is gone and remaining predecessor references are classified in docs/product/cli-surface.md, hidden command identities, or negative tests only."
updated_at: "2026-06-13T23:07:20.377416620+00:00"
---

CLI surface audit: A simple, lean issue tracker CLI

Usage: atelier [OPTIONS] <COMMAND>



Setup:
  init          Initialize Atelier in the current repository

Orientation:
  prime         Show repository operating guidance for recovery and onboarding
  status        Show checkout, mission, work, and tracker signposts
  start         Start tracked work on an issue
  abandon       Clear active local work without changing issue status

Issues:
  issue         Create, list, show, update, and close issues
  dep           Manage issue blockers with add, remove, and list
  search        Search issue text
  link          Manage typed issue links
  graph         Inspect issue hierarchy and impact
  note          Add issue activity notes

Missions and planning:
  mission       Create, list, show, status, and update durable missions
  plan          Create, apply, revise, list, and link durable plans

Records:
  evidence      Capture validation evidence
  history       Inspect canonical repo, mission, issue, or epic activity

Advanced work:
  worktree      Create, inspect, merge, and remove issue worktrees

State management:
  export        Write or check canonical tracker records
  rebuild       Rebuild local SQLite state from canonical tracker records
  import-beads  Import an external Beads JSONL backup

Integrations:
  integrations  Install optional integrations such as Claude hooks

Maintenance:
  maintenance   Run explicit destructive maintenance commands
  diagnostics   Inspect local command diagnostics
  lint          Validate tracker records
  doctor        Check runtime and exported-state health

Common commands:
  atelier prime
  atelier status
  atelier issue list
  atelier issue list --ready
  atelier issue show <id>
  atelier mission list
  atelier mission show <id>
  atelier mission status
  atelier history --mission <id>
  atelier history --issue <id>
  atelier start <issue-id>
  atelier abandon [issue-id] --reason "..."
  atelier issue transition <issue-id> --options
  atelier issue close <issue-id> --reason "..."
  atelier doctor
  atelier help <command>

Options:
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]
  -h, --help                     Print help
  -V, --version                  Print version omits removed roots like work; First-class evidence records

Usage: atelier evidence [OPTIONS] <COMMAND>

Commands:
  record   Record proof manually or by capturing a command transcript
  add      Add validation evidence
  capture  Capture a command transcript as validation evidence
  show     Show an evidence record
  attach   Attach evidence to a target record
  list     List evidence records
  help     Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]
  -h, --help                     Print help shows record/show/attach/list only;  and Work Status
===========
Active: no
Next Commands
-------------
  atelier issue list --ready
  atelier start <issue-id> now fail as unknown root commands. Residue search confirms issue_compat_guidance is gone and remaining predecessor references are classified in docs/product/cli-surface.md, hidden command identities, or negative tests only.
