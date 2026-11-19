---
schema: atelier.item
schema_version: 3
id: atelier-6ve6
title: Give the replay judge the base-tree files a finding cites
status: ready
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers: []
ready:
  by: 'the lead of atelier-kw8u: both change what the replayed reviews measure, so they come before the matrix'
  at: 2026-11-19T02:52:02.027431283Z
trivial: null
caused_by: null
created_at: 2026-11-19T02:50:32.636385898Z
---

## Goal

Taken from deferred finding atelier-z8rg/1-yrne.2, in .atelier/items/atelier-z8rg/reviews/1-yrne.md:

> Finding 2: Give the replay judge the source material needed to assess other findings
> Recommendation: defer
>
> `evals/agent-factory/results/2026-11-18-nine-more-replays.md:58-63` identifies a real limitation, not merely a disagreement with the judge. The retained merge-sync review cites ADR 0019 decision 3 and standards.md's prohibition on Atelier creating commits. I verified both in the fixture's base tree. `replay.judge_context` supplies only the causing and fixing diffs, which omit those unchanged governing lines. The judge consequently labels the finding unsupported because it sees the new receipt-commit contract but not the existing prohibition.
>
> In a shared-harness follow-up, include the item contract and the base-tree material cited by findings, or distinguish missing evidence from unsupported findings. Otherwise a valid finding outside the two diffs can inflate the reported unsupported count. The current report preserves the judge's answer and explains the discrepancy, so its requested smoke results remain reviewable and no change to this item's cases is needed before merge.

## Done when

- [ ] The replay judge can read the base tree's files that a finding cites, or answers `unknown` rather than `fail` where the cited material is not given to it.
- [ ] Re-judging the kept `merge-sync` run (`harness/judge.py`) answers `others-supported` with 0 of 3 unsupported.
- [ ] A test in `replay_tests.py` covers a finding supported only by an unchanged base file.

## Scope
