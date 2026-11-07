---
schema: atelier.review
schema_version: 2
id: atelier-9yms
title: 'G14: land accepts staged canonical record changes when working bytes match HEAD. In /tmp/session-review-canonical-index-yt8wr3id/repo, stage an issue change and restore its working file to HEAD; git status reports MM .atelier/issues/atelier-svma.md but land passes at 5b5dca62c10736c4a13b19f56d350f71e8cadd96. S6 requires committed canonical records and fresh index checks. Refuse canonical status paths, including rename sources, while permitting honest unfinished ordinary code. This also reproduces at aab00773 and is a preexisting invariant gap, not a snapshot regression.'
target: atelier-1ioo
identity: session-reviewer
commit: 5b5dca62c10736c4a13b19f56d350f71e8cadd96
created_at: 2026-10-02T01:31:27.343154858Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: session-reviewer
    commit: 84395a9d3ba6d28550023c3234fb166cc6cd5211
    recorded_at: 2026-10-02T02:02:59.499109972Z
    resolution_commit: 84395a9d3ba6d28550023c3234fb166cc6cd5211
    reason: 'G14 independently verified at root composition 84395a9d: original canonical MM, assume-unchanged and rename-source fixtures now refuse; active ordinary unfinished code and raw-equal filter false positives pass. Pinned production-module harness passed 44 SHA1/SHA256 index inventory, conflict stage, mode, flag, newline/tab/Unicode path and filter boundary checks. Reused snapshot freshly rejects/restores hidden bytes, ignored inventory and modes. Root full-hooks proof 554/554, zero skipped.'
    migrated_from: atelier-0i3x
---

G14: land accepts staged canonical record changes when working bytes match HEAD. In /tmp/session-review-canonical-index-yt8wr3id/repo, stage an issue change and restore its working file to HEAD; git status reports MM .atelier/issues/atelier-svma.md but land passes at 5b5dca62c10736c4a13b19f56d350f71e8cadd96. S6 requires committed canonical records and fresh index checks. Refuse canonical status paths, including rename sources, while permitting honest unfinished ordinary code. This also reproduces at aab00773 and is a preexisting invariant gap, not a snapshot regression.
