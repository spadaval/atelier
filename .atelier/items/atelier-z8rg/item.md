---
schema: atelier.item
schema_version: 3
id: atelier-z8rg
title: Replay every after-merge defect that fits as a review case, now that one review costs two minutes
status: building
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A9d5de7ee-d750-4bea-9d8c-206890e4d262%3Adelegate-task%3Ab1c31788-3c35-43b9-855a-42b57f88f58f
ready:
  by: the lead of atelier-kw8u, under its Authority to extend the review cases once atelier-1z52 reported what one costs
  at: 2026-11-19T01:56:05.793049132Z
trivial: null
caused_by: null
created_at: 2026-11-19T01:56:05.723046822Z
---

## Goal

atelier-1z52 replayed three changes as review cases and reported the cost:
one review takes 1 to 3.5 minutes, and five runs of ten cases take about
1.5 hours in `sol` and 2.3 in `house`. Three cases are too few to tell two
arms apart: both setups found the known defect in one of three. The plan
lets the lead extend the cases once that cost was known, and three of the
five passages atelier-oiwy tries are judged on them.
`docs/architecture/quality/scorecard-baseline/defects.md` traces 12 more
defects in atelier found after the merge that caused them.

## Done when

- [x] Each of the 12 remaining after-merge rows for atelier in `defects.md`
  has a case file, or a line in the README of the replayed cases saying why
  it does not replay: its change is several commits that do not stand
  alone, its defect cannot be seen from the change, or its diff is more
  than one reviewer can hold.
- [x] Each new case builds with the shared setup and runs in the same
  confinement as the first three, with no change to either.
- [x] `test.sh` builds each new fixture and grades a recorded review, with
  no model and no `cargo` build.
- [x] The Report gives one review of each new case by the reviewer of
  `sol`: found or not, unsupported findings, minutes and tokens.

## Scope

Case files and recorded reviews beside the first three, under
`evals/agent-factory/scenarios/`, and their README. No change to the
harness or the shared setup; a case that needs one is left out.

## Report

Nine of the 12 remaining after-merge rows now replay as review cases, so
the replay set has 12 cases instead of 3. One `sol` review of each found
the known defect in 1 of 9 (`priority-contract`). With the first three,
`sol` has found 2 of 12. A review took 1.5 to 5.2 minutes, 28 minutes for
all nine. The runs are in
`evals/agent-factory/results/2026-11-18-nine-more-replays.md`.

| Case (row, item) | Found | Unsupported, by the judge | Minutes | Tokens in (cached) | Tokens out |
| --- | --- | --- | --- | --- | --- |
| `merge-sync` (7, atelier-s2ei) | no | 1 of 3 (I read 0 of 3) | 3.9 | 950k (883k) | 8.5k |
| `brief-transitions` (10, atelier-7u4o) | no, approved | 0 of 0 | 1.5 | 421k (383k) | 2.7k |
| `hierarchy-rules` (11, atelier-59vp) | no | 0 of 4 | 5.2 | 1.91M (1.82M) | 8.9k |
| `project-slug` (12, atelier-2oor) | no | 0 of 1 | 2.8 | 856k (791k) | 5.6k |
| `quiet-tokens` (33, atelier-pr4k) | no | 0 of 1 | 2.6 | 993k (921k) | 5.2k |
| `priority-contract` (45, atelier-6jap) | yes, reproduced | 0 of 2 | 3.0 | 1.25M (1.16M) | 5.8k |
| `readiness-blockers` (49, atelier-p2wk) | no | 0 of 7 | 4.0 | 1.51M (1.42M) | 8.4k |
| `after-merge-checks` (63, atelier-n0qh) | no | 0 of 2 | 2.9 | 593k (516k) | 6.6k |
| `touched-claims` (64, atelier-37uq) | no, approved | 0 of 0 | 2.4 | 415k (337k) | 4.8k |

I read every review, and the judge's nine `found` answers match mine. No
run was invalid, no transcript showed a fix, and each proxy refused only
Codex's telemetry host. The reviewer is `gpt-6.1-sol` at high in every run.

Three rows do not replay, each with its line in the README:

- Row 28: its defect cannot be seen from the change. The test passes in
  7.6 s on Linux and timed out only on macOS CI under load.
- Row 57: its diff is 2,453 lines, more than one reviewer can hold.
- Row 69: its change and fix touch no crate, and the shared tests read
  both under `crates/`, so it would need a harness change, which the Scope
  rules out. This is not one of the Done when's three reasons. Its own
  Report also named the defect as an open item, so review knew of it.

Things the reviewer of this item should know:

- `after-merge-checks` was run twice. The first run made a fix-now finding
  that the Agent Factory lines of the Done when were unmet. That came from
  my case: I had cut the Report's line saying that text was committed in
  the skill's own repository. b480f58d3 puts it back, minus a host path,
  and the second run, in the table, made no such finding.
- The setup's promise that AGENTS.md's process sections are left out does
  not hold for `hierarchy-rules` and `priority-contract`. Their June 2026
  trees give the old process under headings the setup does not strip. One
  run called `atelier issue show`, which that text names and today's binary
  lacks. Both arms see the same text, and the README says so. Stripping
  those headings would change the shared setup, which the Scope rules out.
- The June and July items had no Goal, Done when or Close Reason. Each
  `case.toml` says what stands in for them. `readiness-blockers` holds the
  child atelier-72k4's text, since that child made the change, while its
  `item` stays the epic that defects.md credits.
- `brief-transitions`' Report leaves out the original's "independently
  reviewed in two rounds and approved".
- I count `merge-sync`'s Finding 3 supported where the judge did not. It
  cites ADR 0019's "Atelier never creates commits", which is in the
  fixture's tree, but the judge sees only the two diffs. Proposed
  follow-up, not filed: give the replay judge the files a finding cites
  from the base tree.
- Merging master brought in atelier-xfsn, which made each replay case's
  `rubric.md` a symlink to `harness/replay-rubric.md` and added a reported
  `reasons` question. The new cases now link to it too (merge a7c0f1115).
  The recorded runs predate that question, so they have no answer to it.

Verification: `evals/agent-factory/test.sh` passes on the merged tree (all
twenty scenarios and the harness tests). Each new case's `test_grade.py`
builds its fixture from this repository's history and grades recorded
reviews, with no model and no cargo build, in about 3 seconds. Nothing in
`harness/` or the shared setup changed on this branch.
