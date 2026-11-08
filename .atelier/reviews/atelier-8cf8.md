---
schema: atelier.review
schema_version: 2
id: atelier-8cf8
title: approval on atelier-nz6a @dd46c40d
target: atelier-nz6a
identity: claude-code-reviewer-nz6a (opus-5.5)
commit: dd46c40d9e15b32c55b5af770ed80d89ad7dbafa
created_at: 2026-11-08T23:08:00.396578551Z
event:
  event: approval
---

Checked every Done when line against the branch text at dd46c40d. Description: names Atelier and keeps missions, delegation, review and diagnosis. Opening: comes before Core Rules and covers success, Not an Atelier repository (reproduced the exact message, exit 1), not found, and asked to set up. Its curl line matches README.md byte for byte, and the npx line matches README and the strategy. Plan-file fallback: grepped skills/agent-factory and found none left; ADR 0029 already replaces it. install.md and references/atelier.md agree. Read the nine Opus run transcripts in ~/.cache/af-evals/runs/nz6a: each loaded the db05a14b opening. non-atelier-repo stepped aside with no init and no records. cross-repo-regression and vacuous-test-review stepped aside after reading review.md in the same call and still passed, as the Report says; atelier-wl6u restores their coverage, and costly-hardening, plan-defect and rare-edge-chain still cover review on Atelier fixtures, so the Done when's explain-the-difference clause holds. The grader passes on a fresh setup.sh fixture with that run's transcript. harness test_run passes (4 tests). Counted SKILL.md at 100 and the install path at 265. git diff --check, cargo fmt --check, cargo metadata --locked and atelier check all pass. Two low findings deferred: atelier-0cjk and atelier-1yc4. CI is left to the merge gate.
