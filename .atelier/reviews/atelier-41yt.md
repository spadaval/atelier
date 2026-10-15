---
schema: atelier.review
schema_version: 1
id: atelier-41yt
title: 'crates/atelier-app/src/init.rs:131-140 (main_worktree, else branch): any common dir not named ''.git'' is treated as a bare repository and named via repository_name, but that also covers non-bare repos whose git dir lives elsewhere. Reproduced in /tmp with no origin: ''git init --separate-git-dir /tmp/x/sep.gitdir /tmp/x/sepwork'' then init in sepwork gives ''sep-gitdir'' (the iteration before this fix gave ''sepwork''); the common bare-worktree layout ''hub/.bare'' + worktree ''hub/main'' gives ''bare''. In both cases the source is printed as ''the main worktree''s directory'', which a bare repo does not have. Only applies when origin is absent (git clone --bare sets origin, so most real layouts are unaffected), and the slug is printed, so defer. Suggested fix: use the common dir''s name only when it is bare (''git --git-dir=<common> rev-parse --is-bare-repository'' is true), else fall back to the directory name; for a hidden name such as ''.bare'', use its parent directory''s name; label the bare source as ''the bare repository''s name''.'
target: atelier-5qcs
identity: Independent reviewer (Claude subagent)
commit: 10f4659d4e9773c1fd4fe4bbd2f9e489add52762
created_at: 2026-10-15T23:23:13.714058730Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/init.rs:131-140 (main_worktree, else branch): any common dir not named '.git' is treated as a bare repository and named via repository_name, but that also covers non-bare repos whose git dir lives elsewhere. Reproduced in /tmp with no origin: 'git init --separate-git-dir /tmp/x/sep.gitdir /tmp/x/sepwork' then init in sepwork gives 'sep-gitdir' (the iteration before this fix gave 'sepwork'); the common bare-worktree layout 'hub/.bare' + worktree 'hub/main' gives 'bare'. In both cases the source is printed as 'the main worktree's directory', which a bare repo does not have. Only applies when origin is absent (git clone --bare sets origin, so most real layouts are unaffected), and the slug is printed, so defer. Suggested fix: use the common dir's name only when it is bare ('git --git-dir=<common> rev-parse --is-bare-repository' is true), else fall back to the directory name; for a hidden name such as '.bare', use its parent directory's name; label the bare source as 'the bare repository's name'.
