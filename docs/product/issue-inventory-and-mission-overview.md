# Issue Inventory and Mission Views

`atelier issue list` is flat inventory. It includes missions and work items, with filters for exact status, category, type, label, and priority.
The default limit is 50; output reports omitted matches. Inventory uses neutral
ID ordering and does not pretend every row is actionable work.

`atelier work missions` lists flat mission summaries with complete scope counts,
open blockers, and a command to inspect each mission. Human output shows at
most 20 missions and reports omissions; `--all` shows the whole overview: done
missions too, and every mission and unassigned item past the limits. Quiet
output includes every matching mission ID in the same status, priority, and ID
order. `atelier work mission <id>` shows one mission's outcome, scope,
blockers, and operational work. `atelier work parent <id>` focuses on a parent
issue and its direct children; `--all` lists them all in one list. `issue
show` is the record view: brief by default, with `--full` for the whole
record.

Objective counts use the complete scope. Display limits trim visible rows,
without changing counts or the underlying scope facts. A line that counts
omitted rows names the command that shows them, such as ``... 4 more done
work items omitted; see `atelier work mission atelier-kgcr --done` ``: a mission's
filter for that work state, or `--all` where the view has no such filter.

Below the missions, `work missions` lists unassigned work: open non-mission
issues outside the scope (`advances` links and their descendants) of every
open mission. A mission can close with open scope, so work a done mission left
open is unassigned and listed, with or without `--all`. Unassigned work is
grouped by label, one `<label> (<count>)` line per group. An issue with
several labels is listed under each, so a group holds all unassigned work
with that label; the unassigned total counts each issue once. Groups run
largest first, then by label, with unlabelled work last under `no label`.
Each group shows at most 5 items, one [item line](human-cli-output.md#item-line)
each, with a blocker count when blocked, in the
[work order](work-view-ordering.md), then ``... N more items omitted; see
`atelier work missions --all` ``; `--all` shows every item. A line too wide for
the terminal continues indented past the item lines. `status` shows the same groups (see
[sessions and hooks](sessions-and-hooks.md)).

When no mission is open, `work missions` and `status` name the most recently
closed mission (by close time) and its close date in place of an empty list,
so a session between missions can see what finished.

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
