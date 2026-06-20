---
created_at: "2026-06-20T21:04:25.273354511+00:00"
id: "atelier-i0cf"
evidence_type: "validation"
captured_at: "2026-06-20T21:04:24.283179059+00:00"
command: "bash -lc '\nset -euo pipefail\nbin=\"/root/atelier/target/debug/atelier\"\nif \"$bin\" mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then\n  echo \"mission namespace unexpectedly succeeded\"\n  exit 1\nfi\ntmp=$(mktemp -d)\ntrap \"rm -rf \\\"$tmp\\\"\" EXIT\ncd \"$tmp\"\n\"$bin\" init >/dev/null\ngit init --initial-branch=main >/dev/null\ngit config user.email smoke@example.com\ngit config user.name \"Smoke Test\"\ngit add .\ngit commit -m \"initial tracker state\" >/dev/null\nmission_id=$(\"$bin\" issue create \"Objective\" --issue-type mission --body \"Body text\" --constraint \"Constraint one\" --risk \"Risk one\" --validation \"Validate one\" --quiet)\nissue_id=$(\"$bin\" issue create \"Task\" --description \"Task body\" --quiet)\n\"$bin\" evidence record --target \"mission/$mission_id\" --kind validation \"replacement status proof\" >/dev/null\n\"$bin\" issue link \"$mission_id\" \"$issue_id\" >/dev/null\nstatus_out=$(\"$bin\" issue status \"$mission_id\" --verbose)\ncase \"$status_out\" in *\"$issue_id\"*) ;; *) echo \"$status_out\"; exit 1 ;; esac\n\"$bin\" issue note \"$mission_id\" \"replacement note\" >/dev/null\nshow_out=$(\"$bin\" issue show \"$mission_id\")\ncase \"$show_out\" in *\"replacement note\"*) ;; *) echo \"$show_out\"; exit 1 ;; esac\necho \"mission root rejected; issue objective create/link/status/note/show replacements passed\"\n'"
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
title: "bash -lc '\nset -euo pipefail\nbin=\"/root/atelier/target/debug/atelier\"\nif \"$bin\" mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then\n  echo \"mission namespace unexpectedly succeeded\"\n  exit 1\nfi\ntmp=$(mktemp -d)\ntrap \"rm -rf \\\"$tmp\\\"\" EXIT\ncd \"$tmp\"\n\"$bin\" init >/dev/null\ngit init --initial-branch=main >/dev/null\ngit config user.email smoke@example.com\ngit config user.name \"Smoke Test\"\ngit add .\ngit commit -m \"initial tracker state\" >/dev/null\nmission_id=$(\"$bin\" issue create \"Objective\" --issue-type mission --body \"Body text\" --constraint \"Constraint one\" --risk \"Risk one\" --validation \"Validate one\" --quiet)\nissue_id=$(\"$bin\" issue create \"Task\" --description \"Task body\" --quiet)\n\"$bin\" evidence record --target \"mission/$mission_id\" --kind validation \"replacement status proof\" >/dev/null\n\"$bin\" issue link \"$mission_id\" \"$issue_id\" >/dev/null\nstatus_out=$(\"$bin\" issue status \"$mission_id\" --verbose)\ncase \"$status_out\" in *\"$issue_id\"*) ;; *) echo \"$status_out\"; exit 1 ;; esac\n\"$bin\" issue note \"$mission_id\" \"replacement note\" >/dev/null\nshow_out=$(\"$bin\" issue show \"$mission_id\")\ncase \"$show_out\" in *\"replacement note\"*) ;; *) echo \"$show_out\"; exit 1 ;; esac\necho \"mission root rejected; issue objective create/link/status/note/show replacements passed\"\n'"
updated_at: "2026-06-20T21:04:28.073806401+00:00"
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
"$bin" evidence record --target "mission/$mission_id" --kind validation "replacement status proof" >/dev/null
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
"$bin" evidence record --target "mission/$mission_id" --kind validation "replacement status proof" >/dev/null
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

Bytes: 955
Truncated: no

```text
Mission atelier-j56q [ready] - Objective
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
Records: evidence=1
Work: ready=1 blocked=0 done=0 backlog=0
Mission Blockers: 0

Evidence
--------
  atelier-biez [recorded] - replacement status proof

Mission Blockers
----------------
(none)

Linked Work
-----------
Ready (1)
  atelier-11vl [todo] medium task - Task (advances)

Supporting Records
------------------
(none)

Evidence Gaps
-------------
(none)

Next Commands
-------------
  atelier issue status atelier-j56q
  atelier issue show atelier-j56q
  atelier issue note atelier-j56q "..."
  atelier history --mission atelier-j56q
  atelier issue link atelier-j56q <issue-id> --role advances
  atelier issue status atelier-j56q
```

## Stderr

Bytes: 37
Truncated: no

```text
Error: Lint failed with 1 finding(s)
```

