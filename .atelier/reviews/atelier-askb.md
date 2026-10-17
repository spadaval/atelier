---
schema: atelier.review
schema_version: 1
id: atelier-askb
title: 'Fresh approval at 10618e5f. Reviewed git diff 1246633a HEAD (non-.atelier), focusing on c03d385b/c4a3db09 and the merge resolutions. r5n7 fixed: create_next_step offers start only when the workflow has a start transition from the created status; backlog create in a fixture now offers ''atelier issue transition <id>'', with a test. cd3l fixed: ScopedWork rows follow work_order with the parent row first, Ready Work carries in-flight marks, and the footer starts the first ready item no branch has started; the fixture mission now offers a medium child rather than a low item by ID, and the new session test pins priority order and the in-flight skip. The quiet order change (ID order to work order for scoped views) is acceptable: it matches work ready and the human list, and the test still checks every ID. 6trx deferred to atelier-ytxg is fine. Merge 28a064cf keeps &Issue and derives Mission/Parent Rollup from issue.issue_type, which is correct (the inner loop''s title shadowing is harmless); 161bd3ef and f9879858 needed no resolution in crates or docs. Ran full cargo nextest -j 4: 736 passed; cargo fmt --check and git diff --check on code: clean. Not re-run: land/merge end to end.'
target: atelier-ksx6
identity: Independent reviewer (Claude subagent)
commit: 10618e5fbea439756ff2d35e48c4e1b918180680
created_at: 2026-10-17T08:40:51.678231554Z
event:
  event: approval
---

Fresh approval at 10618e5f. Reviewed git diff 1246633a HEAD (non-.atelier), focusing on c03d385b/c4a3db09 and the merge resolutions. r5n7 fixed: create_next_step offers start only when the workflow has a start transition from the created status; backlog create in a fixture now offers 'atelier issue transition <id>', with a test. cd3l fixed: ScopedWork rows follow work_order with the parent row first, Ready Work carries in-flight marks, and the footer starts the first ready item no branch has started; the fixture mission now offers a medium child rather than a low item by ID, and the new session test pins priority order and the in-flight skip. The quiet order change (ID order to work order for scoped views) is acceptable: it matches work ready and the human list, and the test still checks every ID. 6trx deferred to atelier-ytxg is fine. Merge 28a064cf keeps &Issue and derives Mission/Parent Rollup from issue.issue_type, which is correct (the inner loop's title shadowing is harmless); 161bd3ef and f9879858 needed no resolution in crates or docs. Ran full cargo nextest -j 4: 736 passed; cargo fmt --check and git diff --check on code: clean. Not re-run: land/merge end to end.
