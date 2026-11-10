---
schema: atelier.review
schema_version: 2
id: atelier-uxtd
title: finding on atelier-jq24 @949a8b62
target: atelier-jq24
identity: gpt-6.1-sol
commit: 949a8b62c729bd70f049a47c4539fae77f01f578
created_at: 2026-11-10T20:54:35.656511149Z
event:
  event: finding
  recommendation: fix_now
---

R1, medium: next/crates/atelier-app/src/acts.rs:634 permits start to take over a started item after its mission was dropped. Reproduced with the pinned next/ binary in a scratch repository: new --mission; new --in; ready both and commit; t1 starts the item in a second worktree; drop the mission on main and commit; t2 runs start <item>. It exits 0, commits a new implementer and prints Took over, with no dropped-mission refusal. Done when requires start on an item of a dropped mission to be refused, and dropping a ready mission with started items is supported. commands::start selects TakeOver for these items, but only Act::Start checks the mission. Refuse a takeover when its mission is dropped before switching or writing, and add the started-item scenario; the mission drop itself must continue to leave every item unchanged.
