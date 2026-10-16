---
schema: atelier.review
schema_version: 1
id: atelier-a523
title: 'crates/atelier-cli/src/commands/work.rs:317-360 (scoped_dashboard / mission_dashboard / epic_dashboard) and crates/atelier-app/src/issue_read.rs:120-127: the mission and epic dashboards still treat needs-human items as ordinary ready work. Reproduced: a mission whose scope is two needs-human todo items shows ''Ready Work'' listing both with no marker, its footer offers ''List mission ready work: atelier work mission <id> --ready'', and atelier -q work mission <id> --ready prints both IDs. work epic and work missions count them in ready as well. The ''Live mission orchestration dashboard'' is the path an Agent Factory orchestrator uses to dispatch mission work, so the stall this record exists to prevent still happens there. Fix: partition on row.needs_human in the scoped dashboard as list() does: a ''Needs a Human'' panel, leave those IDs out of --ready --quiet, and either keep them out of the ready count or show them as a separate human= count. Add one integration assertion for work mission --ready -q.'
target: atelier-q8ok
identity: Independent reviewer (Claude subagent)
commit: bd03289966af07bb8f13ec3c855590bf6e8504af
created_at: 2026-10-16T00:09:55.837132072Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/work.rs:317-360 (scoped_dashboard / mission_dashboard / epic_dashboard) and crates/atelier-app/src/issue_read.rs:120-127: the mission and epic dashboards still treat needs-human items as ordinary ready work. Reproduced: a mission whose scope is two needs-human todo items shows 'Ready Work' listing both with no marker, its footer offers 'List mission ready work: atelier work mission <id> --ready', and atelier -q work mission <id> --ready prints both IDs. work epic and work missions count them in ready as well. The 'Live mission orchestration dashboard' is the path an Agent Factory orchestrator uses to dispatch mission work, so the stall this record exists to prevent still happens there. Fix: partition on row.needs_human in the scoped dashboard as list() does: a 'Needs a Human' panel, leave those IDs out of --ready --quiet, and either keep them out of the ready count or show them as a separate human= count. Add one integration assertion for work mission --ready -q.
