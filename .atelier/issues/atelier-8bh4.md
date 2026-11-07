---
created_at: "2026-11-07T05:01:00.329959594+00:00"
id: "atelier-8bh4"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-8bh4
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-jnrh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Track post-merge work as loose ends, outside any mission"
updated_at: "2026-11-07T08:34:52.317697757+00:00"
---

## Goal

Replace after-merge checks (a marker on a Done when line, ADR 0027) with a `loose_end` issue type: work that changes no code and can only happen after a merge or a release, such as pushing a tag, a play-check on a device, or an eval run. A loose end waits on what it follows through `blocked_by`, shows up as ready when that merges, and is never part of a mission, so a mission closes when its code merges. The v0.3.1 tag was lost because it lived as prose in a Close Reason; astromech-cxgq (a play-check) shows the linked-issue form already works.

## Done when

- [x] The starter policy and atelier's own define `loose_end`, and `loose_end` and `validation` share one workflow with no `start` transition: `todo` to `done` through a transition named `done` that requires a one-line result in the `Report` section, plus `supersede`. `atelier start` and `atelier merge` refuse an item whose workflow has no `start` transition (judged by workflow, not type name, so a repository that still keeps `validation` in its `work` workflow is unaffected), naming the `done` transition.
- [x] Marking a loose end or a validation done on the base passes `land`: the exemption from `land_base_done_claim` (crates/atelier-cli/src/commands/git.rs) covers any type whose workflow has no `start` transition, in place of the hard-coded `decision` name, and missions stay exempt as today. Tests: a loose end done on the base and a mission closed on the base both pass `land`.
- [x] A loose end `blocked_by` an issue appears in `work ready` once that issue is merged, and in `work human` when labelled `needs-human`; no `status` or `work` hint suggests starting one (tests).
- [x] A loose end cannot advance a mission or be any issue's child (refused when the link is written), and mission close ignores loose ends.
- [x] Removed: stand-in parsing, `carried to`, the awaiting-validation rows and views, receipts held at close, the mission-close warning, and the after-merge part of the review request brief. The marker is still recognised in old records, only so `review request` can skip such lines (atelier-jnrh) and `migrate` can find them; otherwise they show as ordinary Done when lines.
- [x] `atelier migrate after-merge` (dry run unless `--apply`; refuses with a clear message when the policy has no `loose_end`) turns each unticked After merge line into a loose end `blocked_by` its item: the check becomes its Done when, the reason goes in its Goal, and it gets `needs-human` when the reason is the human's judgment or a device or place no agent can reach, which the dry run shows. On an item that is not done, the line's stand-in stays as an ordinary Done when line. Lines carrying `carried to <id>` are skipped. Idempotent; covered by a test.
- [x] ADR 0028's decision 3 paragraph names the shared workflow, the `done` transition and the migration as built; docs/product, CONTEXT.md and ADR 0027's Status match. The agent-factory guidance is left to atelier-ml3r.

## Scope

Workflow policy (starter and atelier's own), issue types, `start`/`merge` refusals, `land`'s base done-claim exemption, link validation, work views and hints, review request brief, mission close, `migrate`, docs, ADR 0028. Must not change: how ordinary work items merge. Other repositories keep working with the shared binary: policies without `loose_end` and records with the marker still parse; migrating them is atelier-ml3r's job.

## Context

docs/adr/0028-simpler-lifecycle.md (decision 3), docs/adr/0027-proof-has-a-time.md (the machinery being removed), docs/strategies/simpler-lifecycle.md (Shape: Loose ends). The `Report` section comes from atelier-jnrh. What a loose end waits on after a change with no issue is atelier-4nsl's question. Whether astromech's per-feature `validation` play-checks become loose ends is decided when atelier-ml3r migrates it.

## Report

Loose ends replace after-merge checks. The starter policy and atelier's own declare `loose_end` and give it and `validation` one `on_base` workflow: `done` (todo to done, requires a non-empty Report, runs `blockers.none_open`) plus `supersede`, no `start`. Any item whose workflow has no `start` transition is refused by `start` and `merge` (`start_done_on_base`, `merge_done_on_base`, naming its done transition) and is exempt from `land_base_done_claim`; missions stay exempt by type. A loose end is blocked until what it waits on is done, then ready or needs-human, and no status or work hint offers to start it (nor a validation in a mission's footer). A loose end that is a child or advances a mission is an `atelier check` finding, so the write guard refuses the link, and mission scope leaves loose ends out. ADR 0027's machinery is removed; the old marker is read only by `review request` and by the new `atelier migrate after-merge`. Proof: crates/atelier-cli/tests/loose_ends.rs (five tests covering the start and merge refusals, readiness, hints and land, links and mission close, validation under both policy shapes, the migration and its idempotence, and atelier's own policy matching the starter), `after_merge::tests`, the workflow crate's policy tests; full suite 1000/1000 and 1006/1006 with test-hooks; `atelier check` passes here and in astromech, cosplay-studio, demiurge and life-on-rails, whose `work all` and `work ready` lists are unchanged. Mutation checks: restoring the hard-coded `decision` exemption fails the loose-end and validation land assertions, and dropping the mission-scope filter fails the hand-linked mission assertion. A dry run and apply on a copy of astromech's records (with a `loose_end` policy added) made 5 needs-human loose ends, kept the stand-ins, and a rerun found nothing.

### Deviations

- The shared workflow is named `on_base`. The `done` transition requires the policy's `close_reason`, a non-empty Report; Atelier does not count its lines, and "one-line result" lives in the transition description and the docs.
- Judging by workflow puts decisions under the same refusal: `atelier start <decision>` now fails with `start_done_on_base` and names `decide`, where it used to fail with `workflow_branch_decision` (still in `resolve_work_branch`, which atelier-md0v is changing). `merge <decision>` now says `merge_done_on_base` instead of `merge_owner_unstarted`.
- The loose-end link rule is an `atelier check` finding rather than a structural load error, so a hand-edited link is reported without stopping every command; atelier-ilba's guard is what refuses it at write time.
- "Receipts held at close" was read as the ADR 0027 rule (`after_merge::only_validated` in evidence freshness and claim retention) that ticking or carrying an after-merge check changes no acceptance; it is removed. A ticked box on a done record now makes its receipts stale in `evidence list`. No gate reads pass freshness, and `land` still passes: I checked a tick on a done record after a later code merge. `claim_endpoint` keeps an unused `_index` parameter so review_gate.rs, which atelier-md0v rewrites, is untouched.
- The migration leaves a done item's record as it is (no edit to done records on the base) and finds an already-migrated line by its loose end (blocked_by the item, same Done when text), which keeps reruns idempotent. Superseded items are skipped. The loose end's title is the check, capitalised and cut to 100 characters.

### Decisions

- `needs-human` from the reason's words: human, judgment/judgement, taste, device, phone, tablet, PC, desktop, laptop, machine, hardware, place, game, console, headset. Every real reason on this host ("Sumeet's judgement", "needs Sumeet's PC", "needs the game") lands on needs-human. The dry run shows the label so the human can correct it.
- The needs-human child note now suggests making the child a loose end (`issue update <child> --no-parent --issue-type loose_end`, then a blocked_by link), and it no longer lists a child that merge would not mark done (one done on the base).
- The mission views' "Start the top ready item" skips items done on the base.

### Surprises

- proof_timing.rs tested only the removed machinery, so it was deleted; loose_ends.rs replaces it.
- One test fixture rewrote the `work` workflow's done statuses by matching `- validation` as its last applies_to entry, and that edit landed on `on_base` once validation moved there. The fixture now matches `- task`.

### Open items

- atelier-ml3r: add `loose_end` and the `on_base` workflow to the other repositories' policies (and decide astromech's `validation`), then run `atelier migrate after-merge` there; astromech has 5 open lines (astromech-7f11, -7yb5, -icde, -ok29, -v6r3). Until then, the needs-human child note's `--issue-type loose_end` is refused in those repositories. The agent-factory guidance is also ml3r's.
- After this merges, reinstall the binary before doing a loose end or validation on atelier's base. The old binary reads the new policy, but its `land` exempts only decisions.
- atelier-4nsl: what a loose end waits on when the change it follows had no issue.
