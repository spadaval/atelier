---
schema: atelier.review
schema_version: 1
id: atelier-5v68
title: 'crates/atelier-cli/src/commands/session.rs:294-298,319 compares strategy paths as PathBuf from the raw record string. IssueStrategy::new accepts a leading CurDir (''./plans/strategy.md''), and Path comparison keeps a leading ''.'' component, so it never equals the git diff path ''plans/strategy.md''; editing that strategy on the base is then falsely refused. The set also includes strategies of any issue type while docs (reviews-and-merge.md:100, sessions-and-hooks.md:33) say ''mission strategies''. Fix: normalize by dropping CurDir components before inserting, and either filter to missions or reword the docs. Low frequency: no strategy in this repo currently uses ''./''.'
target: atelier-3mjv
identity: Independent reviewer (Claude subagent)
commit: f9b76ba6c8397e90f916a6aff44f8f4915d2a032
created_at: 2026-10-15T23:18:21.092683156Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/session.rs:294-298,319 compares strategy paths as PathBuf from the raw record string. IssueStrategy::new accepts a leading CurDir ('./plans/strategy.md'), and Path comparison keeps a leading '.' component, so it never equals the git diff path 'plans/strategy.md'; editing that strategy on the base is then falsely refused. The set also includes strategies of any issue type while docs (reviews-and-merge.md:100, sessions-and-hooks.md:33) say 'mission strategies'. Fix: normalize by dropping CurDir components before inserting, and either filter to missions or reword the docs. Low frequency: no strategy in this repo currently uses './'.
