---
schema: atelier.review
schema_version: 3
round: 2
commit: 6d1acd1be4acbc25b122df56286a329ebb504031
reviewers:
- 01a201d3-3756-71b3-8fb1-968133daffca
model: grok-4.6
verdict: changes
created_at: 2026-11-22T05:56:07.540235181Z
---

## Summary

Checked the plan as 6d1acd1 holds it: `mission.md`, `design.md`, the six ready items, and the eight dropped items they supersede. Read the round-1 delta (`6a2c26e15..6d1acd1be`, seven plan files, no review files). Checked the code those items name: `plan_changed_at` / `in_plan` ([plan.rs:113](crates/atelier-core/src/plan.rs:113)), `Walker::commit` / `merge` ([log.rs:518](crates/atelier-core/src/log.rs:518)), `finish` after merge ([merge.rs:276](crates/atelier-cli/src/merge.rs:276)), `drop` of a started item ([plan.rs:259](crates/atelier-cli/src/plan.rs:259)), `start_backlog` ([commands.rs:357](crates/atelier-cli/src/commands.rs:357)), `plan_submitted` / `plan_unapproved` ([output.rs:900](crates/atelier-cli/src/output.rs:900)), `garden.md` steps 4 and 7, `lead.md:36-40`, `AGENTS.md:71`, `ci.yml` `changes` / `test`, and the product rule that two branches changing one record conflict ([product.md:464](docs/product.md:464)).

Complexity: high. Six items, three of them still sharing `output.rs` / `say.rs`, plus lease-checked ref deletion and a Desk route. Value: high. Record-only CI, split plan reviews, started-item drop, `submit` / `status`, garden merge, and M5 are paths in current use. Risk of merging it: medium. The lease, the detach-then-delete local path, and keeping `/api/history` on subjects close the silent-loss cases from round 1. Two holes remain: an empty plan-path list would print an unconstrained `git diff`, and dated garden-proposal branches would collide when a daily run updates the same item.

Could not check: the 2026-11-22 conversation itself; a real forge merge; `jy8k`'s scheduler; the other round-2 reviewer's file. Did not rebuild or rerun clippy; round 1 already reported it clean on this tree.

The fourteen carried findings are fixed by the amendment. Two new fix-now findings.

## Finding 1: `log::history` cannot supply the events the Desk's contract needs, and js4a may not change it
Recommendation: fix now
Was: 1-48x6.1
Now: fixed
`atelier-js4a` now adds `Happened::Plan` and `Happened::PlanMerge` in `log.rs`, puts that file in Scope, bounds the walk at 15s, and keeps `/api/history` and `/api/launch` on the subject parser so pre-cutover history and `t3-extension/` stay intact.

## Finding 2: desk-core-history requires tests and fixtures to pass that only a subject parser can satisfy
Recommendation: fix now
Was: 1-48x6.2
Now: fixed
The item no longer replaces the subject parser. The empty-commit fixture and `history_is_read_from_commit_subjects` keep working; Done when now runs `node t3-extension/run-tests.mjs` and `pnpm e2e`.

## Finding 3: `merge` can never delete the local branch, because it always runs on that branch
Recommendation: fix now
Was: 1-48x6.3
Now: fixed
`atelier-gqtw` now detaches `HEAD` at `<head>` and deletes with `git::delete_branch(&root, &branch, &head)` when the checkout is still on the branch and clean; tests require the local branch to be gone.

## Finding 4: deleting the remote branch loses a late push, and AGENTS.md forbids it
Recommendation: fix now
Was: 1-48x6.4
Now: fixed
Remote delete uses `--force-with-lease=refs/heads/<branch>:<head> --delete`, with a race test. Authority records Sumeet's 2026-11-22 approval; `AGENTS.md` and `docs/design.md` ("The branch is kept") are in Scope.

## Finding 5: the skill would still tell a lead to drop a started item on two branches
Recommendation: fix now
Was: 1-48x6.5
Now: fixed
`skills/agent-factory/roles/lead.md` and the two-branch decision row are in `atelier-gqtw`'s Scope and Done when.

## Finding 6: the split exit misses `submit`, which is what starts the next round, and misses amendments
Recommendation: fix now
Was: 1-48x6.6
Now: fixed
After two trailing split rounds, `submit`, `status`, and `plan_unapproved` share the human-ruling exit on both mission and amending `backlog/*` branches; `plan_submitted` no longer unconditionally tells the planner to submit again. The code cap replaces `Then: atelier submit`.

## Finding 7: the plan-review delta is mostly the last round's reviews, not the plan
Recommendation: fix now
Was: 1-48x6.7
Now: fixed
Round 2+ diffs `plan_changed_at` paths (reviews and unrelated items excluded). A test requires no `reviews/*` path in the printed command. See Finding 15 for the empty-list hole that test misses.

## Finding 8: the pre-push criterion can pass with the commit orphaned
Recommendation: fix now
Was: 1-48x6.8
Now: fixed
`atelier-zc9b` requires the printed steps, run verbatim on a checked-out base, to move the commit onto a new item branch and restore `<base>` to `<old>` without discarding uncommitted work; a hook scenario runs those steps.

## Finding 9: zc9b changes documented behaviour with no document in its Scope
Recommendation: fix now
Was: 1-48x6.9
Now: fixed
`docs/design.md` and `docs/product.md` are in `atelier-zc9b`'s Scope and Done when.

## Finding 10: removing the garden session leaves its four jobs with nobody
Recommendation: fix now
Was: 1-48x6.10
Now: fixed
A planner applies answers and closes the proposals item; later runs read carried proposals from `needs-human,garden` items; bugs are `bug,needs-human`; `SKILL.md` is in Scope. See Finding 16 for the daily-run collision the update rule still has.

## Finding 11: the "safe cleanup command" in status is unspecified and would hit live worktrees
Recommendation: fix now
Was: 1-48x6.11
Now: fixed
`status` prints a read-only count and `git branch --merged <base>` / `git worktree list` only; it never suggests a deletion. It counts all merged local branches, `t3/*` included.

## Finding 12: After two split plan rounds, `submit` still opens the next round
Recommendation: fix now
Was: 1-i2iw.1
Now: fixed
`submit` refuses after two trailing splits, with `ATELIER_SKIP` for the human to authorize another round; `plan_submitted` and `plan_unapproved` match that exit.

## Finding 13: js4a does not name the EventKind mapping `log::history` needs
Recommendation: fix now
Was: 1-i2iw.2
Now: fixed
`/api/history` keeps the subject parser, so no `Happened` → `EventKind` table is required. M5 returns `log::history` actions, not `HistoryEvent`.

## Finding 14: Garden decoupling still files bugs and proposals through a session
Recommendation: fix now
Was: 1-i2iw.3
Now: fixed
Bugs and the proposals item are filed on a `backlog/*` branch after merge, with today's action table (`atelier new` on a backlog branch). The dated slug that does that filing is Finding 16.

## Finding 15: An empty plan-path list prints `git diff` with no pathspec
Recommendation: fix now
`atelier-h6ij` prints `git diff <prev> <head> -- <paths>` where `<paths>` is `plan_changed_at` ([plan.rs:113](crates/atelier-core/src/plan.rs:113)). When that list is empty, Git treats `git diff <prev> <head> --` as a full-tree diff: every review file between the rounds, and any other path in the range.

That is the flood Finding 7 closed. It happens on a re-submit whose only new commits are review files (the planner already in `planners`, mission already `open`, so `planned_here` writes nothing ([commands.rs:704](crates/atelier-cli/src/commands.rs:704))). The specified test, that no `reviews/*` path appears in the printed command, still passes: an empty pathspec names no review file on the command line, and then prints them all in the output.

Say: if `plan_changed_at` is empty, print that the plan is unchanged and judge carried findings; do not print `git diff` with an empty pathspec. Test the empty list, not only that the command string omits `reviews/`.

The same Done when also claims `plan_changed_at` excludes `notes.md`. `in_plan` keeps `.atelier/missions/<id>/notes.md` ([plan.rs:150](crates/atelier-core/src/plan.rs:150)). Drop that claim. Filtering `notes.md` out would hide a lead ruling written between rounds.

## Finding 16: Dated garden-proposal branches cannot update the one open item
Recommendation: fix now
`atelier-7637` does two things at once: each run `atelier start --backlog garden-proposals-<YYYY-MM-DD>`, and "updates if an open `needs-human,garden` proposals item already exists … so daily runs (`atelier-jy8k`) do not spam the Inbox."

Those disagree. `start --backlog` refuses when the slug already exists ([commands.rs:375](crates/atelier-cli/src/commands.rs:375)), so the date in the slug forces a new branch every day. Two branches that change one record conflict when the second merges ([product.md:464](docs/product.md:464)). A run that looks only on the base will not see yesterday's proposals item still in review on `backlog/garden-proposals-<yesterday>`, and will file a second item.

Name one stable backlog slug (for example `garden-proposals`). If that branch exists and is unmerged, update the item there. If the item is already on the base, start (or reuse) that same slug, edit it, submit, and merge. Do not put the date in the slug.
