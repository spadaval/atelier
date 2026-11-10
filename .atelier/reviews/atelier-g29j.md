---
schema: atelier.review
schema_version: 2
id: atelier-g29j
title: finding on atelier-67nn @136b0461
target: atelier-67nn
identity: sol-review-1
commit: 136b0461779ef72dee612c10a0365dabaf3b1ce9
created_at: 2026-11-10T19:19:01.359607059Z
event:
  event: finding
  recommendation: fix_now
---

low: evals/agent-factory/scenarios/next-loop/grade.py:128 R2: atelier_calls counts matching shell tool calls rather than Atelier invocations, so the required results table reports incorrect totals. ATELIER.search matches `delegate "Run atelier review slug-k3fa and follow what it prints."`, which invokes delegate only; the real child review is then counted separately. In final Codex bf1f the reported nine calls contain this extra delegation literal: there are eight actual Atelier invocations. Final Codex 9487 and f7ac similarly have nine and eight invocations, rather than ten and nine. Conversely `atelier merge; atelier status` is counted as one, although both run; the final Claude transcripts contain this case. Count actual invocations, excluding literal task text and including separate executed commands within one shell call. Keep recognition of the genuine backtick substitution refusal in round-2 7d8d. Add counting fixtures and correct the recorded table from the existing transcripts.

Verdict: merge once R1 and R2 are fixed and independently re-reviewed; no approval at this head. Sizes: complexity medium (Codex isolation, delegate stand-in and state/transcript grader); value high (required fresh-agent criterion before step B); integration risk low (uninstalled next/ binary and throwaway fixtures; no root crates/ changes). Contract: fail on required call metrics. Fitness: fail on measurement accuracy; the CLI wording and role-view additions otherwise trace to the stated scope, with no unsupported moving part found. Open questions: none needed for these fixes. Residual risk: one small happy-path fixture, no agent-run changes-requested cycle or real T3 native-subagent recovery. No new eval agents were run. Passed-run worktrees were removed by the harness, so their final states were assessed from saved grader results and transcripts, not regraded from their trees.
