---
schema: atelier.review
schema_version: 2
id: atelier-tmq1
title: 'crates/atelier-app/src/init.rs:118-124: remote-name parsing quirks, all harmless: an origin that is a local non-bare path ending in ''/.git'' (e.g. /home/me/projectx/.git) yields '''' and falls through to the main worktree name instead of ''projectx''; an uppercase ''.GIT'' suffix is kept (Repo.Name.GIT -> repo-name-git). Verified correct for https with and without .git, trailing slash, .git/, scp-style, ssh:// with port, absolute paths, file://, ''.'' and names derive_project_slug rejects (falls through). The slug is printed, so the user can see these and rerun with --slug.'
target: atelier-5qcs
identity: Independent reviewer (Claude subagent)
commit: 1abac3dcb9b83de0291f40d840a970264b557a31
created_at: 2026-10-15T23:17:24.191661585Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: f9b5833ef34e339eb965a7a76dc39d5e64d7dbf6
    recorded_at: 2026-10-15T23:22:14.789450622Z
    resolution_commit: f9b5833ef34e339eb965a7a76dc39d5e64d7dbf6
    reason: Fixed in f9b5833ef34e339eb965a7a76dc39d5e64d7dbf6 with tests
    migrated_from: atelier-yw0z
---

crates/atelier-app/src/init.rs:118-124: remote-name parsing quirks, all harmless: an origin that is a local non-bare path ending in '/.git' (e.g. /home/me/projectx/.git) yields '' and falls through to the main worktree name instead of 'projectx'; an uppercase '.GIT' suffix is kept (Repo.Name.GIT -> repo-name-git). Verified correct for https with and without .git, trailing slash, .git/, scp-style, ssh:// with port, absolute paths, file://, '.' and names derive_project_slug rejects (falls through). The slug is printed, so the user can see these and rerun with --slug.
