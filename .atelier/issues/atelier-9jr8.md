---
created_at: "2026-10-17T01:10:57.980264087+00:00"
id: "atelier-9jr8"
issue_type: "bug"
labels:
- "cli-output"
- "land"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T14:43:11.856312190+00:00"
status: "done"
title: "Give each failed land check its item, reason and fix command"
updated_at: "2026-10-23T14:43:11.856312190+00:00"
---

## Goal

`land_invariants_unmet` prints `{target} {check}: {reason}` lines, then one generic `Resolve the named checks, commit the records, then rerun atelier land.` (crates/atelier-cli/src/commands/session.rs:535-566). Observed in a scratch clone:
```
atelier-jb2t blockers.none_open: open blockers: atelier-ihns
atelier-jb2t blockers.transitive_none_open: completion blockers: atelier-ihns
```
and
```
atelier-43m1 review.current: no current independent approval or explicit trivial exemption; authors: sumeet padavala, sumeet padavala <sumeet.padavala@gmail.com>, sumeet.padavala@gmail.com
```
- One cause produces two lines.
- The blocker's title and status are missing.
- The same author appears three times.
- The message does not say that trivial exemption atelier-36rk already targets atelier-43m1 but is not current.
- No line gives the command that fixes it.
- An unrelated behind-base note is appended after the generic line.

## Done when

- Each failed check prints one line with the item, the reason and a fix, for example:
  - `atelier-jb2t done but blocked by atelier-ihns [todo] Show the between-missions state… — close atelier-ihns or remove the link`
  - `atelier-43m1 needs review: no approval at a current commit (atelier-36rk pins 1620b534, not on this branch) — atelier review approve atelier-43m1 --identity <reviewer> "…"`
- A transitive failure that repeats a direct failure on the same item is merged into it.
- Authors are deduplicated by email.
- The behind-base note is printed separately from the failures.
- Tests cover blockers, review and findings.

## Context

The reasons come from `review_gate` (crates/atelier-app/src/review_gate.rs). The `merge --check` gate-reason item in this mission shares these reasons, so they should be changed once at the source.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed. land_invariants_unmet unchanged (session.rs:426-457); verdict always counts 1 check (session.rs:612-619) even with 3 failures; transitive and direct blocker lines both print; review.current lists the same author 3 ways (dedupe by email, review_gate.rs:938, add_author :1279).
- 2026-10-23 [decision] Fix commands keep a placeholder only for a value the command cannot know: the reviewer identity, a resolution commit, a follow-up issue, reason text ('<identity>', '<commit>', '<issue>', '"…"'); every ID, path and command it knows is real
  Reason: Guessing an identity or resolution commit would print a wrong command that runs; human-cli-output.md already keeps placeholders where the command cannot know the value
  Rejected: Filling identity from git config and the resolution from HEAD; printing no fix for findings
- 2026-10-23 [decision] review.current's fix is 'atelier review request <owner>', not 'atelier review approve'
  Reason: The author cannot approve their own work; the brief is what the author hands to an independent reviewer
  Rejected: atelier review approve <owner> --identity <reviewer>

## Close Reason

A refused `land` now prints each failed check as one line per record, `⚠ <record> <check>: <cause>. Next: <fix>`, with real IDs, folds a blocker check that repeats another's cause into that line, lists the passes, prints the behind-base and stale-proof notes after them, and ends `land: refused — N checks (…)` naming every failed check; authors are listed once per person and empty or duplicated gate reasons are gone.

### Deviations
The blocker is shown as an item line (`atelier-ihns todo high Title`), not `[todo]`, because the output contract forbids brackets. The review line keeps the check name (`review.current:`) instead of "needs review", and its fix is `atelier review request <owner>` (decision note). Naming the stale approval it considered (the atelier-36rk part of the example) is left to atelier-imlh (atelier-cisa). `land_invariants_unmet` is no longer printed; the verdict names the checks.

### Decisions
See the decision notes on atelier-9jr8 (placeholders only for values the command cannot know; review request as the review fix) and on atelier-h1nc (verdict naming every failed check, serde-skipped gaps, output streams).

### Surprises
A child's done claim fails both its own `blockers.none_open` validator and `blockers.transitive_none_open` for the same blocker as the owner's `blockers.none_open`, so the fold had to cover direct checks on other records too. The Stop hook's reason now carries the same gate lines, since it formats the same error.

### Open items
Precondition refusals (`land_completion_dirty`, branch mismatch and the rest) still print one code and are counted as one check: atelier-0uaj. Naming the approval considered: atelier-imlh.
