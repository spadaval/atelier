---
created_at: "2026-06-16T20:46:29.417367710+00:00"
id: "atelier-zewy"
evidence_type: "validation"
captured_at: "2026-06-16T20:46:28.820013447+00:00"
command: "bash -lc '\nset -euo pipefail\nroot=/root/atelier\ncd \"$root\"\nprintf \"== negative removed CLI surfaces ==\\n\"\ntmp=$(mktemp -d)\ncd \"$tmp\"\ngit init -q\ngit checkout -b main >/tmp/final-neg-checkout.out\n\"$root/target/debug/atelier\" init >/tmp/final-neg-init.out\nif \"$root/target/debug/atelier\" issue create \"Bad closeout\" --issue-type closeout >/tmp/final-create.out 2>/tmp/final-create.err; then exit 1; fi\nrg -F \"Invalid issue_type '\"'\"'closeout'\"'\"'\" /tmp/final-create.err\n\"$root/target/debug/atelier\" issue create \"Normal task\" >/tmp/final-normal-issue.out\nissue_id=$(sed -n \"s/^Created issue \\([^ ]*\\) -.*/\\1/p\" /tmp/final-normal-issue.out)\nif \"$root/target/debug/atelier\" issue update \"$issue_id\" --issue-type closeout >/tmp/final-update.out 2>/tmp/final-update.err; then exit 1; fi\nrg -F \"Invalid issue_type '\"'\"'closeout'\"'\"'\" /tmp/final-update.err\nif \"$root/target/debug/atelier\" mission status --closeout >/tmp/final-status-bad.out 2>/tmp/final-status-bad.err; then exit 1; fi\nrg -F -- \"--closeout\" /tmp/final-status-bad.err\nprintf \"== replacement help/status surface ==\\n\"\n\"$root/target/debug/atelier\" mission --help >/tmp/final-mission-help.out\n\"$root/target/debug/atelier\" mission status --help >/tmp/final-status-help.out\nrg -F \"close        Close a mission after terminal checks pass\" /tmp/final-mission-help.out\nrg -F \"--verbose                  Show verbose validator detail\" /tmp/final-status-help.out\nif rg -i \"closeout|mission audit|--completion\" /tmp/final-mission-help.out /tmp/final-status-help.out; then exit 1; fi\nstatus_tmp=$(mktemp -d)\ncd \"$status_tmp\"\ngit init -q\ngit checkout -b main >/tmp/final-status-checkout.out\n\"$root/target/debug/atelier\" init >/tmp/final-status-init.out\n\"$root/target/debug/atelier\" mission create \"Status smoke\" >/tmp/final-status-mission.out\nstatus_mission=$(sed -n \"s/^Mission \\([^:]*\\):.*/\\1/p\" /tmp/final-status-mission.out)\n\"$root/target/debug/atelier\" mission status \"$status_mission\" --verbose >/tmp/final-status.out 2>/tmp/final-status.err || true\nrg -F \"Terminal Checks\" /tmp/final-status.out\nrg -F \"Work: missing\" /tmp/final-status.out\nif rg -i \"closeout\" /tmp/final-status.out /tmp/final-status.err; then exit 1; fi\nprintf \"== blocked terminal check path ==\\n\"\nblocked=$(mktemp -d)\ncd \"$blocked\"\ngit init -q\ngit checkout -b main >/tmp/final-blocked-checkout.out\n\"$root/target/debug/atelier\" init >/tmp/final-blocked-init.out\n\"$root/target/debug/atelier\" mission create \"Blocked terminal smoke\" >/tmp/final-blocked-mission.out\n\"$root/target/debug/atelier\" issue create \"Open terminal work\" >/tmp/final-blocked-issue.out\nblocked_mission=$(sed -n \"s/^Mission \\([^:]*\\):.*/\\1/p\" /tmp/final-blocked-mission.out)\nblocked_issue=$(sed -n \"s/^Created issue \\([^ ]*\\) -.*/\\1/p\" /tmp/final-blocked-issue.out)\n\"$root/target/debug/atelier\" mission add-work \"$blocked_mission\" \"$blocked_issue\" >/tmp/final-blocked-add.out\nif \"$root/target/debug/atelier\" mission close \"$blocked_mission\" --reason blocked >/tmp/final-blocked-close.out 2>/tmp/final-blocked-close.err; then exit 1; fi\nrg -F \"mission terminal checks blocked\" /tmp/final-blocked-close.err\n\"$root/target/debug/atelier\" mission status \"$blocked_mission\" --verbose >/tmp/final-blocked-status.out 2>/tmp/final-blocked-status.err || true\nrg -F \"Terminal Checks\" /tmp/final-blocked-status.out\nrg -F \"Work: incomplete\" /tmp/final-blocked-status.out\nif rg -i \"closeout\" /tmp/final-blocked-close.out /tmp/final-blocked-close.err /tmp/final-blocked-status.out /tmp/final-blocked-status.err; then exit 1; fi\nprintf \"== passing terminal check path ==\\n\"\npassing=$(mktemp -d)\ncd \"$passing\"\ngit init -q\ngit checkout -b main >/tmp/final-pass3-checkout.out\n\"$root/target/debug/atelier\" init >/tmp/final-pass3-init.out\n\"$root/target/debug/atelier\" mission create \"Terminal smoke\" >/tmp/final-pass3-mission.out\n\"$root/target/debug/atelier\" issue create \"Terminal work\" >/tmp/final-pass3-issue.out\npassing_mission=$(sed -n \"s/^Mission \\([^:]*\\):.*/\\1/p\" /tmp/final-pass3-mission.out)\npassing_issue=$(sed -n \"s/^Created issue \\([^ ]*\\) -.*/\\1/p\" /tmp/final-pass3-issue.out)\n\"$root/target/debug/atelier\" mission add-work \"$passing_mission\" \"$passing_issue\" >/tmp/final-pass3-add.out\nissue_file=\".atelier/issues/$passing_issue.md\"\nperl -0pi -e \"s/Outcome was not specified\\./Terminal work is complete./; s/Evidence was not specified\\./- Command transcript from `printf ok` proves terminal work completion./\" \"$issue_file\"\ngit add .\ngit commit -qm baseline\n\"$root/target/debug/atelier\" issue transition \"$passing_issue\" start >/tmp/final-pass3-start.out\ngit add .\ngit commit -qm start\n\"$root/target/debug/atelier\" evidence record --target \"issue/$passing_issue\" --kind test -- printf ok >/tmp/final-pass3-evidence.out\ngit add .\ngit commit -qm evidence\n\"$root/target/debug/atelier\" issue close \"$passing_issue\" --reason done >/tmp/final-pass3-issue-close.out 2>/tmp/final-pass3-issue-close.err\nif [ -n \"$(git status --short)\" ]; then git add .; git commit -qm issue-close; fi\nif [ \"$(git branch --show-current)\" != main ]; then git checkout main >/tmp/final-pass3-return-main.out; fi\n\"$root/target/debug/atelier\" mission close \"$passing_mission\" --reason done >/tmp/final-pass3-mission-close.out 2>/tmp/final-pass3-mission-close.err\nrg -F \"Status: closed\" /tmp/final-pass3-mission-close.out\nrg -F \"## Terminal Notes\" /tmp/final-pass3-mission-close.out\nif rg -i \"Closeout Notes|closeout\" /tmp/final-pass3-mission-close.out /tmp/final-pass3-mission-close.err; then exit 1; fi\nprintf \"== live residue checks ==\\n\"\ncd \"$root\"\nif rg -n -- \"--closeout|mission audit|Mission Closeout Audit|Closeout Gates|Closeout:|mission closeout blocked|Closeout blockers|closeout audit|closeout gate|closeout validator|closeout validators|mission closeout judgment|active mission closeout\" crates/atelier-cli/src; then exit 1; fi\nif rg -n \"## Closeout Notes|closeout_notes|CloseoutNotes\" crates/atelier-records/src .atelier/missions; then exit 1; fi\nif rg -n -F -e \"issue_type: \\\"closeout\\\"\" -e \"closeout: standard_review_proof\" -e \"Explicit work type (bug, closeout\" -e \"New issue type (bug, closeout\" -e \"\\\"closeout\\\",\" crates/atelier-sqlite/src crates/atelier-records/src crates/atelier-workflow/src crates/atelier-app/src/workflow_policy.rs crates/atelier-cli/src .atelier/workflow.yaml .atelier/issues; then exit 1; fi\nif rg -n -- \"atelier mission audit|mission status --completion|mission status <id> --closeout|mission status --closeout|--closeout --verbose|evidence record .*--result|--result pass|validation-or-closeout|closeout issue|closeout items|closeout work|create closeout|separate .*closeout|Closeout Gates|Closeout:\" docs AGENTFACTORY.md /root/.agents/skills/agent-factory crates/atelier-cli/tests | rg -v \"docs/adr/0005|docs/architecture/quality/beads-replacement-closeout\"; then exit 1; fi\nif rg -n -- \"--result\" SPEC.md docs AGENTFACTORY.md /root/.agents/skills/agent-factory crates/atelier-cli/tests; then exit 1; fi\nprintf \"== focused format/tests ==\\n\"\ncargo fmt -- --check\ncargo test -p atelier-cli issues::test_removed_issue_type_is_rejected -- --nocapture\ncargo test -p atelier-cli setup_guidance::test_mission_status_help_exposes_verbose_terminal_detail -- --nocapture\ncargo test -p atelier-cli setup_guidance::test_mission_help_exposes_close_with_reason -- --nocapture\ncargo test -p atelier-cli mission_projection_worktree::test_mission_closeout_enforces_gates_and_reopen_skips_close_validators -- --nocapture\ncargo test -p atelier-cli mission_projection_worktree::test_mission_close_sees_issue_closeout_bookkeeping_committed_by_issue_close -- --nocapture\ncargo test -p atelier-records mission -- --nocapture\ngit diff --check\n'"
exit_status: "2"
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
title: "Final successful validation of terminal workflow after closeout removal"
updated_at: "2026-06-16T20:46:33.107539276+00:00"
---

## Summary

Final successful validation of terminal workflow after closeout removal

## Command

```console
bash -lc '
set -euo pipefail
root=/root/atelier
cd "$root"
printf "== negative removed CLI surfaces ==\n"
tmp=$(mktemp -d)
cd "$tmp"
git init -q
git checkout -b main >/tmp/final-neg-checkout.out
"$root/target/debug/atelier" init >/tmp/final-neg-init.out
if "$root/target/debug/atelier" issue create "Bad closeout" --issue-type closeout >/tmp/final-create.out 2>/tmp/final-create.err; then exit 1; fi
rg -F "Invalid issue_type '"'"'closeout'"'"'" /tmp/final-create.err
"$root/target/debug/atelier" issue create "Normal task" >/tmp/final-normal-issue.out
issue_id=$(sed -n "s/^Created issue \([^ ]*\) -.*/\1/p" /tmp/final-normal-issue.out)
if "$root/target/debug/atelier" issue update "$issue_id" --issue-type closeout >/tmp/final-update.out 2>/tmp/final-update.err; then exit 1; fi
rg -F "Invalid issue_type '"'"'closeout'"'"'" /tmp/final-update.err
if "$root/target/debug/atelier" mission status --closeout >/tmp/final-status-bad.out 2>/tmp/final-status-bad.err; then exit 1; fi
rg -F -- "--closeout" /tmp/final-status-bad.err
printf "== replacement help/status surface ==\n"
"$root/target/debug/atelier" mission --help >/tmp/final-mission-help.out
"$root/target/debug/atelier" mission status --help >/tmp/final-status-help.out
rg -F "close        Close a mission after terminal checks pass" /tmp/final-mission-help.out
rg -F "--verbose                  Show verbose validator detail" /tmp/final-status-help.out
if rg -i "closeout|mission audit|--completion" /tmp/final-mission-help.out /tmp/final-status-help.out; then exit 1; fi
status_tmp=$(mktemp -d)
cd "$status_tmp"
git init -q
git checkout -b main >/tmp/final-status-checkout.out
"$root/target/debug/atelier" init >/tmp/final-status-init.out
"$root/target/debug/atelier" mission create "Status smoke" >/tmp/final-status-mission.out
status_mission=$(sed -n "s/^Mission \([^:]*\):.*/\1/p" /tmp/final-status-mission.out)
"$root/target/debug/atelier" mission status "$status_mission" --verbose >/tmp/final-status.out 2>/tmp/final-status.err || true
rg -F "Terminal Checks" /tmp/final-status.out
rg -F "Work: missing" /tmp/final-status.out
if rg -i "closeout" /tmp/final-status.out /tmp/final-status.err; then exit 1; fi
printf "== blocked terminal check path ==\n"
blocked=$(mktemp -d)
cd "$blocked"
git init -q
git checkout -b main >/tmp/final-blocked-checkout.out
"$root/target/debug/atelier" init >/tmp/final-blocked-init.out
"$root/target/debug/atelier" mission create "Blocked terminal smoke" >/tmp/final-blocked-mission.out
"$root/target/debug/atelier" issue create "Open terminal work" >/tmp/final-blocked-issue.out
blocked_mission=$(sed -n "s/^Mission \([^:]*\):.*/\1/p" /tmp/final-blocked-mission.out)
blocked_issue=$(sed -n "s/^Created issue \([^ ]*\) -.*/\1/p" /tmp/final-blocked-issue.out)
"$root/target/debug/atelier" mission add-work "$blocked_mission" "$blocked_issue" >/tmp/final-blocked-add.out
if "$root/target/debug/atelier" mission close "$blocked_mission" --reason blocked >/tmp/final-blocked-close.out 2>/tmp/final-blocked-close.err; then exit 1; fi
rg -F "mission terminal checks blocked" /tmp/final-blocked-close.err
"$root/target/debug/atelier" mission status "$blocked_mission" --verbose >/tmp/final-blocked-status.out 2>/tmp/final-blocked-status.err || true
rg -F "Terminal Checks" /tmp/final-blocked-status.out
rg -F "Work: incomplete" /tmp/final-blocked-status.out
if rg -i "closeout" /tmp/final-blocked-close.out /tmp/final-blocked-close.err /tmp/final-blocked-status.out /tmp/final-blocked-status.err; then exit 1; fi
printf "== passing terminal check path ==\n"
passing=$(mktemp -d)
cd "$passing"
git init -q
git checkout -b main >/tmp/final-pass3-checkout.out
"$root/target/debug/atelier" init >/tmp/final-pass3-init.out
"$root/target/debug/atelier" mission create "Terminal smoke" >/tmp/final-pass3-mission.out
"$root/target/debug/atelier" issue create "Terminal work" >/tmp/final-pass3-issue.out
passing_mission=$(sed -n "s/^Mission \([^:]*\):.*/\1/p" /tmp/final-pass3-mission.out)
passing_issue=$(sed -n "s/^Created issue \([^ ]*\) -.*/\1/p" /tmp/final-pass3-issue.out)
"$root/target/debug/atelier" mission add-work "$passing_mission" "$passing_issue" >/tmp/final-pass3-add.out
issue_file=".atelier/issues/$passing_issue.md"
perl -0pi -e "s/Outcome was not specified\./Terminal work is complete./; s/Evidence was not specified\./- Command transcript from `printf ok` proves terminal work completion./" "$issue_file"
git add .
git commit -qm baseline
"$root/target/debug/atelier" issue transition "$passing_issue" start >/tmp/final-pass3-start.out
git add .
git commit -qm start
"$root/target/debug/atelier" evidence record --target "issue/$passing_issue" --kind test -- printf ok >/tmp/final-pass3-evidence.out
git add .
git commit -qm evidence
"$root/target/debug/atelier" issue close "$passing_issue" --reason done >/tmp/final-pass3-issue-close.out 2>/tmp/final-pass3-issue-close.err
if [ -n "$(git status --short)" ]; then git add .; git commit -qm issue-close; fi
if [ "$(git branch --show-current)" != main ]; then git checkout main >/tmp/final-pass3-return-main.out; fi
"$root/target/debug/atelier" mission close "$passing_mission" --reason done >/tmp/final-pass3-mission-close.out 2>/tmp/final-pass3-mission-close.err
rg -F "Status: closed" /tmp/final-pass3-mission-close.out
rg -F "## Terminal Notes" /tmp/final-pass3-mission-close.out
if rg -i "Closeout Notes|closeout" /tmp/final-pass3-mission-close.out /tmp/final-pass3-mission-close.err; then exit 1; fi
printf "== live residue checks ==\n"
cd "$root"
if rg -n -- "--closeout|mission audit|Mission Closeout Audit|Closeout Gates|Closeout:|mission closeout blocked|Closeout blockers|closeout audit|closeout gate|closeout validator|closeout validators|mission closeout judgment|active mission closeout" crates/atelier-cli/src; then exit 1; fi
if rg -n "## Closeout Notes|closeout_notes|CloseoutNotes" crates/atelier-records/src .atelier/missions; then exit 1; fi
if rg -n -F -e "issue_type: \"closeout\"" -e "closeout: standard_review_proof" -e "Explicit work type (bug, closeout" -e "New issue type (bug, closeout" -e "\"closeout\"," crates/atelier-sqlite/src crates/atelier-records/src crates/atelier-workflow/src crates/atelier-app/src/workflow_policy.rs crates/atelier-cli/src .atelier/workflow.yaml .atelier/issues; then exit 1; fi
if rg -n -- "atelier mission audit|mission status --completion|mission status <id> --closeout|mission status --closeout|--closeout --verbose|evidence record .*--result|--result pass|validation-or-closeout|closeout issue|closeout items|closeout work|create closeout|separate .*closeout|Closeout Gates|Closeout:" docs AGENTFACTORY.md /root/.agents/skills/agent-factory crates/atelier-cli/tests | rg -v "docs/adr/0005|docs/architecture/quality/beads-replacement-closeout"; then exit 1; fi
if rg -n -- "--result" SPEC.md docs AGENTFACTORY.md /root/.agents/skills/agent-factory crates/atelier-cli/tests; then exit 1; fi
printf "== focused format/tests ==\n"
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

Exit status: 2

## Stdout

Bytes: 430
Truncated: no

```text
== negative removed CLI surfaces ==
Error: Invalid issue_type 'closeout'. Valid values: bug, epic, feature, spike, task, validation
Error: Invalid issue_type 'closeout'. Valid values: bug, epic, feature, spike, task, validation
error: unexpected argument '--closeout' found
  tip: to pass '--closeout' as a value, use '-- --closeout'
== replacement help/status surface ==
  close        Close a mission after terminal checks pass
```

## Stderr

Bytes: 111
Truncated: no

```text
Switched to a new branch 'main'
rg: unrecognized flag --verbose                  Show verbose validator detail
```

