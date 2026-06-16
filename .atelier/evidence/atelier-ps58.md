---
created_at: "2026-06-16T20:44:49.230852726+00:00"
id: "atelier-ps58"
evidence_type: "validation"
captured_at: "2026-06-16T20:44:47.893579699+00:00"
command: "bash -lc '\nset -euo pipefail\nroot=/root/atelier\ncd \"$root\"\nprintf \"== negative closeout issue-type checks ==\\n\"\ntmp=$(mktemp -d)\ncd \"$tmp\"\ngit init -q\ngit checkout -b main >/tmp/closeout-neg2-checkout.out\n\"$root/target/debug/atelier\" init >/tmp/closeout-neg2-init.out\nif \"$root/target/debug/atelier\" issue create \"Bad closeout\" --issue-type closeout >/tmp/closeout-create2.out 2>/tmp/closeout-create2.err; then\n  echo \"issue create unexpectedly accepted closeout\"\n  exit 1\nfi\nrg -F \"Invalid issue_type '\"'\"'closeout'\"'\"'\" /tmp/closeout-create2.err\n\"$root/target/debug/atelier\" issue create \"Normal task\" >/tmp/closeout-normal-issue2.out\nissue_id=$(sed -n \"s/^Created issue \\([^ ]*\\) -.*/\\1/p\" /tmp/closeout-normal-issue2.out)\nif \"$root/target/debug/atelier\" issue update \"$issue_id\" --issue-type closeout >/tmp/closeout-update2.out 2>/tmp/closeout-update2.err; then\n  echo \"issue update unexpectedly accepted closeout\"\n  exit 1\nfi\nrg -F \"Invalid issue_type '\"'\"'closeout'\"'\"'\" /tmp/closeout-update2.err\nif \"$root/target/debug/atelier\" mission status --closeout >/tmp/closeout-status2.out 2>/tmp/closeout-status2.err; then\n  echo \"mission status unexpectedly accepted --closeout\"\n  exit 1\nfi\nrg -F -- \"--closeout\" /tmp/closeout-status2.err\nprintf \"== replacement help and status output ==\\n\"\n\"$root/target/debug/atelier\" mission --help | sed -n \"1,90p\"\n\"$root/target/debug/atelier\" mission status --help | sed -n \"1,90p\"\nstatus_tmp=$(mktemp -d)\ncd \"$status_tmp\"\ngit init -q\ngit checkout -b main >/tmp/status-smoke2-checkout.out\n\"$root/target/debug/atelier\" init >/tmp/status-smoke2-init.out\n\"$root/target/debug/atelier\" mission create \"Status smoke\" >/tmp/status-smoke2-mission.out\nstatus_mission=$(sed -n \"s/^Mission \\([^:]*\\):.*/\\1/p\" /tmp/status-smoke2-mission.out)\n{ \"$root/target/debug/atelier\" mission status \"$status_mission\" --verbose || true; } | sed -n \"1,140p\"\nprintf \"== blocked mission terminal path ==\\n\"\nblocked=$(mktemp -d)\ncd \"$blocked\"\ngit init -q\ngit checkout -b main >/tmp/closeout-blocked2-checkout.out\n\"$root/target/debug/atelier\" init >/tmp/closeout-blocked2-init.out\n\"$root/target/debug/atelier\" mission create \"Blocked terminal smoke\" >/tmp/closeout-blocked2-mission.out\n\"$root/target/debug/atelier\" issue create \"Open terminal work\" >/tmp/closeout-blocked2-issue.out\nblocked_mission=$(sed -n \"s/^Mission \\([^:]*\\):.*/\\1/p\" /tmp/closeout-blocked2-mission.out)\nblocked_issue=$(sed -n \"s/^Created issue \\([^ ]*\\) -.*/\\1/p\" /tmp/closeout-blocked2-issue.out)\n\"$root/target/debug/atelier\" mission add-work \"$blocked_mission\" \"$blocked_issue\" >/tmp/closeout-blocked2-add.out\nif \"$root/target/debug/atelier\" mission close \"$blocked_mission\" --reason blocked >/tmp/closeout-blocked2-close.out 2>/tmp/closeout-blocked2-close.err; then\n  echo \"mission close unexpectedly passed with open linked issue\"\n  exit 1\nfi\nrg -F \"Mission terminal checks blocked\" /tmp/closeout-blocked2-close.err\nrg -F \"Terminal check blockers\" /tmp/closeout-blocked2-close.err\nif rg -i \"closeout\" /tmp/closeout-blocked2-close.err; then\n  echo \"blocked terminal output exposed closeout vocabulary\"\n  exit 1\nfi\nsed -n \"1,140p\" /tmp/closeout-blocked2-close.err\nprintf \"== passing mission terminal path ==\\n\"\npassing=$(mktemp -d)\ncd \"$passing\"\ngit init -q\ngit checkout -b main >/tmp/closeout-pass2-checkout.out\n\"$root/target/debug/atelier\" init >/tmp/closeout-pass2-init.out\n\"$root/target/debug/atelier\" mission create \"Terminal smoke\" >/tmp/closeout-pass2-mission.out\n\"$root/target/debug/atelier\" issue create \"Terminal work\" >/tmp/closeout-pass2-issue.out\npassing_mission=$(sed -n \"s/^Mission \\([^:]*\\):.*/\\1/p\" /tmp/closeout-pass2-mission.out)\npassing_issue=$(sed -n \"s/^Created issue \\([^ ]*\\) -.*/\\1/p\" /tmp/closeout-pass2-issue.out)\n\"$root/target/debug/atelier\" mission add-work \"$passing_mission\" \"$passing_issue\" >/tmp/closeout-pass2-add.out\nissue_file=\".atelier/issues/$passing_issue.md\"\nperl -0pi -e \"s/Outcome was not specified\\./Terminal work is complete./; s/Evidence was not specified\\./- Command transcript from `printf ok` proves terminal work completion./\" \"$issue_file\"\ngit add .\ngit commit -qm baseline\n\"$root/target/debug/atelier\" issue transition \"$passing_issue\" start >/tmp/closeout-pass2-start.out\ngit add .\ngit commit -qm start\n\"$root/target/debug/atelier\" evidence record --target \"issue/$passing_issue\" --kind test -- printf ok >/tmp/closeout-pass2-evidence.out\ngit add .\ngit commit -qm evidence\n\"$root/target/debug/atelier\" issue close \"$passing_issue\" --reason done >/tmp/closeout-pass2-issue-close.out 2>/tmp/closeout-pass2-issue-close.err\nif [ -n \"$(git status --short)\" ]; then\n  git add .\n  git commit -qm issue-close\nfi\nif [ \"$(git branch --show-current)\" != main ]; then\n  git checkout main >/tmp/closeout-pass2-return-main.out\nfi\n\"$root/target/debug/atelier\" mission close \"$passing_mission\" --reason done >/tmp/closeout-pass2-mission-close.out 2>/tmp/closeout-pass2-mission-close.err\nrg -F \"Status: closed\" /tmp/closeout-pass2-mission-close.out\nrg -F \"## Terminal Notes\" /tmp/closeout-pass2-mission-close.out\nif rg -i \"Closeout Notes|closeout\" /tmp/closeout-pass2-mission-close.out; then\n  echo \"passing terminal output exposed closeout vocabulary\"\n  exit 1\nfi\nsed -n \"1,140p\" /tmp/closeout-pass2-issue-close.out\nsed -n \"1,140p\" /tmp/closeout-pass2-mission-close.out\nprintf \"== live surface residue checks ==\\n\"\ncd \"$root\"\nif rg -n -- \"--closeout|mission audit|Mission Closeout Audit|Closeout Gates|Closeout:|mission closeout blocked|Closeout blockers|closeout audit|closeout gate|closeout validator|closeout validators|mission closeout judgment|active mission closeout\" crates/atelier-cli/src; then\n  exit 1\nfi\nif rg -n \"## Closeout Notes|closeout_notes|CloseoutNotes\" crates/atelier-records/src .atelier/missions; then\n  exit 1\nfi\nif rg -n -F -e \"issue_type: \\\"closeout\\\"\" -e \"closeout: standard_review_proof\" -e \"Explicit work type (bug, closeout\" -e \"New issue type (bug, closeout\" -e \"\\\"closeout\\\",\" crates/atelier-sqlite/src crates/atelier-records/src crates/atelier-workflow/src crates/atelier-app/src/workflow_policy.rs crates/atelier-cli/src .atelier/workflow.yaml .atelier/issues; then\n  exit 1\nfi\nif rg -n -- \"atelier mission audit|mission status --completion|mission status <id> --closeout|mission status --closeout|--closeout --verbose|evidence record .*--result|--result pass|validation-or-closeout|closeout issue|closeout items|closeout work|create closeout|separate .*closeout|Closeout Gates|Closeout:\" docs AGENTFACTORY.md /root/.agents/skills/agent-factory crates/atelier-cli/tests | rg -v \"docs/adr/0005|docs/architecture/quality/beads-replacement-closeout\"; then\n  exit 1\nfi\nif rg -n -- \"--result\" SPEC.md docs AGENTFACTORY.md /root/.agents/skills/agent-factory crates/atelier-cli/tests; then\n  exit 1\nfi\nprintf \"live surface residue checks passed\\n\"\nprintf \"== focused formatting and tests ==\\n\"\ncargo fmt -- --check\ncargo test -p atelier-cli issues::test_removed_issue_type_is_rejected -- --nocapture\ncargo test -p atelier-cli setup_guidance::test_mission_status_help_exposes_verbose_terminal_detail -- --nocapture\ncargo test -p atelier-cli setup_guidance::test_mission_help_exposes_close_with_reason -- --nocapture\ncargo test -p atelier-cli mission_projection_worktree::test_mission_closeout_enforces_gates_and_reopen_skips_close_validators -- --nocapture\ncargo test -p atelier-cli mission_projection_worktree::test_mission_close_sees_issue_closeout_bookkeeping_committed_by_issue_close -- --nocapture\ncargo test -p atelier-records mission -- --nocapture\ngit diff --check\n'"
exit_status: "1"
target:
  kind: "issue"
  id: "atelier-9p3t"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-9p3t"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Validate closeout removal with expected blocked exits handled"
updated_at: "2026-06-16T20:44:52.795719409+00:00"
---

## Summary

Validate closeout removal with expected blocked exits handled

## Command

```console
bash -lc '
set -euo pipefail
root=/root/atelier
cd "$root"
printf "== negative closeout issue-type checks ==\n"
tmp=$(mktemp -d)
cd "$tmp"
git init -q
git checkout -b main >/tmp/closeout-neg2-checkout.out
"$root/target/debug/atelier" init >/tmp/closeout-neg2-init.out
if "$root/target/debug/atelier" issue create "Bad closeout" --issue-type closeout >/tmp/closeout-create2.out 2>/tmp/closeout-create2.err; then
  echo "issue create unexpectedly accepted closeout"
  exit 1
fi
rg -F "Invalid issue_type '"'"'closeout'"'"'" /tmp/closeout-create2.err
"$root/target/debug/atelier" issue create "Normal task" >/tmp/closeout-normal-issue2.out
issue_id=$(sed -n "s/^Created issue \([^ ]*\) -.*/\1/p" /tmp/closeout-normal-issue2.out)
if "$root/target/debug/atelier" issue update "$issue_id" --issue-type closeout >/tmp/closeout-update2.out 2>/tmp/closeout-update2.err; then
  echo "issue update unexpectedly accepted closeout"
  exit 1
fi
rg -F "Invalid issue_type '"'"'closeout'"'"'" /tmp/closeout-update2.err
if "$root/target/debug/atelier" mission status --closeout >/tmp/closeout-status2.out 2>/tmp/closeout-status2.err; then
  echo "mission status unexpectedly accepted --closeout"
  exit 1
fi
rg -F -- "--closeout" /tmp/closeout-status2.err
printf "== replacement help and status output ==\n"
"$root/target/debug/atelier" mission --help | sed -n "1,90p"
"$root/target/debug/atelier" mission status --help | sed -n "1,90p"
status_tmp=$(mktemp -d)
cd "$status_tmp"
git init -q
git checkout -b main >/tmp/status-smoke2-checkout.out
"$root/target/debug/atelier" init >/tmp/status-smoke2-init.out
"$root/target/debug/atelier" mission create "Status smoke" >/tmp/status-smoke2-mission.out
status_mission=$(sed -n "s/^Mission \([^:]*\):.*/\1/p" /tmp/status-smoke2-mission.out)
{ "$root/target/debug/atelier" mission status "$status_mission" --verbose || true; } | sed -n "1,140p"
printf "== blocked mission terminal path ==\n"
blocked=$(mktemp -d)
cd "$blocked"
git init -q
git checkout -b main >/tmp/closeout-blocked2-checkout.out
"$root/target/debug/atelier" init >/tmp/closeout-blocked2-init.out
"$root/target/debug/atelier" mission create "Blocked terminal smoke" >/tmp/closeout-blocked2-mission.out
"$root/target/debug/atelier" issue create "Open terminal work" >/tmp/closeout-blocked2-issue.out
blocked_mission=$(sed -n "s/^Mission \([^:]*\):.*/\1/p" /tmp/closeout-blocked2-mission.out)
blocked_issue=$(sed -n "s/^Created issue \([^ ]*\) -.*/\1/p" /tmp/closeout-blocked2-issue.out)
"$root/target/debug/atelier" mission add-work "$blocked_mission" "$blocked_issue" >/tmp/closeout-blocked2-add.out
if "$root/target/debug/atelier" mission close "$blocked_mission" --reason blocked >/tmp/closeout-blocked2-close.out 2>/tmp/closeout-blocked2-close.err; then
  echo "mission close unexpectedly passed with open linked issue"
  exit 1
fi
rg -F "Mission terminal checks blocked" /tmp/closeout-blocked2-close.err
rg -F "Terminal check blockers" /tmp/closeout-blocked2-close.err
if rg -i "closeout" /tmp/closeout-blocked2-close.err; then
  echo "blocked terminal output exposed closeout vocabulary"
  exit 1
fi
sed -n "1,140p" /tmp/closeout-blocked2-close.err
printf "== passing mission terminal path ==\n"
passing=$(mktemp -d)
cd "$passing"
git init -q
git checkout -b main >/tmp/closeout-pass2-checkout.out
"$root/target/debug/atelier" init >/tmp/closeout-pass2-init.out
"$root/target/debug/atelier" mission create "Terminal smoke" >/tmp/closeout-pass2-mission.out
"$root/target/debug/atelier" issue create "Terminal work" >/tmp/closeout-pass2-issue.out
passing_mission=$(sed -n "s/^Mission \([^:]*\):.*/\1/p" /tmp/closeout-pass2-mission.out)
passing_issue=$(sed -n "s/^Created issue \([^ ]*\) -.*/\1/p" /tmp/closeout-pass2-issue.out)
"$root/target/debug/atelier" mission add-work "$passing_mission" "$passing_issue" >/tmp/closeout-pass2-add.out
issue_file=".atelier/issues/$passing_issue.md"
perl -0pi -e "s/Outcome was not specified\./Terminal work is complete./; s/Evidence was not specified\./- Command transcript from `printf ok` proves terminal work completion./" "$issue_file"
git add .
git commit -qm baseline
"$root/target/debug/atelier" issue transition "$passing_issue" start >/tmp/closeout-pass2-start.out
git add .
git commit -qm start
"$root/target/debug/atelier" evidence record --target "issue/$passing_issue" --kind test -- printf ok >/tmp/closeout-pass2-evidence.out
git add .
git commit -qm evidence
"$root/target/debug/atelier" issue close "$passing_issue" --reason done >/tmp/closeout-pass2-issue-close.out 2>/tmp/closeout-pass2-issue-close.err
if [ -n "$(git status --short)" ]; then
  git add .
  git commit -qm issue-close
fi
if [ "$(git branch --show-current)" != main ]; then
  git checkout main >/tmp/closeout-pass2-return-main.out
fi
"$root/target/debug/atelier" mission close "$passing_mission" --reason done >/tmp/closeout-pass2-mission-close.out 2>/tmp/closeout-pass2-mission-close.err
rg -F "Status: closed" /tmp/closeout-pass2-mission-close.out
rg -F "## Terminal Notes" /tmp/closeout-pass2-mission-close.out
if rg -i "Closeout Notes|closeout" /tmp/closeout-pass2-mission-close.out; then
  echo "passing terminal output exposed closeout vocabulary"
  exit 1
fi
sed -n "1,140p" /tmp/closeout-pass2-issue-close.out
sed -n "1,140p" /tmp/closeout-pass2-mission-close.out
printf "== live surface residue checks ==\n"
cd "$root"
if rg -n -- "--closeout|mission audit|Mission Closeout Audit|Closeout Gates|Closeout:|mission closeout blocked|Closeout blockers|closeout audit|closeout gate|closeout validator|closeout validators|mission closeout judgment|active mission closeout" crates/atelier-cli/src; then
  exit 1
fi
if rg -n "## Closeout Notes|closeout_notes|CloseoutNotes" crates/atelier-records/src .atelier/missions; then
  exit 1
fi
if rg -n -F -e "issue_type: \"closeout\"" -e "closeout: standard_review_proof" -e "Explicit work type (bug, closeout" -e "New issue type (bug, closeout" -e "\"closeout\"," crates/atelier-sqlite/src crates/atelier-records/src crates/atelier-workflow/src crates/atelier-app/src/workflow_policy.rs crates/atelier-cli/src .atelier/workflow.yaml .atelier/issues; then
  exit 1
fi
if rg -n -- "atelier mission audit|mission status --completion|mission status <id> --closeout|mission status --closeout|--closeout --verbose|evidence record .*--result|--result pass|validation-or-closeout|closeout issue|closeout items|closeout work|create closeout|separate .*closeout|Closeout Gates|Closeout:" docs AGENTFACTORY.md /root/.agents/skills/agent-factory crates/atelier-cli/tests | rg -v "docs/adr/0005|docs/architecture/quality/beads-replacement-closeout"; then
  exit 1
fi
if rg -n -- "--result" SPEC.md docs AGENTFACTORY.md /root/.agents/skills/agent-factory crates/atelier-cli/tests; then
  exit 1
fi
printf "live surface residue checks passed\n"
printf "== focused formatting and tests ==\n"
cargo fmt -- --check
cargo test -p atelier-cli issues::test_removed_issue_type_is_rejected -- --nocapture
cargo test -p atelier-cli setup_guidance::test_mission_status_help_exposes_verbose_terminal_detail -- --nocapture
cargo test -p atelier-cli setup_guidance::test_mission_help_exposes_close_with_reason -- --nocapture
cargo test -p atelier-cli mission_projection_worktree::test_mission_closeout_enforces_gates_and_reopen_skips_close_validators -- --nocapture
cargo test -p atelier-cli mission_projection_worktree::test_mission_close_sees_issue_closeout_bookkeeping_committed_by_issue_close -- --nocapture
cargo test -p atelier-records mission -- --nocapture
git diff --check
'
```

Exit status: 1

## Stdout

Bytes: 4131
Truncated: yes

```text
== negative closeout issue-type checks ==
Error: Invalid issue_type 'closeout'. Valid values: bug, epic, feature, spike, task, validation
Error: Invalid issue_type 'closeout'. Valid values: bug, epic, feature, spike, task, validation
error: unexpected argument '--closeout' found
  tip: to pass '--closeout' as a value, use '-- --closeout'
== replacement help and status output ==
First-class mission records

Usage: atelier mission [OPTIONS] <COMMAND>

Commands:
  create       Create a mission with generated Intent, Constraints, Risks, and Validation sections
  show         Show a mission with linked plans, work, blockers, and evidence
  start        Focus a mission as the active orchestration context
  status       Show mission-control status for one mission or all current missions
  close        Close a mission after terminal checks pass
  list         List missions
  update       Update mission fields
  note         Add an activity note to a mission
  add-work     Add issue work to a mission
  unlink       Remove issue work from a mission
  add-blocker  Add an issue blocker to a mission
  help         Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]
  -h, --help                     Print help
Show mission-control status for one mission or all current missions

Usage: atelier mission status [OPTIONS] [ID]

Arguments:
  [ID]  

Options:
  -q, --quiet                    Quiet mode: only output essential data (IDs, counts)
      --verbose                  Show verbose validator detail in the status summary
      --log-level <LOG_LEVEL>    Log level for diagnostic output (error, warn, info, debug, trace) [env: ATELIER_LOG=] [default: warn]
      --log-format <LOG_FORMAT>  Log format (text, json) [env: ATELIER_LOG_FORMAT=] [default: text]
  -h, --help                     Print help
Lint passed.
Mission Status atelier-lc3d [ready] - Status smoke
==================================================
Health:   steady
Tracker:  ok
Terminal: blocked

Work
----
Total: none
Epics: none

Selectable Work
---------------
(none)

Blocked Work
------------
(none)

Blockers
--------
(none)

Evidence
--------
Direct mission evidence: none

Reliability
-----------
Projection Freshness: current
Malformed Work: none
Missing Outcome Sections: none
Missing Evidence Sections: none
Graph Hygiene: clear
Attached Proof: complete
Docs/Help Drift: clear
Ignored Test Review: current
Open Blockers: none
Drill-downs:
  atelier mission status atelier-lc3d --verbose
  atelier lint
  atelier doctor

Terminal Checks
---------------
Work: missing
  Next: atelier mission add-work <mission-id> <issue-id>
Blockers: clear
Tracker State: current
Linked Issue Records: parseable
Validation Criteria: satisfied
Blocking Lints: clear
Docs/Help Drift: clear
Ignored Test Review: current
Worktree: dirty - git worktree has 3 dirty entries: ?? .atelier/missions/atelier-lc3d.md; ?? .atelier/workflow.yaml; ?? .gitignore
  Next: commit or remove untracked worktree changes

Advanced Validator Detail
-------------------------
1 advanced terminal validator failure detected.
  fail  git_worktree_clean - git worktree has 3 dirty entries: ?? .atelier/missions/atelier-lc3d.md; ?? .atelier/workflow.yaml; ?? .gitignore

Branch Lifecycle
----------------
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
  Inspect mission record (durable intent and linked work): atelier mission show atelier-lc3d
  Refresh mission status (current blockers and terminal checks): atelier mission status atelier-lc3d
  Inspect terminal check detail: atelier mission status atelier-lc3d --verbose
  Check runtime health (tracker and projection state): atelier doctor
=
```

## Stderr

Bytes: 96
Truncated: no

```text
Switched to a new branch 'main'
Switched to a new branch 'main'
Switched to a new branch 'main'
```

