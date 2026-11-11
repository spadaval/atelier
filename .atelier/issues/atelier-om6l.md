---
created_at: "2026-10-17T01:10:57.988066400+00:00"
id: "atelier-om6l"
issue_type: "feature"
labels:
- "cli-output"
- "work-views"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show what holds a mission up: a Waiting on section, what Needs a Human items hold, and the overview's Waiting on line"
updated_at: "2026-11-07T21:03:27.350492537+00:00"
---

## Goal

`atelier show <mission>` and `atelier list` name each item's direct blockers with their status, but not their titles, the end of a blocker chain, or what each blocker holds, so "what is holding this mission up" takes one `show` per ID. With atelier-1a5k blocked by atelier-1ci8, itself blocked by atelier-5qes (links set for the test in a scratch clone), `list` shows only `blocked by atelier-1ci8, which is draft` on atelier-1a5k (checked 2026-11-11 with the new binary in a converted scratch clone).

The same facts answer two smaller gaps: needs-human rows (`list --needs-human`, and "Waiting on a human" in `status`) do not say what an item holds up, so a human glancing at them cannot tell what to do first (atelier-46nq, merged here), and the heading `list` gives each mission has no line for what it waits on (from atelier-lkuy).

## Done when

- [ ] `show <mission>` opens with a section listing each open item at the end of a blocker chain (no open blockers of its own) that holds mission work, with its state, priority, title, mission (or "no mission", or "mission X dropped"), a ⚠ when it needs a human, and what it holds. Chains deeper than 1 print as a path. For the fixture:
  ```
  Waiting on (5 hold 7 items)
    ⚠ atelier-spvd  needs a human  medium  Pick widget wire format
        holds atelier-qhnh, atelier-ly7a
    ⚠ atelier-97yk  needs a human  critical  Approve widget vendor account — holds atelier-1jl2
    atelier-ubti  ready · no mission  low  Audit legacy widget names
        holds atelier-il31 → atelier-fhep → atelier-5hp9 → atelier-g0q4
    atelier-9vdd  ready · mission atelier-ba3h dropped  Define gadget schema — holds atelier-1gc5
    atelier-s47h  active  Tokenise widget input — holds atelier-9t9q
  ```
- [ ] Each blocker appears once, however many items it holds. Done or dropped blockers drop out and what they held moves to Ready.
- [ ] Needs-human rows in `list --needs-human`, `status` and `show <mission>` say what each item is and what it holds, e.g. `⚠ atelier-spvd  medium  Pick widget wire format — holds 4 (mission atelier-j76w)`, sorted by how many items each holds, then priority.
- [ ] Each mission's heading in `list` has a `Waiting on:` line naming up to two items at the end of blocker chains with how many each holds, e.g. `Waiting on: atelier-spvd (needs a human, holds 4), atelier-ubti (no mission, holds 3)`, omitted when nothing waits.
- [ ] Rendering tests cover a cross-mission blocker, an unassigned blocker, a needs-human item that blocks others, a 4-deep chain, a done or dropped blocker, and a needs-human item with and without dependents.

## Scope

Read side and rendering of `show <mission>`, `list` and `status`. No change to what counts as ready or blocked.

## Context

Merged in decision atelier-mzpn: atelier-46nq's "holds N" (its "shown for context" label is already gone) and atelier-lkuy's Waiting on line; atelier-fdxj (ranking by unblock count) was superseded there, and this section is what remains of it. atelier-tf9n names each row's direct blockers and the far end of its chain. Astromech transcripts show `atelier work mission astromech-7q0z | head -60` followed by `issue show` on blockers (2 sessions). Fixture: scratch clone with mission atelier-j76w; parent atelier-qhnh (children s47h active, 9t9q blocked_by s47h, 1gc5 blocked_by 9vdd); qhnh blocked_by decision atelier-spvd (needs a human); chain 5hp9 <- fhep <- il31 <- ubti (ubti ready, low, no mission); 9vdd advances superseded mission atelier-ba3h.

Rewritten for the new tool in the cutover sort (atelier-5zre): `work mission`, `work human`, `work ready` and `work missions` became `show <mission>`, `list --needs-human`, `status` and `list`; superseded became dropped; the decision type, parents and inherited blockers went, since the new tool has none; the Goal now quotes the new binary. The fixture above is the old tool's.

## Notes

- 2026-11-04 [decision] Show what holds a mission up as a Waiting on section of work mission, with per-chain detail under --blocked; no new command
  Reason: It is the view people already open, and a separate command or per-item tree would repeat shared blockers
  Rejected: A dedicated 'what do I do to get this mission moving' command
