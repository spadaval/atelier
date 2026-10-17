---
schema: atelier.review
schema_version: 1
id: atelier-uxrs
title: 'crates/atelier-app/src/review_gate.rs:1006 with crates/atelier-cli/src/commands/session.rs:538: on a base branch with no upstream, claim_nodes uses HEAD^ as the claim base, so a done claim committed one commit before HEAD counts as historical and its report parts are skipped outright, while close_reason is still checked on the same path. Reproduced: on main (policy requires all four parts, no upstream), commit a done record with an Outcome-only Close Reason; land fails; commit any unrelated file; land passes. With an upstream, land_base_done_claim catches it first, and land already prints ''Base commits unchecked'' here, so impact is limited to repositories without an upstream. Breaks ''land checks the required parts on committed claims, as it checks Close Reason today'' in that mode. Fix later: skip report parts for historical claims only when retained at a trusted point, or check them against the policy at the retained parent.'
target: atelier-sfsq
identity: Independent reviewer 3 (Claude subagent)
commit: 81a42d6dc49f9b1504c564482ab8ee507e8a1263
created_at: 2026-10-17T18:20:22.129418547Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/review_gate.rs:1006 with crates/atelier-cli/src/commands/session.rs:538: on a base branch with no upstream, claim_nodes uses HEAD^ as the claim base, so a done claim committed one commit before HEAD counts as historical and its report parts are skipped outright, while close_reason is still checked on the same path. Reproduced: on main (policy requires all four parts, no upstream), commit a done record with an Outcome-only Close Reason; land fails; commit any unrelated file; land passes. With an upstream, land_base_done_claim catches it first, and land already prints 'Base commits unchecked' here, so impact is limited to repositories without an upstream. Breaks 'land checks the required parts on committed claims, as it checks Close Reason today' in that mode. Fix later: skip report parts for historical claims only when retained at a trusted point, or check them against the policy at the retained parent.
