---
created_at: "2026-11-07T05:01:00.329959594+00:00"
id: "atelier-8bh4"
issue_type: "feature"
labels: []
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
status: "todo"
title: "Track post-merge work as loose ends, outside any mission"
updated_at: "2026-11-07T06:05:08.393934690+00:00"
---

## Goal

Replace after-merge checks (a marker on a Done when line, ADR 0027) with a `loose_end` issue type: work that changes no code and can only happen after a merge or a release, such as pushing a tag, a play-check on a device, or an eval run. A loose end waits on what it follows through `blocked_by`, shows up as ready when that merges, and is never part of a mission, so a mission closes when its code merges. The v0.3.1 tag was lost because it lived as prose in a Close Reason; astromech-cxgq (a play-check) shows the linked-issue form already works.

## Done when

- [ ] The starter policy and atelier's own define `loose_end`, and `loose_end` and `validation` share one workflow with no `start` transition: `todo` to `done` through a transition named `done` that requires a one-line result in the `Report` section, plus `supersede`. `atelier start` and `atelier merge` refuse an item whose workflow has no `start` transition (judged by workflow, not type name, so a repository that still keeps `validation` in its `work` workflow is unaffected), naming the `done` transition.
- [ ] Marking a loose end or a validation done on the base passes `land`: the exemption from `land_base_done_claim` is for workflows with no `start` transition, not for the hard-coded `mission` and `decision` names (crates/atelier-cli/src/commands/git.rs).
- [ ] A loose end `blocked_by` an issue appears in `work ready` once that issue is merged, and in `work human` when labelled `needs-human`; no `status` or `work` hint suggests starting one (tests).
- [ ] A loose end cannot advance a mission or be any issue's child (refused when the link is written), and mission close ignores loose ends.
- [ ] Removed: stand-in parsing, `carried to`, the awaiting-validation rows and views, receipts held at close, the mission-close warning, and the after-merge part of the review request brief. The marker is still recognised in old records, only so `review request` can skip such lines (atelier-jnrh) and `migrate` can find them; otherwise they show as ordinary Done when lines.
- [ ] `atelier migrate after-merge` (dry run unless `--apply`; refuses with a clear message when the policy has no `loose_end`) turns each unticked After merge line into a loose end `blocked_by` its item: the check becomes its Done when, the reason goes in its Goal, and it gets `needs-human` when the reason is the human's judgment or a device or place no agent can reach, which the dry run shows. On an item that is not done, the line's stand-in stays as an ordinary Done when line. Lines carrying `carried to <id>` are skipped. Idempotent; covered by a test.
- [ ] ADR 0028's decision 3 paragraph names the shared workflow, the `done` transition and the migration as built; docs/product, CONTEXT.md and ADR 0027's Status match. The agent-factory guidance is left to atelier-ml3r.

## Scope

Workflow policy (starter and atelier's own), issue types, `start`/`merge` refusals, `land`'s base done-claim exemption, link validation, work views and hints, review request brief, mission close, `migrate`, docs, ADR 0028. Must not change: how ordinary work items merge. Other repositories keep working with the shared binary: policies without `loose_end` and records with the marker still parse; migrating them is atelier-ml3r's job.

## Context

docs/adr/0028-simpler-lifecycle.md (decision 3), docs/adr/0027-proof-has-a-time.md (the machinery being removed), docs/strategies/simpler-lifecycle.md (Shape: Loose ends). The `Report` section comes from atelier-jnrh. What a loose end waits on after a change with no issue is atelier-4nsl's question. Whether astromech's per-feature `validation` play-checks become loose ends is decided when atelier-ml3r migrates it.
