---
schema: atelier.review
schema_version: 2
id: atelier-1ymx
title: finding on atelier-pz1y @94aa3f83
target: atelier-pz1y
identity: sol-pz1y-r3
commit: 94aa3f83d442674ded786355c883e89266357e16
created_at: 2026-11-10T22:43:08.949048835Z
event:
  event: finding
  recommendation: fix_now
---

Low: next/crates/atelier-cli/src/output.rs:732: R5. The recovery instruction for a plain commit setting an item done does not get the push past the hooks. Reproduced with both real hooks and a local bare remote: commit ready to done on main while demo-ab12 exists; pre-push refuses and says to put the status back on main. Restoring ready and committing is itself refused by pre-commit (done to ready has no act row). Even a corrective commit made with --no-verify still leaves pre-push refusing the original Call done commit. The contract requires a way out. Explain that the offending unpublished commit must be removed or rewritten, not followed by a corrective commit, and test the stated recovery with both hooks enabled. Account for pre-commit comparing against HEAD if the advice uses amend.
