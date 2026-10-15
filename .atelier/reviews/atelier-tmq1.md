---
schema: atelier.review
schema_version: 1
id: atelier-tmq1
title: 'crates/atelier-app/src/init.rs:118-124: remote-name parsing quirks, all harmless: an origin that is a local non-bare path ending in ''/.git'' (e.g. /home/me/projectx/.git) yields '''' and falls through to the main worktree name instead of ''projectx''; an uppercase ''.GIT'' suffix is kept (Repo.Name.GIT -> repo-name-git). Verified correct for https with and without .git, trailing slash, .git/, scp-style, ssh:// with port, absolute paths, file://, ''.'' and names derive_project_slug rejects (falls through). The slug is printed, so the user can see these and rerun with --slug.'
target: atelier-5qcs
identity: Independent reviewer (Claude subagent)
commit: 1abac3dcb9b83de0291f40d840a970264b557a31
created_at: 2026-10-15T23:17:24.191661585Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-app/src/init.rs:118-124: remote-name parsing quirks, all harmless: an origin that is a local non-bare path ending in '/.git' (e.g. /home/me/projectx/.git) yields '' and falls through to the main worktree name instead of 'projectx'; an uppercase '.GIT' suffix is kept (Repo.Name.GIT -> repo-name-git). Verified correct for https with and without .git, trailing slash, .git/, scp-style, ssh:// with port, absolute paths, file://, '.' and names derive_project_slug rejects (falls through). The slug is printed, so the user can see these and rerun with --slug.
