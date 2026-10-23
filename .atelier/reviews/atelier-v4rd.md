---
schema: atelier.review
schema_version: 1
id: atelier-v4rd
title: finding on atelier-0uaj @6760597b
target: atelier-0uaj
identity: Independent reviewer 3 (Claude Sonnet subagent)
commit: 6760597bcc51657bcbb5abd5c88975c779ad12e0
created_at: 2026-10-23T16:18:11.568731141Z
event:
  event: finding
  recommendation: fix_now
---

major: crates/atelier-app/src/committed_snapshot.rs:72 (commit_command) and crates/atelier-cli/src/gate_report.rs Uncommitted::new: when a review record is deleted or modified in the checkout (reviews are append-only), land and merge --check print 'land_records_uncommitted/canonical_snapshot_uncommitted: ... (deleted). Next: git add .atelier && git commit -m "…"'. Running that fix commits the deletion, after which land and merge refuse permanently with review_append_only ('append a successor event'), no Next line, and every other gate unevaluated ('Not checked'). Reproduced in scratch repo with an approved owner: rm .atelier/reviews/<id>.md; atelier land; run the fix; atelier land -> refused with no fix; git checkout HEAD~1 -- the file then fails with the same error because the bad commit stays in the range. Same for a modified review record. The fix for a deleted or modified review record must be git restore --source=HEAD --staged --worktree -- <path> (or git checkout HEAD -- <path>); the review_append_only line should also carry a runnable Next. Pre-existing on master, but the requirement (S1) names a runnable fix for a deleted review record and this change reworked this exact path.
