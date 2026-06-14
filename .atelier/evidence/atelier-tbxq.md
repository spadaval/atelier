---
created_at: "2026-06-13T19:41:31.114639291+00:00"
id: "atelier-tbxq"
evidence_type: "transcript"
captured_at: "2026-06-13T19:41:28.371004647+00:00"
command: "bash -lc '\nset -u\nAT=/root/atelier/target/debug/atelier\nTMP=$(mktemp -d)\nrun() { printf \"\\n$ %s\\n\" \"$*\"; \"$@\" 2>&1; printf \"[exit %s]\\n\" \"$?\"; }\nissue_by_title() { rg -l \"title: \\\"$1\\\"\" .atelier/issues | head -1 | xargs -r basename | sed \"s/.md$//\"; }\ncommit_all() { git add . >/dev/null 2>&1; git commit -q -m \"$1\" >/dev/null 2>&1 || true; }\nmkdir -p \"$TMP/main\" && cd \"$TMP/main\"\ngit init -q && git config user.email validator@example.com && git config user.name Validator\nrun \"$AT\" init >/dev/null\nrun \"$AT\" issue create \"Main workflow item\"\nISS=$(issue_by_title \"Main workflow item\")\nrun \"$AT\" workflow init\nrun \"$AT\" workflow migrate-statuses\nrun \"$AT\" workflow check\ncommit_all baseline\nrun \"$AT\" issue transition \"$ISS\" --options\nrun \"$AT\" start \"$ISS\"\nrun \"$AT\" issue transition \"$ISS\" request_review\nrun \"$AT\" issue transition \"$ISS\" request_validation\nrun \"$AT\" issue transition \"$ISS\" close\nrun \"$AT\" evidence record --target \"issue/$ISS\" --kind validation --result pass \"temp close proof\"\ncommit_all proof\nrun \"$AT\" issue close \"$ISS\" --reason \"validated close\"\nrun \"$AT\" issue create \"Post migration issue\"\nPOST=$(issue_by_title \"Post migration issue\")\nrun \"$AT\" issue transition \"$POST\" --options\nmkdir -p \"$TMP/migration\" && cd \"$TMP/migration\"\ngit init -q && git config user.email validator@example.com && git config user.name Validator\nrun \"$AT\" init >/dev/null\nrun \"$AT\" issue create \"Legacy open\"\nrun \"$AT\" issue create \"Legacy closed\"\nrun \"$AT\" issue create \"Legacy archived\"\nperl -0pi -e \"s/status: \\\\\\\"open\\\\\\\"/closed_at: \\\\\\\"2026-06-13T00:00:00+00:00\\\\\\\"\\nstatus: \\\\\\\"closed\\\\\\\"/\" $(rg -l \"Legacy closed\" .atelier/issues)\nperl -0pi -e \"s/status: \\\\\\\"open\\\\\\\"/closed_at: \\\\\\\"2026-06-13T00:00:00+00:00\\\\\\\"\\nstatus: \\\\\\\"archived\\\\\\\"/\" $(rg -l \"Legacy archived\" .atelier/issues)\nrun \"$AT\" workflow init\nrun \"$AT\" workflow migrate-statuses\nrun \"$AT\" workflow check\nrg -n \"title: \\\\\\\"Legacy|status: \\\\\\\"(todo|done|archived)\\\\\\\"|closed_at:\" .atelier/issues\nmkdir -p \"$TMP/missing\" && cd \"$TMP/missing\"\ngit init -q && git config user.email validator@example.com && git config user.name Validator\nrun \"$AT\" init >/dev/null\nrun \"$AT\" issue create \"Missing policy item\"\nMISS=$(issue_by_title \"Missing policy item\")\nrun \"$AT\" workflow init\nrm .atelier/workflow.yaml\nrun \"$AT\" issue transition \"$MISS\" --options\nmkdir -p \"$TMP/unmigrated\" && cd \"$TMP/unmigrated\"\ngit init -q && git config user.email validator@example.com && git config user.name Validator\nrun \"$AT\" init >/dev/null\nrun \"$AT\" issue create \"Unmigrated item\"\nUNMIG=$(issue_by_title \"Unmigrated item\")\nrun \"$AT\" workflow init\nrun \"$AT\" issue transition \"$UNMIG\" --options\ncd \"$TMP/main\"\nrun \"$AT\" workflow --help\nrun \"$AT\" status\n'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fyms"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "fail"
title: "Bounded workflow validation transcript for atelier-fyms; result is fail because new issue creation still emits legacy open status after workflow migration."
updated_at: "2026-06-13T19:41:33.287649697+00:00"
---

Bounded workflow validation transcript for atelier-fyms; result is fail because new issue creation still emits legacy open status after workflow migration.

Command: bash -lc '
set -u
AT=/root/atelier/target/debug/atelier
TMP=$(mktemp -d)
run() { printf "\n$ %s\n" "$*"; "$@" 2>&1; printf "[exit %s]\n" "$?"; }
issue_by_title() { rg -l "title: \"$1\"" .atelier/issues | head -1 | xargs -r basename | sed "s/.md$//"; }
commit_all() { git add . >/dev/null 2>&1; git commit -q -m "$1" >/dev/null 2>&1 || true; }
mkdir -p "$TMP/main" && cd "$TMP/main"
git init -q && git config user.email validator@example.com && git config user.name Validator
run "$AT" init >/dev/null
run "$AT" issue create "Main workflow item"
ISS=$(issue_by_title "Main workflow item")
run "$AT" workflow init
run "$AT" workflow migrate-statuses
run "$AT" workflow check
commit_all baseline
run "$AT" issue transition "$ISS" --options
run "$AT" start "$ISS"
run "$AT" issue transition "$ISS" request_review
run "$AT" issue transition "$ISS" request_validation
run "$AT" issue transition "$ISS" close
run "$AT" evidence record --target "issue/$ISS" --kind validation --result pass "temp close proof"
commit_all proof
run "$AT" issue close "$ISS" --reason "validated close"
run "$AT" issue create "Post migration issue"
POST=$(issue_by_title "Post migration issue")
run "$AT" issue transition "$POST" --options
mkdir -p "$TMP/migration" && cd "$TMP/migration"
git init -q && git config user.email validator@example.com && git config user.name Validator
run "$AT" init >/dev/null
run "$AT" issue create "Legacy open"
run "$AT" issue create "Legacy closed"
run "$AT" issue create "Legacy archived"
perl -0pi -e "s/status: \\\"open\\\"/closed_at: \\\"2026-06-13T00:00:00+00:00\\\"\nstatus: \\\"closed\\\"/" $(rg -l "Legacy closed" .atelier/issues)
perl -0pi -e "s/status: \\\"open\\\"/closed_at: \\\"2026-06-13T00:00:00+00:00\\\"\nstatus: \\\"archived\\\"/" $(rg -l "Legacy archived" .atelier/issues)
run "$AT" workflow init
run "$AT" workflow migrate-statuses
run "$AT" workflow check
rg -n "title: \\\"Legacy|status: \\\"(todo|done|archived)\\\"|closed_at:" .atelier/issues
mkdir -p "$TMP/missing" && cd "$TMP/missing"
git init -q && git config user.email validator@example.com && git config user.name Validator
run "$AT" init >/dev/null
run "$AT" issue create "Missing policy item"
MISS=$(issue_by_title "Missing policy item")
run "$AT" workflow init
rm .atelier/workflow.yaml
run "$AT" issue transition "$MISS" --options
mkdir -p "$TMP/unmigrated" && cd "$TMP/unmigrated"
git init -q && git config user.email validator@example.com && git config user.name Validator
run "$AT" init >/dev/null
run "$AT" issue create "Unmigrated item"
UNMIG=$(issue_by_title "Unmigrated item")
run "$AT" workflow init
run "$AT" issue transition "$UNMIG" --options
cd "$TMP/main"
run "$AT" workflow --help
run "$AT" status
'
Exit status: 0

Stdout summary (truncated):

$ /root/atelier/target/debug/atelier issue create Main workflow item
Refreshed projection in /tmp/tmp.sWoH88xe8r/main/.atelier/state.db from /tmp/tmp.sWoH88xe8r/main/.atelier
Created issue atelier-uf5k - Main workflow item
Type:     task
Priority: medium
File:     /tmp/tmp.sWoH88xe8r/main/.atelier/issues/atelier-uf5k.md

Next Commands
-------------
  Edit issue Markdown: /tmp/tmp.sWoH88xe8r/main/.atelier/issues/atelier-uf5k.md
  Validate this issue: atelier lint atelier-uf5k
  Inspect this issue: atelier issue show atelier-uf5k
  Start tracked work: atelier start atelier-uf5k
[exit 0]

$ /root/atelier/target/debug/atelier workflow init
Created .atelier/workflow.yaml
Starter workflows: standard_review_proof, lightweight_spike

Next Commands
-------------
  atelier workflow migrate-statuses
  atelier workflow check
[exit 0]

$ /root/atelier/target/debug/atelier workflow migrate-statuses
Refreshed projection in /tmp/tmp.sWoH88xe8r/main/.atelier/state.db from /tmp/tmp.sWoH88xe8r/main/.atelier
Workflow Status Migration
=========================
Path:      .atelier/workflow.yaml
Scanned:   1
Migrated:  1
Current:   0
Applied
-------
  open -> initial(todo): 1
Next Commands
-------------
  atelier workflow check
  atelier lint
  atelier export --check
[exit 0]

$ /root/atelier/target/debug/atelier workflow check
Workflow Check
==============
Path:           .atelier/workflow.yaml
Policy:         pass
Issue Types:    7
Statuses:       7
Validators:     6
Workflows:      2
Record Health:  pass
Issues Checked: 1
[exit 0]

$ /root/atelier/target/debug/atelier issue transition atelier-uf5k --options
Issue Transitions atelier-uf5k - Main workflow item
===================================================
State
-----
Status:   todo
Type:     task
Options:  2

block [allowed]
  From: todo, in_progress, review, validation
  To:   blocked
  Command: atelier issue transition atelier-uf5k block
Validators
----------
(none)
Blockers
--------
(none)
Guidance
--------
(none)

start [allowed]
  From: todo, blocked
  To:   in_progress
  Command: atelier issue transition atelier-uf5k start
Validators
----------
(none)
Blockers
--------
(none)
Guidance
--------
(none)
[exit 0]

$ /root/atelier/target/debug/atelier start atelier-uf5k
Refreshed projection in /tmp/tmp.sWoH88xe8r/main/.atelier/state.db from /tmp/tmp.sWoH88xe8r/main/.atelier
Applied transition start to atelier-uf5k
From:     todo
To:       in_progress
Next Commands
-------------
  atelier issue show atelier-uf5k
  atelier issue transition atelier-uf5k --options
Started work on atelier-uf5k Main workflow item
Branch: master
Worktree: /tmp/tmp.sWoH88xe8r/main
[exit 0]

$ /root/atelier/target/debug/atelier issue transition atelier-uf5k request_review
Refreshed projection in /tmp/tmp.sWoH88xe8r/main/.atelier/state.db from /tmp/tmp.sWoH88xe8r/main/.atelier
Applied transition request_review to atelier-uf5k
From:     in_progress
To:       review
Next Commands
-------------
  atelier issue show atelier-uf5k
  atelier issue transition atelier-uf5k --options
[exit 0]

$ /root/atelier/target/debug/atelier issue transition atelier-uf5k request_validation
Refreshed projection in /tmp/tmp.sWoH88xe8r/main/.atelier/state.db from /tmp/tmp.sWoH88xe8r/main/.atelier
Applied transition request_validation to atelier-uf5k
From:     review
To:       validation
Next Commands
-------------
  atelier issue show atelier-uf5k
  atelier issue transition atelier-uf5k --options
[exit 0]

$ /root/atelier/target/debug/atelier issue transition atelier-uf5k close
Lint passed.
Issue Transition atelier-uf5k - Main workflow item
==================================================
Transition: close
From:       validation
To:         done
Command:    atelier issue transition atelier-uf5k close --reason "..."
Validators
----------
  fail  proof_attached
      expected at least 1 passing evidence record(s); found 0
  pass  blockers_clear
      no open blockers
  pass  lint_clear
      lint passed
  pass  durable_current
      canonical export is current
  fail  closeout_clean
      git worktree has 5 dirty entries:  M .atelier/i

Stderr summary:
(none)

