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
    id: "atelier-501y"
    type: "advances"
  - kind: "issue"
    id: "atelier-5qep"
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
    id: "atelier-dmh5"
    type: "advances"
  - kind: "issue"
    id: "atelier-e151"
    type: "advances"
  - kind: "issue"
    id: "atelier-e6fl"
    type: "advances"
  - kind: "issue"
    id: "atelier-fpxc"
    type: "advances"
  - kind: "issue"
    id: "atelier-hugc"
    type: "advances"
  - kind: "issue"
    id: "atelier-jcki"
    type: "advances"
  - kind: "issue"
    id: "atelier-k55o"
    type: "advances"
  - kind: "issue"
    id: "atelier-l9ty"
    type: "advances"
  - kind: "issue"
    id: "atelier-lehs"
    type: "advances"
  - kind: "issue"
    id: "atelier-oxnw"
    type: "advances"
  - kind: "issue"
    id: "atelier-pu2i"
    type: "advances"
  - kind: "issue"
    id: "atelier-rh0d"
    type: "advances"
  - kind: "issue"
    id: "atelier-rz2l"
    type: "advances"
  - kind: "issue"
    id: "atelier-xw8r"
    type: "advances"
  - kind: "issue"
    id: "atelier-yzuv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Make issue commands and start show where an item stands and confirm what changed"
updated_at: "2026-10-17T01:44:17.636200248+00:00"
---

## Outcome

`issue show` and `issue list` show where an item stands: its owner branch, proof, review and mission, with blocked and needs-human work marked. Commands that change records (create, update, note, link, transition and start) confirm the resulting values and the next step, and refuse incomplete records. Records can be changed one section at a time, or imported from a drafted file, without shell quoting. Text search finds issues.

## Success criteria

- S1: `issue show` shows branch, proof, review and mission state. `issue list` marks blocked and needs-human items, lists open work before done work, and takes several filter values (atelier-jcki, atelier-l9ty, atelier-pu2i, atelier-5qep, atelier-lehs, atelier-yzuv). Shown by CLI tests.
- S2: Mutations confirm the resulting values and the next step. Closing an item says what its done claim still needs before it can merge. Transition options render one per line, and the `--reason` and link-role errors say what to use instead (atelier-7ebi, atelier-8bve, atelier-4ppu, atelier-6gpl, atelier-e6fl, atelier-9pa6, atelier-ceu3, atelier-rh0d). Shown by CLI tests.
- S3: `issue create` refuses work items without a real Done when, and directional links keep their direction (atelier-k55o, atelier-dmh5). Shown by tests.
- S4: `start` shows the item's title, acceptance and mission when it succeeds. A refusal reports one line per blocker. Parked items are refused. Hints point at `atelier start`. Start creates the owner worktree or shows the owner branch (atelier-501y, atelier-oxnw, atelier-hugc, atelier-xw8r, atelier-e151). Shown by CLI tests.
- S5: `issue update --section` replaces one body section, and a drafted record file can be validated and imported. Issues can be searched by text (atelier-cj54, atelier-rz2l, atelier-fpxc). Shown by CLI tests.
- S6: Claude Code transcripts recorded after S1-S5 land show `issue show` and `issue list` piped through head, grep, sed or tail in under a third of runs. Shown by a dated note with the counts.

## Background

Split out of atelier-o3lm on 2026-10-16 at the human's direction. Its items come from that mission's six-surface CLI audit; see atelier-o3lm's Background for the method and the transcript scripts. Section edits (atelier-cj54) and record import (atelier-rz2l) come from the 2026-10-16 transcript retrospective. Agents also edit records directly; that stays supported for bulk work. Text search (atelier-fpxc) was deferred out of atelier-1w8c. The output contract (atelier-o3lm S1, atelier-i2qq) and ADR 0022 (atelier-bw0b) govern how output looks. Brief `issue show` with several IDs is atelier-1w8c's atelier-7u4o; S1 builds on it.
