---
created_at: "2026-06-16T20:48:42.136834486+00:00"
id: "atelier-awru"
evidence_type: "validation"
captured_at: "2026-06-16T20:48:26.586427375+00:00"
command: "bash -lc '\nset -euo pipefail\nroot=/root/atelier\ncd \"$root\"\nprintf \"== negative removed CLI surfaces ==\\n\"\ntmp=$(mktemp -d)\ncd \"$tmp\"\ngit init -q\ngit checkout -b main >/tmp/ok-neg-checkout.out\n\"$root/target/debug/atelier\" init >/tmp/ok-neg-init.out\nif \"$root/target/debug/atelier\" issue create \"Bad closeout\" --issue-type closeout >/tmp/ok-create.out 2>/tmp/ok-create.err; then exit 1; fi\nrg -F \"Invalid issue_type '\"'\"'closeout'\"'\"'\" /tmp/ok-create.err\n\"$root/target/debug/atelier\" issue create \"Normal task\" >/tmp/ok-normal-issue.out\nissue_id=$(sed -n \"s/^Created issue \\([^ ]*\\) -.*/\\1/p\" /tmp/ok-normal-issue.out)\nif \"$root/target/debug/atelier\" issue update \"$issue_id\" --issue-type closeout >/tmp/ok-update.out 2>/tmp/ok-update.err; then exit 1; fi\nrg -F \"Invalid issue_type '\"'\"'closeout'\"'\"'\" /tmp/ok-update.err\nif \"$root/target/debug/atelier\" mission status --closeout >/tmp/ok-status-bad.out 2>/tmp/ok-status-bad.err; then exit 1; fi\nrg -F -- \"--closeout\" /tmp/ok-status-bad.err\nprintf \"== replacement help/status surface ==\\n\"\n\"$root/target/debug/atelier\" mission --help >/tmp/ok-mission-help.out\n\"$root/target/debug/atelier\" mission status --help >/tmp/ok-status-help.out\nrg -F \"close        Close a mission after terminal checks pass\" /tmp/ok-mission-help.out\nrg -F -- \"--verbose                  Show verbose validator detail\" /tmp/ok-status-help.out\nif rg -i \"closeout|mission audit|--completion\" /tmp/ok-mission-help.out /tmp/ok-status-help.out; then exit 1; fi\nstatus_tmp=$(mktemp -d)\ncd \"$status_tmp\"\ngit init -q\ngit checkout -b main >/tmp/ok-status-checkout.out\n\"$root/target/debug/atelier\" init >/tmp/ok-status-init.out\n\"$root/target/debug/atelier\" mission create \"Status smoke\" >/tmp/ok-status-mission.out\nstatus_mission=$(sed -n \"s/^Mission \\([^:]*\\):.*/\\1/p\" /tmp/ok-status-mission.out)\n\"$root/target/debug/atelier\" mission status \"$status_mission\" --verbose >/tmp/ok-status.out 2>/tmp/ok-status.err || true\nrg -F \"Terminal Checks\" /tmp/ok-status.out\nrg -F \"Work: missing\" /tmp/ok-status.out\nif rg -i \"closeout\" /tmp/ok-status.out /tmp/ok-status.err; then exit 1; fi\nprintf \"== blocked terminal check path ==\\n\"\nblocked=$(mktemp -d)\ncd \"$blocked\"\ngit init -q\ngit checkout -b main >/tmp/ok-blocked-checkout.out\n\"$root/target/debug/atelier\" init >/tmp/ok-blocked-init.out\n\"$root/target/debug/atelier\" mission create \"Blocked terminal smoke\" >/tmp/ok-blocked-mission.out\n\"$root/target/debug/atelier\" issue create \"Open terminal work\" >/tmp/ok-blocked-issue.out\nblocked_mission=$(sed -n \"s/^Mission \\([^:]*\\):.*/\\1/p\" /tmp/ok-blocked-mission.out)\nblocked_issue=$(sed -n \"s/^Created issue \\([^ ]*\\) -.*/\\1/p\" /tmp/ok-blocked-issue.out)\n\"$root/target/debug/atelier\" mission add-work \"$blocked_mission\" \"$blocked_issue\" >/tmp/ok-blocked-add.out\nif \"$root/target/debug/atelier\" mission close \"$blocked_mission\" --reason blocked >/tmp/ok-blocked-close.out 2>/tmp/ok-blocked-close.err; then exit 1; fi\nrg -F \"mission terminal checks blocked\" /tmp/ok-blocked-close.err\n\"$root/target/debug/atelier\" mission status \"$blocked_mission\" --verbose >/tmp/ok-blocked-status.out 2>/tmp/ok-blocked-status.err || true\nrg -F \"Terminal Checks\" /tmp/ok-blocked-status.out\nrg -F \"Work: open\" /tmp/ok-blocked-status.out\nrg -F \"fail  no_open_work\" /tmp/ok-blocked-status.out\nif rg -i \"closeout\" /tmp/ok-blocked-close.out /tmp/ok-blocked-close.err /tmp/ok-blocked-status.out /tmp/ok-blocked-status.err; then exit 1; fi\nprintf \"== passing terminal check path ==\\n\"\npassing=$(mktemp -d)\ncd \"$passing\"\ngit init -q\ngit checkout -b main >/tmp/ok-pass-checkout.out\n\"$root/target/debug/atelier\" init >/tmp/ok-pass-init.out\n\"$root/target/debug/atelier\" mission create \"Terminal smoke\" >/tmp/ok-pass-mission.out\n\"$root/target/debug/atelier\" issue create \"Terminal work\" >/tmp/ok-pass-issue.out\npassing_mission=$(sed -n \"s/^Mission \\([^:]*\\):.*/\\1/p\" /tmp/ok-pass-mission.out)\npassing_issue=$(sed -n \"s/^Created issue \\([^ ]*\\) -.*/\\1/p\" /tmp/ok-pass-issue.out)\n\"$root/target/debug/atelier\" mission add-work \"$passing_mission\" \"$passing_issue\" >/tmp/ok-pass-add.out\nissue_file=\".atelier/issues/$passing_issue.md\"\nperl -0pi -e \"s/Outcome was not specified\\./Terminal work is complete./; s/Evidence was not specified\\./- Command transcript from `printf ok` proves terminal work completion./\" \"$issue_file\"\ngit add .\ngit commit -qm baseline\n\"$root/target/debug/atelier\" issue transition \"$passing_issue\" start >/tmp/ok-pass-start.out\ngit add .\ngit commit -qm start\n\"$root/target/debug/atelier\" evidence record --target \"issue/$passing_issue\" --kind test -- printf ok >/tmp/ok-pass-evidence.out\ngit add .\ngit commit -qm evidence\n\"$root/target/debug/atelier\" issue close \"$passing_issue\" --reason done >/tmp/ok-pass-issue-close.out 2>/tmp/ok-pass-issue-close.err\nif [ -n \"$(git status --short)\" ]; then git add .; git commit -qm issue-close; fi\nif [ \"$(git branch --show-current)\" != main ]; then git checkout main >/tmp/ok-pass-return-main.out; fi\n\"$root/target/debug/atelier\" mission close \"$passing_mission\" --reason done >/tmp/ok-pass-mission-close.out 2>/tmp/ok-pass-mission-close.err\nrg -F \"Status: closed\" /tmp/ok-pass-mission-close.out\nrg -F \"## Terminal Notes\" /tmp/ok-pass-mission-close.out\nif rg -i \"Closeout Notes|closeout\" /tmp/ok-pass-mission-close.out /tmp/ok-pass-mission-close.err; then exit 1; fi\nprintf \"== live residue checks ==\\n\"\ncd \"$root\"\nif rg -n -- \"--closeout|mission audit|Mission Closeout Audit|Closeout Gates|Closeout:|mission closeout blocked|Closeout blockers|closeout audit|closeout gate|closeout validator|closeout validators|mission closeout judgment|active mission closeout\" crates/atelier-cli/src; then exit 1; fi\nif rg -n \"## Closeout Notes|closeout_notes|CloseoutNotes\" crates/atelier-records/src .atelier/missions; then exit 1; fi\nif rg -n -F -e \"issue_type: \\\"closeout\\\"\" -e \"closeout: standard_review_proof\" -e \"Explicit work type (bug, closeout\" -e \"New issue type (bug, closeout\" -e \"\\\"closeout\\\",\" crates/atelier-sqlite/src crates/atelier-records/src crates/atelier-workflow/src crates/atelier-app/src/workflow_policy.rs crates/atelier-cli/src .atelier/workflow.yaml .atelier/issues; then exit 1; fi\nif rg -n -- \"atelier mission audit|mission status --completion|mission status <id> --closeout|mission status --closeout|--closeout --verbose|evidence record .*--result|--result pass|validation-or-closeout|closeout issue|closeout items|closeout work|create closeout|separate .*closeout|Closeout Gates|Closeout:\" docs AGENTFACTORY.md /root/.agents/skills/agent-factory crates/atelier-cli/tests | rg -v \"docs/adr/0005|docs/architecture/quality/beads-replacement-closeout\"; then exit 1; fi\nif rg -n -- \"--result\" SPEC.md docs AGENTFACTORY.md /root/.agents/skills/agent-factory crates/atelier-cli/tests; then exit 1; fi\nprintf \"== focused format/tests ==\\n\"\ncargo fmt -- --check\ncargo test -p atelier-cli issues::test_removed_issue_type_is_rejected -- --nocapture\ncargo test -p atelier-cli setup_guidance::test_mission_status_help_exposes_verbose_terminal_detail -- --nocapture\ncargo test -p atelier-cli setup_guidance::test_mission_help_exposes_close_with_reason -- --nocapture\ncargo test -p atelier-cli mission_projection_worktree::test_mission_closeout_enforces_gates_and_reopen_skips_close_validators -- --nocapture\ncargo test -p atelier-cli mission_projection_worktree::test_mission_close_sees_issue_closeout_bookkeeping_committed_by_issue_close -- --nocapture\ncargo test -p atelier-records mission -- --nocapture\ngit diff --check\n'"
exit_status: "0"
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
title: "Successful closeout-removal validation transcript"
updated_at: "2026-06-16T20:48:45.678708398+00:00"
---

## Summary

Successful closeout-removal validation transcript

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
git checkout -b main >/tmp/ok-neg-checkout.out
"$root/target/debug/atelier" init >/tmp/ok-neg-init.out
if "$root/target/debug/atelier" issue create "Bad closeout" --issue-type closeout >/tmp/ok-create.out 2>/tmp/ok-create.err; then exit 1; fi
rg -F "Invalid issue_type '"'"'closeout'"'"'" /tmp/ok-create.err
"$root/target/debug/atelier" issue create "Normal task" >/tmp/ok-normal-issue.out
issue_id=$(sed -n "s/^Created issue \([^ ]*\) -.*/\1/p" /tmp/ok-normal-issue.out)
if "$root/target/debug/atelier" issue update "$issue_id" --issue-type closeout >/tmp/ok-update.out 2>/tmp/ok-update.err; then exit 1; fi
rg -F "Invalid issue_type '"'"'closeout'"'"'" /tmp/ok-update.err
if "$root/target/debug/atelier" mission status --closeout >/tmp/ok-status-bad.out 2>/tmp/ok-status-bad.err; then exit 1; fi
rg -F -- "--closeout" /tmp/ok-status-bad.err
printf "== replacement help/status surface ==\n"
"$root/target/debug/atelier" mission --help >/tmp/ok-mission-help.out
"$root/target/debug/atelier" mission status --help >/tmp/ok-status-help.out
rg -F "close        Close a mission after terminal checks pass" /tmp/ok-mission-help.out
rg -F -- "--verbose                  Show verbose validator detail" /tmp/ok-status-help.out
if rg -i "closeout|mission audit|--completion" /tmp/ok-mission-help.out /tmp/ok-status-help.out; then exit 1; fi
status_tmp=$(mktemp -d)
cd "$status_tmp"
git init -q
git checkout -b main >/tmp/ok-status-checkout.out
"$root/target/debug/atelier" init >/tmp/ok-status-init.out
"$root/target/debug/atelier" mission create "Status smoke" >/tmp/ok-status-mission.out
status_mission=$(sed -n "s/^Mission \([^:]*\):.*/\1/p" /tmp/ok-status-mission.out)
"$root/target/debug/atelier" mission status "$status_mission" --verbose >/tmp/ok-status.out 2>/tmp/ok-status.err || true
rg -F "Terminal Checks" /tmp/ok-status.out
rg -F "Work: missing" /tmp/ok-status.out
if rg -i "closeout" /tmp/ok-status.out /tmp/ok-status.err; then exit 1; fi
printf "== blocked terminal check path ==\n"
blocked=$(mktemp -d)
cd "$blocked"
git init -q
git checkout -b main >/tmp/ok-blocked-checkout.out
"$root/target/debug/atelier" init >/tmp/ok-blocked-init.out
"$root/target/debug/atelier" mission create "Blocked terminal smoke" >/tmp/ok-blocked-mission.out
"$root/target/debug/atelier" issue create "Open terminal work" >/tmp/ok-blocked-issue.out
blocked_mission=$(sed -n "s/^Mission \([^:]*\):.*/\1/p" /tmp/ok-blocked-mission.out)
blocked_issue=$(sed -n "s/^Created issue \([^ ]*\) -.*/\1/p" /tmp/ok-blocked-issue.out)
"$root/target/debug/atelier" mission add-work "$blocked_mission" "$blocked_issue" >/tmp/ok-blocked-add.out
if "$root/target/debug/atelier" mission close "$blocked_mission" --reason blocked >/tmp/ok-blocked-close.out 2>/tmp/ok-blocked-close.err; then exit 1; fi
rg -F "mission terminal checks blocked" /tmp/ok-blocked-close.err
"$root/target/debug/atelier" mission status "$blocked_mission" --verbose >/tmp/ok-blocked-status.out 2>/tmp/ok-blocked-status.err || true
rg -F "Terminal Checks" /tmp/ok-blocked-status.out
rg -F "Work: open" /tmp/ok-blocked-status.out
rg -F "fail  no_open_work" /tmp/ok-blocked-status.out
if rg -i "closeout" /tmp/ok-blocked-close.out /tmp/ok-blocked-close.err /tmp/ok-blocked-status.out /tmp/ok-blocked-status.err; then exit 1; fi
printf "== passing terminal check path ==\n"
passing=$(mktemp -d)
cd "$passing"
git init -q
git checkout -b main >/tmp/ok-pass-checkout.out
"$root/target/debug/atelier" init >/tmp/ok-pass-init.out
"$root/target/debug/atelier" mission create "Terminal smoke" >/tmp/ok-pass-mission.out
"$root/target/debug/atelier" issue create "Terminal work" >/tmp/ok-pass-issue.out
passing_mission=$(sed -n "s/^Mission \([^:]*\):.*/\1/p" /tmp/ok-pass-mission.out)
passing_issue=$(sed -n "s/^Created issue \([^ ]*\) -.*/\1/p" /tmp/ok-pass-issue.out)
"$root/target/debug/atelier" mission add-work "$passing_mission" "$passing_issue" >/tmp/ok-pass-add.out
issue_file=".atelier/issues/$passing_issue.md"
perl -0pi -e "s/Outcome was not specified\./Terminal work is complete./; s/Evidence was not specified\./- Command transcript from `printf ok` proves terminal work completion./" "$issue_file"
git add .
git commit -qm baseline
"$root/target/debug/atelier" issue transition "$passing_issue" start >/tmp/ok-pass-start.out
git add .
git commit -qm start
"$root/target/debug/atelier" evidence record --target "issue/$passing_issue" --kind test -- printf ok >/tmp/ok-pass-evidence.out
git add .
git commit -qm evidence
"$root/target/debug/atelier" issue close "$passing_issue" --reason done >/tmp/ok-pass-issue-close.out 2>/tmp/ok-pass-issue-close.err
if [ -n "$(git status --short)" ]; then git add .; git commit -qm issue-close; fi
if [ "$(git branch --show-current)" != main ]; then git checkout main >/tmp/ok-pass-return-main.out; fi
"$root/target/debug/atelier" mission close "$passing_mission" --reason done >/tmp/ok-pass-mission-close.out 2>/tmp/ok-pass-mission-close.err
rg -F "Status: closed" /tmp/ok-pass-mission-close.out
rg -F "## Terminal Notes" /tmp/ok-pass-mission-close.out
if rg -i "Closeout Notes|closeout" /tmp/ok-pass-mission-close.out /tmp/ok-pass-mission-close.err; then exit 1; fi
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

Exit status: 0

## Stdout

Bytes: 3983
Truncated: no

```text
== negative removed CLI surfaces ==
Error: Invalid issue_type 'closeout'. Valid values: bug, epic, feature, spike, task, validation
Error: Invalid issue_type 'closeout'. Valid values: bug, epic, feature, spike, task, validation
error: unexpected argument '--closeout' found
  tip: to pass '--closeout' as a value, use '-- --closeout'
== replacement help/status surface ==
  close        Close a mission after terminal checks pass
      --verbose                  Show verbose validator detail in the status summary
Terminal Checks
Work: missing
== blocked terminal check path ==
Error: mission terminal checks blocked; run `atelier mission status atelier-5z9g` for next commands
Terminal Checks
Work: open - atelier-sq56
  fail  no_open_work - open linked work: atelier-sq56
== passing terminal check path ==
Status: closed
## Terminal Notes
== live residue checks ==
== focused format/tests ==

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test issues::test_removed_issue_type_is_rejected ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 0.16s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test setup_guidance::test_mission_status_help_exposes_verbose_terminal_detail ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test setup_guidance::test_mission_help_exposes_close_with_reason ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test mission_projection_worktree::test_mission_closeout_enforces_gates_and_reopen_skips_close_validators ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 1.41s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test mission_projection_worktree::test_mission_close_sees_issue_closeout_bookkeeping_committed_by_issue_close ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 1.54s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 3 tests
test tests::mission_render_normalizes_legacy_evidence_attachments ... ok
test tests::mission_record_renders_and_parses_deterministically_without_data_blob ... ok
test tests::legacy_mission_data_record_loads_into_typed_sections_and_relationships ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 40 filtered out; finished in 0.00s
```

## Stderr

Bytes: 2781
Truncated: no

```text
Switched to a new branch 'main'
Switched to a new branch 'main'
Switched to a new branch 'main'
Switched to a new branch 'main'
2026-06-16T20:48:28.393075Z  WARN Projection index was stale; rebuilt local SQLite projection from /tmp/tmp.iHUZxFep0o/.atelier
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.09s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.66s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.63s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.65s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.70s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/atelier_records-6e5299045d686aef)
```

