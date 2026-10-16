---
schema: atelier.review
schema_version: 1
id: atelier-c1ws
title: 'Re-reviewed 9c48a595 as merged into decb65a8. Replayed 8x3h (move a blocks entry to relates while keeping both labels) and as4e (take ours on the owner after the base added a child): both FAIL review.current; the union control passes. Checked record_three_way and merged: list order is ignored and entries compare whole; absent and null are treated alike (harmless for these records); unparsed front matter falls back to a string and mismatched types fail; a body both sides edited fails, so it needs review; updated_at and closed_at must equal one side wholesale, so they cannot carry other changes. Duplicate entries allow injection (atelier-6dwg, deferred). Evidence and review conflicts go to allowed_edge, which allows new files only; non-record conflicts and edits outside conflicts make merge_resolution None; more than one merge base also makes it None. n7g1 clean merges still need in-scope records to be status-only, and a base-added child record is refused. Docs match the code, including blocked_by being stored in the owner''s blocks. merge_gates passes 43/43 locally.'
target: atelier-1isk
identity: Independent reviewer (Claude subagent)
commit: decb65a8e086ca314995b9534104ce64d67261eb
created_at: 2026-10-16T20:21:30.012463646Z
event:
  event: approval
---

Re-reviewed 9c48a595 as merged into decb65a8. Replayed 8x3h (move a blocks entry to relates while keeping both labels) and as4e (take ours on the owner after the base added a child): both FAIL review.current; the union control passes. Checked record_three_way and merged: list order is ignored and entries compare whole; absent and null are treated alike (harmless for these records); unparsed front matter falls back to a string and mismatched types fail; a body both sides edited fails, so it needs review; updated_at and closed_at must equal one side wholesale, so they cannot carry other changes. Duplicate entries allow injection (atelier-6dwg, deferred). Evidence and review conflicts go to allowed_edge, which allows new files only; non-record conflicts and edits outside conflicts make merge_resolution None; more than one merge base also makes it None. n7g1 clean merges still need in-scope records to be status-only, and a base-added child record is refused. Docs match the code, including blocked_by being stored in the owner's blocks. merge_gates passes 43/43 locally.
