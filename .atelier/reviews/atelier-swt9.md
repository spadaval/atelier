---
schema: atelier.review
schema_version: 1
id: atelier-swt9
title: 'The new error text reverses the recovery order. Before, a closing parenthesis ended the ''record proof and review again'' aside, so ''then merge the base in'' followed the cherry-pick step. Now the message reads ''...: those pin rewritten commits, so record proof and review again, then merge the base in with `git merge` instead'', which tells the user to record proof and review before merging the base. Merging the base afterward makes that proof and review stale: the test itself shows FAIL review.current and FAIL evidence.attached right after the merge. Reword so the order is: reset, cherry-pick without records, merge the base, then record proof and review again. Shorter sentences would also make the message easier to read.'
target: atelier-imbo
identity: Independent reviewer (Claude subagent)
commit: 503453f194deb61db01ab1f93e29e7bd1f58fd62
created_at: 2026-10-16T20:47:42.208927792Z
event:
  event: finding
  recommendation: fix_now
---

The new error text reverses the recovery order. Before, a closing parenthesis ended the 'record proof and review again' aside, so 'then merge the base in' followed the cherry-pick step. Now the message reads '...: those pin rewritten commits, so record proof and review again, then merge the base in with `git merge` instead', which tells the user to record proof and review before merging the base. Merging the base afterward makes that proof and review stale: the test itself shows FAIL review.current and FAIL evidence.attached right after the merge. Reword so the order is: reset, cherry-pick without records, merge the base, then record proof and review again. Shorter sentences would also make the message easier to read.
