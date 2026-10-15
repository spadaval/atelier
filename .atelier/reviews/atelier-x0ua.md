---
schema: atelier.review
schema_version: 1
id: atelier-x0ua
title: 'crates/atelier-cli/src/commands/session.rs:215: a branch with no common ancestor with the base (an orphan branch, or a repo whose base history was rewritten) makes git merge-base exit 1, and land fails with the bare message ''git merge-base <sha> HEAD failed:'', which has no land_* code and no guidance. Reproduced with git checkout --orphan from main and a commit. Suggested fix: map the no-merge-base case to a land_work_untraceable error (or a note) that says the branch shares no history with <base>.'
target: atelier-fxbg
identity: Independent reviewer (Claude subagent)
commit: e4e1b6e78c9291ecbd3923b465f2cfd7f2e4a7a9
created_at: 2026-10-15T22:59:09.143676187Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/session.rs:215: a branch with no common ancestor with the base (an orphan branch, or a repo whose base history was rewritten) makes git merge-base exit 1, and land fails with the bare message 'git merge-base <sha> HEAD failed:', which has no land_* code and no guidance. Reproduced with git checkout --orphan from main and a commit. Suggested fix: map the no-merge-base case to a land_work_untraceable error (or a note) that says the branch shares no history with <base>.
