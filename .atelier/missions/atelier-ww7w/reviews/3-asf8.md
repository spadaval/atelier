---
schema: atelier.review
schema_version: 3
round: 3
commit: f89b57b9447f3d65e7ce3df06c763a99f02dad76
reviewers:
- thread:delegated-task:command%3Amcp%3A42ca7c4b-9f7f-469d-9389-3cf3e5d281ff%3Adelegate-task%3Aww7w-plan-review-r3-claude
model: claude-opus-5-5
verdict: changes
created_at: 2026-11-22T06:23:37.477139344Z
---

## Summary

Changes requested. Six of the nine carried findings are fixed and three are kept. The round 2 fixes add two fix-now problems: a race rule in `start` that deletes another implementer's unmerged work (Finding 11), and an amendment delta that hides the amendment's own item edits (Finding 10). The second follows from what round 2's Finding 7 asked for, which was too broad.

- **Complexity: high.** Six items change the contracts of `start`, `submit`, `merge`, `drop`, `status`, the log and the Desk's API.
- **Value: high.** The split rule replayed on this repository stops atelier-to83 before round 5 and atelier-osel's amendment after round 6, and the delta for a proposed plan is small (9 to 134 lines a round on atelier-to83).
- **Risk of merging it: high until Finding 11 is fixed**, since that loss cannot be reverted from the remote. The other problems are small edits to the plan.

What I checked:

- The round 2 delta (`6d1acd1be..f89b57b94`, seven plan files besides the reviews), each carried finding against the code at f89b57b, and the rest of the plan for serious misses.
- In scratch repositories with a bare origin and the CLI built from this commit: a take-over racing a drop followed by the specified deletes, a second garden run on the stable slug, a commit on a detached HEAD with and without a T3 checkpoint ref, and `git reset --keep` with uncommitted work.
- On this repository's history: the specified delta and split rules over the plan reviews of atelier-osel, atelier-kw8u, atelier-to83 and this mission.
- `evals/agent-factory/test.sh` passes, and still passes through a wrapper that refuses planning commands on a detached HEAD. `scripts/ci-jobs.test.sh` and `atelier check` pass.

What I could not check: the conversation that gave the Authority, GitHub's behaviour on deleting a branch or closing a pull request, and CI timings. The cleanup and race rules are not built, so I ran their Git commands by hand. I did not run clippy, `cargo nextest`, the Desk's app tests or `t3-extension/run-tests.mjs`. I did not read the other round 3 review.

## Finding 1: the plan-review delta is mostly the last round's reviews, not the plan
Recommendation: fix now
Was: 2-jo28.7
Now: fixed

The flood is gone. Replayed, atelier-kw8u's round 4 to `master` is 1 file and 21 lines, down from 61 files and 4,501. The rule this finding asked for drops too much on an amendment, which is Finding 10.

## Finding 2: removing the garden session leaves its four jobs with nobody
Recommendation: fix now
Was: 2-jo28.10
Now: kept

Both cases from round 2 are now written: an update keeps unanswered proposals, and a later run reads the last Report's Proposals. Two losses remain in the rules that were added.

- **Bugs of a run that stops after its merge.** `atelier-7637` now files bugs on `backlog/garden-proposals`, which is made after the run's merge, and its recovery reads only the Report's `Proposals` section. Today step 5 of `garden.md` files a bug at once and the Report's `Bugs filed:` line has its ID. Under the plan that line has no ID to hold, so a stop between the merge and the filing loses the bug. Have the Report list each bug still to file and have step 4 recover those too, or keep filing bugs at once.
- **An answer to only some proposals.** "Once the human answers" the item, the planner closes it. The item now gathers every unanswered proposal, up to three more a day, so partial answers will be usual. Closing then removes the unanswered ones from the Inbox, and recovery skips them because a closed item "already lists" them, against the item's own "only the human's answer removes a proposal". Say that the planner closes the item only when every proposal in it is answered, and otherwise removes just the answered ones.

## Finding 3: deleting the merged branch lets a stale clone start finished work again
Recommendation: fix now
Was: 2-jo28.15
Now: fixed

`start`, take-over and `submit` judge by the fetched base, the push window is rechecked, and `merge` deletes nothing until `base_tip` contains the head, each with a test. What the recheck does when it loses is Finding 11.

## Finding 4: the split count never resets, so atelier-kw8u's next amendment is refused before any review
Recommendation: fix now
Was: 2-jo28.16
Now: fixed

Only rounds absent from the base count, and the amendment test is named. An amendment of atelier-kw8u starts at 0, and atelier-osel's rounds 5 and 6 still reach 2.

## Finding 5: detaching after a merge lets the next record commit vanish
Recommendation: fix now
Was: 2-jo28.17
Now: kept

The refusal of planning commands works: the evals pass with it. The guard for a commit already made does not work under T3 Code, where these agents run.

`atelier-gqtw` defines an unattached commit as "`git for-each-ref --contains HEAD` is empty". Without a pattern that command lists every ref. T3 Code writes a checkpoint commit under `refs/t3/orchestration-v2/checkpoints/` whose parent is the checkout's HEAD. This clone has 3,206 of those refs, all with parents, and three contain this review's HEAD, f89b57b.

I ran the item's test sequence with one such ref:

1. Commit on the detached HEAD: the command is empty, as the plan expects.
2. Add a checkpoint whose parent is that commit: the command prints the checkpoint ref, so `status` would say nothing and `start` would not refuse.
3. `atelier start --backlog next`: the commit is then on no branch, remote-tracking branch or tag.

Say `git for-each-ref --contains HEAD refs/heads refs/remotes refs/tags` in the item and the design, and put a ref outside those three in the test. A test in a plain scratch repository passes with the broken command.

## Finding 6: deleting the branch removes done items' pull request links from the Desk and the extension
Recommendation: fix now
Was: 2-jo28.18
Now: fixed

The Desk's files are in Scope, with the `branch_of(id)` lookup and a test with neither ref. The tag this finding also mentioned is Finding 15.

## Finding 7: gqtw cannot pass the evals it must pass, because `evals/` is outside its Scope
Recommendation: fix now
Was: 2-jo28.19
Now: fixed

`evals/agent-factory/` is in Scope and Done when, and the design names both fixtures.

## Finding 8: An empty plan-path list prints `git diff` with no pathspec
Recommendation: fix now
Was: 2-nld2.15
Now: fixed

An empty list prints no command and has a test. The false claim about `plan_changed_at` is gone: the helper follows `plan_diff`, which does leave out `notes.md`, and the brief still lists that file among the documents. When the list is wrongly empty is Finding 10.

## Finding 9: Dated garden-proposal branches cannot update the one open item
Recommendation: fix now
Was: 2-nld2.16
Now: kept

The date is gone, but the "start (or reuse)" this finding asked for is only half there. `atelier-7637` covers a slug that exists unmerged and a slug that does not exist. For one that exists and is merged it says `atelier start --backlog garden-proposals`, which I ran: exit 1, "Branch backlog/garden-proposals exists on origin. Pick another slug, or work on that branch."

That state is normal under this plan:

- `atelier-7637` has no `blocked_by`, and the design runs it before or beside `atelier-gqtw`. A branch it merges before that is never deleted, and `AGENTS.md` still forbids deleting it by hand, so every later run is refused.
- `atelier-gqtw` deliberately keeps both refs when `catch_up` fails, and only warns when the remote refuses the delete.

Say that a merged branch is reused: switch to it, bring it up to the base (a fast-forward), then edit. The step then does not depend on deletion. Say also to work in the checkout that holds the branch if one does, since `git switch` refuses it elsewhere ("already used by worktree").

## Finding 10: an amendment's delta hides the item edits the amendment made, and calls the plan unchanged
Recommendation: fix now

For an agreed mission, `atelier-h6ij`, the design and the **converging-reviews** criterion now drop all `.atelier/items/*` paths. An amendment often is item edits. On atelier-osel's amendment:

| Rounds | As specified | Item edits by the amendment that it hides |
| --- | --- | --- |
| 4 to 5 | `mission.md` | 6 item files, 57 changed lines, with the new item atelier-diru |
| 5 to 6 | `mission.md` | 2 item files, 10 changed lines |
| 6 to 7 | nothing: "plan is unchanged" | atelier-diru's `item.md`, the whole fix of round 6's fix-now finding |

In round 7 both reviewers would be told that nothing changed while judging a finding fixed by that change. A new defect in that text would also be deferred as being in "unchanged plan sections". `plan_diff` leaves items out because their edits need no new approvals at merge, which is a different question from what a reviewer must read. Round 2's Finding 7 took the amendment's own edits for noise when it asked for that rule.

On an amendment, add the mission's item paths that the branch itself changes (`git diff <base>...<head>`), less their reviews, and keep those that differ between `prev` and `head`. Replayed, that gives exactly the three sets of item files above, and none for an atelier-kw8u amendment that only merged the base. Test both: an item the base merged stays out, and an item the amendment edited is in. Change the mission's criterion with the item and the design, since it now requires the omission.

## Finding 11: the take-over race rule deletes the previous implementer's unmerged work
Recommendation: fix now

`atelier-gqtw`'s Claim safety line and the design say that after "the claim or take-over push", if the item became done or dropped, `start` "lease-deletes the branch it just pushed, deletes the local branch, and refuses". A fresh claim's branch holds only the claim commit. A take-over's branch holds all the earlier implementer's work, and a drop on a backlog branch, which this item introduces, leaves that work unmerged.

I ran it: A starts the item, commits and pushes work, the drop reaches `origin/main`, B's take-over push succeeds on the base it had read, then the two specified deletes run.

- The lease passes, because the tip is B's own take-over commit.
- No ref on origin or in B contains A's work.
- `take_over` switches before it pushes (`crates/atelier-cli/src/commands.rs`), and `git::delete_branch` is `update-ref -d`, so B is left on an unborn branch, "No commits yet on demo-ab12", with every file staged.

For an item that became done, the same rule deletes a late push that the merge's lease had just saved. It also breaks this item's "Out: deleting unmerged branches", and it loses the new records that `drop` names as still on that branch.

Say that a losing take-over deletes neither ref: it refuses and names the branch. If a losing fresh claim still removes its one claim commit's branch, say so in the Authority, which today permits a remote deletion "specifically for `atelier merge`". Test a take-over that loses to a drop: the earlier work is still on origin and the checkout has a HEAD.

## Finding 12: the first decoupled run would take atelier-j86z for the proposals item
Recommendation: defer

`atelier-7637` finds the one item to update as an open item labelled `needs-human,garden`. atelier-j86z, the old digest of 10 recommendations that `garden.md` says is not a run, is open with both labels today. Identify the proposals item by more than those labels, such as its title.

## Finding 13: one pre-push test case cannot restore the base as its Done when says
Recommendation: defer

`atelier-zc9b` says the printed steps restore the base, tested with "uncommitted work in touched files". There `git reset --keep <old>` exits 128 ("Entry 'f' not uptodate") and does not move the base. The work is kept, and the design correctly says "or refuses cleanly". Word the Done when the same way, and have the printed steps say what to do after that refusal.

## Finding 14: the `plan` filter still dates two converted missions to the conversion
Recommendation: defer

`atelier-js4a` skips a first appearance with a lead or a Result, to avoid false dates. atelier-g6rp and atelier-qr5s first appear in 7f499285e as `status: ready`, which reads as open, with `lead: null`, so both would be logged as planned on 2026-11-11. Their `created_at` says 2026-11-08 and 2026-11-10. Comparing `created_at` with the commit, or naming the conversion, would catch them.

## Finding 15: done items still lose their "merged" tag
Recommendation: no action

`desk/src/components/ItemLine.tsx` draws the tag from `row.branch`, which the pull request fallback does not fill. The status glyph still says done.
