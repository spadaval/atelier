---
schema: atelier.review
schema_version: 3
round: 1
commit: 440d415721f5e67fe5488354ffdf2355f1d69996
reviewers:
- thread:delegated-task:command%3Amcp%3A7a27ac3f-fe25-418b-ba19-e1770846debc%3Adelegate-task%3A463c9695-6977-4897-838d-edbd6258e864
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-19T03:02:43.108299896Z
---

## Summary

Complexity: low. One shared fixture sanitizer changes, with tests and documentation; there is no new runtime state or dependency.

Value: high. Removing competing historical workflow instructions is needed to interpret the replay comparison as a measurement of today's skill.

Risk of merging it: low. The change affects reproducible eval fixtures and documentation, and can be reverted without changing the product or losing data.

The value warrants the scope. I found no materially smaller change that delivers the same coverage, but the remaining historical merge policy and the failed CI check need fixing.

Reviewed `git diff master...440d415721f5e67fe5488354ffdf2355f1d69996`, the complete shared setup and replay assertions, every replay case's historical AGENTS.md and CLAUDE.md, the excluded guidance files and patch paths, the README, and each Done when line. Independently generated and read the filtered instructions for all twelve cases, then built an additional after-merge-checks fixture to confirm the retained policy below.

Ran `evals/agent-factory/test.sh "$(command -v atelier)"` using installed atelier 0.7.0: all 84 harness tests and every scenario passed, including hierarchy-rules and priority-contract. The shared fixture checks establish that the old guidance files are absent, the product tree and change remain intact outside the stated exclusions, and the base test commands survive. The README correctly distinguishes previously recorded runs from fixtures built after this change. Checked PR #236 and its failed `changes` job log; its whitespace error reproduces locally with `git diff --check master...440d415721f5e67fe5488354ffdf2355f1d69996`.

I did not start model reviews, rebuild the historical Rust workspaces, or rerun the recorded comparison. The passing fixture tests do not establish the behavioral effect of changing the guidance, and they miss the surviving process sentence below.

## Finding 1: Strip the historical CI merge rule still retained in after-merge-checks
Recommendation: fix now

`evals/agent-factory/harness/replay.py:128-133` accepts an entire list item when it contains a cargo command or an existing file and none of the process markers. In the actual after-merge-checks base, the final Command References item contains both `cargo nextest run -p <crate>` and `.github/workflows/ci.yml`, followed by the historical workflow rule that CI runs on the pull request "and that run gates the merge." Building this case with the submitted code leaves that whole item in the generated root AGENTS.md. This is agent-facing merge-process guidance, rather than a product document that the Report intentionally retains, and the fixture has no remote.

The first Done when line requires the old tracker's process text to be absent. This is an existing era that still carries such text, not just the future-marker limitation disclosed in the Report. `replay_tests.py:175-180` passes it because that rule names none of its marker strings. Remove the merge-policy portion while preserving the useful local test commands, and protect this existing fixture against retaining that policy. This does not require stripping the historical product documentation.

## Finding 2: Remove the extra EOF blank line that fails the required CI check
Recommendation: fix now

`.atelier/items/atelier-s63j/item.md:91` adds an empty line after the Report's final paragraph. `git diff --check master...440d415721f5e67fe5488354ffdf2355f1d69996` exits 2 with `new blank line at EOF`. PR #236's CI run 44448571588 fails its `changes` job on exactly that error, and the dependent evals job is skipped. Remove the extra blank line and resubmit so the required whitespace check passes and CI can run the evals. This is a confirmed merge blocker despite the local eval suite passing.
