---
schema: atelier.review
schema_version: 1
id: atelier-bj72
title: 'crates/atelier-app/src/record_index.rs:229: the restore command ''git show C^:.atelier/... > .atelier/...'' redirects to a path relative to the current directory. atelier commands work from a subdirectory (check ran from <repo>/sub and printed this command), but there the redirect fails with ''No such file or directory''. Fix: say ''from the repository root'', or give a command that works from any directory, such as ''git checkout C^ -- :/.atelier/issues/<id>.md''.'
target: atelier-r7cd
identity: Independent reviewer (Claude subagent)
commit: 85972cb43f9213363053eabf17e9a29dd7569afa
created_at: 2026-10-16T01:37:08.962948377Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/record_index.rs:229: the restore command 'git show C^:.atelier/... > .atelier/...' redirects to a path relative to the current directory. atelier commands work from a subdirectory (check ran from <repo>/sub and printed this command), but there the redirect fails with 'No such file or directory'. Fix: say 'from the repository root', or give a command that works from any directory, such as 'git checkout C^ -- :/.atelier/issues/<id>.md'.
