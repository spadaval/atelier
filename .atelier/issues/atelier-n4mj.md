---
created_at: "2026-11-06T21:26:18.039205049+00:00"
id: "atelier-n4mj"
issue_type: "mission"
labels: []
strategy:
  path: docs/strategies/lighter-workflow.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T04:11:07.064478270+00:00"
status: "done"
title: "Record what proves work, when it can be proven, and nothing more"
updated_at: "2026-11-07T04:11:07.064478270+00:00"
---

## Outcome

Atelier records what proves work, at the time it can be proven, and nothing more, while every non-trivial change is still independently reviewed before merge. The strategy, docs/strategies/lighter-workflow.md, owns the outcome, success criteria, shape and authority.

## Close Reason

Atelier now records what proves work, when it can be proven, and little else, with review before merge unchanged. A finding carries its disposition in one record (atelier-hwxm, ADR 0025; old disposition events are still read, so every repository on this host works with the shared binary). Receipts are for checks nothing else records, and no hint, view or command asks agents to record or re-record proof (atelier-s66u, ADR 0026). A check that only use can show goes after merge with a reason and a stand-in, shown in the brief, the views and the debrief, and validated on the base (atelier-n0qh, ADR 0027; atelier-secy fixed that path for reviewed items). A Done when edited after start is shown to the reviewer (atelier-r9hb, from atelier-ll3p). The starter and atelier's own policy drop the `blocked` and `review` statuses (atelier-bqbz). All three criteria are proven at the end (lifecycle-cost, proof-timing, gates-hold), validated independently (receipts atelier-9ojt, atelier-c7vy, atelier-lxf3).

Gate rules removed, and the merge/land tests that changed with them (gates-hold; full list in atelier-lxf3):
- `merge --sync` no longer reruns receipts (ADR 0026): the sync_rerun_failed and sync_rerun_error refusals and their tests are gone; two sync tests now expect no rerun.
- A pass no longer stops clearing an older failure once later commits make it stale (ADR 0026): three session land tests and two close tests that refused now pass; land's "Uncommitted changes make passing proof stale" note is gone.
- The separate disposition event is gone (ADR 0025): tests retarget the finding's file and messages are reworded; every one still refuses.
- Hints asking for fresh proof are gone (ADR 0026): merge_diverged, rebase-recovery and rewrite hints; the "no proof at head" and Proof gaps rows are views, not gates.
- `blocked` and `review` statuses (atelier-bqbz): their tests run on the earlier starter policy and still pass; no gate removed.
No gate refuses that did not refuse before.

### Deviations
- atelier-8s1m (a fix re-check counts as its review) was dropped on Sumeet's direction: re-reviewing a fix is cheap and the replacement needed new rules.
- atelier-5822 (records ride with commits) was dropped on Sumeet's direction: the record-only commit count is a symptom, and amending could rewrite a pushed commit. lifecycle-cost was restated as one review file per finding, with the commit count reported but not targeted.
- After-merge validation became a show-not-enforce path (no new refusals) on Sumeet's direction, and evidence simplification (atelier-s66u) was added after an audit of 1,725 receipts.
- atelier-r9hb was taken out from under atelier-ma4a so it could merge on its own; it still advances atelier-ll3p.
- atelier-secy was found by the mission validation and fixed inside the mission.

### Decisions
- After-merge checks are a marker on the Done when line, not a status or field (atelier-n0qh, ADR 0027).
- Old disposition events are read as folded rather than requiring migration (expand and contract), because one installed binary serves every repository on this host.
- Repositories without CI or a suite merge check (demiurge, cosplay-studio, life-on-rails) lose the merged-tree test `merge --sync` reruns gave; Sumeet accepted this, tracked in atelier-bp3s.
- Evidence capture keeps refusing uncommitted code (Sumeet).

### Surprises
- The installed `atelier` is shared by every repository on this host, so a record format change has to keep reading the old form.
- No policy required a passing receipt any more; almost all receipts were prompted by text.
- Squash-merging or amending to cut record commits would orphan the commits records pin.

### Open items
- For Sumeet's post-merge look: dropping `review` from the starter is a simplicity call (astromech agents use request_review on branches; other repositories keep the status); manual passes can clear a failure without checking the checkout.
- Follow-ups outside the mission: atelier-akkp (re-measure agent effort after release), atelier-bp3s (suite merge checks for repositories without CI), atelier-42th (remove the old disposition read path once every repository has migrated), atelier-8ghm (stale-pass land test assertion).
- The debrief shows receipt summaries cut short, so the full gate list is here and in atelier-lxf3.
- A merged branch, task/atelier-5fus, was briefly deleted on GitHub and restored at PR #27's head; the closed branch feature/atelier-5822 (PR #32) is still on GitHub for Sumeet to keep or delete.
