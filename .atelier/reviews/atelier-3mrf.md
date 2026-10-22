---
schema: atelier.review
schema_version: 1
id: atelier-3mrf
title: 'low: crates/atelier-cli/src/commands/session.rs:143-168 (foreign_base_commits_note): the owner-scoped definition is sound for what records and history show (authorship is shared, message ids are the only signal, and it only warns), but it applies only on a work branch, where land --push does not push the base. On the base branch itself, the one place land --push publishes base commits, there is no owner, so every unpushed commit is named, including the session''s own planning commits, and the warning cannot tell ''another owner'' apart there. Acceptable as a warning; consider scoping it by the mission or the commits'' named owners on the base in a follow-up.'
target: atelier-yim7
identity: Independent reviewer (Claude subagent)
commit: 6b8b1ccb31042d1a3a7b167903f6a43b4a9acdf7
created_at: 2026-10-22T21:06:42.584378475Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/session.rs:143-168 (foreign_base_commits_note): the owner-scoped definition is sound for what records and history show (authorship is shared, message ids are the only signal, and it only warns), but it applies only on a work branch, where land --push does not push the base. On the base branch itself, the one place land --push publishes base commits, there is no owner, so every unpushed commit is named, including the session's own planning commits, and the warning cannot tell 'another owner' apart there. Acceptable as a warning; consider scoping it by the mission or the commits' named owners on the base in a follow-up.
