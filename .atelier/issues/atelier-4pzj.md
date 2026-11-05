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
    id: "atelier-7emi"
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
    id: "atelier-fdxj"
    type: "advances"
  - kind: "issue"
    id: "atelier-mfyd"
    type: "advances"
  - kind: "issue"
    id: "atelier-mi3x"
    type: "advances"
  - kind: "issue"
    id: "atelier-om6l"
    type: "advances"
  - kind: "issue"
    id: "atelier-tf9n"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Let agents tell which work they can start, what holds the rest up, and what to do next"
updated_at: "2026-11-05T06:07:19.198719+00:00"
---

## Outcome

An agent or the human can tell from `status`, the `work` views and `start` which work can be taken now, what holds the rest up, and what to do next. `start` takes what those views offer as ready, and refuses anything else with one line per blocker or reason.

## Success criteria

- ready-means-startable: in a fixture repository with in-mission, cross-mission and unassigned blockers, parked items, and an item in flight on another branch, every ready row `status` and `work ready` show without a mark is accepted by `start`; every item `start` refuses is either not offered as ready or carries the mark that explains why; and a refusal for a blocker names the same blocker that `work blocked` and `issue list` show. Shown by one scenario test.

## Background

Regrouped on 2026-11-05 at the human's direction: missions are organised by outcome, not by the command surface or the audit that found their items. This mission keeps the choosing-work part of the old status-and-work-views mission and takes `start`'s refusals and ready marks from atelier-uf97 (atelier-hugc, atelier-oxnw, atelier-xw8r, atelier-l9ty, atelier-e151, atelier-501y). Status facts, the Mission Overview entry, truncation and view tidiness moved to atelier-l63c. The former S1-S4 restated their items and were replaced under ADR 0024 by ready-means-startable, which keeps the lesson of planning findings atelier-ub58 and atelier-8n3f: in-flight rows stay listed with a mark (atelier-mfyd). atelier-fdxj's revisit condition (atelier-stsa done) is met. The 2026-11-05 currency check found atelier-03kt and atelier-8zxh already fixed, and atelier-31l2's remaining test moved into atelier-5qes.

Split out of atelier-o3lm on 2026-10-16 at the human's direction. Its items come from that mission's six-surface CLI audit, which built on the human's review of `work missions` and `work mission`; see atelier-o3lm's Background for the method and the transcript scripts. Unblock ranking (atelier-fdxj) was deferred out of atelier-1w8c. The output contract (atelier-o3lm S1, atelier-i2qq) and ADR 0022 (atelier-bw0b) govern how output looks. Where an item here rewrites the same renderer as an atelier-1w8c item, it is blocked by it. atelier-lkuy carries the part of atelier-1w8c's S3 that its validation found unmet: `work missions` spends four lines per mission.

Re-scoped on 2026-11-04 at the human's direction: the session-start and digest items (former S6) moved to atelier-l63c; the `prime` removal (atelier-jjvc) and custom views (atelier-e95f, former S7) stand alone; the transcript measure (former S5) was dropped under ADR 0024. The ordering, truncation, COLUMNS and ready-work correctness items that no criterion named joined S1 and S4.
