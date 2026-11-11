---
schema: atelier.review
schema_version: 3
round: 2
commit: c112ce1c7c19ebaa01e4e3fa924188ee4e759548
reviewers:
- thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3A121fa026-8514-448b-9197-893a2aed7b0b
model: gpt-6-astra
verdict: approve
created_at: 2026-11-11T17:09:18.539222021Z
---

## Summary

Approve the plan at c112ce1. Both carried findings are fixed; I found no
additional problem that must be settled before steps A and B start.

- Complexity: medium. One Git-history reader and its text contract feed
  existing Python consumers, alongside a bounded readiness checklist.
- Value: medium. This supports repeatable measurement on repositories in
  current use and removes the obsolete readers.
- Risk of merging it: medium. Incorrect history or population selection
  could distort the comparison; the plan now requires fixtures for the two
  identified failure modes and preserves the original baseline separately.

The value warrants the scope. A smaller checklist-only change would leave
both the new-format history and the existing scorecard consumers unserved.
The command and consumer split gives each a practical review boundary.

I read the mission, design, notes and all five items, and checked their
contracts against The log and thread identity sections of docs/design.md,
the scorecard README, freeze.py, records.py and draw_samples.py. I also
checked atelier-c1z3's dependency and scope to confirm the remaining rerun
changes have an owner. This is a plan review, not an implementation test:
I did not run the proposed command, measure its performance, read private
snapshot inputs, or perform checklist trials in the other repositories.
Steps C and D remain explicitly provisional and require their boundary
planning and review before implementation; this approval does not replace
those reviews.

## Finding 1: Preserve frozen Git inputs when replacing the scorecard history readers
Recommendation: fix now
Was: 1-c27c.1
Now: fixed

atelier-x3n5's first Done when line now requires freeze.py to save the
whole log as a hashed snapshot input, and consumers to read only that
artifact. It requires a rerun fixture after the live base advances and a
branch disappears. The mission design records the same decision. This
addresses the original reproducibility problem without a new log mode.

## Finding 2: Keep record-only merges out of the baseline's counted-item population
Recommendation: fix now
Was: 1-c27c.2
Now: fixed

atelier-x3n5's second Done when line now treats merge lines as candidates
and requires an outside-.atelier/ change against the named commit's first
parent. Its Scope expressly permits that diff, and its fixture must include
both a product merge and a record-only merge, counting only the former.
The mission design also preserves this population. This resolves the
unintended change to the measured population.
