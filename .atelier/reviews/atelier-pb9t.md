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
  disposition:
    decision: no_action
    identity: gpt-6.1-sol
    commit: 00dc91fefc0f7b7741630a4529891811155da5e0
    recorded_at: 2026-11-11T02:10:34.944914197Z
    reason: 'The malformed-item case is fixed by 5a990a0a and its regression passes. Accept the rebuttal for the remaining malformed mission.md case: independently reproduced review and verdict approve exiting 2 naming mission.md, with no plan printed, no verdict committed and HEAD unchanged; check reports the malformed record. This fails safely under the product specification Hooks and the bypass rule that a record Atelier cannot parse cannot be skipped past. Reading review files independently of their malformed mission would change the existing record boundary beyond this item. No action on that remaining case. Reviewed at 00dc91fefc0f7b7741630a4529891811155da5e0.'
---

R3 low: next/crates/atelier-cli/src/review.rs:186 A committed plan still cannot be reviewed when an uncommitted item edit does not parse. plan_changed() eagerly loads every checkout item with Store::items() before using historical membership. Reproduced: commit a valid mission and its draft item, change only that item's checkout status to broken without committing, then review the mission; exit 2, with no committed plan or exclusion line. An uncommitted broken mission also fails earlier at store.mission(). The first Done when line requires the committed plan and a line naming uncommitted plan changes. Separate the checkout's review files from the committed definition and determine changed-path membership without requiring every uncommitted definition to parse; test malformed and deleted plan records.
