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
status: "backlog"
title: "Remove one-way decision notes and migrate the existing ones"
updated_at: "2026-10-30T06:52:24.678869557+00:00"
---

## Goal

Carry out the decision to remove one-way decision notes: one way to record a costly-to-reverse choice, the `decision` record.

## Done when

`atelier issue note --help` has no `--one-way`, and passing it is a usage error. `work decisions` lists decision records first, then notes, with no one-way group. The brief decision view that `status` and `issue show` print (`decision_brief_lines`) shows the current decision records bearing on the item, at most three, then `+N more` pointing at `issue show --full`, which also does atelier-y8xw's cap. The debrief page has no one-way section, sidebar entry or `.oneway` style. The four one-way notes (atelier-fip8 lines 57 and 61, atelier-ho7o, atelier-sfsq) read `[decision]`; fip8's Design text and any note text that quotes the tag stay as written. `git grep -n -i "one-way\|one_way\|oneway" crates docs/product` prints nothing. `atelier check`, `cargo nextest run` and the test-hooks suite pass.

## Scope

crates/atelier-core (note tag and parser), atelier-app decisions.rs and mission_review.rs, atelier-cli issue_cli.rs, issue.rs, work.rs, mission_review.rs, review_page.rs, human_output.rs and main.rs (help text); docs/product/work-model.md, cli-surface.md, human-cli-output.md, mission-reviews.md; docs/strategies/review-and-introspection.md (atelier-r6bt's strategy): the one-way lines in its principles, S4 ("decision log with one-way decisions first" becomes decision records first) and S6 ("the count and the one-way decisions" becomes the count and up to three decision records), with a dated change entry naming atelier-6306 as the authority. The parser rejects unknown tags, so the migration ships in the same commit as the removal.

## Context

Decision record linked as `implements`. Skill rule: `skills/agent-factory/references/tracker-records.md`, Decision Notes, in spadaval/.agents.
