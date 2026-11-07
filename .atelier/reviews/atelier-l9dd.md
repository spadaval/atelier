---
schema: atelier.review
schema_version: 2
id: atelier-l9dd
title: 'The land_base_done_claim recovery guidance cannot be followed (crates/atelier-cli/src/commands/git.rs:273-276). It says to move the commit to a work branch and then run ''atelier start <id> there'', but the moved commit already has <id> done, so start stops at start.rs:85 with ''start_issue_done: <id> has done status done; there is no work to start''. The alternative it offers, ''supersede <id> instead'', also fails because the starter policy has no transitions out of done. Reproduced with the built binary: create, start and close an issue on main with an upstream, commit, run land (refused), then git switch -c task/<id>, git branch -f main main@{upstream}, atelier start <id>, which fails with start_issue_done. start (start.rs:193) shows the same message when the base has such a commit. The guidance has to say to undo the done claim first (for example, drop or revert the record change so the item is back in todo or in_progress), or it should name a recovery that works. The test asserts only that the text contains ''atelier start <id>'', so it does not catch this.'
target: atelier-fj1t
identity: Independent reviewer (Claude subagent)
commit: 2ceeb45048383ed43904476d464e056de03d8e8c
created_at: 2026-10-16T18:40:05.540973349Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: 943c7ee83e3c3e9a1d43a3bb50280b263bf25ef1
    recorded_at: 2026-10-16T18:48:25.526839164Z
    resolution_commit: 5e071c9c66a28c48db50f62de7c803104fab1021
    reason: 'Resolved: the advice now says git revert <commit> (or edit back) and then atelier start; base_done_claim only refuses a claim still at the same status at the tip, so the revert withdraws it. The extended test follows the advice end to end (revert, land passes, start succeeds), and replacing the tip condition with true makes that test fail.'
    migrated_from: atelier-k7xl
---

The land_base_done_claim recovery guidance cannot be followed (crates/atelier-cli/src/commands/git.rs:273-276). It says to move the commit to a work branch and then run 'atelier start <id> there', but the moved commit already has <id> done, so start stops at start.rs:85 with 'start_issue_done: <id> has done status done; there is no work to start'. The alternative it offers, 'supersede <id> instead', also fails because the starter policy has no transitions out of done. Reproduced with the built binary: create, start and close an issue on main with an upstream, commit, run land (refused), then git switch -c task/<id>, git branch -f main main@{upstream}, atelier start <id>, which fails with start_issue_done. start (start.rs:193) shows the same message when the base has such a commit. The guidance has to say to undo the done claim first (for example, drop or revert the record change so the item is back in todo or in_progress), or it should name a recovery that works. The test asserts only that the text contains 'atelier start <id>', so it does not catch this.
