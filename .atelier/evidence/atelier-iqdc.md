---
created_at: "2026-06-16T20:31:39.229492198+00:00"
id: "atelier-iqdc"
evidence_type: "validation"
captured_at: "2026-06-16T20:31:30.563126282+00:00"
command: "bash -lc 'set -euo pipefail\ncargo fmt -- --check\ncargo test -p atelier-cli setup_guidance::test_evidence_record_help_shows_issue_targeted_manual_and_command_flows -- --nocapture\ncargo test -p atelier-cli setup_guidance::test_spec_representative_commands_match_signpost_surfaces -- --nocapture\ncargo test -p atelier-cli commands::workflow::tests::default_validators_are_target_and_transition_aware -- --nocapture\ntarget/debug/atelier workflow check\n! rg -n -- \"atelier mission audit|mission status --completion|mission status <id> --closeout|mission status --closeout|--closeout --verbose|evidence record .*--result|--result pass|validation-or-closeout|closeout issue|closeout items|closeout work|create closeout|separate .*closeout|Closeout Gates|Closeout:\" docs AGENTFACTORY.md /root/.agents/skills/agent-factory crates/atelier-cli/tests\n! rg -n -- \"--result\" SPEC.md docs AGENTFACTORY.md /root/.agents/skills/agent-factory crates/atelier-cli/tests\ntarget/debug/atelier lint atelier-tpuc\ntarget/debug/atelier export --check\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-tpuc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-tpuc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Docs tests and Agent Factory guidance updated for closeout removal"
updated_at: "2026-06-16T20:31:42.764961289+00:00"
---

## Summary

Docs tests and Agent Factory guidance updated for closeout removal

## Command

```console
bash -lc 'set -euo pipefail
cargo fmt -- --check
cargo test -p atelier-cli setup_guidance::test_evidence_record_help_shows_issue_targeted_manual_and_command_flows -- --nocapture
cargo test -p atelier-cli setup_guidance::test_spec_representative_commands_match_signpost_surfaces -- --nocapture
cargo test -p atelier-cli commands::workflow::tests::default_validators_are_target_and_transition_aware -- --nocapture
target/debug/atelier workflow check
! rg -n -- "atelier mission audit|mission status --completion|mission status <id> --closeout|mission status --closeout|--closeout --verbose|evidence record .*--result|--result pass|validation-or-closeout|closeout issue|closeout items|closeout work|create closeout|separate .*closeout|Closeout Gates|Closeout:" docs AGENTFACTORY.md /root/.agents/skills/agent-factory crates/atelier-cli/tests
! rg -n -- "--result" SPEC.md docs AGENTFACTORY.md /root/.agents/skills/agent-factory crates/atelier-cli/tests
target/debug/atelier lint atelier-tpuc
target/debug/atelier export --check
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 2345
Truncated: no

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test setup_guidance::test_evidence_record_help_shows_issue_targeted_manual_and_command_flows ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test setup_guidance::test_spec_representative_commands_match_signpost_surfaces ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 1 test
test commands::workflow::tests::default_validators_are_target_and_transition_aware ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 169 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 340 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s

Workflow Check
==============
Path:           .atelier/workflow.yaml
Policy:         pass
Issue Types:    6
Statuses:       7
Validators:     7
Workflows:      3
Record Health:  pass
Issues Checked: 556
Docs/Help Drift: clear
docs/adr/0005-repo-owned-issue-workflow-state.md:12:and closeout work lands.
docs/architecture/quality/beads-replacement-closeout.md:26:| A real planning, update, and closeout workflow is executed through Atelier. | pass | `agent-factory-atelier-validation.md` records issue create, ready, show, update, dependency add/remove, notes, close, lint, historical export/rebuild diagnostics, and sync proof. |
Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 1419
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.74s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.67s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.75s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
```

