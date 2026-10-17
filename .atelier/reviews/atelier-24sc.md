---
schema: atelier.review
schema_version: 1
id: atelier-24sc
title: 'crates/atelier-app/src/criteria.rs:252: code_commits_between spawns a ''git rev-list --count <pin>..HEAD -- . :(exclude).atelier'' for every (criterion, receipt) pair in history, uncached, though only the deciding receipt''s count is shown. In a scratch clone, 6 receipts naming 8 criteria pinned ~780 commits back took ''work mission atelier-r6bt'' from 1.0s to 3.0s, and it grows linearly as done missions accumulate old proof. Also line 319''s parse().unwrap_or(0) turns an unparsable count into ''fresh''. Fix: compute the count only for deciding() (or memoize by commit), and propagate the parse error.'
target: atelier-ho7o
identity: Independent reviewer (Claude subagent)
commit: 9d56d55fc73d6ac4a9dd5b30848f5f364f5eb0e2
created_at: 2026-10-17T18:21:22.617469064Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/criteria.rs:252: code_commits_between spawns a 'git rev-list --count <pin>..HEAD -- . :(exclude).atelier' for every (criterion, receipt) pair in history, uncached, though only the deciding receipt's count is shown. In a scratch clone, 6 receipts naming 8 criteria pinned ~780 commits back took 'work mission atelier-r6bt' from 1.0s to 3.0s, and it grows linearly as done missions accumulate old proof. Also line 319's parse().unwrap_or(0) turns an unparsable count into 'fresh'. Fix: compute the count only for deciding() (or memoize by commit), and propagate the parse error.
