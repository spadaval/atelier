---
schema: atelier.review
schema_version: 1
id: atelier-dk92
title: 'crates/atelier-workflow/src/lib.rs:137-141 (starter work.supersede). The ''supersede'' transition has no work.terminal check, so a parent can reach ''superseded'', a terminal status, while its children are open. The open children are then classed as retired under a terminal ancestor (work_classification). Read literally, the o6h9 Done-when says a parent ''cannot reach a terminal status until every child is terminal''. That also covers supersede, but the retire-descendants behaviour suggests the gap is intended. Suggest the human confirm the intent: either add work.terminal to supersede, or narrow the wording in ADR 0021, work-model.md and the requirement to ''closes (done) only after its children are terminal''.'
target: atelier-o6h9
identity: Independent reviewer (Claude subagent)
commit: 4f66aab9fee9062a0f8cf6b3f586acc04a0cc218
created_at: 2026-10-16T00:37:31.225255100Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-workflow/src/lib.rs:137-141 (starter work.supersede). The 'supersede' transition has no work.terminal check, so a parent can reach 'superseded', a terminal status, while its children are open. The open children are then classed as retired under a terminal ancestor (work_classification). Read literally, the o6h9 Done-when says a parent 'cannot reach a terminal status until every child is terminal'. That also covers supersede, but the retire-descendants behaviour suggests the gap is intended. Suggest the human confirm the intent: either add work.terminal to supersede, or narrow the wording in ADR 0021, work-model.md and the requirement to 'closes (done) only after its children are terminal'.
