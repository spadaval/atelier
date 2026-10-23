---
created_at: "2026-10-23T14:59:33.847573854+00:00"
id: "atelier-w253"
evidence_type: "test"
commit: "c0b9a62fbf9ad8139d45113ccd816d55dc6665a7"
result: "fail"
captured_at: "2026-10-23T14:58:44.359957640+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "100"
agent_identity: "atelier merge --sync"
target:
  kind: "issue"
  id: "atelier-h1nc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h1nc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "## Summary\n\nfull workspace suite with test hooks passes (898 tests) at the review-fix commit\n\n## Command\n\n```console\ncargo nextest run -j 4 --workspace --features test-hooks\n```\n\nExit status: 0\n\n## Stdout\n\nBytes: 0\nSHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\n\n## Stderr\n\nBytes: 112854\nSHA-256: 4e99792ffadd863d25b9a5a6c5fe5a911b7b3cb3639e1d99c651483509696440\nTest summary: Summary [  37.709s] 898 tests run: 898 passed (1 slow), 0 skipped (rerun after merging master)"
updated_at: "2026-10-23T14:59:33.847869636+00:00"
---

## Summary

## Summary

full workspace suite with test hooks passes (898 tests) at the review-fix commit

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 112854
SHA-256: 4e99792ffadd863d25b9a5a6c5fe5a911b7b3cb3639e1d99c651483509696440
Test summary: Summary [  37.709s] 898 tests run: 898 passed (1 slow), 0 skipped (rerun after merging master)

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 100

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 120245
Truncated: yes
SHA-256: b3f95399fafc4c1518cf421734eca5fa10d058624e7db188ee12dfdf6fd8c563
Test summary: test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.29s

```text
   Compiling atelier-core v0.2.0 (/home/supadava/projects/atelier-wt/atelier-h1nc/crates/atelier-core)
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-h1nc/crates/atelier-cli)
   Compiling atelier-workflow v0.2.0 (/home/supadava/projects/atelier-wt/atelier-h1nc/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/home/supadava/projects/atelier-wt/atelier-h1nc/crates/atelier-records)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/atelier-h1nc/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 11.14s
────────────
 Nextest run ID 3bb05a70-a333-49a7-b4fd-ce58df98583e with nextest profile: default
    Starting 912 tests across 10 binaries
        PASS [   0.044s] (  1/912) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.111s] (  2/912) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kin
… 116149 bytes omitted …
SS [   0.005s] (911/912) atelier-workflow tests::validates_workflow_branch_field_shape
        PASS [  13.697s] (912/912) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
────────────
     Summary [  38.021s] 912 tests run: 911 passed (1 slow), 1 failed, 0 skipped
        SLOW [  13.697s] (912/912) atelier-cli::session a_close_after_approval_is_covered_by_merge_and_land_alike
        FAIL [   0.297s] (608/912) atelier-cli::merge_gates merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it
  stdout ───

    running 1 test
    test merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it ... FAILED

    failures:

    failures:
        merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.29s

  stderr ───

    thread 'merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it' (550843) panicked at crates/atelier-cli/tests/merge_gates.rs:3554:5:
    assertion `left == right` failed
      left: "⚠ atelier-i6fx findings.disposed: atelier-3rxm fix_now finding \"Contract failure in src.rs\" has no disposition (also claims.reviewed). Next: atelier review disposition atelier-3rxm --decision fix_now --resolution-commit <commit> --identity <identity> \"…\"\n⚠ atelier-i6fx findings.disposed: atelier-pwtd defer finding \"Rename later\": defer disposition atelier-jlei needs acceptance by a non-author. Next: atelier review request atelier-i6fx\n⚠ atelier-4cmc work.terminal: todo, open in atelier-i6fx's scope. Next: `atelier issue transition atelier-4cmc start`, then `atelier issue transition atelier-4cmc close --reason \"…\"`\n✓ 4 checks pass (blockers.none_open, blockers.transitive_none_open, required_field.close_reason, review.current via atelier-vm9c)\nmerge: refused — 3 checks (findings.disposed, work.terminal, claims.reviewed)\n"
     right: "⚠ atelier-i6fx findings.disposed: atelier-3rxm fix_now finding \"Contract failure in src.rs\" has no disposition (also claims.reviewed). Next: atelier review disposition atelier-3rxm --decision fix_now --resolution-commit <commit> --identity <identity> \"…\"\n⚠ atelier-i6fx findings.disposed: atelier-pwtd defer finding \"Rename later\": defer disposition atelier-jlei\n⚠ identity matches a Git author; this will not satisfy findings.disposed needs acceptance by a non-author. Next: atelier review request atelier-i6fx\n⚠ atelier-4cmc work.terminal: todo, open in atelier-i6fx's scope. Next: `atelier issue transition atelier-4cmc start`, then `atelier issue transition atelier-4cmc close --reason \"…\"`\n✓ 4 checks pass (blockers.none_open, blockers.transitive_none_open, required_field.close_reason, review.current via atelier-vm9c)\nmerge: refused — 3 checks (findings.disposed, work.terminal, claims.reviewed)\n"
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: test run failed
```
