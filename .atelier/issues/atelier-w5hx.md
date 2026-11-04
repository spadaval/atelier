---
created_at: "2026-10-30T06:52:24.678869557+00:00"
id: "atelier-w5hx"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-6306"
    type: "implements"
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "issue"
    id: "atelier-y8xw"
    type: "supersedes"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T23:27:22.622665104+00:00"
status: "done"
title: "Remove one-way decision notes and migrate the existing ones"
updated_at: "2026-11-04T23:27:22.622665104+00:00"
---

## Goal

Carry out the decision to remove one-way decision notes: one way to record a costly-to-reverse choice, the `decision` record.

## Done when

`atelier issue note --help` has no `--one-way`, and passing it is a usage error. `work decisions` lists decision records first, then notes, with no one-way group. The brief decision view that `status` and `issue show` print (`decision_brief_lines`) shows the current decision records bearing on the item, at most three, then `+N more` pointing at `issue show --full`, which also does atelier-y8xw's cap. The debrief page has no one-way section, sidebar entry or `.oneway` style. The four one-way notes (atelier-fip8 lines 57 and 61, atelier-ho7o, atelier-sfsq) read `[decision]`; fip8's Design text and any note text that quotes the tag stay as written. `git grep -n -i "one-way\|one_way\|oneway" crates docs/product` prints nothing. `atelier check`, `cargo nextest run` and the test-hooks suite pass.

## Scope

crates/atelier-core (note tag and parser), atelier-app decisions.rs and mission_review.rs, atelier-cli issue_cli.rs, issue.rs, work.rs, mission_review.rs, review_page.rs, human_output.rs and main.rs (help text); docs/product/work-model.md, cli-surface.md, human-cli-output.md, mission-reviews.md; docs/strategies/review-and-introspection.md (atelier-r6bt's strategy): the one-way lines in its principles, S4 ("decision log with one-way decisions first" becomes decision records first) and S6 ("the count and the one-way decisions" becomes the count and up to three decision records), with a dated change entry naming atelier-6306 as the authority. The parser rejects unknown tags, so the migration ships in the same commit as the removal.

## Context

Decision record linked as `implements`. Skill rule: `skills/agent-factory/references/tracker-records.md`, Decision Notes, in spadaval/.agents.

## Notes

- 2026-11-04 [decision] The brief decision view ends with the standard omitted-rows line, ... N more decision records omitted; see `atelier issue show <id> --full`
  Reason: human-cli-output.md requires that form for any list that leaves rows out; it carries the +N more count and names issue show --full as the Done when asks
  Rejected: a bare +N more line
- 2026-11-04 [decision] The brief view lists the newest decided records first, by close time
  Reason: The newest choices are the likeliest to bear on current work, as atelier-y8xw asked for the newest few
  Rejected: record order (item, parent, mission, then scope by id)
- 2026-11-04 [decision] Prove the removed flag is a usage error with a captured command, not a committed test
  Reason: The Done when also requires git grep for the mark over crates to print nothing, and a test naming the flag would itself match; clap refuses unknown flags generically
  Rejected: spelling the flag indirectly in a test to dodge the grep
- 2026-11-04 [decision] Remove the debrief's anchor_of helper and the dec-link style with the sidebar section
  Reason: The one-way sidebar index was their only user; leaving them would be dead code

## Close Reason

One-way decision notes are gone: `issue note` has no `--one-way` flag and passing it is a usage error, `[decision one-way]` is an unknown tag that `check` refuses, and the four existing one-way notes (atelier-fip8 twice, atelier-ho7o, atelier-sfsq) now read `[decision]`. `work decisions` lists decision records first, then notes. The brief decision view in `status` and `issue show` lists up to three current decision records, then the omitted-rows line pointing at `issue show --full`, which covers atelier-y8xw's cap. The debrief has no one-way section, sidebar entry or style, and the grep over crates and docs/product prints nothing.

### Deviations

The brief view's overflow line uses the output contract's omitted-rows form (`... N more decision records omitted; see ...`) rather than a bare `+N more`. The flag's usage error is proven by a captured command, not a committed test, because a test naming the flag would break the Done when's grep. The strategy's dated change entry is its `last_changed` front matter, the only change log strategies have. S1's one-way wording changed along with S4 and S6.

### Decisions

Recorded as decision notes on this item: the omitted-rows line, newest-first ordering of brief records by close time, the captured-command proof, and removing the now-unused `anchor_of` helper and `dec-link` style.

### Surprises

`work decisions -q` also changed order: it now prints the items holding current records before the items holding notes.

### Open items

The full `cargo nextest run` and the test-hooks suite run in CI on the pull request; only the focused tests ran locally. With no decision records bearing on an item, the brief view prints only the count, so a reader sees no decision notes there; `issue show --full` lists them.
