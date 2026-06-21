---
created_at: "2026-06-21T20:05:33.899653975+00:00"
id: "atelier-jhf6"
evidence_type: "validation"
captured_at: "2026-06-21T20:05:29.163742560+00:00"
command: "sh -c 'set -eu\nprintf \"== root help ==\\n\"\ntarget/debug/atelier --help | sed -n \"1,80p\"\nprintf \"== issue help ==\\n\"\ntarget/debug/atelier issue --help | sed -n \"1,120p\"\nprintf \"== mission objective status excerpt ==\\n\"\ntarget/debug/atelier issue status atelier-53bu --verbose | sed -n \"1,120p\"\nprintf \"== targeted stale mission command search ==\\n\"\nrg -n \"atelier mission|mission (start|status|list|view|close)|mission command\" AGENTS.md .agents/skills/agent-factory docs -g \"*.md\" || true\nprintf \"== lint ==\\n\"\ntarget/debug/atelier lint'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-y3fj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-y3fj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Docs and Agent Factory guidance validation after mission rework"
updated_at: "2026-06-21T20:05:39.041620663+00:00"
---

## Summary

Docs and Agent Factory guidance validation after mission rework

## Command

```console
sh -c 'set -eu
printf "== root help ==\n"
target/debug/atelier --help | sed -n "1,80p"
printf "== issue help ==\n"
target/debug/atelier issue --help | sed -n "1,120p"
printf "== mission objective status excerpt ==\n"
target/debug/atelier issue status atelier-53bu --verbose | sed -n "1,120p"
printf "== targeted stale mission command search ==\n"
rg -n "atelier mission|mission (start|status|list|view|close)|mission command" AGENTS.md .agents/skills/agent-factory docs -g "*.md" || true
printf "== lint ==\n"
target/debug/atelier lint'
```

Exit status: 0

## Stdout

Bytes: 12809
Truncated: yes

```text
== root help ==
Mission and proof oriented work coordination for agents

Usage: atelier [OPTIONS] <COMMAND>



Setup:
  init          Initialize Atelier in the current repository

Orientation:
  man           Show role-specific operating guidance
  status        Show checkout, mission, work, and tracker signposts

Issues:
  issue         Create, list, show, update, close, and manage blockers
  search        Search issue text

Planning:
  bundle        Preview and apply one-shot graph bundle files

Records:
  evidence      Capture validation evidence
  review        Manage configured review artifacts
  forgejo       Configure and verify Forgejo integration
  history       Inspect canonical repo, mission, issue, or epic activity

Advanced work:
  branch        Inspect and repair epic review branches

Maintenance:
  prune         Prune accumulated local artifacts safely
  maintenance   Run explicit destructive maintenance commands
  lint          Validate tracker records
  doctor        Check runtime and derived-state health; use --fix for local repair

Common commands:
  atelier man
  atelier man worker
  atelier man reviewer
  atelier man validator
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
  atelier issue create "..." --issue-type mission
  atelier issue show <mission-id>
  atelier issue table --kind mission
  atelier issue transition <mission-id> close --reason "..."
  atelier bundle preview <file>
  atelier bundle apply <file> --yes
  atelier forgejo roles check
  atelier history --mission <id>
  atelier history --issue <id>
  atelier issue transition <issue-id> --options
  atelier issue transition <issue-id> start
  atelier issue transition <issue-id> close --reason "..."
  atelier prune
  atelier prune --apply
  atelier help <command>

Options:
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]
  -h, --help                     Print help
  -V, --version                  Print version
== issue help ==
Issue lifecycle commands (create, show, list, transition, ...)

Usage: atelier issue [OPTIONS] <COMMAND>

Commands:
  create      Create a new issue
  list        List issues
  table       Show a homogeneous objective inventory table
  show        Show issue details
  status      Show type-aware issue status for objective records
  transition  Show issue transition options and blockers
  update      Update an issue
  note        Add an activity note to an issue
  link        Add a typed link from one issue to another
  unlink      Remove a typed link from one issue to another
  block       Mark an issue as blocked by another
  unblock     Remove a blocking relationship
  blocked     List blocked issues, or show blockers for one issue
  help        Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]
  -h, --help                     Print help
== mission objective status excerpt ==
Mission Status atelier-53bu [ready] - Make workflow obligations explicit and minimal
====================================================================================
Health:   ready
Tracker:  ok
Terminal: blocked

Work
----
Total: 1 ready, 20 done, 2 backlog
  [epic] atelier-f9ci [in_progress] high - Epic: Validate mission rework end to end | 1 ready, 1 backlog
  [epic] atelier-nbhp [done] high - Ep
```

## Stderr

Bytes: 0
Truncated: no

```text
```

