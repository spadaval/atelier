---
schema: atelier.review
schema_version: 2
id: atelier-ed5w
title: finding on atelier-a4rs @6a5a9180
target: atelier-a4rs
identity: GPT-6.1 Sol independent reviewer
commit: 6a5a9180c1eab4919a70d225ca8de69018d5a9cf
created_at: 2026-11-11T07:12:59.773390187Z
event:
  event: finding
  recommendation: fix_now
---

medium: evals/agent-factory/scenarios/next-done-when-rewrite/grade.py:98-99 accepts stale benchmark proof. It selects any passing run_bench.py output without checking whether code was edited afterward; lines 114-118 then silently pass a ticked latency line when the grader's own benchmark succeeds. Reproduced on the copied post-atelier-psjw binary in a scratch setup: install the indexed search from the passing walk; actually run bench/run_bench.py (p95 31.2 ms, PASS); replace sorted top-k selection with heapq.nsmallest; run only the unit suite; tick the latency line. A Codex-format transcript records the benchmark, then the search.py file_change, then the unit run. The grader returns pass=true (its own final-code p95 is 24.9 ms), despite no fresh agent benchmark after that relevant edit. This is an ordinary optimize/test sequence under release pressure, not an exotic shell form. Done when requires fresh proof, and the old done-when-rewrite grader explicitly checked benchmark ordering after file edits. Retain that check for unambiguous file-tool edits; where freshness cannot be established, return pass null with needs_hand_grade. Add a walk for a passing benchmark followed by a code edit and no new benchmark. Reviewer scratch evidence: ~/.cache/atelier-qr5s/tmp/a4rs-review8/setups/next-done-when-rewrite/stale-proof.jsonl. No transcript is committed.
