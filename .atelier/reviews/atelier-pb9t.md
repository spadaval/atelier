---
schema: atelier.review
schema_version: 2
id: atelier-pb9t
title: finding on atelier-8myy @5f7737c1
target: atelier-8myy
identity: gpt-6.1-sol
commit: 5f7737c1d5c68bc04379a944f589437a75e86734
created_at: 2026-11-11T02:00:35.336482848Z
event:
  event: finding
  recommendation: fix_now
---

R3 low: next/crates/atelier-cli/src/review.rs:186 A committed plan still cannot be reviewed when an uncommitted item edit does not parse. plan_changed() eagerly loads every checkout item with Store::items() before using historical membership. Reproduced: commit a valid mission and its draft item, change only that item's checkout status to broken without committing, then review the mission; exit 2, with no committed plan or exclusion line. An uncommitted broken mission also fails earlier at store.mission(). The first Done when line requires the committed plan and a line naming uncommitted plan changes. Separate the checkout's review files from the committed definition and determine changed-path membership without requiring every uncommitted definition to parse; test malformed and deleted plan records.
