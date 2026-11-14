---
schema: atelier.review
schema_version: 3
round: 2
commit: 1bdf226669dc9b517845899836c47e2e0409a5a6
reviewers:
- thread:delegated-task:command%3Amcp%3Ab13f1839-3e0a-4726-bee5-0d8939767c53%3Adelegate-task%3A816d4389-3ddf-46f0-b43c-8fd2d74d1112
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-14T02:18:23.833840344Z
---

## Summary

Complexity: high. The change adds fetching, forge state, persistent news and
stop publication, changes the API and app, and moves the shared mission
stop into atelier-app.

Value: high. These are the live view and veto the mission requires after
removing the go-ahead.

Risk of merging it: high. A stop can silently appear complete without a
confirmed pull-request close, and the proposed-work view can show stopped
work or omit an outstanding change.

The scope serves the requested outcome and is worth its complexity, but the
three problems below must be fixed before merging.

I reviewed the branch diff, focusing on the changes since round 1, the
shared drop and removed decide paths, the server and app, the service
scripts and the Desk documents. I checked each carried finding against its fix and regression.

Validation on the reviewed code:

- `cargo nextest run -p atelier-desk -p atelier-app`: 74 passed.
- `pnpm test`: 41 passed; typecheck and build passed; `pnpm e2e`: 48 passed
  across desktop and phone.
- `cargo fmt -- --check`, `git diff --check master...HEAD`, and
  `desk/service/notify-mail.test.sh` passed. The pull request's test and Desk
  CI jobs passed.
- The installed config names the five live repositories, fetches every 300
  seconds, and leaves mail disabled as the accepted product document says.
  All five live API responses read without errors and carry fetch marks.
  `desk-matches-cli` against the installed service, with the CLI built from
  this branch, passed: all 16 missions and five inboxes match.
- In an external scratch test crate, using this branch's crates, lockfile
  and bare-remote/gh fixture, I extended the scenarios below. A stop on an
  agreed mission's own branch correctly became stopped after its remote
  merge. Three other assertions failed, as described in Findings 2, 7 and 8.
  No implementation or committed test file was changed.

I did not stop a live plan, push to a live repository, close a real pull
request, or send mail. Authentication and publication under the installed
systemd service remain untested; publication tests used a bare remote and
stand-in commands.

## Finding 1: Recheck whether the plan has merged after fetching the stop clone
Recommendation: fix now
Was: 1-oo0v.1
Response: fixed in 90aa748
Now: fixed

The fresh clone repeats the agreed check before writing. The regression
refuses a plan merged since the served fetch and leaves its remote branch
unchanged. The agreed own-branch stop stays pending before merge, and my
additional remote-merge test confirms it becomes stopped after merge.

## Finding 2: Keep an unconfirmed pull-request close from becoming a completed stop
Recommendation: fix now
Was: 1-oo0v.2
Response: fixed in 90aa748
Now: kept

The first-read failure now works, but failure after a successful read can
still falsely complete a stop. At `crates/atelier-desk/src/fetch.rs:76`, an
error retains the previous PR map; `pull_request` at line 104 returns that
map's missing entry as `Known(None)` despite `forge_error`. Then
`read.rs:218` treats it as confirmation that the PR is not open.

I first cached a successful `gh pr list` returning `[]`, then made both
`gh pr close` and subsequent `gh pr list` fail while stopping the plan.
This models a pull request opened after the previous poll and an
unavailable forge when the stop runs. The act says its pull request was
not closed, but the detail page says `stopped`; the active list omits it
and the completed list includes it. The fetch error is now visible, but
does not prevent the false completed stop. The new committed test only
covers failure after a previously confirmed CLOSED entry, not after a
cached absence.

Keep a failed close unconfirmed until a current forge result establishes
closure or absence. A stale negative cache must not certify publication.
Add the successful-empty-list followed by failed-close/read regression,
checking the detail and both mission lists.

## Finding 3: Settle the stop state before filtering active and completed missions
Recommendation: fix now
Was: 1-oo0v.3
Response: fixed in 90aa748
Now: fixed

The reader returns all mission rows and the server settles their state
before partitioning active and completed. Both list endpoints are exercised
by the new `shown` helper in the stop-state regressions.

## Finding 4: List amendments on an agreed mission's own branch as proposed work
Recommendation: fix now
Was: 1-oo0v.4
Response: fixed in 90aa748
Now: fixed

An agreed mission's amendment is now a proposed plan with its branch and
round. Mission branches are no longer mistaken for started-item branches,
so an item added alongside the amendment appears as a record change. The
new amendment regression passes. Finding 8 covers the distinct remaining
case of a mission's own pending drop.

## Finding 5: Report newly merged plans even when no poll saw them proposed
Recommendation: fix now
Was: 1-oo0v.5
Response: fixed in 90aa748
Now: fixed

Merged news now depends on a newly agreed ID, without requiring a previous
observed proposal. The news test includes a plan first seen already merged
on a later observation and passes.

## Finding 6: Show fetch freshness on Inbox and Disputed
Recommendation: fix now
Was: 1-oo0v.6
Response: fixed in 90aa748
Now: fixed

Both API types now have fetch marks, joined after the cached read. Both
repository sections render FetchedNote, including their empty states. I
confirmed marks on all five repositories in the installed API responses;
the frontend tests pass.

## Finding 7: Keep a stopped plan's items out of Proposed
Recommendation: fix now

At `crates/atelier-desk/src/read.rs:785`, a dropped plan is skipped before
its branch is added to `plan_branches`. The item loop at lines 818-819
then processes that branch's items independently: `changes_of` still
considers each unmerged item added, so the stopped plan reappears under
record changes.

I added a draft item belonging to the proposed mission in the bare-remote
fixture, pushed it, and stopped the plan through `/api/act`. The drop was
pushed, the PR was CLOSED, and the mission page correctly said `stopped`.
However `/api/proposed` returned a record-change entry for
`origin/demo-pppp`, with the closed PR and `demo-i001` marked `added`.
The existing stop regression has no plan items, so its empty-plans
assertion misses this. `Records::read` suppresses dropped-plan entries for
other views, but this path iterates the unsuppressed located items.

Exclude the stopped plan's item proposals from record changes as well as
the plan list. Cover a stopped plan with actual items, checking both
`plans` and `changes`. This is the Done when requirement that a plan
dropped on its branch is not listed as proposed.

## Finding 8: Show an agreed mission's pending own-branch stop in Proposed
Recommendation: fix now

`crates/atelier-desk/src/read.rs:867-875` enumerates only
`Located::changes()`, which contains other branches and excludes the
record's own branch. The amendment path at lines 783-786 covers only
`MissionState::Open`, so an own-branch drop with `StopPending` is omitted
from both proposed plans and proposed record changes.

Using the same valid own-branch stop as the committed
`an_agreed_missions_drop_on_its_own_branch_is_pending_in_both_lists_until_it_merges`
test, I checked `/api/proposed` before merging. The mission page says
`stop_pending` and names `origin/demo-aaaa` in `stop_on`, but the proposed
record-change list is empty. The core supplies this branch and its dropped
record, so this is not a missing reader capability outside the scope.

Include the own branch's unmerged drop of an agreed mission as a proposed
record change, with its branch and pull request. Extend the existing
own-branch stop test to assert that entry before merge and its removal
after merge.
