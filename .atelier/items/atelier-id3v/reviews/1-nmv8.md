---
schema: atelier.review
schema_version: 3
round: 1
commit: b94ed6405cf4abfd80aff2cbd717ec9fe6dc6af3
reviewers:
- thread:delegated-task:command%3Amcp%3Aa29a583f-b604-49eb-b0b2-eff2bf6491cf%3Adelegate-task%3A49de7f55-9907-4627-8f77-45181aa4dd25
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-19T11:23:44.411085391Z
---

## Summary

- **Complexity: low.** This records existing runs and rewrites documentation; it adds no runtime or harness behavior.
- **Value: high.** The mission needs these measurements to decide what the skill contributes.
- **Risk of merging it: medium.** Incorrect measurements can steer later skill changes, although the raw runs survive and the documents are easy to correct.

The scope is worth keeping. I compared the four-file change at b94ed6405 with master, read the item and mission criteria, and checked the README against all 38 current scenarios and the linked result reports. I loaded the local results under `~/.cache/af-evals/runs/id3v/`: 469 current graded runs, no current nulls, and the prescribed five or three runs per cell. Regenerating both main tables with `harness/table.py` reproduced every row. I also checked the threads-only and pre-mtzj medians, all 35 lead outcome cells, the 40 runs with recorded departures, all 21 stopped leads' final replies, and the grades, final replies and relevant rubric answers for the 30 listed hand reads. The pass counts and the reported hand-read classifications agree with those records. I checked the skill refs, grader code behind the lead facts, README links and whitespace. The README covers the three questions and names the unmeasured procedures; its later cases explicitly state which setups were not run.

I did not repeat the model matrix or independently rejudge every run or finding. Passing runs' work trees are deleted, so some checks necessarily rely on their retained facts and transcripts. No production code changes, so I ran data and documentation checks rather than the Rust suite. The main blocking issue is a transcription of uncertain judge answers into definite unsupported-finding counts.

## Finding 1: Keep unknown judgments out of the unsupported-finding count
Recommendation: fix now

`evals/agent-factory/results/2026-11-19-both-setups.md:180`, `:192` and `:202` each count one unsupported finding, and lines 27 and 238-239 summarize them as three unsupported findings. None of those three judges said a finding was unsupported. Their `others-supported` answer is `unknown` because necessary code was missing:

- `sol-replays/20261119-004815-quiet-tokens-skill-sol-0-cceb`: whether the omission is real, "0 or 1 of 1 unsupported", cannot be settled.
- `house-replays/20261119-043157-hierarchy-rules-skill-house-1-b9e7`: missing test fixtures decide whether the count is "0 or 1 of 5 unsupported".
- `house-replays/20261119-052925-after-merge-checks-skill-house-0-d54c`: missing gate code decides whether it is "0 or 1 of 3 unsupported".

The retained aggregation script, `~/.cache/af-evals/runs/id3v/replays.py`, extracts `(\d+) of (\d+) unsupported` from the explanation without considering the answer. It therefore turns each conditional upper bound into an observed count of one. Across all 144 replays the actual answers are 126 pass, 18 unknown and zero fail. This currently makes the skill arm alone appear to produce unsupported findings that the judge never established.

Recalculate the affected cells and their summaries from the actual judgments, preserving the unknowns separately. Do not describe unknown findings as supported either. If numeric unsupported counts are extracted from explanations, verify that they are definite counts rather than conditional possibilities. The raw runs do not need to be repeated to correct this report.

## Finding 2: Describe the baseline as pinned rather than unchanged master
Recommendation: defer

`evals/agent-factory/results/2026-11-19-both-setups.md:45` says "Master's crates did not change over the matrix." The matrix ran from the evening of November 18 into the morning of November 19, but master merged atelier-hlsw at 02:53 and atelier-d73k at 02:57 local time on November 19. Both changed crates; hlsw also changed the skill. `git diff 4f36b1fa6 HEAD -- crates skills/agent-factory` confirms those differences.

The runs consistently name the frozen `atelier-4f36b1fa6` binary and the intended skill refs, so this does not invalidate the comparisons or require another matrix. Correct the sentence to say that the matrix kept its pinned binary and skill while master advanced, so readers do not mistake these results for a run of the later binary and skill.
