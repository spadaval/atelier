---
schema: atelier.review
schema_version: 3
round: 1
commit: c5042f7befdb93b764c7bd29c4f35fce3ab53826
reviewers:
- thread:delegated-task:command%3Amcp%3A4ebe826c-0e43-4490-8e07-54be9533dffc%3Adelegate-task%3Ae206450f-65eb-421a-b9a6-3d57f1eea5aa
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-23T05:40:26.763693017Z
---

## Summary

Complexity: low. One fixture uses the existing harness, fact helpers and
judge, and the two procedure edits follow the binary's branch rules.

Value: medium. The case protects filing on an item's branch without
interrupting its merge, and the procedure edits remove unnecessary branch
changes from work in current use.

Risk of merging it: low. No binary or harness behavior changes, and the
fixture and instruction edits are easy to revert.

The value warrants the change. The case uses the existing machinery, and
its passing baseline is useful for the regression the README names. No
smaller change supplies both that evidence and the procedure corrections.

Reviewed the complete diff from master through c5042f7befdb93b764c7bd29c4f35fce3ab53826,
each Done when line, the fixture, grader, rubric and all 13 grader tests.
Checked the instruction added in a8ae546b6 and its removal in 161da1c50:
SKILL.md ends unchanged from master, as the no-gain condition requires.
The first Done when line was changed by the mission's lead to permit that
result, consistently with the mission's told-to-file criterion and Authority.

Read all ten retained new records and review records, the final replies,
results and call logs, and the staged skills in both providers' locations.
Confirmed the Atelier merge trailers on main and the implementer as each
filing's caller. Each new record describes the planted refund bug. The
five runs per arm passed, with two agents at most, and the medians reproduce:
121 seconds and 552,641/8,396 input/output tokens without the line;
108 seconds and 433,697/6,970 with it. The staged instruction is present
only in the with arm. The without arm checks new's help and records five
deferred findings; the with arm does neither. The report's decision to
remove the instruction follows the agreed filing measure.

Checked lead.md and garden.md against the current new/set implementation.
In a temporary fixture, independently exercised filing on the item's
branch, approval and merge, then setting bug,needs-human and caused-by on
backlog/garden-proposals and merging those labels. Setting them on the
item's branch is refused, as the procedure explains. The carry-forward
instructions retain an existing unlabelled bug's ID rather than filing it
again.

Ran `evals/agent-factory/test.sh target/debug/atelier` with TMPDIR under
~/.cache, outside all repositories: exit 0, 565 tests, with the four
expected sized-control skips. All 13 outside-problem tests passed.
`atelier check` and `git diff --check` pass. CI had no failures when checked;
its evals job was still pending then.

No findings.

No model eval or judge was rerun, as the review brief requires. The model
behavior assessment rests on the retained evidence, not fresh runs. No
production gardener run was performed.
