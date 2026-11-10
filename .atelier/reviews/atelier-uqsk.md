---
schema: atelier.review
schema_version: 2
id: atelier-uqsk
title: finding on atelier-syyj @2d35d0a5
target: atelier-syyj
identity: gpt-6.1-sol
commit: 2d35d0a574fe64d9c96455410160e593e9fe16c2
created_at: 2026-11-10T17:50:34.790110504Z
event:
  event: finding
  recommendation: fix_now
---

high: next/crates/atelier-cli/src/merge.rs:73 Capture and validate one head before reading approval facts. repo.checkout at line 57 checks staleness against HEAD, but line 73 snapshots HEAD only after that check. A concurrent source commit in between becomes the head used by merge-tree and moved(), so the final confirmation accepts code the review did not cover. Reproduced offline in a scratch repository with a local bare origin: a Git wrapper first returns the clean status from repo.checkout, then commits hello.txt before the next Git call. merge exits 0, main contains the unreviewed source, and the item is done. A control with that source change committed before merge exits 1 and requests submit. Snapshot the branch head before loading the record and approval facts, read committed inputs against that exact hash, and confirm the branch and checkout still match it before writing. Add a regression covering movement during the initial reads, not only during the shell checks. Required proof passed at this head: root and next formatting, root offline locked metadata, both requested and whole-range whitespace checks, and nextest offline with build-jobs 4 and test jobs 4 (100 passed). No code was edited; the scratch repositories were removed.
