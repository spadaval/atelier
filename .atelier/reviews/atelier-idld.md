---
schema: atelier.review
schema_version: 1
id: atelier-idld
title: finding on atelier-5fus @58e3a1bd
target: atelier-5fus
identity: lighter-workflow reviewer for atelier-5fus (claude-opus-5-5)
commit: 58e3a1bdbf6121f97858de03012351421bb43e84
created_at: 2026-11-06T23:53:48.313749725Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-5fus.md Notes (classification item 7 and Reachability) The note forces finding 4 because the findings must reach the author 'with the reviewer in its own checkout, through a commit', but does not apply that same hand-off in the other direction. The author's fix_now disposition names the fix commit (so it cannot ride with it) and is the author's, so under that premise it must reach the reviewer in an author commit before the reviewer's acceptances and approval; it cannot join 9-13 as the note proposes ('7, 9-13 into one'), and it cannot join the follow-up issue's commit, which the scenario fixes as exactly that issue's records. So one post-fix author commit is forced too: forced is 4 (2, 4, 7-or-its-batch, 13), the floor is 6 not 5, and 'at most 6' is reachable with no commit to spare. The stated path to 6 (batch the review round and the post-fix records) actually gives 7 (start, receipt, close, findings, follow-up, author dispositions, reviewer acceptances+approval); reaching 6 needs all three batchings, including receipt+close, which the note presents as optional. atelier-5822 would plan against the wrong margin. Fix: reclassify 7 (or the batch of the author's post-fix dispositions) as forced by hand-off, restate the floor as 6 and the needed batchings as all three, and recheck the 'cannot batch post-fix' best case and the example rewrite against the corrected floor (or drop the hand-off premise for finding 4 too and say the floor is lower, consistently).
