---
schema: atelier.review
schema_version: 1
id: atelier-hwfm
title: 'medium: crates/atelier-cli/src/commands/session.rs:526-547 (push_checked_branch) with crates/atelier-cli/src/commands/git.rs:136-149 (check_base_commits): on the base branch, when branch.<base>.remote/.merge are set but <base>@{upstream} does not resolve (the remote branch was deleted and pruned, shown as [gone], or the remote is configured as a URL/path, as `git push -u <url> main` writes), land prints ''Base commits unchecked'', skips land_base_unreviewed, passes, and --push then publishes unreviewed code commits. Reproduced in a scratch repo with a local bare remote: main tracking origin/main [gone] plus an unreviewed commit changing a code file: ''land: passed, pushed to origin/main'', exit 0, remote main holds the commit; same with branch.main.remote set to the bare repo''s path. The same commit is refused (land_base_unreviewed) when the upstream resolves. Not a regression over `atelier land && git push` and the precondition is rare, so defer. Fix: under --push, refuse (e.g. land_push_upstream_unresolved) when <branch>@{upstream} does not resolve, or when the base check reported unchecked on the base branch.'
target: atelier-yim7
identity: Independent reviewer (Claude subagent)
commit: 06c2b327ae27786c9677118f6e4cecf8118fc638
created_at: 2026-10-22T21:17:50.557734127Z
event:
  event: finding
  recommendation: defer
---

medium: crates/atelier-cli/src/commands/session.rs:526-547 (push_checked_branch) with crates/atelier-cli/src/commands/git.rs:136-149 (check_base_commits): on the base branch, when branch.<base>.remote/.merge are set but <base>@{upstream} does not resolve (the remote branch was deleted and pruned, shown as [gone], or the remote is configured as a URL/path, as `git push -u <url> main` writes), land prints 'Base commits unchecked', skips land_base_unreviewed, passes, and --push then publishes unreviewed code commits. Reproduced in a scratch repo with a local bare remote: main tracking origin/main [gone] plus an unreviewed commit changing a code file: 'land: passed, pushed to origin/main', exit 0, remote main holds the commit; same with branch.main.remote set to the bare repo's path. The same commit is refused (land_base_unreviewed) when the upstream resolves. Not a regression over `atelier land && git push` and the precondition is rare, so defer. Fix: under --push, refuse (e.g. land_push_upstream_unresolved) when <branch>@{upstream} does not resolve, or when the base check reported unchecked on the base branch.
