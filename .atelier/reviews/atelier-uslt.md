---
schema: atelier.review
schema_version: 1
id: atelier-uslt
title: 'Docs are incomplete for the new flag. (1) The synopsis block at the top of docs/product/sessions-and-hooks.md and the ''land [id]'' line in docs/product/cli-surface.md don''t list ''atelier land --install''. (2) sessions-and-hooks.md doesn''t say what the issue''s Design states: the Stop hook runs plain land, so an install session is blocked at Stop with land_install_unconfirmed (it becomes a warning on the repeat) until the install merges. (3) The code comment in tracker_changes_since (session.rs ~L260), ''this branch installs Atelier'', now overstates things; None only means the base has no policy.'
target: atelier-bibd
identity: Independent reviewer (Claude subagent)
commit: 5ee51dd7f06deac69692a6ac9af9b8d4dc8a80d5
created_at: 2026-10-16T23:26:18.593325262Z
event:
  event: finding
  recommendation: fix_now
---

Docs are incomplete for the new flag. (1) The synopsis block at the top of docs/product/sessions-and-hooks.md and the 'land [id]' line in docs/product/cli-surface.md don't list 'atelier land --install'. (2) sessions-and-hooks.md doesn't say what the issue's Design states: the Stop hook runs plain land, so an install session is blocked at Stop with land_install_unconfirmed (it becomes a warning on the repeat) until the install merges. (3) The code comment in tracker_changes_since (session.rs ~L260), 'this branch installs Atelier', now overstates things; None only means the base has no policy.
