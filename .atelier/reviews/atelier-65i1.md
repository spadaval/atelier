---
schema: atelier.review
schema_version: 1
id: atelier-65i1
title: 'docs/product/work-model.md:57-58: ''a rejected review moves the item back with the revise transition'' is wrong. In .atelier/workflow.yaml (and the default policy) revise is ''from: [review]'' only, and no transition leaves done; close is the step taken before review, so a done owner cannot be revised. A reader following the doc would try ''atelier issue transition <id> revise'' on a done item and be refused. Suggested fix: describe what actually happens to a rejected review of a done claim (findings are dispositioned fix_now with resolution commits on the branch while the claim stays done, or whatever Agent Factory prescribes), or add a done -> in_progress transition if that is the intended behaviour, and adjust the sentence to match.'
target: atelier-9k7o
identity: Independent reviewer (Claude subagent)
commit: 841273234e11924575876c5ae52c5356d0dc27ed
created_at: 2026-10-15T23:10:24.249044328Z
event:
  event: finding
  recommendation: fix_now
---

docs/product/work-model.md:57-58: 'a rejected review moves the item back with the revise transition' is wrong. In .atelier/workflow.yaml (and the default policy) revise is 'from: [review]' only, and no transition leaves done; close is the step taken before review, so a done owner cannot be revised. A reader following the doc would try 'atelier issue transition <id> revise' on a done item and be refused. Suggested fix: describe what actually happens to a rejected review of a done claim (findings are dispositioned fix_now with resolution commits on the branch while the claim stays done, or whatever Agent Factory prescribes), or add a done -> in_progress transition if that is the intended behaviour, and adjust the sentence to match.
