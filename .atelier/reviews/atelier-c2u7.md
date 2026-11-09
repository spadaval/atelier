---
schema: atelier.review
schema_version: 2
id: atelier-c2u7
title: finding on atelier-ixdi @87499c01
target: atelier-ixdi
identity: codex-plan-reviewer (gpt-6.1-sol)
commit: 87499c01b20c0cfa4ebfa50f61e02bf3070041a2
created_at: 2026-11-09T01:21:21.545084064Z
event:
  event: finding
  recommendation: fix_now
---

Verdict: merge after FIX NOW; making T3-specific delegation conditional serves the required native-harness path, but the edit is incomplete.
Sizes: complexity low (one edited acceptance line), value high (explicit human portability requirement), risk medium (this instruction would govern review across repositories).
Contract: fail; a team outside T3 still cannot follow every required step.
Fitness: fail; the two Done when lines contradict each other and the parent.

Findings
- R1 .atelier/issues/atelier-ixdi.md:31: medium severity. Every new review round is a new delegate_task is still unconditional. A Codex-only team without T3 has no delegate_task, so it cannot meet this line even though line 30 says to use its native subagent tool and that nothing requires T3. The linked parent atelier-ss4y:48,66 also makes delegate_task conditional on T3. The title at line 16 and Goal at line 24 retain universal T3 phrasing, reinforcing the conflicting reading. Recommendation: FIX NOW. State that every round starts a fresh reviewer through the harness-appropriate tool, with delegate_task and the prohibition on reusing t3_thread_send applying under T3; scope the title and Goal to T3 as well. Keep the changed native-harness and single-provider guarantee. The same root contradiction is recorded against the parent because it incorporates this child.

Inspection: reviewed git show 87499c01 and both whole records, confirmed the parent/child relationship and read prerequisite atelier-dkul plus ADR 0029. Read the shipped SKILL.md and procedures/orchestrate.md under ~/projects/atelier-skill; neither requires a second provider or T3 today, and both match the copies in this checkout. The changed first Done when is a checkable text-and-behavior result. The parent recommends a different-lab reviewer while allowing the same model in a fresh context for a single-provider team. No other independent provider blocker was found.

Open questions: none needed for the minimal correction.
Residual risk and tests not run: planning inspection only; future review-loop behavior was not executed and runtime tests were not warranted. git diff --check 87499c01^ 87499c01 passed. No approval is recorded until this explicit T3 requirement is made conditional.
