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
title: "Let agents write tracker records safely, one at a time, by section or as a bundle"
updated_at: "2026-11-05T06:07:19.198719+00:00"
---

## Outcome

An agent can create and change tracker records without shell quoting: one at a time, one section at a time, or many at once as a bundle. Every record-writing command refuses bad input before writing anything and names the entry and field at fault, or, where the edit is allowed but risky (a done transition or an edit to a branch-owned record on the base), warns at once. On success it confirms exactly what it wrote and the next step. `bundle preview` refuses exactly what `bundle apply` refuses.

## Success criteria

- refuse-then-confirm: for `issue create`, `update`, `note`, `link`, `unlink`, `transition` and `bundle apply`, a table-driven CLI test shows that invalid input writes nothing, exits non-zero and names the field at fault, and that valid input prints the resulting values.
- S6: Claude Code transcripts recorded after S1-S5 land (the former `issue show`/`list` and record-writing criteria) show `issue show` and `issue list` piped through head, grep, sed or tail in under a third of runs. Shown by a dated note with the counts.

## Background

Regrouped on 2026-11-05 at the human's direction: missions are organised by outcome, not by the command surface or the audit that found their items. This mission now holds record writing. It took the bundle items from atelier-2mky (atelier-guew, atelier-x3ru, atelier-z6vj, atelier-dpvz, atelier-6esg, atelier-h0nt, atelier-woul, atelier-vsh8, atelier-0cal, atelier-0xcr, atelier-1oez), and the unassigned atelier-hat4. Its `issue show`/`list` items moved to atelier-l63c, and its `start` items to atelier-4pzj. The former S1-S5 restated their items and were replaced under ADR 0024 by refuse-then-confirm. S6 is kept until Sumeet decides it: it is a post-ship measurement, which ADR 0024 keeps out of missions, and its S1-S5 meant the `issue show`/`list` items (now atelier-l63c's) and the record-writing items still here. The open question atelier-o3lm left, a Markdown input format for records, moved here from atelier-2mky with the bundle items: it decides atelier-rz2l.

Split out of atelier-o3lm on 2026-10-16 at the human's direction. Its items come from that mission's six-surface CLI audit; see atelier-o3lm's Background for the method and the transcript scripts. Section edits (atelier-cj54) and record import (atelier-rz2l) come from the 2026-10-16 transcript retrospective. Agents also edit records directly; that stays supported for bulk work. The output contract (atelier-o3lm S1, atelier-i2qq) and ADR 0022 (atelier-bw0b) govern how output looks.
