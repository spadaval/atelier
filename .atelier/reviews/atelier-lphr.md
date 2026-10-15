---
schema: atelier.review
schema_version: 1
id: atelier-lphr
title: 'crates/atelier-app/src/init.rs:117-131 (default_project_slug): git is consulted from any directory inside a work tree, so ''atelier init'' in a directory that is not itself a repository root inherits an enclosing repository''s identity. Reproduced: /tmp/outer is a git repo with origin git@h:o/dotfiles.git; ''atelier init'' in /tmp/outer/newproj (not yet git-initialised) prints ''Project slug: dotfiles (from the origin remote...)'' and writes project_slug="dotfiles"; a plain subdir of a repo without origin takes the enclosing repo''s directory name. Before this change both got the directory name. Atelier places .atelier at the cwd, so this applies to monorepo subprojects and to a new project created under a home directory or parent tracked by git, a common order (init atelier, then git init). Since IDs are hard to change once minted, a wrong silent default is the failure this record exists to prevent. Suggested fix: use the remote and main-worktree sources only when ''git rev-parse --show-toplevel'' (canonicalised) equals the init path; otherwise fall back to the directory name. Add a test for init in a non-repo directory nested inside a repo that has an origin.'
target: atelier-5qcs
identity: Independent reviewer (Claude subagent)
commit: 1abac3dcb9b83de0291f40d840a970264b557a31
created_at: 2026-10-15T23:17:24.077695462Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/init.rs:117-131 (default_project_slug): git is consulted from any directory inside a work tree, so 'atelier init' in a directory that is not itself a repository root inherits an enclosing repository's identity. Reproduced: /tmp/outer is a git repo with origin git@h:o/dotfiles.git; 'atelier init' in /tmp/outer/newproj (not yet git-initialised) prints 'Project slug: dotfiles (from the origin remote...)' and writes project_slug="dotfiles"; a plain subdir of a repo without origin takes the enclosing repo's directory name. Before this change both got the directory name. Atelier places .atelier at the cwd, so this applies to monorepo subprojects and to a new project created under a home directory or parent tracked by git, a common order (init atelier, then git init). Since IDs are hard to change once minted, a wrong silent default is the failure this record exists to prevent. Suggested fix: use the remote and main-worktree sources only when 'git rev-parse --show-toplevel' (canonicalised) equals the init path; otherwise fall back to the directory name. Add a test for init in a non-repo directory nested inside a repo that has an origin.
