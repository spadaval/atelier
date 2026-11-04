---
schema: atelier.review
schema_version: 1
id: atelier-rbyl
title: finding on atelier-ot1c @c4b7c6f5
target: atelier-ot1c
identity: claude-reviewer-ot1c
commit: c4b7c6f59fd4657696d74f90954c132a8bb4ac6d
created_at: 2026-11-04T22:40:56.907974854Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/merge-ci-passed.sh:15 A record-only work branch (no change outside .atelier/ between fork and head) can never pass: the loop checks every branch commit, none has a CI run because record-only work opens no PR, and the script exits 1. AGENTS.md and repository-contribution.md say record-only changes skip the PR, and history shows record-only branches merged through atelier merge (task/atelier-gddg 728aa41c, validation/atelier-1o56 11e654e7, task/atelier-k4xg, validation/atelier-prjr). Reproduced with a synthetic merge of master plus one .atelier/issues commit: exit 1 'no successful CI run'. Fix: right after computing fork, exit 0 when `git diff --quiet "$fork" "$head" -- . ':!.atelier'` (before the base-moved check, since merged code then equals the base's), and add that case to the header comment.
