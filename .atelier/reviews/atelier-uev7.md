---
schema: atelier.review
schema_version: 1
id: atelier-uev7
title: finding on atelier-r0fj @7defff87
target: atelier-r0fj
identity: claude-review-r0fj
commit: 7defff87584386fc5d76c94031efb428fc378dcb
created_at: 2026-11-04T23:37:23.100641644Z
event:
  event: finding
  recommendation: fix_now
---

high: crates/atelier-app/src/record_history.rs:126-148 (first_parent_lines, atelier-jnec) walks the base line to the fork, then fork..HEAD with --first-parent, which still includes the branch's own commits from before it merged the base. Their record versions now sort after newer base versions, so a scope record the branch touched before merging the base reads as done (base) then open (stale branch version) then done (merge): a false 'Reopened ... after a done claim' deviation and 'Changed course' line on the branch that the base's debrief does not show, contrary to jnec's Done when (same start and deviations as from the base). Before this change the single first-parent walk did not misreport this case, so it is a regression. Repro (target/debug/atelier at 7defff87): mission M started on main with scope item X; branch feature runs 'issue update X --label touched' and commits; main closes X; feature merges main (-X theirs); 'atelier debrief M' on feature prints 'Changed: Reopened on ... after a done claim.', main and the installed master binary do not. The same ordering can move the mission's close moment. Fix: on the branch's line, do not let a version from a commit that the fork does not descend from follow base versions of the same record (e.g. walk --ancestry-path fork..HEAD and count each base merge as its diff against the base parent, so branch-only changes made before the merge arrive with it), and add this case to a_debrief_from_a_branch_that_merged_the_base_keeps_the_base_history (a branch commit touching a scope record before the merge).
