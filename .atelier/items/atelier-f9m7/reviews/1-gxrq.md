---
schema: atelier.review
schema_version: 3
round: 1
commit: 2b0be49f331b2d75fae69025bea8feb2b1f08192
reviewers:
- thread:delegated-task:command%3Amcp%3A34b37870-9f8f-4699-873f-144ceb2a5464%3Adelegate-task%3Aee78b666-7890-412d-a274-9dca4c731594
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T19:13:40.884174931Z
---

## Summary

Complexity: low. This changes one prose rule without adding a moving part.
Value: medium. Agents using the extension receive a concrete purpose test
instead of wording that can discourage useful tests and diagnostics.
Risk of merging it: low. The wording is visible, changes no runtime code,
and is cheap to revert.

The value justifies the size. No smaller change would cover the requested
examples and explicitly reject line count as the measure.

Reviewed `git diff master...2b0be49f331b2d75fae69025bea8feb2b1f08192`
and the full extension rules against all five Done when lines. Rule 3 has
the required heading, requires a stated purpose before writing, asks concrete
questions about guards, tests and tools, preserves the simplest-implementation
sentence, and says size is not the measure. Rules 1 and 2 are unchanged.
The only other changed file is this item's required tracker record.

`atelier check` passes, with existing advisory messages about three unrelated
missions missing their Why. `git diff --check` passes. Searching outside
tracker records finds no remaining reference to the old heading. The PR and
remote branch both point to the reviewed commit. Available CI results pass;
the extension CI job was still running when checked. No runtime tests were
run for this prose-only change. I could not verify the earlier conversation
cited in the Report; the written acceptance criteria are satisfied.

No findings.
