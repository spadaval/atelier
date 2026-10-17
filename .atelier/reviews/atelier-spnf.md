---
schema: atelier.review
schema_version: 1
id: atelier-spnf
title: 'crates/atelier-cli/src/main.rs:617 with crates/atelier-app/src/git_freshness.rs:75 — process-wide SIG_DFL also covers ObjectReader''s writes to the stdin of the long-lived git cat-file --batch child. If that child has already exited when atelier writes a request, the write used to return EPIPE, ObjectReader::read returned Err, ReadContext::object dropped the reader, and the caller degraded. Now SIGPIPE kills atelier with no message of its own. Reproduced with a git wrapper on PATH that makes cat-file exit 128 at once: in this repo, the fix-disabled build gives status and work all exit 0 with 72 lines of degraded output; HEAD gives exit 141 and empty stdout, 5 of 5 runs. A real git that dies before it reads stdin (killed, fatal at startup) is the trigger; I could not produce one with real git (a broken .git/config did not reach it). The kill fails closed, and no rollback window uses ReadContext (start/prune install paths use git_output and the filesystem). Hence defer, not fix_now. Possible follow-up: note the trade-off in restore_default_sigpipe''s comment, or check child.try_wait() before the write.'
target: atelier-5rs0
identity: Independent reviewer (Claude subagent)
commit: e3ac5198f39683addf8584c26576f99f55871ea4
created_at: 2026-10-17T06:30:54.402113080Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/main.rs:617 with crates/atelier-app/src/git_freshness.rs:75 — process-wide SIG_DFL also covers ObjectReader's writes to the stdin of the long-lived git cat-file --batch child. If that child has already exited when atelier writes a request, the write used to return EPIPE, ObjectReader::read returned Err, ReadContext::object dropped the reader, and the caller degraded. Now SIGPIPE kills atelier with no message of its own. Reproduced with a git wrapper on PATH that makes cat-file exit 128 at once: in this repo, the fix-disabled build gives status and work all exit 0 with 72 lines of degraded output; HEAD gives exit 141 and empty stdout, 5 of 5 runs. A real git that dies before it reads stdin (killed, fatal at startup) is the trigger; I could not produce one with real git (a broken .git/config did not reach it). The kill fails closed, and no rollback window uses ReadContext (start/prune install paths use git_output and the filesystem). Hence defer, not fix_now. Possible follow-up: note the trade-off in restore_default_sigpipe's comment, or check child.try_wait() before the write.
