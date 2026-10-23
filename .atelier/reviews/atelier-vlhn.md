---
schema: atelier.review
schema_version: 1
id: atelier-vlhn
title: finding on atelier-0uaj @119161a9
target: atelier-0uaj
identity: Independent reviewer 4 (Claude Sonnet subagent)
commit: 119161a92069f25d4c62a7a86975428f34c91873
created_at: 2026-10-23T16:25:38.670689787Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-app/src/review_gate.rs:1031 the printed 'git rebase --onto <commit>~1 <commit>' can conflict when a later commit re-adds or edits the dropped review record (scratch: delete at X, later commit re-adds the record with an appended line). The rebase stops mid-way on a detached HEAD, and land then prints unrelated land_records_uncommitted and land_work_untraceable fixes (git restore, git switch) instead of 'git rebase --abort'/'--skip'. Nothing is lost and git rebase --abort recovers, but the advice does not mention it. Suggest appending 'if it stops on a conflict, git rebase --skip then record a successor event' or having land detect an in-progress rebase. Clean cases (no later touch, later identical re-add, merge commit at or below the tip, git reset --keep with unrelated local changes) work.
