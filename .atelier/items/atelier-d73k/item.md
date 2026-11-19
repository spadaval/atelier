---
schema: atelier.item
schema_version: 3
id: atelier-d73k
title: Say in a later round's brief what the round is for, and show with two cases that it narrows without hiding
status: draft
mission: atelier-8az0
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T04:12:27.403171799Z
---

## Goal

A reviewer in round 4 is told what a reviewer in round 1 is told. The brief
names the round and repeats round 1's instruction word for word, with the
diff of the whole change first. In a survey of 143 Sol reviews on
2026-11-18 the share of findings that were critical or important fell from
75% in round 1 to 39% from round 3 on, and 18 findings were another case of
a gap already reported on the same item. Seven of the 18 findings from
round 3 on were still important, so the brief must narrow where a reviewer
looks without hiding a real defect.

## Done when

- [ ] For a round after the first, the brief says how many rounds have
  reviewed the whole change, and that this round checks the earlier
  findings and what changed since. The diff since the last reviewed commit
  comes before the diff of the whole change, and both are given.
- [ ] It says a new finding in code that did not change blocks only if the
  work would come out wrong without it, and is otherwise for later work.
- [ ] Round 1's brief is as it was, and a test covers the brief of rounds
  1, 2 and 3 of a fixture item.
- [ ] Two round-2 review cases are added, each a fixture, facts and a
  rubric: round 1's findings are fixed, and text that did not change holds
  a flaw. In one the flaw is cosmetic, and the case passes when the review
  approves or does not block on it. In the other the flaw makes the work
  come out wrong, and the case passes when the review blocks on it.
  `test.sh` runs both with no model.
- [ ] The results give both cases in `sol`, ten runs a cell, with the
  binary before this change and after. The second flaw is found no less
  often after, or the brief's wording is changed until it is.
- [ ] `docs/design.md` says what a later round's brief holds and why.

## Scope

The review brief in `crates/` and its tests; two new directories under
`evals/agent-factory/scenarios/`; `evals/agent-factory/results/`;
`docs/design.md`.
