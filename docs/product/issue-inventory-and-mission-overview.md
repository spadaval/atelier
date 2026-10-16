# Issue Inventory and Mission Views

`atelier issue list` is flat inventory. It includes missions and work items, with filters for exact status, category, type, label, and priority.
The default limit is 50; output reports omitted matches. Inventory uses neutral
ID ordering and does not pretend every row is actionable work.

`atelier work missions` lists flat mission summaries with complete scope counts,
open blockers, and a command to inspect each mission; `--all` includes terminal
missions. Human output shows at most 20 missions and reports omissions. Quiet
output includes every matching mission ID in the same status, priority, and ID
order. `atelier work mission <id>` shows one mission's outcome, scope,
blockers, and operational work. `atelier work parent <id>` focuses on a parent
issue and its direct children. `issue show` remains the detailed record view.

Objective counts use the complete scope. Display limits trim visible rows,
without changing counts or the underlying scope facts.

Mission membership follows `advances`, not parent hierarchy. An `advances`
link joins one mission to one non-mission issue; the bundle and
`atelier issue link <issue-id> <mission-id> --role advances` record the same
two-sided link. A linked parent
includes its children in mission execution context. Work outside mission
scope remains visible through repository-wide work and inventory commands.

`work ready`, `blocked`, `active`, and `all` answer operational questions.
Ready means a direct active transition is available and dependencies permit it;
draft planning objectives do not become ready merely because their status has
a todo category. See [work ordering](work-view-ordering.md).

Views are read-only and branch-local. `status` can show other local branches in
separate labeled summaries. It never replaces current records with another
branch's newer-looking state. Use Git integration to reconcile branches.
