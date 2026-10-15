---
schema: atelier.review
schema_version: 1
id: atelier-ccku
title: 'Re-reviewed f9b5833e and 10f4659d. atelier-lphr resolved: Git sources are used only when the canonicalised --show-toplevel equals the init path; verified a non-repo dir nested in a repo with an origin gets ''newproj'', a repo subdir gets its own name, a symlinked path to a repo still resolves, and the top level still uses origin. atelier-86y8 resolved: a worktree of proj.git gives ''proj''; also checked a bare repo without a .git suffix. atelier-tmq1 resolved: repository_name handles /.git paths, uppercase .GIT, https/scp/ssh-with-port/file:// forms, trailing slashes and empty names (unit test). Linked non-bare worktree, submodule (origin), plain dir and existing-config reruns unchanged. cargo build is clean; the new unit test and the extended integration test (nested dir, bare-linked worktree) pass; the fresh workspace evidence atelier-xk4f is in HEAD. One new defer finding: separate-git-dir and ''.bare'' layouts without an origin take the git dir''s name.'
target: atelier-5qcs
identity: Independent reviewer (Claude subagent)
commit: 10f4659d4e9773c1fd4fe4bbd2f9e489add52762
created_at: 2026-10-15T23:23:13.773027683Z
event:
  event: approval
---

Re-reviewed f9b5833e and 10f4659d. atelier-lphr resolved: Git sources are used only when the canonicalised --show-toplevel equals the init path; verified a non-repo dir nested in a repo with an origin gets 'newproj', a repo subdir gets its own name, a symlinked path to a repo still resolves, and the top level still uses origin. atelier-86y8 resolved: a worktree of proj.git gives 'proj'; also checked a bare repo without a .git suffix. atelier-tmq1 resolved: repository_name handles /.git paths, uppercase .GIT, https/scp/ssh-with-port/file:// forms, trailing slashes and empty names (unit test). Linked non-bare worktree, submodule (origin), plain dir and existing-config reruns unchanged. cargo build is clean; the new unit test and the extended integration test (nested dir, bare-linked worktree) pass; the fresh workspace evidence atelier-xk4f is in HEAD. One new defer finding: separate-git-dir and '.bare' layouts without an origin take the git dir's name.
