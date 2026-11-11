---
schema: atelier.review
schema_version: 2
id: atelier-o6x7
title: finding on atelier-a4rs @d7a45eef
target: atelier-a4rs
identity: GPT-6.1 Sol independent reviewer
commit: d7a45eefb56cd775f5634b8ec998d5f53d0fbbef
created_at: 2026-11-11T05:51:19.320027801Z
event:
  event: finding
  recommendation: fix_now
---

medium: evals/agent-factory/scenarios/next-decide/grade.py:67-71 checks only the net branch diff and the final worktree, so a decider can commit code changes and undo them without being caught. The grader docstring promises to inspect every commit since setup, and the scenario requires the decider to change no code. Reproduced with its real setup and the copied 399fca6a binary: change urls.py to fall back to untitled, commit the out-of-scope fix, restore and commit the original urls.py, then run decide --defer. grade.py returns pass=true, other_files_changed=[], and says nothing else changed despite the two code commits. Scratch reproduction: next-decide-grade-inqczmxf under the review TMPDIR. The same tip-only check also misses transient edits to protected item fields. Inspect each post-setup commit and the final worktree for forbidden file/item changes, and add a failing fix-then-revert walk alongside the valid decide-only paths.
