---
schema: atelier.review
schema_version: 1
id: atelier-gwkt
title: 'Re-reviewed code commit 02fb5681 against my findings atelier-320e, atelier-x0ua, atelier-blgs and atelier-cunf. Read tracker_changes_since and the unowned branch path in land_text, rebuilt, ran both unowned_branch tests (pass), and reran scratch repos in /tmp. 320e: a feature branch that merged a newer origin/main while local main was stale now exits 0, and a record that branch really added still fails with its path named. Also checked with no local main (remote only) and with an extra unrelated remote that has a main (skipped because there is no merge base, still exits 0). Comparing against the intersection over every copy of the base can only drop paths that some copy already contains. x0ua: an orphan branch fails with land_work_untraceable and guidance, and the new test covers it. blgs: a detached HEAD prints ''detached HEAD'' and is tested. cunf: error, note and docs now all say ''tracker files'' (anything under .atelier/), and a workflow.yaml-only branch is refused under that wording. land is still read-only. Minor point, no action needed: any merge-base failure, not only ''no common ancestor'', is skipped and can end up reported as ''shares no history''.'
target: atelier-fxbg
identity: Independent reviewer (Claude subagent)
commit: b155db3a02d7da7ae6bcadf5b690035747a320ea
created_at: 2026-10-15T23:01:52.964634371Z
event:
  event: approval
---

Re-reviewed code commit 02fb5681 against my findings atelier-320e, atelier-x0ua, atelier-blgs and atelier-cunf. Read tracker_changes_since and the unowned branch path in land_text, rebuilt, ran both unowned_branch tests (pass), and reran scratch repos in /tmp. 320e: a feature branch that merged a newer origin/main while local main was stale now exits 0, and a record that branch really added still fails with its path named. Also checked with no local main (remote only) and with an extra unrelated remote that has a main (skipped because there is no merge base, still exits 0). Comparing against the intersection over every copy of the base can only drop paths that some copy already contains. x0ua: an orphan branch fails with land_work_untraceable and guidance, and the new test covers it. blgs: a detached HEAD prints 'detached HEAD' and is tested. cunf: error, note and docs now all say 'tracker files' (anything under .atelier/), and a workflow.yaml-only branch is refused under that wording. land is still read-only. Minor point, no action needed: any merge-base failure, not only 'no common ancestor', is skipped and can end up reported as 'shares no history'.
