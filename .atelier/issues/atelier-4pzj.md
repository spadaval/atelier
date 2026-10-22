---
created_at: "2026-10-17T01:44:17.818841106+00:00"
id: "atelier-4pzj"
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
    id: "atelier-03kt"
    type: "advances"
  - kind: "issue"
    id: "atelier-1a5k"
    type: "advances"
  - kind: "issue"
    id: "atelier-1ci8"
    type: "advances"
  - kind: "issue"
    id: "atelier-31l2"
    type: "advances"
  - kind: "issue"
    id: "atelier-46nq"
    type: "advances"
  - kind: "issue"
    id: "atelier-5q5v"
    type: "advances"
  - kind: "issue"
    id: "atelier-5qes"
    type: "advances"
  - kind: "issue"
    id: "atelier-5zta"
    type: "advances"
  - kind: "issue"
    id: "atelier-7emi"
    type: "advances"
  - kind: "issue"
    id: "atelier-8g9d"
    type: "advances"
  - kind: "issue"
    id: "atelier-8zxh"
    type: "advances"
  - kind: "issue"
    id: "atelier-9bm2"
    type: "advances"
  - kind: "issue"
    id: "atelier-cy58"
    type: "advances"
  - kind: "issue"
    id: "atelier-d2xe"
    type: "advances"
  - kind: "issue"
    id: "atelier-d4b6"
    type: "advances"
  - kind: "issue"
    id: "atelier-dvnv"
    type: "advances"
  - kind: "issue"
    id: "atelier-fdxj"
    type: "advances"
  - kind: "issue"
    id: "atelier-ftqn"
    type: "advances"
  - kind: "issue"
    id: "atelier-lkuy"
    type: "advances"
  - kind: "issue"
    id: "atelier-mfyd"
    type: "advances"
  - kind: "issue"
    id: "atelier-mi3x"
    type: "advances"
  - kind: "issue"
    id: "atelier-n831"
    type: "advances"
  - kind: "issue"
    id: "atelier-om6l"
    type: "advances"
  - kind: "issue"
    id: "atelier-oyzb"
    type: "advances"
  - kind: "issue"
    id: "atelier-tf9n"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Make status and work views show what holds work up and what to do next"
updated_at: "2026-10-17T01:44:17.818841106+00:00"
---

## Outcome

`status` and every `work` view show, for each item:
- its direct blockers, with inherited ones marked;
- its mission and parent.

Section counts sit in the headings, and no row is dropped without naming the flag that shows it. `work mission` shows the mission's outcome, what holds it up, and its parent and child structure. `status` reports checkout facts in words and chooses its next actions from the checkout's state. Ready work shows which item unblocks the most. This mission follows atelier-1w8c, which makes `status` the session view and sets the one-line row (atelier-deyy). It took over atelier-1w8c's remaining items and its transcript measure when that mission closed on 2026-10-22.

## Success criteria

- S1: Work-view rows show direct blockers (inherited ones marked), mission and parent, in one shared blocker-first order. Each section has its count in the heading. No rows are silently dropped. Errors for the wrong record kind point at the right view (atelier-tf9n, atelier-cy58, atelier-5qes, atelier-5zta, atelier-8zxh, atelier-oyzb, atelier-1a5k, atelier-03kt). Shown by rendering tests.
- S2: The `work mission` header shows the outcome, the mission's own blockers and its next transition. The view has a Waiting on section and shows parent and child structure. The Mission Overview entry reads in plain terms. Needs a Human items say what they hold up. Draft and retired missions are classified consistently (atelier-9bm2, atelier-om6l, atelier-5q5v, atelier-lkuy, atelier-46nq, atelier-7emi, atelier-d4b6). Shown by fixture tests that include cross-mission and unassigned blockers.
- S3: `status` shows active work once with its configured status, and names the missions behind its mission count. The owner block shows every scope item. Status warns when land would fail, renders other branches as a dense section, chooses Next Actions from the checkout's state, and reports branch facts in words (atelier-1ci8, atelier-8g9d, atelier-d2xe, atelier-dvnv, atelier-ftqn, atelier-mi3x, atelier-n831). Shown by session tests.
- S4: Ready work shows blocker chains and how much each item unblocks (atelier-fdxj). Shown by a fixture test.
- S5: Claude Code transcripts recorded after S1-S4 land show `status` and `work` views piped through head, grep, sed or tail in under a third of runs. Shown by a dated note with the counts.
- S6: Session start shows recent tracker activity and a since-commit digest of a mission. In-flight marks are computed without reading each branch's whole tracker. `work missions` shows planned versus built work and backlog health. Docs that exist only on unmerged branches are shown. The deprecated `prime` alias is removed once adopters' hooks stop calling it (atelier-majk, atelier-avx0, atelier-3i7x, atelier-jbwi, atelier-kglz, atelier-jjvc). Shown by session and CLI tests, and a timing recorded as evidence for atelier-3i7x.
- S7: Repositories can define custom work views declaratively (atelier-e95f). Shown by a CLI test.

## Background

Split out of atelier-o3lm on 2026-10-16 at the human's direction. Its items come from that mission's six-surface CLI audit, which built on the human's review of `work missions` and `work mission`; see atelier-o3lm's Background for the method and the transcript scripts. Unblock ranking (atelier-fdxj) was deferred out of atelier-1w8c. The output contract (atelier-o3lm S1, atelier-i2qq) and ADR 0022 (atelier-bw0b) govern how output looks. Where an item here rewrites the same renderer as an atelier-1w8c item, it is blocked by it. S5 also carries atelier-1w8c's S6, whose before-measure was about four runs in five piping atelier output. atelier-lkuy carries the part of atelier-1w8c's S3 that its validation found unmet: `work missions` spends four lines per mission.
