---
schema: atelier.review
schema_version: 2
id: atelier-0rem
title: finding on atelier-l58v @7ac8f4eb
target: atelier-l58v
identity: claude-code-rereviewer (opus-5.5)
commit: 7ac8f4eb21e5e6fcda105e20e7e879695fb82b0e
created_at: 2026-11-08T22:23:14.682820006Z
event:
  event: finding
  recommendation: fix_now
---

low: .atelier/issues/atelier-l58v.md:41 (Done when line 9, narrowed at cc009eb6) and the Report's Open items. The narrowing is acceptable (the strategy's criteria do not include this bar, pju0 rebuilds the fixture, costly-hardening still shows 0/3 vs 3/3), but its stated reason is not true of every run: it says the fixture 'carries unplanted defects that every run fixes or declines over'. New rerun run 1 (~/.cache/af-evals/runs/l58v-fq99-new/*-opus-1-cf46, final.md) declined over the planted fix_now itself, without making the one-line fix, citing the stop signals 'a fix drawing findings on its own changes' and the user's 'third round on clock skew'. That is the skill's stop signal over-firing, and it has now happened in 2 of 6 new-skill runs (old run 2, rerun run 1) and 0 of 6 main runs. The Report's Open items cite only the old run, so the one observed cost of orchestrate's new rules, which ship with no positive eval evidence now, is understated. Also stale in the Report: Open items 'Review outstanding ... has not been requested'; Deviations 'review and orchestrate paths are 19 and 12 lines over' (branch now 19/10, merged with c2ed148 22/13, plan 3/6). Fix: reword the narrowed line's reason (unplanted defects drew most runs; one new-skill run declined over the planted fix_now), say in Open items that stop-signal over-application recurred 2/6 and that pju0's rerun should watch for it, and refresh the stale lines.
