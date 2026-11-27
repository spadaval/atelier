---
schema: atelier.review
schema_version: 3
round: 1
commit: 1bdbb26865dc41fa65d728f79419ef1ea1b2c6ce
reviewers:
- thread:delegated-task:command%3Amcp%3Ae759a211-a7d0-4f39-b1b1-737fb314ff02%3Adelegate-task%3A8f2a431d-0e3d-492d-8d90-54462643a09e
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-27T08:17:21.882176212Z
---

## Summary

Complexity: medium. The new check-naming rule crosses the core skip API, CLI outcomes, merge execution and output.
Value: high. Refusing a failed check when the reason was given for another rule prevents the accidental broken merges this item targets.
Risk of merging: medium. This changes merge authorization in other repositories; refusals and output problems are visible and reversible.

Reviewed the diff from master to 1bdbb26865dc41fa65d728f79419ef1ea1b2c6ce against every Done when line and the Report. Read the skip matcher, both merge completion paths, commit-message generation, history assertions and documentation. The unrelated-reason refusal, intentional override, failed/passed commit sections and log/show assertions are covered. The approach is proportionate to the goal, but the final-line requirement is not met for multiline checks.

Independently ran `cargo nextest run -p atelier-cli -p atelier-core` (400 passed, one skipped), `cargo clippy -p atelier-cli -p atelier-core --all-targets -- -D warnings`, `cargo fmt -- --check` and `git diff --check`; all passed. Also drove the built CLI through two complete start/submit/review/verdict/merge flows in temporary repositories to reproduce the findings below. No live GitHub merge was performed; the existing forge tests use a fake gh and a local bare repository.

## Finding 1: Shell-quote the check name in the printed override command
Recommendation: fix now

`crates/atelier-cli/src/output.rs:1919` inserts the check's command directly inside double quotes in the suggested `ATELIER_SKIP` assignment. Those command characters must survive as literal reason text for `pass_naming` to match them, but the shell consumes embedded double quotes and expands dollar expressions and backticks.

Reproduced with a merge check `echo "broken"; exit 3`. The refusal suggests `ATELIER_SKIP="echo "broken"; exit 3: <why it may fail>"`. Replacing only the explanation and running that assignment before `atelier merge` exits 1 again: the shell supplies `echo broken; exit 3: deliberate scratch reproduction`, which does not name the configured check. Supplying the original command literally through the process environment succeeds, so this is a broken recovery instruction rather than an authorization failure.

Quote the complete reason as one shell word, using the existing `atelier_core::say::quoted` helper or equivalent escaping. Add a test that runs the printed assignment through a shell, including a check with quotes and dollar syntax, and verifies that the deliberate override succeeds without evaluating the command text in the assignment.

## Finding 2: Normalize multiline commands in the final merge warning
Recommendation: fix now

`crates/atelier-cli/src/output.rs:1868` formats `check.run` verbatim in the line intended to finish a successful override. A multiline check splits that warning into several physical lines, so the last line no longer identifies the complete check or says the merge went ahead. This fails the second Done when line and the Report's claim that the last stdout line carries that information.

Reproduced with the same two-line command used by the new integration test, `echo broken` followed by `exit 3`, and an explicitly matching reason. The merge succeeds, but `stdout.lines().last()` is only `exit 3` followed by `` ` failed (exit 3). ``. The current assertion at `crates/atelier-cli/tests/merge.rs:2264` expects this embedded newline as part of the whole output, so it misses the requirement.

Make each command one line in `merged_past`, as the skip matcher and trailer writer already do, and assert the actual last stdout line contains the full normalized check, its failure status and the statement that the merge went ahead.
