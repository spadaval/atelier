---
schema: atelier.review
schema_version: 2
id: atelier-bddp
title: 'low: crates/atelier-cli/src/commands/review_request.rs:215-230 The re-review range is <last review commit>..HEAD, so after the branch merges its base (this repo merges master rather than rebasing) the ''Review'' range, its commit count and the suggested ''git diff'' include every base commit merged in, sending a re-reviewer through already-reviewed master code. Also line 489: ''commits only the record it writes'' is inaccurate when an earlier --stage-records group is pending. Fix: exclude base commits (e.g. ''git log <last>..HEAD ^<base>'', and diff against a merge of last review and base), and say ''its records and any staged with --stage-records''. Fits atelier-rc8j''s output rework.'
target: atelier-zuow
identity: Independent reviewer (Claude subagent)
commit: eb6ee5cf688f70c5521050bad0eac7d1d58da725
created_at: 2026-10-22T21:45:00.770084439Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 84f8d13a93b7fba587090b51451adb8a26cf2b8c
    recorded_at: 2026-10-22T21:48:06.535237618Z
    resolution_commit: aecb73084c8af59cdf31bfa9229dd7d55fd12db9
    reason: Fixed in aecb7308
    migrated_from: atelier-5w7y
---

low: crates/atelier-cli/src/commands/review_request.rs:215-230 The re-review range is <last review commit>..HEAD, so after the branch merges its base (this repo merges master rather than rebasing) the 'Review' range, its commit count and the suggested 'git diff' include every base commit merged in, sending a re-reviewer through already-reviewed master code. Also line 489: 'commits only the record it writes' is inaccurate when an earlier --stage-records group is pending. Fix: exclude base commits (e.g. 'git log <last>..HEAD ^<base>', and diff against a merge of last review and base), and say 'its records and any staged with --stage-records'. Fits atelier-rc8j's output rework.
