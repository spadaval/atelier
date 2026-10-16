---
schema: atelier.review
schema_version: 1
id: atelier-71io
title: 'Reviewed e05ab103. stale_pass is correct: only passes that fail proof_matches are tracked, the max is taken whatever the iteration order, and it is reported only when strictly newer than the newest failure, which itself must be newer than every matching pass. The public newest_failure keeps its return type. The dirty-path list is right: read_record_files removes canonical .atelier paths, so initial_paths holds exactly the non-record paths that make ReadContext::with_working_paths mark the checkout unclean. No duplication: the ''Unfinished checkout'' line goes into lines, which are printed only on success, while the note is part of the bail. The test drives the real CLI (fail then pass committed on a done child, land succeeds, then a dirty code file), checks both the stale-pass reason and the named path, and passes locally under nextest. One defer finding is filed: wording and substring coupling for historical or later-commit staleness.'
target: atelier-749l
identity: Independent reviewer (Claude subagent)
commit: 0860b910f2663ccc7aa445ac13ce8abf6a722165
created_at: 2026-10-16T19:11:57.049124691Z
event:
  event: approval
---

Reviewed e05ab103. stale_pass is correct: only passes that fail proof_matches are tracked, the max is taken whatever the iteration order, and it is reported only when strictly newer than the newest failure, which itself must be newer than every matching pass. The public newest_failure keeps its return type. The dirty-path list is right: read_record_files removes canonical .atelier paths, so initial_paths holds exactly the non-record paths that make ReadContext::with_working_paths mark the checkout unclean. No duplication: the 'Unfinished checkout' line goes into lines, which are printed only on success, while the note is part of the bail. The test drives the real CLI (fail then pass committed on a done child, land succeeds, then a dirty code file), checks both the stale-pass reason and the named path, and passes locally under nextest. One defer finding is filed: wording and substring coupling for historical or later-commit staleness.
