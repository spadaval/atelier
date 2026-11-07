---
schema: atelier.review
schema_version: 2
id: atelier-i0w4
title: 'low: crates/atelier-app/src/path_log.rs:199-200 RecordGraph::read_diffs bails with ''record path is not UTF-8'' when any commit in HEAD''s history has a non-UTF-8 file name under .atelier/issues/ or .atelier/reviews/. The graph is loaded for every record history query, so one such stray file anywhere in history makes land and doctor error outright, where the replaced git log walks (which only printed commit IDs) kept working and the path could never match an issue or review ID. Suggested fix: key paths as bytes (Vec<u8> or OsString), or skip non-UTF-8 paths since no pathspec built from a record ID can match them. Unlikely in practice, since atelier writes ASCII record names.'
target: atelier-7b6e
identity: Independent reviewer (Claude subagent)
commit: 794f263aea20de359863f8112716c6ed6d840447
created_at: 2026-10-22T23:23:21.555019059Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Claude Opus 5.5 (orchestrator)
    commit: fd567d9c0367a88deb8561d6b686b48a4fa5db3c
    recorded_at: 2026-10-22T23:23:56.310022267Z
    follow_up: atelier-j458
    reason: Atelier writes ASCII record names; tracked in atelier-j458.
    migrated_from: atelier-7a9h
---

low: crates/atelier-app/src/path_log.rs:199-200 RecordGraph::read_diffs bails with 'record path is not UTF-8' when any commit in HEAD's history has a non-UTF-8 file name under .atelier/issues/ or .atelier/reviews/. The graph is loaded for every record history query, so one such stray file anywhere in history makes land and doctor error outright, where the replaced git log walks (which only printed commit IDs) kept working and the path could never match an issue or review ID. Suggested fix: key paths as bytes (Vec<u8> or OsString), or skip non-UTF-8 paths since no pathspec built from a record ID can match them. Unlikely in practice, since atelier writes ASCII record names.
