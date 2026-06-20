---
created_at: "2026-06-20T21:04:03.642855651+00:00"
id: "atelier-3125"
evidence_type: "validation"
captured_at: "2026-06-20T21:04:02.674794185+00:00"
command: "bash -lc '\nset -euo pipefail\nbin=\"/root/atelier/target/debug/atelier\"\nif \"$bin\" mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then\n  echo \"mission namespace unexpectedly succeeded\"\n  exit 1\nfi\ntmp=$(mktemp -d)\ntrap \"rm -rf \\\"$tmp\\\"\" EXIT\ncd \"$tmp\"\n\"$bin\" init >/dev/null\ngit init --initial-branch=main >/dev/null\ngit config user.email smoke@example.com\ngit config user.name \"Smoke Test\"\ngit add .\ngit commit -m \"initial tracker state\" >/dev/null\nmission_id=$(\"$bin\" issue create \"Objective\" --issue-type mission --body \"Body text\" --constraint \"Constraint one\" --risk \"Risk one\" --validation \"Validate one\" --quiet)\nissue_id=$(\"$bin\" issue create \"Task\" --description \"Task body\" --quiet)\n\"$bin\" issue link \"$mission_id\" \"$issue_id\" >/dev/null\nstatus_out=$(\"$bin\" issue status \"$mission_id\" --verbose)\ncase \"$status_out\" in *\"$issue_id\"*) ;; *) echo \"$status_out\"; exit 1 ;; esac\n\"$bin\" issue note \"$mission_id\" \"replacement note\" >/dev/null\nshow_out=$(\"$bin\" issue show \"$mission_id\")\ncase \"$show_out\" in *\"replacement note\"*) ;; *) echo \"$show_out\"; exit 1 ;; esac\necho \"mission root rejected; issue objective create/link/status/note/show replacements passed\"\n'"
exit_status: "1"
target:
  kind: "issue"
  id: "atelier-v2o6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-v2o6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc '\nset -euo pipefail\nbin=\"/root/atelier/target/debug/atelier\"\nif \"$bin\" mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then\n  echo \"mission namespace unexpectedly succeeded\"\n  exit 1\nfi\ntmp=$(mktemp -d)\ntrap \"rm -rf \\\"$tmp\\\"\" EXIT\ncd \"$tmp\"\n\"$bin\" init >/dev/null\ngit init --initial-branch=main >/dev/null\ngit config user.email smoke@example.com\ngit config user.name \"Smoke Test\"\ngit add .\ngit commit -m \"initial tracker state\" >/dev/null\nmission_id=$(\"$bin\" issue create \"Objective\" --issue-type mission --body \"Body text\" --constraint \"Constraint one\" --risk \"Risk one\" --validation \"Validate one\" --quiet)\nissue_id=$(\"$bin\" issue create \"Task\" --description \"Task body\" --quiet)\n\"$bin\" issue link \"$mission_id\" \"$issue_id\" >/dev/null\nstatus_out=$(\"$bin\" issue status \"$mission_id\" --verbose)\ncase \"$status_out\" in *\"$issue_id\"*) ;; *) echo \"$status_out\"; exit 1 ;; esac\n\"$bin\" issue note \"$mission_id\" \"replacement note\" >/dev/null\nshow_out=$(\"$bin\" issue show \"$mission_id\")\ncase \"$show_out\" in *\"replacement note\"*) ;; *) echo \"$show_out\"; exit 1 ;; esac\necho \"mission root rejected; issue objective create/link/status/note/show replacements passed\"\n'"
updated_at: "2026-06-20T21:04:06.421985420+00:00"
---

## Summary

bash -lc '
set -euo pipefail
bin="/root/atelier/target/debug/atelier"
if "$bin" mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then
  echo "mission namespace unexpectedly succeeded"
  exit 1
fi
tmp=$(mktemp -d)
trap "rm -rf \"$tmp\"" EXIT
cd "$tmp"
"$bin" init >/dev/null
git init --initial-branch=main >/dev/null
git config user.email smoke@example.com
git config user.name "Smoke Test"
git add .
git commit -m "initial tracker state" >/dev/null
mission_id=$("$bin" issue create "Objective" --issue-type mission --body "Body text" --constraint "Constraint one" --risk "Risk one" --validation "Validate one" --quiet)
issue_id=$("$bin" issue create "Task" --description "Task body" --quiet)
"$bin" issue link "$mission_id" "$issue_id" >/dev/null
status_out=$("$bin" issue status "$mission_id" --verbose)
case "$status_out" in *"$issue_id"*) ;; *) echo "$status_out"; exit 1 ;; esac
"$bin" issue note "$mission_id" "replacement note" >/dev/null
show_out=$("$bin" issue show "$mission_id")
case "$show_out" in *"replacement note"*) ;; *) echo "$show_out"; exit 1 ;; esac
echo "mission root rejected; issue objective create/link/status/note/show replacements passed"
'

## Command

```console
bash -lc '
set -euo pipefail
bin="/root/atelier/target/debug/atelier"
if "$bin" mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then
  echo "mission namespace unexpectedly succeeded"
  exit 1
fi
tmp=$(mktemp -d)
trap "rm -rf \"$tmp\"" EXIT
cd "$tmp"
"$bin" init >/dev/null
git init --initial-branch=main >/dev/null
git config user.email smoke@example.com
git config user.name "Smoke Test"
git add .
git commit -m "initial tracker state" >/dev/null
mission_id=$("$bin" issue create "Objective" --issue-type mission --body "Body text" --constraint "Constraint one" --risk "Risk one" --validation "Validate one" --quiet)
issue_id=$("$bin" issue create "Task" --description "Task body" --quiet)
"$bin" issue link "$mission_id" "$issue_id" >/dev/null
status_out=$("$bin" issue status "$mission_id" --verbose)
case "$status_out" in *"$issue_id"*) ;; *) echo "$status_out"; exit 1 ;; esac
"$bin" issue note "$mission_id" "replacement note" >/dev/null
show_out=$("$bin" issue show "$mission_id")
case "$show_out" in *"replacement note"*) ;; *) echo "$show_out"; exit 1 ;; esac
echo "mission root rejected; issue objective create/link/status/note/show replacements passed"
'
```

Exit status: 1

## Stdout

Bytes: 952
Truncated: no

```text
Mission atelier-53eg [ready] - Objective
========================================
Status:   ready
Created:  2026-06-20 17:04 -04:00
Updated:  2026-06-20 17:04 -04:00

Intent
------
Body text

Constraints
-----------
- Constraint one

Risks
-----
- Risk one

Validation
----------
- Validate one

Progress
--------
Records: evidence=0
Work: ready=1 blocked=0 done=0 backlog=0
Mission Blockers: 0

Evidence
--------
(none)

Mission Blockers
----------------
(none)

Linked Work
-----------
Ready (1)
  atelier-pgxh [todo] medium task - Task (advances)

Supporting Records
------------------
(none)

Evidence Gaps
-------------
  No evidence records are linked to this mission.

Next Commands
-------------
  atelier issue status atelier-53eg
  atelier issue show atelier-53eg
  atelier issue note atelier-53eg "..."
  atelier history --mission atelier-53eg
  atelier issue link atelier-53eg <issue-id> --role advances
  atelier issue status atelier-53eg
```

## Stderr

Bytes: 37
Truncated: no

```text
Error: Lint failed with 1 finding(s)
```

