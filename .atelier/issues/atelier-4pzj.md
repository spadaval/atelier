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
title: "Let agents and the human see where things stand, which work they can start, and what to do next"
updated_at: "2026-11-07T21:05:42.397074201+00:00"
---

## Outcome

An agent or the human learns where things stand from Atelier's own views, without rebuilding it from `git log` or several commands: the checkout, each mission and each item (its branch, review state and links), which work can be started now, what holds the rest up, what changed since a given commit, and what to do next. `start` takes what those views offer as ready, and refuses anything else with one line per blocker or reason. Every view is complete or names the command that shows the rest, and reads the same when piped.

## Success criteria

- ready-means-startable: in a fixture repository with in-mission, cross-mission and unassigned blockers, parked items, and an item in flight on another branch, every ready row `status` and `work ready` show without a mark is accepted by `start`; every item `start` refuses is either not offered as ready or carries the mark that explains why; and a refusal for a blocker names the same blocker that `work blocked` and `issue list` show. Shown by one scenario test.

## Background

Revised on 2026-11-07 in decision atelier-mzpn, Sumeet's approval of the triage of the six draft missions against the simpler lifecycle (atelier-mmpv). This mission took in atelier-l63c, which showed where things stand: the two shared renderers and had to run one after the other, and l63c had no success criteria of its own. Its 18 surviving items moved here, with atelier-efnf and atelier-abdp from the superseded atelier-ll3p; the Outcome now says both halves. Superseded in the same decision: atelier-e151 (no type-named branch left to guess), atelier-cy58, atelier-5q5v, atelier-7emi and atelier-fdxj (views that did not earn their place; atelier-om6l covers what holds work up), and from l63c atelier-dvnv, atelier-jbwi, atelier-kglz and atelier-oyzb (already done). atelier-46nq merged into atelier-om6l, atelier-majk into atelier-avx0, and atelier-t18i into atelier-yzuv and atelier-x3ru. Items that rewrite one renderer (`status`, `work mission`, `issue list`, `issue show`) run one after the other. With about 33 items the mission is large; when it is made ready, leaving the P3 polish unassigned is the first cut to consider. l63c's items came from the 2026-10-17 retrospective of the 10-15..10-17 runs, where orchestrator wake-ups re-reading their whole context were the largest single line of spend (about 1.4B cache-read tokens), and from the CLI audit of atelier-o3lm.

Regrouped on 2026-11-05 at the human's direction: missions are organised by outcome, not by the command surface or the audit that found their items. This mission keeps the choosing-work part of the old status-and-work-views mission and takes `start`'s refusals and ready marks from atelier-uf97 (atelier-hugc, atelier-oxnw, atelier-xw8r, atelier-l9ty, atelier-e151, atelier-501y). Status facts, the Mission Overview entry, truncation and view tidiness moved to atelier-l63c. The former S1-S4 restated their items and were replaced under ADR 0024 by ready-means-startable, which keeps the lesson of planning findings atelier-ub58 and atelier-8n3f: in-flight rows stay listed with a mark (atelier-mfyd). atelier-fdxj's revisit condition (atelier-stsa done) is met. The 2026-11-05 currency check found atelier-03kt and atelier-8zxh already fixed, and atelier-31l2's remaining test moved into atelier-5qes.

Split out of atelier-o3lm on 2026-10-16 at the human's direction. Its items come from that mission's six-surface CLI audit, which built on the human's review of `work missions` and `work mission`; see atelier-o3lm's Background for the method and the transcript scripts. Unblock ranking (atelier-fdxj) was deferred out of atelier-1w8c. The output contract (atelier-o3lm S1, atelier-i2qq) and ADR 0022 (atelier-bw0b) govern how output looks. Where an item here rewrites the same renderer as an atelier-1w8c item, it is blocked by it. atelier-lkuy carries the part of atelier-1w8c's S3 that its validation found unmet: `work missions` spends four lines per mission.

Re-scoped on 2026-11-04 at the human's direction: the session-start and digest items (former S6) moved to atelier-l63c; the `prime` removal (atelier-jjvc) and custom views (atelier-e95f, former S7) stand alone; the transcript measure (former S5) was dropped under ADR 0024. The ordering, truncation, COLUMNS and ready-work correctness items that no criterion named joined S1 and S4.

## Notes

- 2026-11-07 [plan] Revised on Sumeet's authority (decision atelier-mzpn, approved 2026-11-07): took in atelier-l63c's 18 surviving items and atelier-efnf and atelier-abdp from atelier-ll3p, superseded 6 of its own items (one merged into atelier-om6l), revised atelier-mi3x, atelier-501y and atelier-9bm2 to the simpler lifecycle, and widened the Outcome and title to where things stand. The criterion is unchanged.
