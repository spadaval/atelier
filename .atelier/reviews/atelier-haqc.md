---
schema: atelier.review
schema_version: 1
id: atelier-haqc
title: 'Checked the rest. The test proves the mixed-commit path for review records: in a scratch copy, replacing the cherry-pick -n / git rm block with a plain `git cherry-pick` of the fix makes the merge refuse with ''review atelier-... pins a commit outside the work branch''. Evidence records are not ancestry-checked (they only go stale), so the evidence half of the advice is not gate-enforced and the test does not cover it; that is acceptable. Planning edits are kept correctly, because only files added under .atelier/reviews and .atelier/evidence are removed, and modified .atelier/issues records come through the cherry-pick. The CLI only adds review and evidence files and never modifies them, so the add-only filter is enough. cargo nextest run --workspace: 642 passed.'
target: atelier-imbo
identity: Independent reviewer (Claude subagent)
commit: 503453f194deb61db01ab1f93e29e7bd1f58fd62
created_at: 2026-10-16T20:47:42.235758198Z
event:
  event: finding
  recommendation: no_action
---

Checked the rest. The test proves the mixed-commit path for review records: in a scratch copy, replacing the cherry-pick -n / git rm block with a plain `git cherry-pick` of the fix makes the merge refuse with 'review atelier-... pins a commit outside the work branch'. Evidence records are not ancestry-checked (they only go stale), so the evidence half of the advice is not gate-enforced and the test does not cover it; that is acceptable. Planning edits are kept correctly, because only files added under .atelier/reviews and .atelier/evidence are removed, and modified .atelier/issues records come through the cherry-pick. The CLI only adds review and evidence files and never modifies them, so the add-only filter is enough. cargo nextest run --workspace: 642 passed.
