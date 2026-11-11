# Notes of atelier-4pzj

What the old mission record held beyond its mission sections.

## Background

Revised on 2026-11-07 in decision atelier-mzpn, Sumeet's approval of the triage of the six draft missions against the simpler lifecycle (atelier-mmpv). This mission took in atelier-l63c, which showed where things stand: the two shared renderers and had to run one after the other, and l63c had no success criteria of its own. Its 18 surviving items moved here, with atelier-efnf and atelier-abdp from the superseded atelier-ll3p; the Outcome now says both halves. Superseded in the same decision: atelier-e151 (no type-named branch left to guess), atelier-cy58, atelier-5q5v, atelier-7emi and atelier-fdxj (views that did not earn their place; atelier-om6l covers what holds work up), and from l63c atelier-dvnv, atelier-jbwi, atelier-kglz and atelier-oyzb (already done). atelier-46nq merged into atelier-om6l, atelier-majk into atelier-avx0, and atelier-t18i into atelier-yzuv and atelier-x3ru. Items that rewrite one renderer (`status`, `work mission`, `issue list`, `issue show`) run one after the other. With about 33 items the mission is large; when it is made ready, leaving the P3 polish unassigned is the first cut to consider. l63c's items came from the 2026-10-17 retrospective of the 10-15..10-17 runs, where orchestrator wake-ups re-reading their whole context were the largest single line of spend (about 1.4B cache-read tokens), and from the CLI audit of atelier-o3lm.

Regrouped on 2026-11-05 at the human's direction: missions are organised by outcome, not by the command surface or the audit that found their items. This mission keeps the choosing-work part of the old status-and-work-views mission and takes `start`'s refusals and ready marks from atelier-uf97 (atelier-hugc, atelier-oxnw, atelier-xw8r, atelier-l9ty, atelier-e151, atelier-501y). Status facts, the Mission Overview entry, truncation and view tidiness moved to atelier-l63c. The former S1-S4 restated their items and were replaced under ADR 0024 by ready-means-startable, which keeps the lesson of planning findings atelier-ub58 and atelier-8n3f: in-flight rows stay listed with a mark (atelier-mfyd). atelier-fdxj's revisit condition (atelier-stsa done) is met. The 2026-11-05 currency check found atelier-03kt and atelier-8zxh already fixed, and atelier-31l2's remaining test moved into atelier-5qes.

Split out of atelier-o3lm on 2026-10-16 at the human's direction. Its items come from that mission's six-surface CLI audit, which built on the human's review of `work missions` and `work mission`; see atelier-o3lm's Background for the method and the transcript scripts. Unblock ranking (atelier-fdxj) was deferred out of atelier-1w8c. The output contract (atelier-o3lm S1, atelier-i2qq) and ADR 0022 (atelier-bw0b) govern how output looks. Where an item here rewrites the same renderer as an atelier-1w8c item, it is blocked by it. atelier-lkuy carries the part of atelier-1w8c's S3 that its validation found unmet: `work missions` spends four lines per mission.

Re-scoped on 2026-11-04 at the human's direction: the session-start and digest items (former S6) moved to atelier-l63c; the `prime` removal (atelier-jjvc) and custom views (atelier-e95f, former S7) stand alone; the transcript measure (former S5) was dropped under ADR 0024. The ordering, truncation, COLUMNS and ready-work correctness items that no criterion named joined S1 and S4.

## Notes

- 2026-11-07 [plan] Revised on Sumeet's authority (decision atelier-mzpn, approved 2026-11-07): took in atelier-l63c's 18 surviving items and atelier-efnf and atelier-abdp from atelier-ll3p, superseded 6 of its own items (one merged into atelier-om6l), revised atelier-mi3x, atelier-501y and atelier-9bm2 to the simpler lifecycle, and widened the Outcome and title to where things stand. The criterion is unchanged.

## Links

Relationships of the old record that the new format has no field for.

- advances: atelier-03kt (Point wrong-kind work mission and work parent errors at the right view)
- advances: atelier-31l2 (Order work mission and work parent rows by the shared work order)
- advances: atelier-46nq (Say what each Needs a Human item is and what it holds up)
- advances: atelier-5q5v (Show parent and child structure in work mission)
- advances: atelier-7emi (Show retired missions as retired and flag the work they leave stranded)
- advances: atelier-8zxh (Stop work ready silently dropping rows past 20)
- advances: atelier-cy58 (Show each row's mission and parent in repo-wide work views)
- advances: atelier-fdxj (Show what to do next: blocker chains and the work each item unblocks)
