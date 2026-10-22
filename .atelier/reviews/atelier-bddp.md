---
schema: atelier.review
schema_version: 1
id: atelier-bddp
title: 'low: crates/atelier-cli/src/commands/review_request.rs:215-230 The re-review range is <last review commit>..HEAD, so after the branch merges its base (this repo merges master rather than rebasing) the ''Review'' range, its commit count and the suggested ''git diff'' include every base commit merged in, sending a re-reviewer through already-reviewed master code. Also line 489: ''commits only the record it writes'' is inaccurate when an earlier --stage-records group is pending. Fix: exclude base commits (e.g. ''git log <last>..HEAD ^<base>'', and diff against a merge of last review and base), and say ''its records and any staged with --stage-records''. Fits atelier-rc8j''s output rework.'
target: atelier-zuow
identity: Independent reviewer (Claude subagent)
commit: eb6ee5cf688f70c5521050bad0eac7d1d58da725
created_at: 2026-10-22T21:45:00.770084439Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/review_request.rs:215-230 The re-review range is <last review commit>..HEAD, so after the branch merges its base (this repo merges master rather than rebasing) the 'Review' range, its commit count and the suggested 'git diff' include every base commit merged in, sending a re-reviewer through already-reviewed master code. Also line 489: 'commits only the record it writes' is inaccurate when an earlier --stage-records group is pending. Fix: exclude base commits (e.g. 'git log <last>..HEAD ^<base>', and diff against a merge of last review and base), and say 'its records and any staged with --stage-records'. Fits atelier-rc8j's output rework.
