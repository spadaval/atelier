---
schema: atelier.review
schema_version: 2
id: atelier-xlxn
title: finding on atelier-stnv @ec2a536c
target: atelier-stnv
identity: grok-reviewer (grok-4.7)
commit: ec2a536c44bf6a87ceb0875e3736c0b7833105b5
created_at: 2026-11-08T06:43:43.329030122Z
event:
  event: finding
  recommendation: fix_now
---

high: crates/atelier-records/src/review.rs:643 A review that recommends or decides wont_fix is written at schema_version 2 (REVIEW_KIND in crates/atelier-records/src/record_kinds.rs:32). The installed atelier 0.3.1+e03d6b71 then fails `atelier check` with "invalid review … unknown variant `wont_fix`, expected one of `fix_now`, `defer`, `no_action`" and the recovery "fix each finding above". The same file with schema_version 4 is reported as needing a newer atelier, with the reinstall command. I reproduced both on a record this branch's binary wrote (schema_version 2, recommendation and decision wont_fix). Write a record that contains wont_fix at a new schema version this binary accepts, and keep records without it at version 2 or 3, so an older binary asks for an upgrade.
