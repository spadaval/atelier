---
schema: atelier.item
schema_version: 3
id: atelier-nv22
title: List every rule a mission's commits passed, with its reason and thread, in atelier show <mission>
status: changes_requested
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A15ed7340-46a6-4e3a-b0c2-fdc0661b938f%3Adelegate-task%3Ada3ec70a-5299-4fbc-91f2-e169a6e2701f
ready:
  by: 'the human, in conversation on 2026-11-18: put the mission together and then run it'
  at: 2026-11-18T23:49:47.284668623Z
trivial: null
caused_by: null
created_at: 2026-11-18T20:28:22.241930090Z
---

## Goal

A lead may pass a rule by giving its reason, and what keeps that honest is
that the reason is read. Today it is kept and not shown: the commit carries
an `Atelier-Skip` trailer and `atelier log` prints it at the end of a line,
but `atelier show <mission>`, where the human reads a mission, lists none.
One rule has been passed since the cutover, and it can only be found with
`grep`.

The trailer also does not say who passed the rule. It holds the rule and
the reason, and `atelier log` names a merge's thread from the item's
implementers. A lead that merges an implementer's item past its review
would be shown as the implementer.

## Done when

- [x] A commit that passes a rule records the thread that ran the command,
  in its trailers, beside the rule and the reason. The installed binary
  still reads a trailer without one.
- [x] `atelier show <mission>` lists each rule a commit of the mission or of
  its items passed: the item, the rule, the reason and the thread that
  passed it. `atelier log` names the same thread. For a trailer written
  before this change both say the thread is not recorded.
- [x] A mission with none says nothing about it.
- [x] A test covers a fixture mission whose lead merged an implementer's
  item past a rule: both name the lead, not the implementer.
- [x] `docs/design.md` says what a passed rule records and where it is
  shown.

## Scope

Where a passed rule is written to a commit and read back, `atelier show
<mission>` and `atelier log`, in `crates/` and their tests; `docs/design.md`.
Not the Desk, and no new record.

## Report

A rule passed with `ATELIER_SKIP` now names the thread that passed it, and
`atelier show <mission>` lists every one its commits passed. Commit
2887410b1.

- Recorded: each commit that passes a rule ends with one
  `Atelier-Skip-By: <thread>` after its `Atelier-Skip:` lines. `Skip`
  carries the thread (`Skip::by`, set in `main.rs` from the same identity
  that opens the repository), and `Skip::trailers` is now the one place the
  trailers are written: by `git::with_skips` for the commits Atelier makes,
  and by `say::commit_line` for the commit lines `ready`, `set`, `drop`,
  `close`, `lead` and `verdict` print. The thread is its own key, so the
  installed 0.7.0 binary (its `git log` asks only for `Atelier-Skip`, an
  exact key match I checked in Git) reads a new commit as before, and the
  new binary reads an old commit as having no thread.
- `atelier log`: each skip reads `skip <rule>: <reason> (by <thread>)`, or
  `(thread not recorded)` for an older trailer. An action whose commit
  names the thread shows that thread in its thread column, so a lead's
  merge of an implementer's item is the lead's line, not the
  implementer's.
- `atelier show <mission>`: a `Rules passed:` section after `Disputed:`,
  one line per rule: `<item>, <action> in <commit>, by <thread>: <rule>:
  <reason>`. It reads the mission's log (`log::history`), so it agrees
  with `atelier log` on the thread. A mission with none prints no section.
  On this repository `show atelier-qr5s` lists the one rule passed since the
  cutover (atelier-6w8d's merge past CI), as "thread not recorded".
- Tests: `disputes::a_lead_rules_for_the_implementer_by_merging_past_a_kept_finding_with_its_reason`
  now checks that the merge commit's trailer, the log line and the mission
  page all name `lead1`, and that the page never says `by t1`.
  `log::a_skip_from_before_its_thread_was_recorded_says_so_in_the_log_and_the_missions_page`
  checks that a mission with no passed rule prints no section, that the
  page names `t1` for a new trailer, and that both say "thread not
  recorded" once the commit is rewritten without its `Atelier-Skip-By`
  line. The other trailer tests in `skip.rs` and `merge.rs` now expect the
  thread line.
- `docs/design.md`: Hooks and the bypass says what the trailers hold, why
  the thread is its own key, and where each passed rule is shown. The log
  section covers the thread column and the skip's thread, and fixes its list
  of actions whose commit carries a skip, which had not been updated for the
  commit lines atelier-mtzj added.

Validation: `cargo nextest run` passes (393 tests), `cargo clippy
--all-targets -- -D warnings`, `cargo fmt -- --check` and `git diff
--check` are clean, and `evals/agent-factory/test.sh` passes against the new
`target/debug/atelier`. The eval graders match `^Atelier-Skip: `, which the
new trailer does not.

Outside the Scope, not done:

- `log::history` finds a mission only on the base's tip, so a proposed
  mission, whose record is only on its plan branch, gets no log, and its
  page lists no passed rules. `docs/design.md` says an ID on "a branch named
  for it" is found. This was already so before this change; it is worth an
  item of its own.
