---
created_at: "2026-06-20T21:06:08.906373485+00:00"
id: "atelier-0xqf"
evidence_type: "validation"
captured_at: "2026-06-20T21:06:07.949614042+00:00"
command: "bash -lc 'set -euo pipefail\nbin=\"/root/atelier/target/debug/atelier\"\nif \"$bin\" mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then\n  echo \"mission namespace unexpectedly succeeded\"\n  exit 1\nfi\ntmp=$(mktemp -d)\ntrap \"rm -rf \\\"$tmp\\\"\" EXIT\ncd \"$tmp\"\n\"$bin\" init >/dev/null\ngit init --initial-branch=main >/dev/null\ngit config user.email smoke@example.com\ngit config user.name \"Smoke Test\"\ngit add .\ngit commit -m \"initial tracker state\" >/dev/null\nmission_id=$(\"$bin\" issue create \"Objective\" --issue-type mission --body \"Body text\" --constraint \"Constraint one\" --risk \"Risk one\" --validation \"Validate one\" --quiet)\nissue_body=$(printf \"## Description\\n\\nTask body.\\n\\n## Outcome\\n\\nTask remains visible from the objective status.\\n\\n## Evidence\\n\\n- manual check: objective status lists this task.\\n\")\nissue_id=$(\"$bin\" issue create \"Task\" --description \"$issue_body\" --quiet)\n\"$bin\" evidence record --target \"mission/$mission_id\" --kind validation \"replacement status proof\" >/dev/null\n\"$bin\" issue link \"$mission_id\" \"$issue_id\" >/dev/null\nstatus_out=$(\"$bin\" issue status \"$mission_id\" --verbose 2>&1 || true)\ncase \"$status_out\" in *\"$issue_id\"*) ;; *) echo \"$status_out\"; exit 1 ;; esac\nnote_out=$(\"$bin\" issue note \"$mission_id\" \"replacement note\")\ncase \"$note_out\" in *\"Added note\"*) ;; *) echo \"$note_out\"; exit 1 ;; esac\nshow_out=$(\"$bin\" issue show \"$mission_id\")\ncase \"$show_out\" in *\"Constraint one\"*) ;; *) echo \"$show_out\"; exit 1 ;; esac\necho \"mission root rejected; issue objective create/link/status/note/show replacements passed\"'"
exit_status: "0"
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
title: "bash -lc 'set -euo pipefail\nbin=\"/root/atelier/target/debug/atelier\"\nif \"$bin\" mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then\n  echo \"mission namespace unexpectedly succeeded\"\n  exit 1\nfi\ntmp=$(mktemp -d)\ntrap \"rm -rf \\\"$tmp\\\"\" EXIT\ncd \"$tmp\"\n\"$bin\" init >/dev/null\ngit init --initial-branch=main >/dev/null\ngit config user.email smoke@example.com\ngit config user.name \"Smoke Test\"\ngit add .\ngit commit -m \"initial tracker state\" >/dev/null\nmission_id=$(\"$bin\" issue create \"Objective\" --issue-type mission --body \"Body text\" --constraint \"Constraint one\" --risk \"Risk one\" --validation \"Validate one\" --quiet)\nissue_body=$(printf \"## Description\\n\\nTask body.\\n\\n## Outcome\\n\\nTask remains visible from the objective status.\\n\\n## Evidence\\n\\n- manual check: objective status lists this task.\\n\")\nissue_id=$(\"$bin\" issue create \"Task\" --description \"$issue_body\" --quiet)\n\"$bin\" evidence record --target \"mission/$mission_id\" --kind validation \"replacement status proof\" >/dev/null\n\"$bin\" issue link \"$mission_id\" \"$issue_id\" >/dev/null\nstatus_out=$(\"$bin\" issue status \"$mission_id\" --verbose 2>&1 || true)\ncase \"$status_out\" in *\"$issue_id\"*) ;; *) echo \"$status_out\"; exit 1 ;; esac\nnote_out=$(\"$bin\" issue note \"$mission_id\" \"replacement note\")\ncase \"$note_out\" in *\"Added note\"*) ;; *) echo \"$note_out\"; exit 1 ;; esac\nshow_out=$(\"$bin\" issue show \"$mission_id\")\ncase \"$show_out\" in *\"Constraint one\"*) ;; *) echo \"$show_out\"; exit 1 ;; esac\necho \"mission root rejected; issue objective create/link/status/note/show replacements passed\"'"
updated_at: "2026-06-20T21:06:11.790362554+00:00"
---

## Summary

bash -lc 'set -euo pipefail
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
issue_body=$(printf "## Description\n\nTask body.\n\n## Outcome\n\nTask remains visible from the objective status.\n\n## Evidence\n\n- manual check: objective status lists this task.\n")
issue_id=$("$bin" issue create "Task" --description "$issue_body" --quiet)
"$bin" evidence record --target "mission/$mission_id" --kind validation "replacement status proof" >/dev/null
"$bin" issue link "$mission_id" "$issue_id" >/dev/null
status_out=$("$bin" issue status "$mission_id" --verbose 2>&1 || true)
case "$status_out" in *"$issue_id"*) ;; *) echo "$status_out"; exit 1 ;; esac
note_out=$("$bin" issue note "$mission_id" "replacement note")
case "$note_out" in *"Added note"*) ;; *) echo "$note_out"; exit 1 ;; esac
show_out=$("$bin" issue show "$mission_id")
case "$show_out" in *"Constraint one"*) ;; *) echo "$show_out"; exit 1 ;; esac
echo "mission root rejected; issue objective create/link/status/note/show replacements passed"'

## Command

```console
bash -lc 'set -euo pipefail
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
issue_body=$(printf "## Description\n\nTask body.\n\n## Outcome\n\nTask remains visible from the objective status.\n\n## Evidence\n\n- manual check: objective status lists this task.\n")
issue_id=$("$bin" issue create "Task" --description "$issue_body" --quiet)
"$bin" evidence record --target "mission/$mission_id" --kind validation "replacement status proof" >/dev/null
"$bin" issue link "$mission_id" "$issue_id" >/dev/null
status_out=$("$bin" issue status "$mission_id" --verbose 2>&1 || true)
case "$status_out" in *"$issue_id"*) ;; *) echo "$status_out"; exit 1 ;; esac
note_out=$("$bin" issue note "$mission_id" "replacement note")
case "$note_out" in *"Added note"*) ;; *) echo "$note_out"; exit 1 ;; esac
show_out=$("$bin" issue show "$mission_id")
case "$show_out" in *"Constraint one"*) ;; *) echo "$show_out"; exit 1 ;; esac
echo "mission root rejected; issue objective create/link/status/note/show replacements passed"'
```

Exit status: 0

## Stdout

Bytes: 88
Truncated: no

```text
mission root rejected; issue objective create/link/status/note/show replacements passed
```

## Stderr

Bytes: 0
Truncated: no

```text
```

