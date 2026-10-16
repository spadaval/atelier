---
schema: atelier.review
schema_version: 1
id: atelier-f1vy
title: 'Test gaps in crates/atelier-cli/tests/session.rs:2745 (a_kept_branch_whose_done_owner_was_pruned_from_the_base_shows_merged): (1) it asserts only prime and status, not the other-branch row from branch_lines_for (git.rs:551) that the requirement also names; that path shares unmerged_done_label so it should work, but a regression in how branch_lines_for passes the branch would go unseen. (2) The uncommitted-close check uses an issue whose record is still on main, so it goes through the base-copy branch of the match (git.rs:390), not the new fallback. No test covers the fallback''s negative case: base lacks the record, tip is in the base, but the tip''s committed status is not done (e.g. a kept branch merged while in progress, closed and pruned on main, then an uncommitted close on the branch). Dropping the status check in the fallback would still pass every test.'
target: atelier-lwdu
identity: Independent reviewer (Claude subagent)
commit: 05251a36b7000a50e4929205754fb27cb429f36a
created_at: 2026-10-16T19:32:05.050628892Z
event:
  event: finding
  recommendation: defer
---

Test gaps in crates/atelier-cli/tests/session.rs:2745 (a_kept_branch_whose_done_owner_was_pruned_from_the_base_shows_merged): (1) it asserts only prime and status, not the other-branch row from branch_lines_for (git.rs:551) that the requirement also names; that path shares unmerged_done_label so it should work, but a regression in how branch_lines_for passes the branch would go unseen. (2) The uncommitted-close check uses an issue whose record is still on main, so it goes through the base-copy branch of the match (git.rs:390), not the new fallback. No test covers the fallback's negative case: base lacks the record, tip is in the base, but the tip's committed status is not done (e.g. a kept branch merged while in progress, closed and pruned on main, then an uncommitted close on the branch). Dropping the status check in the fallback would still pass every test.
