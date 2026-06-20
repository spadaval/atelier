---
created_at: "2026-06-20T21:03:11.289156714+00:00"
id: "atelier-0mj7"
evidence_type: "validation"
captured_at: "2026-06-20T21:03:10.513926220+00:00"
command: "bash -lc '\nset -euo pipefail\nbin=\"/root/atelier/target/debug/atelier\"\nif \"$bin\" mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then\n  echo \"mission namespace unexpectedly succeeded\"\n  exit 1\nfi\ntmp=$(mktemp -d)\ntrap \"rm -rf \\\"$tmp\\\"\" EXIT\ncd \"$tmp\"\n\"$bin\" init >/dev/null\ngit init >/dev/null\ngit branch -M main\ngit config user.email smoke@example.com\ngit config user.name \"Smoke Test\"\ngit add .\ngit commit -m \"initial tracker state\" >/dev/null\nmission_id=$(\"$bin\" issue create \"Objective\" --issue-type mission --body \"Body text\" --constraint \"Constraint one\" --risk \"Risk one\" --validation \"Validate one\" --quiet)\nissue_id=$(\"$bin\" issue create \"Task\" --description \"Task body\" --quiet)\n\"$bin\" issue link \"$mission_id\" \"$issue_id\" >/dev/null\n\"$bin\" issue status \"$mission_id\" --verbose | grep -q \"$issue_id\"\n\"$bin\" issue note \"$mission_id\" \"replacement note\" >/dev/null\n\"$bin\" issue show \"$mission_id\" | grep -q \"replacement note\"\necho \"mission root rejected; issue objective create/link/status/note/show replacements passed\"\n'"
exit_status: "101"
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
title: "bash -lc '\nset -euo pipefail\nbin=\"/root/atelier/target/debug/atelier\"\nif \"$bin\" mission --help >/tmp/atelier-mission-help.out 2>/tmp/atelier-mission-help.err; then\n  echo \"mission namespace unexpectedly succeeded\"\n  exit 1\nfi\ntmp=$(mktemp -d)\ntrap \"rm -rf \\\"$tmp\\\"\" EXIT\ncd \"$tmp\"\n\"$bin\" init >/dev/null\ngit init >/dev/null\ngit branch -M main\ngit config user.email smoke@example.com\ngit config user.name \"Smoke Test\"\ngit add .\ngit commit -m \"initial tracker state\" >/dev/null\nmission_id=$(\"$bin\" issue create \"Objective\" --issue-type mission --body \"Body text\" --constraint \"Constraint one\" --risk \"Risk one\" --validation \"Validate one\" --quiet)\nissue_id=$(\"$bin\" issue create \"Task\" --description \"Task body\" --quiet)\n\"$bin\" issue link \"$mission_id\" \"$issue_id\" >/dev/null\n\"$bin\" issue status \"$mission_id\" --verbose | grep -q \"$issue_id\"\n\"$bin\" issue note \"$mission_id\" \"replacement note\" >/dev/null\n\"$bin\" issue show \"$mission_id\" | grep -q \"replacement note\"\necho \"mission root rejected; issue objective create/link/status/note/show replacements passed\"\n'"
updated_at: "2026-06-20T21:03:14.134724509+00:00"
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
git init >/dev/null
git branch -M main
git config user.email smoke@example.com
git config user.name "Smoke Test"
git add .
git commit -m "initial tracker state" >/dev/null
mission_id=$("$bin" issue create "Objective" --issue-type mission --body "Body text" --constraint "Constraint one" --risk "Risk one" --validation "Validate one" --quiet)
issue_id=$("$bin" issue create "Task" --description "Task body" --quiet)
"$bin" issue link "$mission_id" "$issue_id" >/dev/null
"$bin" issue status "$mission_id" --verbose | grep -q "$issue_id"
"$bin" issue note "$mission_id" "replacement note" >/dev/null
"$bin" issue show "$mission_id" | grep -q "replacement note"
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
git init >/dev/null
git branch -M main
git config user.email smoke@example.com
git config user.name "Smoke Test"
git add .
git commit -m "initial tracker state" >/dev/null
mission_id=$("$bin" issue create "Objective" --issue-type mission --body "Body text" --constraint "Constraint one" --risk "Risk one" --validation "Validate one" --quiet)
issue_id=$("$bin" issue create "Task" --description "Task body" --quiet)
"$bin" issue link "$mission_id" "$issue_id" >/dev/null
"$bin" issue status "$mission_id" --verbose | grep -q "$issue_id"
"$bin" issue note "$mission_id" "replacement note" >/dev/null
"$bin" issue show "$mission_id" | grep -q "replacement note"
echo "mission root rejected; issue objective create/link/status/note/show replacements passed"
'
```

Exit status: 101

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 892
Truncated: no

```text
hint: Using 'master' as the name for the initial branch. This default branch name
hint: is subject to change. To configure the initial branch name to use in all
hint: of your new repositories, which will suppress this warning, call:
hint:
hint: 	git config --global init.defaultBranch <name>
hint:
hint: Names commonly chosen instead of 'master' are 'main', 'trunk' and
hint: 'development'. The just-created branch can be renamed via this command:
hint:
hint: 	git branch -m <name>
2026-06-20T21:03:11.029498Z  WARN Unknown relation type 'advances'. Known types: related, assumption, falsifies, derived
Error: Lint failed with 1 finding(s)

thread 'main' (3110188) panicked at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/io/stdio.rs:1165:9:
failed printing to stdout: Broken pipe (os error 32)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

