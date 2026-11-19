---
schema: atelier.item
schema_version: 3
id: atelier-7h71
title: Find why reviews with the skill miss a defect that reviews without it find
status: building
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A2ec81be6-36b2-4b12-a37f-0052f9f2be6c%3Adelegate-task%3A7475e1ef-65b3-43de-bfd9-7c2f5a5d8c6b
ready:
  by: 'the lead of atelier-kw8u: the matrix found the skill costing a reviewer a defect, and atelier-oiwy should not add passages before it is understood'
  at: 2026-11-19T11:37:11.739669970Z
trivial: null
caused_by: null
created_at: 2026-11-19T11:37:11.695173322Z
---

## Goal

The matrix of atelier-id3v found a case where the skill makes a reviewer
worse. On the replayed review `touched-claims`, five of six reviews with
no skill found the known defect, and none of six with the skill did: all
six approved, five with no finding at all. Across the twelve replayed
reviews the skill found the known defect in 12 of 72 reviews against 17 of
72 without it, and helped in no case in both setups. The same skill takes
`costly-hardening` from 0 of 10 to 10 of 10, by `roles/reviewer.md`'s Size
and Verdict, the one passage of the reviewer's file that the evals of
2026-11-11 kept.

Six reviews are few, so first confirm it, then find which text does it.
atelier-oiwy tries passages for the reviewer next, and should know what
it is adding them to.

## Done when

- [ ] `touched-claims` is run again with no skill and with the skill, ten
  runs a cell in `sol` and in `house`. The Report says whether the gap
  holds.
- [ ] The Report reads the kept reviews of both arms and says what the
  skill's reviews did differently: what they checked, what they wrote under
  Size and Verdict, and where the reviews with no skill found the defect.
- [ ] If the gap holds, the case is run with two more arms, each from a
  branch that is not merged, ten runs a cell in `sol`: the skill without
  `roles/reviewer.md`'s Size and Verdict, and the skill with no
  `roles/reviewer.md` at all. `costly-hardening` and the five overbuilt
  cases of atelier-s6xk are run with the same two arms, five runs a cell,
  to show what each costs there. The Report gives all of it in one table.
- [ ] Nothing in the skill is changed. The Report says what the runs point
  to, for the human and for atelier-oiwy.

## Scope

`evals/agent-factory/results/`. No change to the skill, the cases or the
harness.
