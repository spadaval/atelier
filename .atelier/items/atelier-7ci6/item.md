---
schema: atelier.item
schema_version: 3
id: atelier-7ci6
title: Refuse a merge whose merge check failed unless the reason was given for that failure, and say plainly when one is passed
status: in_review
mission: atelier-oqsz
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A7729b331-a9bb-417c-90ef-4521f33e62cd%3Adelegate-task%3Aoqsz-impl-7ci6
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T07:19:32.427676472Z
---

## Goal

`ATELIER_SKIP="<reason>"` passes every refusal a command meets. On the
Hub's `v2`, merges `122403ea` and `acf8a7f4` were run with a reason
written for "you are not this item's implementer", and the same reason
carried them past `npm exec -- vitest run` failing. The trailer says so in
one clause, `passes (it failed: exit 1)`, of a line shaped like the two
expected ones, and both merge commits list that check under "These merge
checks passed on this tree". A failed merge check
should stop a merge unless the caller means to pass that failure, and the
record should say what happened.

## Done when

- [x] A test replays those merges: a reason that passes another refusal,
  and a merge check that fails. The merge is refused, nothing is merged,
  and the refusal names the check and how it failed.
- [x] A lead can still merge past a failed check on purpose, as
  `roles/lead.md` allows. When it does, the last line `merge` prints says
  which check failed and that the merge went ahead.
- [x] That merge's commit lists the check as failed, not among those that
  passed, and `atelier show <mission>` and `atelier log` show it as they
  show every rule passed.
- [x] `docs/design.md` and `roles/lead.md` say how a failed merge check is
  passed.

## Scope

`atelier merge` and how a skip meets a merge check, in `crates/` and their
tests; `docs/design.md` and `skills/agent-factory/roles/lead.md`. Not the
skip for any other refusal, and not what a refusal offers: the Hub's lead
also found that every refusal ends by offering the skip, which stays as it
is here.

## Report

A failed merge check now stops `atelier merge` unless the skip's reason
names that check: it must write the check's command followed by a colon,
at its start or after a space, quote or backtick, with whitespace made one
line (`Skip::pass_naming` in `crates/atelier-core/src/skip.rs`). The colon
and the word boundary keep a short check such as `true` from being named
by "construe:". Every other rule still passes on any reason, as Scope asks.

- Replay: `a_reason_given_for_another_rule_does_not_carry_the_merge_past_a_failing_merge_check`
  in `crates/atelier-cli/tests/merge.rs` gives a reason that passes the
  uncommitted-change rule while a two-line check fails. The merge exits 1,
  main, the branch, HEAD and the worktree list are as before, and stderr
  names the check, `exit 3`, its output, that the reason does not name it,
  and the way to pass it: `ATELIER_SKIP="<check>: <why it may fail>"`. It
  ends with that line, not "cannot be skipped": a new `Outcome::Unnamed`
  in `commands.rs` keeps `main.rs` from adding that line.
- On purpose: `a_reason_that_names_a_failing_merge_check_merges_past_it_and_says_so_last`
  names the check, merges, and asserts the last stdout line is "The merge
  went ahead past a failed merge check, as ATELIER_SKIP's reason asked:
  `<check>` failed (exit 3)." The forge path ends with the same line.
- The merge commit now lists `true` under "These merge checks passed on
  this tree" and the broken check under "These merge checks failed on this
  tree, and ATELIER_SKIP passed them (see Atelier-Skip)". The trailer's
  rule reads "The merge check `<check>` failed (exit 3), and the merge went
  ahead", in place of "passes (it failed: exit 3)". The test checks both
  `atelier log` and `atelier show <mission>`'s "Rules passed:" print it.
- Without a reason, the refusal of a failed check now offers the named form
  instead of the generic skip line, since the generic one would no longer
  pass it (`a_failing_merge_check_on_the_base_tip_...` updated).
- `docs/design.md` (merge step 7, and the list of rules), `docs/product.md`
  (which said the same as step 7, and would otherwise be wrong) and
  `skills/agent-factory/roles/lead.md` say how a failed check is passed.
  `docs/product.md` is outside the Scope's list; I changed it because it
  described the old behaviour.
- A unit test in `crates/atelier-core/tests/library.rs` covers what names a
  check and what does not.

Why naming, not a second variable: `docs/design.md` keeps `ATELIER_SKIP`
the whole interface, and a reason that names the check also makes the
trailer say what was ruled on. The cost is that a lead types the check's
command; the refusal prints it ready to copy.

Checks run: `cargo nextest run -p atelier-cli -p atelier-core` (400
passed), `cargo clippy -p atelier-cli -p atelier-core --all-targets -- -D
warnings`, `cargo fmt -- --check`, `git diff --check`. Commits that passed a
check under the old wording keep their old trailer; nothing parses it.
