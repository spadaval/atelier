---
schema: atelier.review
schema_version: 3
round: 6
commit: 5757c9c7cc4dd5ea0b7389c3ac882741ebce18b4
reviewers:
- thread:delegated-task:command%3Amcp%3Aec287d19-72ad-48dc-8e79-f6d17557d1a5%3Adelegate-task%3A91743164-1df3-4542-9b8d-2a9da7c1e914
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-19T01:12:27.457302338Z
---

## Summary

- Complexity: medium. Trailer writing, branch discovery and action inference span the core and CLI, within the existing Git history reader.
- Value: high. The mission outcome requires the human to read departures and identify who made them on the mission page.
- Risk of merging: high. Incorrect attribution or omitted commits would silently misstate the audit, although this review found no remaining instance.

The value warrants the change. Reusing the existing history reader keeps the mission page and log consistent; I found no smaller change that preserves the required coverage.

I read the full change through 5757c9c7cc4dd5ea0b7389c3ac882741ebce18b4 and the latest fix, checked every Done when line and the Report against the code, and compared the new branch lookup with the repository's record selection. I inspected the tests for lead attribution, legacy trailers, empty sections, fallback actions, settlements, remote-only and remote-ahead missions, and pending backlog branches. The new backlog-only item regression exercises the carried finding's reproduction and passes.

All 399 Rust tests passed, with one additional test skipped, using this checkout's target directory, cargo -j 4 and TMPDIR=$HOME/.cache/atelier-tmp. Formatting, clippy with warnings denied, git diff --check and evals/agent-factory/test.sh passed. CI on the reviewed head passed. I also ran the built CLI against atelier-kw8u and confirmed its two legacy plan-merge departures appear with their reasons and thread not recorded. I did not perform a live forge merge or replace the installed binary.

## Finding 1: Include fetched and backlog branches in the mission's passed-rule history
Recommendation: fix now
Was: 5-wlv1.1
Response: fixed in cac67983b
Now: fixed

cac67983b fixes the remaining membership omission. in_mission now reads an item absent from the base and its own branch from the first unmerged named branch that holds it, including a backlog branch. It uses the committed tips from Repo::walked, so uncommitted membership changes do not enter history. The new regression creates and readies a mission item entirely on backlog/tidy, commits a skipped second ready, then reads from the base and checks that show and log both retain that commit's rule, reason and p1. The fetched-mission and existing-item backlog regressions also pass. No fix-now finding remains.
