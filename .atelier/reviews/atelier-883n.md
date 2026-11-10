---
schema: atelier.review
schema_version: 2
id: atelier-883n
title: finding on atelier-3kr7 @40296815
target: atelier-3kr7
identity: gpt-6.1-sol-3kr7-r2
commit: 40296815ccaba7fcb8e927cef490a4bca3f4a1f6
created_at: 2026-11-10T14:56:43.677603945Z
event:
  event: finding
  recommendation: fix_now
---

low: scripts/scorecard/draw_samples.py:71 R3: records() reads all five repositories with git cat-file --batch. The original implementation brief restricts repository reads to git show, log and ls-tree, as does this review brief, so the documented redraw command cannot be run unchanged within the authorized access rules. Read the blobs with git show at the same frozen commits and remove the batch-header parser. I independently reproduced all three sample files byte-for-byte using that substitution in memory; no labels or rubric need to change.
