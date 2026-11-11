---
created_at: "2026-10-17T01:10:57.961551559+00:00"
id: "atelier-k55o"
issue_type: "bug"
labels:
- "cli-output"
- "issue-commands"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
  - kind: "issue"
    id: "atelier-w4ef"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Refuse to create work items without a real Goal and Done when"
updated_at: "2026-11-07T21:03:33.627830447+00:00"
---

## Goal

`atelier new "x"` writes a draft item with empty Goal, Done when and Scope sections, `atelier ready` makes it ready as it stands, and `atelier check` passes it (checked 2026-11-11 with the new binary in a scratch clone). `submit` asks only that Done when lines be ticked, so an empty Done when has nothing to tick and passes. Refuse the empty sections when the item is made ready, where it is cheapest to fix.

## Done when

- [ ] `atelier ready` refuses an item without a real Goal and Done when (a mission without an Outcome; Success criteria stay optional) and writes nothing; the error names the missing section. Drafts are exempt: `new` writes empty sections for the planner to fill.
- [ ] CLI tests cover `ready` on an item with empty sections, with a Goal only, and with both sections written.

## Context

The refusal can come from atelier-ilba's write guard once atelier-w4ef makes the whole-repository `check` apply the placeholder rule, or from `create` itself, as `issue update --body` checks its sections today (crates/atelier-cli/src/commands/issue.rs). Dropped in decision atelier-mzpn: refusing `atelier start` on a placeholder record. Also dropped: counting a Done when with no boxed lines as unticked at `review request`, which atelier-jnrh considered and rejected (decision note: a prose Done when has nothing to tick, and requiring boxes everywhere would be a new rule).

Rewritten for the new tool in the cutover sort (atelier-5zre): the refusal moved from `issue create` to `ready`, since the new `new` takes only a title and writes empty sections by design, and the decision type is gone.

## Unknowns

Whether backlog items may be filed with placeholders (capture now, refine later); atelier-w4ef follows the same answer for `check`.
