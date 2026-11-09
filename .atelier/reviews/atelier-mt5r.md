---
schema: atelier.review
schema_version: 2
id: atelier-mt5r
title: finding on atelier-fyay @91fa9de4
target: atelier-fyay
identity: gpt-6.1-sol
commit: 91fa9de4b438a6439f6381dcc4f74ef029603657
created_at: 2026-11-09T00:52:00.385306924Z
event:
  event: finding
  recommendation: fix_now
---

low: skills/agent-factory/procedures/plan.md:46 R1: The added guidance grows the plan role's read path from 302 to 315 lines (SKILL.md 97 + plan.md 113 + references/atelier.md 105), exceeding atelier-qj9d's 300-line limit by 15. The baseline was already two lines over, but this change adds another 13 and .atelier/issues/atelier-fyay.md:38 incorrectly claims it is well inside the limit. Compact the read path to at most 300 lines without losing the new guidance, and correct the Report.

Verdict: merge after R1 is fixed. Complexity low (one existing procedure, no new mechanism); value high (directly serves the requested mission-confidence outcome); risk medium (planning guidance affects work across repositories, but is reversible). Contract: pass on all four Done when lines. Fitness: fail on the read-path budget.

The strategy-versus-route test is usable with SKILL.md's strategy definition and plan.md's Layers: feasibility or a required contract that could change the target must be settled first, while implementation choices within that target can wait for evidence. The explicit instruction not to settle every unknown and the residual Risks and Unknowns preserve judgment. No contradiction found with initial choices, issue TBD-to-spike guidance, orchestrate.md's disposable spike branches, or references/atelier.md's choice records and planning review. The readiness sentence usefully reinforces the shaping test.

Proof at 91fa9de4: the prescribed HEAD^ whitespace check, whole-branch whitespace check, cargo fmt -- --check, and cargo metadata --locked --format-version 1 all pass. scripts/merge-ci-passed.sh exits 2 because HEAD is not a merge commit and it runs only inside atelier merge's merge checkout; CI gating remains unverified here. Read the entire requested diff and surrounding guidance, and counted both base and head read paths. No agent behavior evaluation was run, so actual planner compliance remains unproven. No approval recorded.
