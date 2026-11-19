---
schema: atelier.review
schema_version: 3
round: 1
commit: 1c80445a40922595794d401bdcbadd11258968d1
reviewers:
- thread:delegated-task:command%3Amcp%3A6e98161a-1f6b-4b19-8e49-d40cd32fa376%3Adelegate-task%3Ab21bbf43-51ba-42c6-be8d-e9595f3e4e80
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-19T02:46:55.446305664Z
---

## Summary

Complexity: medium. Nine historical baselines, item contracts and recorded runs need checking, although the cases add no new harness machinery.

Value: high. Expanding three replay cases to twelve supplies the larger review set the mission needs.

Risk of merging it: low. This changes eval fixtures and reports, with no production behavior change, and the known measurement limitations are disclosed and reversible.

The value warrants the scope. I found no smaller change that delivers the requested coverage and no merge-blocking defect.

Reviewed `git diff master...1c80445a40922595794d401bdcbadd11258968d1`, all nine case definitions and scenario wrappers, their historical item text and later fixes, the defects.md row mapping, the README exclusions, and the shared fixture/grading/confinement code. The new setup, grade and test wrappers match the first replay case byte for byte; each prompt targets its case ID, each rubric links to the shared rubric, and each scenario enables the same confinement and cargo support. Neither the harness nor the shared setup changes.

Ran `evals/agent-factory/test.sh "$(command -v atelier)"`: all twenty scenarios and all 83 harness tests passed. Each new case built its historical fixture and exercised recorded grading answers, without a model call or cargo build. Whitespace validation passed. Read the retained round-1 reviews and result.json files from both reported run directories. Their found answers, unsupported counts, timings and tokens match the table: 1 of 9 found, 1,701 seconds, 8,904,393 input tokens (8,226,432 cached) and 56,504 output tokens. The repeated after-merge-checks run is explicitly distinguished. The nine selected runs report no invalid result and only the stated telemetry-host refusal.

The twelve remaining rows are accounted for by nine cases and three exclusions. Row 69's harness limitation is outside the first Done when line's enumerated reasons, as the Report acknowledges, but the Scope explicitly says to leave out a case needing a harness change. Its exclusion is therefore reasonable without changing that harness. The historical item and Report adaptations are documented rather than presented as newly authored requirements.

I did not start new model reviews, rebuild the old Rust workspaces, reproduce the macOS timeout, or run the release artifacts on an older Linux host. The recorded runs establish the requested smoke results, not a comparison between skill arms. The following shared-harness limitations merit follow-up but do not block this bounded case expansion.

## Finding 1: Remove retained historical process instructions in a shared-setup follow-up
Recommendation: defer

`evals/agent-factory/README.md:123-128` correctly discloses that hierarchy-rules and priority-contract retain old process guidance. I verified their base trees: hierarchy-rules' AGENTS.md explicitly requires the removed `.agents/skills/agent-factory/SKILL.md` and obsolete tracker commands, while priority-contract also loads AGENTFACTORY.md with its own workflow instructions. `replay.strip_process` only removes three differently named headings, so these instructions survive in the fixtures. The retained priority-contract result records a failed `atelier issue show atelier-zivs` against today's tracker, consistent with the Report.

Equal exposure in both arms does not remove the competing workflow instructions or the extra process guidance in the none arm. Extend the shared sanitization in a separate item and protect it with fixture assertions before treating these two cases as clean measurements of the current skill's contribution. This is deferred because changing the shared setup is explicitly outside this item's scope, the limitation is disclosed, and the recorded run still completes its independent review.

## Finding 2: Give the replay judge the source material needed to assess other findings
Recommendation: defer

`evals/agent-factory/results/2026-11-18-nine-more-replays.md:58-63` identifies a real limitation, not merely a disagreement with the judge. The retained merge-sync review cites ADR 0019 decision 3 and standards.md's prohibition on Atelier creating commits. I verified both in the fixture's base tree. `replay.judge_context` supplies only the causing and fixing diffs, which omit those unchanged governing lines. The judge consequently labels the finding unsupported because it sees the new receipt-commit contract but not the existing prohibition.

In a shared-harness follow-up, include the item contract and the base-tree material cited by findings, or distinguish missing evidence from unsupported findings. Otherwise a valid finding outside the two diffs can inflate the reported unsupported count. The current report preserves the judge's answer and explains the discrepancy, so its requested smoke results remain reviewable and no change to this item's cases is needed before merge.
