---
schema: atelier.review
schema_version: 1
id: atelier-9ep5
title: Checked commit 76d7af94 against review finding atelier-09wh, and reproduced on git 2.53 in a scratch repo. With a configured merge=ours CHANGELOG driver and a merge=union file, plain merge-tree resolves both cleanly. --attr-source=<empty tree> turns both into conflicts. Dropping the driver config leaves the union merge and gives a different clean text merge for the custom driver. So isolation fails closed and would send driver-resolved base merges to review, as the new Design and Close Reason say. The Close Reason matches Design items 1-3. Design item 2 records why it departs from Done when item 2 (HEAD^ is kept, with the reason). docs/product/reviews-and-merge.md agrees with the record. atelier check atelier-7hh4 passes. One deferred finding on imprecise wording in item 1.
target: atelier-7hh4
identity: Independent reviewer (Claude subagent)
commit: 76d7af940461b7572283b461b9ec56f28cafd73a
created_at: 2026-10-16T20:47:24.503060400Z
event:
  event: approval
---

Checked commit 76d7af94 against review finding atelier-09wh, and reproduced on git 2.53 in a scratch repo. With a configured merge=ours CHANGELOG driver and a merge=union file, plain merge-tree resolves both cleanly. --attr-source=<empty tree> turns both into conflicts. Dropping the driver config leaves the union merge and gives a different clean text merge for the custom driver. So isolation fails closed and would send driver-resolved base merges to review, as the new Design and Close Reason say. The Close Reason matches Design items 1-3. Design item 2 records why it departs from Done when item 2 (HEAD^ is kept, with the reason). docs/product/reviews-and-merge.md agrees with the record. atelier check atelier-7hh4 passes. One deferred finding on imprecise wording in item 1.
