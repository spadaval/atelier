---
schema: atelier.review
schema_version: 2
id: atelier-4isy
title: review on atelier-u1ko @85b10d67
target: atelier-u1ko
identity: GPT-6.1 Sol independent reviewer
commit: 85b10d67787fc4bf3e07f8244e5ff0b663b1292a
created_at: 2026-11-11T08:14:22.021771511Z
event:
  event: review
---

Round 1 independent review of 009cdab9..85b10d67. Verdict: merge in scope and value, after the blocking findings are fixed; no approval. Sizes: complexity high (five-repository journalled cutover, rollback and adoption), value high (required adoption of the rewritten tracker), risk high (records, refs, hooks and host-wide binary). Contract: fail, because preflight snapshot completion and retry after close violate the required resume contract. Fitness: fail pending R1 atelier-1fod (high, fix_now) and R2 atelier-cf70 (medium, fix_now). The settings, journal, scripts, workflow changes, transition docs, process templates and moved-file contents were inspected; all 79 files from next are present, with the eight explained content changes. Fresh proof: merge whitespace check, fmt, locked metadata, locked build -j 4, clippy all-targets -j 4 with warnings denied, nextest -j 4 (283 passed), converter tests (16 passed), CI-routing tests (24 passed), scratch release install -j 4 (atelier 0.5.0, 3968496 bytes), every supplied sandbox trial including full, resume and adopt/rollback (all passed), and the converted-record CI check commands (passed). The trial release binary matches the fresh scratch install. Read-only GitHub confirms CI and release plan success at latest code 1fc48e33; subsequent pinned-head changes are records only. Additional isolated trials reproduced both findings and were rolled back. The suite reported the real host fingerprint unchanged. Open questions: none needed to establish the findings. Residual risk: the actual cutover is not run or approved; live Claude-hook first-use proof was read in the recorded evidence, not repeated, and no real database content or private repository files were inspected. Every added mechanism traces to the cutover requirements. Only review records changed in the worktree.
