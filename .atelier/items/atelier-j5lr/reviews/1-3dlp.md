---
schema: atelier.review
schema_version: 3
round: 1
commit: 0ccc918afe2c82d3629899dbcdfc0c487a798285
reviewers:
- thread:delegated-task:command%3Amcp%3A0c140748-831a-4ee9-953a-eda11815ca8a%3Adelegate-task%3Aa6c35328-871d-4bb5-86be-17a4532d8db4
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-14T01:33:38.868928885Z
---

## Summary

- Complexity: low. This updates existing guidance in one pass and introduces
  no new tool behavior.
- Value: high. Agents need these instructions to use the branch model that
  the preceding mission items implemented.
- Risk of merging: medium. The shared skill guides work in other
  repositories, but an incorrect instruction is visible and reversible.

The value warrants the change. I found no problems requiring correction
and recommend approval.

Reviewed `git diff master...0ccc918afe2c82d3629899dbcdfc0c487a798285`
against every Done when line and the Report. Read all changed guidance and
cross-checked the branch, plan-review, verdict, lead, dispute and merge
instructions against `docs/product.md`, the CLI implementation and its
existing planning and branch tests. The additional garden edits remove
base-commit instructions as the acceptance criteria require; they do not
introduce a separate gardening redesign.

Verified the deletion of the decider role and its routing, the question and
rebuttal instructions, blind plan reviews, the lead's scope intervention,
record-only branches, and readiness line 10. Recounted every skill file:
516 lines before, 515 after, with each per-file count matching the Report.
Searched the skill, AGENTS.md and CONTRIBUTING.md for remaining instructions
to commit records on the base or use removed commands; found none.

`atelier check` passed for 159 items, 9 missions and config.toml.
`git diff --check` on the full change passed. Readiness line 3's link
script exited successfully with no output. PR #160's checks passed; the
Rust and Desk jobs were skipped for this Markdown change.

I did not run the new branch workflows end to end: the installed CLI
predates them, and this change contains no executable code. Their behavior
was checked against source and existing tests, which were not rerun.
