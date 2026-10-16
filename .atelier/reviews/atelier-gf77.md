---
schema: atelier.review
schema_version: 1
id: atelier-gf77
title: 'crates/atelier-app/src/epic_migration.rs:46-56 (used by review_gate.rs:41-54 issue_status_only): migrate_epic_text is lossy - it keeps Outcome and silently drops a Goal section when both exist, drops a ''Done when'' section when Proof or Delivers exists, and keeps only the first of duplicate headings. Since the review gate now treats an exact migration as a non-authoring edit, an approved legacy epic whose migration drops acceptance text (e.g. Proof ''X'' plus Done when ''Y and security review'') passes as status-only with Y removed. Reach is limited to records that were approved in the retired epic shape (a new-shape record cannot be turned into an epic without the front-matter check catching the type change). Suggested fix: make migrate_epic_text bail (migrate by hand) when it would drop a non-empty section or a duplicate heading, so the shared rewrite is lossless.'
target: atelier-0349
identity: Independent reviewer (Claude subagent)
commit: 6d0300f6202777ceeb5a1b28f93efc8d5541dd00
created_at: 2026-10-16T02:50:22.086231323Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/epic_migration.rs:46-56 (used by review_gate.rs:41-54 issue_status_only): migrate_epic_text is lossy - it keeps Outcome and silently drops a Goal section when both exist, drops a 'Done when' section when Proof or Delivers exists, and keeps only the first of duplicate headings. Since the review gate now treats an exact migration as a non-authoring edit, an approved legacy epic whose migration drops acceptance text (e.g. Proof 'X' plus Done when 'Y and security review') passes as status-only with Y removed. Reach is limited to records that were approved in the retired epic shape (a new-shape record cannot be turned into an epic without the front-matter check catching the type change). Suggested fix: make migrate_epic_text bail (migrate by hand) when it would drop a non-empty section or a duplicate heading, so the shared rewrite is lossless.
