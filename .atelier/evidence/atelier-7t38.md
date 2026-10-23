---
created_at: "2026-10-23T14:59:38.350056353+00:00"
id: "atelier-7t38"
evidence_type: "test"
commit: "c0b9a62fbf9ad8139d45113ccd816d55dc6665a7"
result: "fail"
captured_at: "2026-10-23T14:59:36.116889671+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it) | test(merge_commit_strategy_refuses_a_merge_that_breaks_a_claim_the_base_completed)'"
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
title: "## Summary\n\natelier-5nk6: open scoped items get the workflow's closing transitions in order; an open todo blocker gets atelier start plus the unlink naming the declaring record\n\n## Command\n\n```console\ncargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it) | test(merge_commit_strategy_refuses_a_merge_that_breaks_a_claim_the_base_completed)'\n```\n\nExit status: 0\n\n## Stdout\n\nBytes: 0\nSHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\n\n## Stderr\n\nBytes: 843\nSHA-256: 3b47e534a9341d655ad91a226c0fdd28314e49af09a6660a4ec9064ad1e8ea08\nTest summary: Summary [   0.520s] 3 tests run: 3 passed, 895 skipped (rerun after merging master)"
updated_at: "2026-10-23T14:59:38.350353648+00:00"
---

## Summary

## Summary

atelier-5nk6: open scoped items get the workflow's closing transitions in order; an open todo blocker gets atelier start plus the unlink naming the declaring record

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it) | test(merge_commit_strategy_refuses_a_merge_that_breaks_a_claim_the_base_completed)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 843
SHA-256: 3b47e534a9341d655ad91a226c0fdd28314e49af09a6660a4ec9064ad1e8ea08
Test summary: Summary [   0.520s] 3 tests run: 3 passed, 895 skipped (rerun after merging master)

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it) | test(merge_commit_strategy_refuses_a_merge_that_breaks_a_claim_the_base_completed)'
```

Exit status: 100

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 5966
Truncated: yes
SHA-256: ff6e516cb3319465e76631bfa692f9b42e5e5436671e4f0c7902919cec9b430d
Test summary: test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.31s

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-h1nc/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.46s
────────────
 Nextest run ID ff622ca1-7928-4a59-943c-4f6d6c942812 with nextest profile: default
    Starting 3 tests across 10 binaries (909 tests skipped)
        PASS [   0.269s] (1/3) atelier-cli::session land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check
        FAIL [   0.320s] (2/3) atelier-cli::merge_gates merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it
  stdout ───

    running 1 test
    test merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it ... FAILED

    failures:

    failures:
        merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.31s

  stderr ───

    thread 'merge_check_names_eac
… 1870 bytes omitted …
iew.current via atelier-fcio)\nmerge: refused — 3 checks (findings.disposed, work.terminal, claims.reviewed)\n"
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.514s] (3/3) atelier-cli::merge_gates merge_commit_strategy_refuses_a_merge_that_breaks_a_claim_the_base_completed
────────────
     Summary [   0.515s] 3 tests run: 2 passed, 1 failed, 909 skipped
        FAIL [   0.320s] (2/3) atelier-cli::merge_gates merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it
  stdout ───

    running 1 test
    test merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it ... FAILED

    failures:

    failures:
        merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.31s

  stderr ───

    thread 'merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it' (596342) panicked at crates/atelier-cli/tests/merge_gates.rs:3554:5:
    assertion `left == right` failed
      left: "⚠ atelier-tvlv findings.disposed: atelier-7pii fix_now finding \"Contract failure in src.rs\" has no disposition (also claims.reviewed). Next: atelier review disposition atelier-7pii --decision fix_now --resolution-commit <commit> --identity <identity> \"…\"\n⚠ atelier-tvlv findings.disposed: atelier-kl6o defer finding \"Rename later\": defer disposition atelier-cblh needs acceptance by a non-author. Next: atelier review request atelier-tvlv\n⚠ atelier-j5ok work.terminal: todo, open in atelier-tvlv's scope. Next: `atelier issue transition atelier-j5ok start`, then `atelier issue transition atelier-j5ok close --reason \"…\"`\n✓ 4 checks pass (blockers.none_open, blockers.transitive_none_open, required_field.close_reason, review.current via atelier-fcio)\nmerge: refused — 3 checks (findings.disposed, work.terminal, claims.reviewed)\n"
     right: "⚠ atelier-tvlv findings.disposed: atelier-7pii fix_now finding \"Contract failure in src.rs\" has no disposition (also claims.reviewed). Next: atelier review disposition atelier-7pii --decision fix_now --resolution-commit <commit> --identity <identity> \"…\"\n⚠ atelier-tvlv findings.disposed: atelier-kl6o defer finding \"Rename later\": defer disposition atelier-cblh\n⚠ identity matches a Git author; this will not satisfy findings.disposed needs acceptance by a non-author. Next: atelier review request atelier-tvlv\n⚠ atelier-j5ok work.terminal: todo, open in atelier-tvlv's scope. Next: `atelier issue transition atelier-j5ok start`, then `atelier issue transition atelier-j5ok close --reason \"…\"`\n✓ 4 checks pass (blockers.none_open, blockers.transitive_none_open, required_field.close_reason, review.current via atelier-fcio)\nmerge: refused — 3 checks (findings.disposed, work.terminal, claims.reviewed)\n"
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: test run failed
```
