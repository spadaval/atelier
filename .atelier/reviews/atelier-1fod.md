---
schema: atelier.review
schema_version: 2
id: atelier-1fod
title: finding on atelier-u1ko @85b10d67
target: atelier-u1ko
identity: GPT-6.1 Sol independent reviewer
commit: 85b10d67787fc4bf3e07f8244e5ff0b663b1292a
created_at: 2026-11-11T08:11:11.744571162Z
event:
  event: finding
  recommendation: fix_now
---

high: scripts/cutover/cutover.sh:164 R1: Preflight is marked done before its rollback snapshot is complete. record writes done mode before any hooks backup, binary, skill or service snapshot. If a backup fails, rerunning skips record entirely at lines 489-490 and starts conversion without those recovery records. Reproduced with a one-shot cp failure on the first hooks backup in a review sandbox: the first run stopped in step 1; the retry committed step 2 with only 1 of 5 base snapshots and no service snapshot. Continuing would install and publish without enough state to undo it. Mark preflight complete only after every snapshot succeeds, resume incomplete snapshots without overwriting originals, and add this failure-and-retry trial.
