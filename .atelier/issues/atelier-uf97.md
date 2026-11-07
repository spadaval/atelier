---
created_at: "2026-10-17T01:44:17.636200248+00:00"
id: "atelier-uf97"
issue_type: "mission"
labels:
- "cli-output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4ppu"
    type: "advances"
  - kind: "issue"
    id: "atelier-6gpl"
    type: "advances"
  - kind: "issue"
    id: "atelier-7ebi"
    type: "advances"
  - kind: "issue"
    id: "atelier-8bve"
    type: "advances"
  - kind: "issue"
    id: "atelier-9pa6"
    type: "advances"
  - kind: "issue"
    id: "atelier-ceu3"
    type: "advances"
  - kind: "issue"
    id: "atelier-cj54"
    type: "advances"
  - kind: "issue"
    id: "atelier-e6fl"
    type: "advances"
  - kind: "issue"
    id: "atelier-k55o"
    type: "advances"
  - kind: "issue"
    id: "atelier-rh0d"
    type: "advances"
  - kind: "issue"
    id: "atelier-rz2l"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Let agents write, check and repair tracker records: one at a time, by section or as a bundle"
updated_at: "2026-11-07T21:05:42.626573767+00:00"
---

## Outcome

An agent can create and change tracker records without shell quoting: one at a time, one section at a time, or many at once as a bundle. Each record-writing command confirms exactly what it wrote and the next step, and `bundle preview` refuses exactly what `bundle apply` refuses; refusing an invalid record before it is written is atelier-ilba's, done. `atelier check` applies one rule set however it is called, reports each fault once against the record to fix, says what it checked, and gives a fault that arrives by merge or hand edit a repair route that works while the graph does not load.

## Success criteria

- refuse-then-confirm: for `issue create`, `update`, `note`, `link`, `unlink`, `transition` and `bundle apply`, a table-driven CLI test shows that valid input prints the values written, and an integration test shows that `bundle preview` refuses exactly the inputs `bundle apply` refuses, writing nothing.

## Background

Revised on 2026-11-07 in decision atelier-mzpn, Sumeet's approval of the triage of the six draft missions against the simpler lifecycle (atelier-mmpv). atelier-ilba now refuses an invalid record when it is written, so this mission narrows to confirming what was written and to preview agreeing with apply, and refuse-then-confirm narrowed with it. It took the `check` items of the superseded atelier-2mky (atelier-fb1i, atelier-w4j2, atelier-kset with atelier-ry7d and atelier-4t5l merged in, atelier-sfd9, atelier-w4ef), making it one write, check and repair mission. Superseded in the same decision: atelier-4ppu and atelier-9pa6 (no close step), atelier-bywq (record edits no longer make approvals stale), atelier-6esg (receipts prove nothing) and atelier-rz2l (a fourth way to write records; atelier-sxcj's `--body-file` answers the quoting problem and the Markdown-input question atelier-o3lm left). The warnings for risky edits on the base went with atelier-9pa6 and atelier-bywq. S6 was dropped: a measurement after shipping, which ADR 0024 keeps out of missions; at most a loose end once this mission's work merges. atelier-w4ef, atelier-k55o and atelier-guew were meant to fold into atelier-ilba, which merged without them, so they stand here.

Regrouped on 2026-11-05 at the human's direction: missions are organised by outcome, not by the command surface or the audit that found their items. This mission now holds record writing. It took the bundle items from atelier-2mky (atelier-guew, atelier-x3ru, atelier-z6vj, atelier-dpvz, atelier-6esg, atelier-h0nt, atelier-woul, atelier-vsh8, atelier-0cal, atelier-0xcr, atelier-1oez), and the unassigned atelier-hat4. Its `issue show`/`list` items moved to atelier-l63c, and its `start` items to atelier-4pzj. The former S1-S5 restated their items and were replaced under ADR 0024 by refuse-then-confirm. S6 is kept until Sumeet decides it: it is a post-ship measurement, which ADR 0024 keeps out of missions, and its S1-S5 meant the `issue show`/`list` items (now atelier-l63c's) and the record-writing items still here. The open question atelier-o3lm left, a Markdown input format for records, moved here from atelier-2mky with the bundle items: it decides atelier-rz2l.

Split out of atelier-o3lm on 2026-10-16 at the human's direction. Its items come from that mission's six-surface CLI audit; see atelier-o3lm's Background for the method and the transcript scripts. Section edits (atelier-cj54) and record import (atelier-rz2l) come from the 2026-10-16 transcript retrospective. Agents also edit records directly; that stays supported for bulk work. The output contract (atelier-o3lm S1, atelier-i2qq) and ADR 0022 (atelier-bw0b) govern how output looks.

## Notes

- 2026-11-07 [plan] Revised on Sumeet's authority (decision atelier-mzpn, approved 2026-11-07): took in atelier-2mky's check items, superseded 5 of its own items, revised 5 to the simpler lifecycle, dropped S6 (a post-ship measurement) and narrowed refuse-then-confirm to confirming what was written and preview agreeing with apply, since atelier-ilba now refuses invalid records at write.
