---
schema: atelier.review
schema_version: 3
round: 2
commit: 976b66fbdda0e092de08d996eab457177257b47b
reviewers:
- thread:delegated-task:command%3Amcp%3Ae8993a67-827e-4bae-9bb9-2c15542e6e0f%3Adelegate-task%3Ac7848dba-0414-49ae-a9fc-ecdb6f7bf77d
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-19T12:56:41.228007871Z
---

## Summary

- Complexity: medium. The report combines two setups, three ablations,
  earlier baselines and a qualitative reading of saved reviews.
- Value: high. atelier-oiwy needs accurate evidence before changing the
  reviewer guidance.
- Risk of merging: medium. An inaccurate interpretation can steer subsequent
  experiments, although the report is easy to correct.

The investigation is worth keeping at its current scope. No smaller change
would preserve the requested comparisons.

Reviewed the commits from 072c11c8c to 976b66fbd, both carried findings,
all Done when lines, and the resulting report and item Report. Independently
reaggregated the saved result.json detection counts for the confirmation,
cut and threads-only arms, the sized cases, and id3v's touched-claims
replays. Those counts agree with the report. Inspected the Value statements
in all 22 failing whole-skill reviews, read the two previously cited
counterexamples and their CLI reproduction scripts, and read the additional
counterexample below in full. The causal corrections now distinguish the
observed removal results from untested additions and interactions.

The submitted change still touches only the results document and item/review
records, with no skill, case or harness changes. The reviewed diff passes
`git diff --check`. I did not rerun the model matrix, reproduce the historical
binary, or regrade every run. The remaining finding concerns a statement
contradicted by the kept review itself.

## Finding 1: Do not rule out new reviewer guidance from removal-only experiments
Recommendation: fix now
Was: 1-t0j9.1
Response: fixed in fae6eddc1
Now: fixed

The results document and item Report now say the cuts did not restore
detection, identify the retained SKILL.md sections as the next suspect,
acknowledge the zero floor and possible interactions, and leave the effect
of new reviewer guidance untested. This addresses the finding without
requiring further experiments.

## Finding 2: Account for failing skill reviews that explicitly state the Goal's boundary
Recommendation: fix now
Was: 1-t0j9.2
Response: fixed in fae6eddc1
Now: kept

The two cited house counterexamples are correctly included, and the new
distinction between stating a boundary and testing it is supported. However,
`evals/agent-factory/results/2026-11-19-why-touched-claims-missed.md:130-135`
now says only two failing reviews state the boundary and the other 20 do not.
A third kept failing review contradicts that count:

- Run: `id3v/sol-replays/20261119-012454-touched-claims-skill-sol-1-b165`
  under `~/.cache/af-evals/runs/`.
- Review: `1-hlko.md`, retained at
  `/var/tmp/af-evals/20261119-012454-touched-claims-skill-sol-1-b165/repo/.atelier/items/atelier-sodr/reviews/1-hlko.md`.
- Its Value statement says: "landing must recheck a completed mission when
  its advancing work changes, to preserve the completion invariant while
  skipping unaffected history."

Its result.json records `pass: false` and an approving review with no
findings. Its validation ran the session suite, formatting and whitespace
checks, with no added sibling reproduction. This is the same kind of
counterexample as the two house reviews and also occurs in sol. The item's
Report at `.atelier/items/atelier-7h71/item.md:77-78` repeats the count of two.
Include this review and correct both counts, or describe the cited reviews
as examples without asserting that all remaining reviews omit the boundary.
No new model runs are needed. The supported distinction is still whether
the reviewer tested the boundary, rather than whether it stated it.
