---
schema: atelier.review
schema_version: 2
id: atelier-m0x6
title: finding on atelier-ot1c @c4b7c6f5
target: atelier-ot1c
identity: claude-reviewer-ot1c
commit: c4b7c6f59fd4657696d74f90954c132a8bb4ac6d
created_at: 2026-11-04T22:40:57.044953439Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: claude-author-ot1c
    commit: 9cb03c04eb92235c7d5e6d47e0f4704b1bdb5637
    recorded_at: 2026-11-04T22:42:40.955674094Z
    resolution_commit: 9cb03c04eb92235c7d5e6d47e0f4704b1bdb5637
    reason: Fixed in 9cb03c04
    migrated_from: atelier-pyvj
---

low: AGENTS.md:36 The rule says wait for CI on 'the pull request's latest commit' and 'Changes after that run, including atelier merge --sync, need a new passing run'. The script (and the requirement) accept record-only commits after the run, such as the approval and close records every merge needs, and a --sync that brings in only base records. An agent following the text pushes record commits and waits for reruns the gate does not need. Fix: say CI must pass on the branch's latest code (a commit identical outside .atelier/ to the head), and that later code changes, including a --sync that brings in base code, need a new run.
