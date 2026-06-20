---
created_at: "2026-06-20T21:04:50.051252571+00:00"
id: "atelier-c9st"
evidence_type: "validation"
captured_at: "2026-06-20T21:04:49.091960864+00:00"
command: "bash -lc '\nset -euo pipefail\nbin=\"/root/atelier/target/debug/atelier\"\nif \"$bin\" mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then\n  echo \"mission namespace unexpectedly succeeded\"\n  exit 1\nfi\ntmp=$(mktemp -d)\ntrap \"rm -rf \\\"$tmp\\\"\" EXIT\ncd \"$tmp\"\n\"$bin\" init >/dev/null\ngit init --initial-branch=main >/dev/null\ngit config user.email smoke@example.com\ngit config user.name \"Smoke Test\"\ngit add .\ngit commit -m \"initial tracker state\" >/dev/null\nmission_id=$(\"$bin\" issue create \"Objective\" --issue-type mission --body \"Body text\" --constraint \"Constraint one\" --risk \"Risk one\" --validation \"Validate one\" --quiet)\nissue_id=$(\"$bin\" issue create \"Task\" --description \"Task body\" --quiet)\n\"$bin\" evidence record --target \"mission/$mission_id\" --kind validation \"replacement status proof\" >/dev/null\n\"$bin\" issue link \"$mission_id\" \"$issue_id\" >/dev/null\nstatus_out=$(\"$bin\" issue status \"$mission_id\" --verbose 2>&1 || true)\ncase \"$status_out\" in *\"$issue_id\"*) ;; *) echo \"$status_out\"; exit 1 ;; esac\ncase \"$status_out\" in *\"Linked Work\"*) ;; *) echo \"$status_out\"; exit 1 ;; esac\n\"$bin\" issue note \"$mission_id\" \"replacement note\" >/dev/null\nshow_out=$(\"$bin\" issue show \"$mission_id\")\ncase \"$show_out\" in *\"replacement note\"*) ;; *) echo \"$show_out\"; exit 1 ;; esac\necho \"mission root rejected; issue objective create/link/status/note/show replacements passed\"\n'"
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
title: "bash -lc '\nset -euo pipefail\nbin=\"/root/atelier/target/debug/atelier\"\nif \"$bin\" mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then\n  echo \"mission namespace unexpectedly succeeded\"\n  exit 1\nfi\ntmp=$(mktemp -d)\ntrap \"rm -rf \\\"$tmp\\\"\" EXIT\ncd \"$tmp\"\n\"$bin\" init >/dev/null\ngit init --initial-branch=main >/dev/null\ngit config user.email smoke@example.com\ngit config user.name \"Smoke Test\"\ngit add .\ngit commit -m \"initial tracker state\" >/dev/null\nmission_id=$(\"$bin\" issue create \"Objective\" --issue-type mission --body \"Body text\" --constraint \"Constraint one\" --risk \"Risk one\" --validation \"Validate one\" --quiet)\nissue_id=$(\"$bin\" issue create \"Task\" --description \"Task body\" --quiet)\n\"$bin\" evidence record --target \"mission/$mission_id\" --kind validation \"replacement status proof\" >/dev/null\n\"$bin\" issue link \"$mission_id\" \"$issue_id\" >/dev/null\nstatus_out=$(\"$bin\" issue status \"$mission_id\" --verbose 2>&1 || true)\ncase \"$status_out\" in *\"$issue_id\"*) ;; *) echo \"$status_out\"; exit 1 ;; esac\ncase \"$status_out\" in *\"Linked Work\"*) ;; *) echo \"$status_out\"; exit 1 ;; esac\n\"$bin\" issue note \"$mission_id\" \"replacement note\" >/dev/null\nshow_out=$(\"$bin\" issue show \"$mission_id\")\ncase \"$show_out\" in *\"replacement note\"*) ;; *) echo \"$show_out\"; exit 1 ;; esac\necho \"mission root rejected; issue objective create/link/status/note/show replacements passed\"\n'"
updated_at: "2026-06-20T21:04:53.079555632+00:00"
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
status_out=$("$bin" issue status "$mission_id" --verbose 2>&1 || true)
case "$status_out" in *"$issue_id"*) ;; *) echo "$status_out"; exit 1 ;; esac
case "$status_out" in *"Linked Work"*) ;; *) echo "$status_out"; exit 1 ;; esac
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
status_out=$("$bin" issue status "$mission_id" --verbose 2>&1 || true)
case "$status_out" in *"$issue_id"*) ;; *) echo "$status_out"; exit 1 ;; esac
case "$status_out" in *"Linked Work"*) ;; *) echo "$status_out"; exit 1 ;; esac
"$bin" issue note "$mission_id" "replacement note" >/dev/null
show_out=$("$bin" issue show "$mission_id")
case "$show_out" in *"replacement note"*) ;; *) echo "$show_out"; exit 1 ;; esac
echo "mission root rejected; issue objective create/link/status/note/show replacements passed"
'
```

Exit status: 1

## Stdout

Bytes: 2794
Truncated: no

```text
Lint found 1 issue(s):
  atelier-nqdn: Issue section Evidence entry 1 must name an observable proof target (command, transcript, evidence record, test, review artifact, file change, or manual check) for issue atelier-nqdn, section Evidence, path .atelier/issues/atelier-nqdn.md
Error: Lint failed with 1 finding(s)
Mission Status atelier-a6p9 [ready] - Objective
===============================================
Health:   ready
Tracker:  ok
Terminal: blocked

Work
----
Total: 1 ready
Epics: none
Other: 1 ready

Selectable Work
---------------
  ready atelier-nqdn - Task | no open blockers; mission-linked root; proof missing

Blocked Work
------------
(none)

Blockers
--------
(none)

Evidence
--------
Direct mission evidence: 1

Reliability
-----------
Projection Freshness: current
Malformed Work: none
Missing Outcome Sections: none
Missing Evidence Sections: none
Graph Hygiene: clear
Attached Proof: missing - issue proof gaps: atelier-nqdn
  Next: atelier evidence record --target issue/<id> --kind validation "..."
  Next: atelier evidence attach <evidence-id> issue <issue-id>
Docs/Help Drift: clear
Ignored Test Review: current
Open Blockers: none
Drill-downs:
  atelier issue status atelier-a6p9 --verbose
  atelier lint

Terminal Checks
---------------
Work: open - atelier-nqdn
  Next: atelier issue transition <issue-id> close --reason "..."
Blockers: clear
Tracker State: current
Linked Issue Records: parseable
Validation Criteria: satisfied
Blocking Lints: failing - atelier lint failed
  Next: atelier lint
Docs/Help Drift: clear
Ignored Test Review: current
Checkout: dirty - git checkout has 2 dirty entries: ?? .atelier/issues/atelier-nqdn.md; ?? .atelier/missions/atelier-a6p9.md
  Next: commit or remove untracked checkout changes

Advanced Validator Detail
-------------------------
3 advanced terminal validator failure detected.
  fail  no_open_work - open linked work: atelier-nqdn
  fail  lint.none_blocking - atelier lint failed
  fail  git.worktree_clean - git checkout has 2 dirty entries: ?? .atelier/issues/atelier-nqdn.md; ?? .atelier/missions/atelier-a6p9.md

Branch Policy
-------------
Current branch: main
Base branch:    main
Owner branches: none
Dirty state: clean
Branch mismatches: none

Active Work
-----------
(none)

Next Commands
-------------
  Inspect mission record (durable intent and linked work): atelier issue show atelier-a6p9
  Refresh mission status (current blockers and terminal checks): atelier issue status atelier-a6p9
  Inspect terminal check detail: atelier issue status atelier-a6p9 --verbose
  Inspect selectable mission work transitions (1 selectable issue(s)): atelier issue transition atelier-nqdn --options
  Record validation proof (1 evidence gap(s)): atelier evidence record --target issue/<id> --kind validation "..."
```

## Stderr

Bytes: 0
Truncated: no

```text
```

