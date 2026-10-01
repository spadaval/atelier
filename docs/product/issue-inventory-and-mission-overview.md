# Issue Inventory and Mission Views

`atelier issue list` is flat inventory. It includes missions, epics, and ordinary
issues, with filters for exact status, category, type, label, and priority.
The default limit is 50; output reports omitted matches. Inventory uses neutral
ID ordering and does not pretend every row is actionable work.

`atelier work missions` summarizes current missions; `--all` includes terminal
missions. `atelier work mission <id>` shows one mission's outcome, scope,
blockers, and operational work. `atelier work epic <id>` focuses on an epic and
its direct children. `issue show` remains the detailed record view.

Mission membership follows `advances`, not parent hierarchy. A linked epic
includes its direct children in mission execution context. Work outside mission
scope remains visible through repository-wide work and inventory commands.

`work ready`, `blocked`, `active`, and `all` answer operational questions.
Ready means a direct active transition is available and dependencies permit it;
draft planning objectives do not become ready merely because their status has
a todo category. See [work ordering](work-view-ordering.md).

Views are read-only and branch-local. `status` can show other local branches in
separate labeled summaries. It never replaces current records with another
branch's newer-looking state. Use Git integration to reconcile branches.
