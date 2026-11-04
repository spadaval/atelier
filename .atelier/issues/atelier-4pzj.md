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

Section counts sit in the headings, and no row is dropped without naming the flag that shows it. `work mission` shows the mission's outcome, what holds it up, and its parent and child structure. `status` reports checkout facts in words and chooses its next actions from the checkout's state. Ready work shows which item unblocks the most. This mission follows atelier-1w8c, which makes `status` the session view and sets the one-line row (atelier-deyy). It took over atelier-1w8c's remaining items when that mission closed on 2026-10-22.

## Success criteria

- S1: Work-view rows show direct blockers (inherited ones marked), mission and parent, in one shared blocker-first order. Each section has its count in the heading. No rows are silently dropped. Errors for the wrong record kind point at the right view. Truncations name the command that shows the rest, and redirected output ignores COLUMNS (atelier-tf9n, atelier-cy58, atelier-5qes, atelier-31l2, atelier-5zta, atelier-8zxh, atelier-pvo0, atelier-o1bv, atelier-oyzb, atelier-1a5k, atelier-03kt). Shown by rendering tests.
- S2: The `work mission` header shows the outcome, the mission's own blockers and its next transition. The view has a Waiting on section and shows parent and child structure. The Mission Overview entry reads in plain terms. Needs a Human items say what they hold up. Draft and retired missions are classified consistently (atelier-9bm2, atelier-om6l, atelier-5q5v, atelier-lkuy, atelier-46nq, atelier-7emi, atelier-d4b6). Shown by fixture tests that include cross-mission and unassigned blockers.
- S3: `status` shows active work once with its configured status, and names the missions behind its mission count. The owner block shows every scope item. Status warns when land would fail, chooses Next Actions from the checkout's state, and reports branch facts in words (atelier-1ci8, atelier-8g9d, atelier-d2xe, atelier-dvnv, atelier-mi3x, atelier-n831). Shown by session tests.
- S4: Ready work lists exactly the items `start` would accept, net of items in flight on other branches, with one readiness gate. It shows blocker chains and how much each item unblocks (atelier-hueh, atelier-mfyd, atelier-b17k, atelier-fdxj). Shown by fixture tests.

## Background

Split out of atelier-o3lm on 2026-10-16 at the human's direction. Its items come from that mission's six-surface CLI audit, which built on the human's review of `work missions` and `work mission`; see atelier-o3lm's Background for the method and the transcript scripts. Unblock ranking (atelier-fdxj) was deferred out of atelier-1w8c. The output contract (atelier-o3lm S1, atelier-i2qq) and ADR 0022 (atelier-bw0b) govern how output looks. Where an item here rewrites the same renderer as an atelier-1w8c item, it is blocked by it. atelier-lkuy carries the part of atelier-1w8c's S3 that its validation found unmet: `work missions` spends four lines per mission.

Re-scoped on 2026-11-04 at the human's direction: the transcript measure, the session-start and digest items, the `prime` removal and custom views (former S5-S7) moved to atelier-l63c, because they wait on calendar time or on this mission's result. The ordering, truncation, COLUMNS and ready-work correctness items that no criterion named joined S1 and S4.
