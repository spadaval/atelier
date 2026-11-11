---
schema: atelier.review
schema_version: 3
round: 3
commit: 3a9a1f72a011426b3afa71d6d47e44bda655a4ef
reviewers:
- thread:delegated-task:command%3Amcp%3Af03ae543-67b7-4101-adec-2ed71d3b4324%3Adelegate-task%3Ad93dd380-5523-4fdf-a88b-c48845e43703
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-11T17:28:06.610220260Z
---

## Summary

Complexity: low. One static procedure with nine entries and two short shell snippets; no runtime behavior changes.

Value: high. This item supplies the preconditions the gardener needs before interpreting the scorecard.

Risk of merging it: low. A mistaken check can mislead a readiness report, but the procedure is easy to correct and changes no product data.

The value is worth the size. The nine entries cover the required properties without adding a command, scoring system or repository-specific policy.

I reviewed `master...3a9a1f72a011426b3afa71d6d47e44bda655a4ef`, the changes since round 2, each Done when line, the Report, the mission design and the Scorecard. The entries name their tests and measures, bound local commands, keep trials in scratch clones, and specify reviewed repository additions at `.atelier/readiness.md` without adding that file anywhere. I checked the Report's pillar mapping against [Factory's overview](https://docs.factory.com/agent-readiness/overview.md) and [launch post](https://factory.com/news/agent-readiness). I also checked the record loader and pre-push path classification behind the repository-specific file's treatment. `git diff --check` passed.

I ran the exact line 3 and line 9 snippets from the proposed file in the retained trial clones, atelier `0f0d8f334c06fe4cf8c52d804aa20138a6fceeb7` and demiurge `13b07f334aa5fbcf64889a39e57f191ce1e86caa`. Line 3 prints nothing for atelier and reports the missing `src/docs/editor` for demiurge. Line 9 reports 39 and 2 bugs without causes, respectively. A scratch fixture confirmed the path script reports a missing root document and broken links in root and nested documents while accepting a valid relative link with a fragment and ignoring an external URL, slash command and scoped package name. I checked both clones' maps and startup hooks, the documented commands and package scripts, and the merge/CI configuration against the trial table.

I did not repeat dependency installation, builds, linters, unit tests or browser suites, so their pass counts and timings remain the author's validation. I did not launch a Claude session to inspect its loaded context. PR #144's checks were still pending when inspected; this review does not certify CI completion.

## Finding 1: Test whether Claude loads the map using its documented loading rules
Recommendation: fix now
Was: 2-cmz4.2
Response: fixed in b744cb04d
Now: fixed

Line 1 now requires the repository to provide a `CLAUDE.md` import or symlink instead of certifying direct loading from an incomplete list of host conditions. The [Claude memory documentation](https://code.claude.com/docs/en/memory#share-one-file-with-other-coding-tools) supports both approaches, including when direct AGENTS support is unavailable. Its [instruction-setting table](https://code.claude.com/docs/en/memory#choose-which-instruction-files-load) supports the stated `managed-only` exception. The Report now marks demiurge not met because it lacks that repository-controlled bridge; atelier's `@AGENTS.md` import and both startup hooks satisfy the revised test. This resolves the carried finding.
