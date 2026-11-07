---
created_at: "2026-11-04T23:03:20.457889128+00:00"
id: "atelier-l63c"
issue_type: "mission"
labels:
- "cli-output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T21:05:55.883182347+00:00"
status: "superseded"
title: "Let agents and the human see where things stand without rebuilding it from git"
updated_at: "2026-11-07T21:05:55.883182347+00:00"
---

## Outcome

Agents and the human learn where things stand from Atelier's own views: the checkout, each mission, each item (its owner branch, proof, review and links), and what changed recently. Every view is complete or names the command that shows the rest, and reads the same when piped. Nobody has to rebuild this from `git log`, from several commands, or by re-reading a whole mission's context.

## Background

Regrouped on 2026-11-05 at the human's direction: missions are organised by outcome, not by the command surface or the audit that found their items. This mission widened from what changed and what is in flight to where things stand. It took the status facts, Mission Overview, truncation and view-tidiness items from atelier-4pzj (atelier-n831, atelier-1ci8, atelier-d2xe, atelier-8g9d, atelier-lkuy, atelier-pvo0, atelier-o1bv, atelier-5zta, atelier-1a5k, atelier-oyzb, atelier-d4b6, atelier-dvnv). It also took the `issue show` and `issue list` items from atelier-uf97 (atelier-jcki, atelier-pu2i, atelier-5qep, atelier-yzuv, atelier-lehs, atelier-t18i, atelier-dmh5), and atelier-x0vs. Brief `issue show` with several IDs (atelier-7u4o, done) is what atelier-jcki builds on. atelier-fpxc (text search) stays outside any mission, as Sumeet deferred it on 2026-10-17. Where this mission and atelier-4pzj change the same renderer (`status`, `work mission`, `issue list`), they run one after the other.

Re-shaped on 2026-11-04 under ADR 0024. It began as the follow-on criteria split out of atelier-4pzj, which mixed this theme with a transcript measure, the `prime` removal and custom work views. The transcript measure was dropped, and atelier-3i7x, atelier-jjvc and atelier-e95f stand alone, because none of them shares this outcome and two wait on things this mission's work cannot finish. The items come from the 2026-10-17 retrospective of the 10-15..10-17 runs, where orchestrator wake-ups re-reading their whole context were the largest single line of spend (about 1.4B cache-read tokens).

## Report

Merged into atelier-4pzj in decision atelier-mzpn: the two shared renderers, and l63c had no criteria of its own. Its 18 surviving items moved to 4pzj; atelier-dvnv, jbwi, kglz and oyzb are superseded, atelier-majk merged into avx0, and atelier-t18i into yzuv and x3ru.
