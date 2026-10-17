---
schema: atelier.review
schema_version: 1
id: atelier-u0i3
title: 'crates/atelier-app/src/review_gate.rs:902: the merge-base fix closes the weaken-and-restore route, but a branch can still skip parts the base requires by forging the claim''s topology: after the owner''s branch has merged the base that requires the report, create a side branch from a commit before that merge (e.g. the branch''s pre-merge tip), edit a scoped child to done with an Outcome-only Close Reason there, and merge the side branch back. merge-base(claim commit, base) is then the old base, whose policy lacks the parts, so land passes (reproduced in a scratch repo; the control that closes the same child directly after the base merge fails with required_field.report.*). This topology is indistinguishable from the honest atelier-tqgi case (a merge whose one parent holds the claim and the other the policy change; parent order is author-controlled), so no branch-topology rule can keep tqgi and close this. Not FIX NOW: it needs a deliberately constructed history and the record visibly lacks the report in the reviewed diff. Decide explicitly: either accept it as a documented limitation of the tqgi exemption, or drop the branch-side exemption so scope claims always meet the base policy at land/merge (honest children then add their report, a small cost).'
target: atelier-sfsq
identity: Independent reviewer 3 (Claude subagent)
commit: 81a42d6dc49f9b1504c564482ab8ee507e8a1263
created_at: 2026-10-17T18:20:21.543148642Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/review_gate.rs:902: the merge-base fix closes the weaken-and-restore route, but a branch can still skip parts the base requires by forging the claim's topology: after the owner's branch has merged the base that requires the report, create a side branch from a commit before that merge (e.g. the branch's pre-merge tip), edit a scoped child to done with an Outcome-only Close Reason there, and merge the side branch back. merge-base(claim commit, base) is then the old base, whose policy lacks the parts, so land passes (reproduced in a scratch repo; the control that closes the same child directly after the base merge fails with required_field.report.*). This topology is indistinguishable from the honest atelier-tqgi case (a merge whose one parent holds the claim and the other the policy change; parent order is author-controlled), so no branch-topology rule can keep tqgi and close this. Not FIX NOW: it needs a deliberately constructed history and the record visibly lacks the report in the reviewed diff. Decide explicitly: either accept it as a documented limitation of the tqgi exemption, or drop the branch-side exemption so scope claims always meet the base policy at land/merge (honest children then add their report, a small cost).
