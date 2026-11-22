---
schema: atelier.review
schema_version: 3
round: 4
commit: 96227cc29de0a2d40e3dd14769bf58cc012595bc
reviewers:
- thread:delegated-task:command%3Amcp%3A42ca7c4b-9f7f-469d-9389-3cf3e5d281ff%3Adelegate-task%3Aww7w-plan-review-r4-claude
model: claude-opus-5-5
verdict: approve
created_at: 2026-11-22T06:42:13.874940247Z
---

## Summary

Approve. All six carried fix-now findings are fixed, and each fix works when run. I found no serious miss in the unchanged sections. Three smaller problems are below, none blocking.

- **Complexity: high.** Six items change the contracts of `start`, `submit`, `merge`, `drop`, `status`, the log and the Desk's API.
- **Value: high.** Record-only CI, plan-review rounds, drops and garden runs are paths in daily use, and each has counted waste behind it.
- **Risk of merging it: medium.** The one loss a revert could not undo, a take-over deleting another thread's work, is out of the plan. What remains shows when it fails and can be reverted.

What I checked:

- The round 3 delta (`f89b57b94..96227cc29`: mission, design and five items), each carried finding against the code at 96227cc, and the rest of the plan for serious misses.
- In scratch repositories with a bare origin, a fake `gh` and the CLI built from this commit: five merge cycles of one `backlog/garden-proposals` branch, the detached HEAD guard with and without T3 checkpoint refs, a take-over and a fresh claim that each lose to a drop, and `git reset --keep` in six states of uncommitted work.
- On this repository's history: the specified delta over 34 pairs of plan-review rounds, and the 24-hour rule of `atelier-js4a` over the first commit of all 17 missions.
- Round 3's deferred findings 12, 13 and 14 are addressed. The fixed title tells the proposals item from atelier-j86z, the recovery's refusal case is worded and behaves as written, and the 24-hour rule separates the converted missions (38 hours or more) from the planned ones (under an hour).
- `atelier check`, `scripts/ci-jobs.test.sh` and `evals/agent-factory/test.sh` pass with that CLI.

What I could not check: the conversation that gave the Authority, GitHub itself (branch deletion, pull requests, CI timings), and code that is not built yet. I ran the planned guard, race and cleanup rules as Git commands by hand. I did not run clippy, `cargo nextest`, the Desk's app tests or `t3-extension/run-tests.mjs`. I did not read the other round 4 review.

## Finding 1: removing the garden session leaves its four jobs with nobody
Recommendation: fix now
Was: 3-asf8.2
Now: fixed

Both losses are closed in `atelier-7637`, the design and the criterion. The Report gets `Bugs to file` and step 4 recovers bugs as well as proposals. The planner closes the item only when every proposal is answered, and otherwise removes only the answered ones. A draft `needs-human,garden` item does show under "Waiting on a human" once merged, and `atelier close` takes it, so the asynchronous hand-off works with today's commands.

## Finding 2: detaching after a merge lets the next record commit vanish
Recommendation: fix now
Was: 3-asf8.5
Now: fixed

The item, design and criterion now name `git for-each-ref --contains HEAD refs/heads refs/remotes refs/tags`, and the test has a `refs/t3/` ref. I ran it. A manual commit under a checkpoint ref lists nothing, where the old command listed the checkpoint. A HEAD just merged, a pushed commit and a tagged commit each list a ref, so there is no false alarm. It takes 0.012 seconds in this clone, with 3,212 T3 refs.

## Finding 3: Dated garden-proposal branches cannot update the one open item
Recommendation: fix now
Was: 3-asf8.9
Now: fixed

The branch is reused merged or not, and worked on in the checkout that holds it. I ran five cycles: no ref, merged with both refs, remote only, both refs deleted, and held by another worktree. Each submitted and merged under the one slug, and none depends on `atelier-gqtw`.

## Finding 4: an amendment's delta hides the item edits the amendment made, and calls the plan unchanged
Recommendation: fix now
Was: 3-asf8.10
Now: fixed

The amendment's own item edits are back in the delta, with a test for both sides and the criterion changed to match. Replayed on atelier-osel it gives the item files round 3 listed (6, 2 and 1), including atelier-diru in round 7. It still leaves out what the base merged, such as 3 of the 6 plan paths changed between rounds 2 and 3 of atelier-295u. An edit the amendment takes back is Finding 7.

## Finding 5: the take-over race rule deletes the previous implementer's unmerged work
Recommendation: fix now
Was: 3-asf8.11
Now: fixed

A losing take-over deletes neither ref and leaves a valid HEAD, with a test. A losing fresh claim deletes only its local branch, so the Authority's one permitted remote deletion stays `merge`'s. I ran both against a drop: the earlier work stays on origin, and after a fetch the base's `dropped` wins over the branch left behind, in `show`, `status` and `start`.

## Finding 6: The stable garden-proposals slug cannot be started after the first merge while the ref remains
Recommendation: fix now
Was: 3-g7hi.10
Now: fixed

The text this finding asked for is in `atelier-7637`, the design and the criterion: reuse the branch whether merged or not, start it only when neither ref exists, never a second slug. The same five cycles as Finding 3 pass.

## Finding 7: an amendment that takes back an item edit still reads as "plan unchanged"
Recommendation: defer

`atelier-h6ij` and the design keep an item path only if it is in `git diff <base>...<head>`. A path the amendment edited by the last round and has since restored, or a new item it has since withdrawn, is not in that diff, although it differs between `prev` and `head`.

I ran it: round 1 edits one item and adds another, round 2 merges the base and takes both back. `prev..head` lists both, the specified list is empty, and the brief would say that the plan is unchanged.

Take the paths of `<base>...<prev>` as well as `<base>...<head>` before intersecting with `prev..head`. That lists both and still leaves out what the base merged. It does not block: none of the 34 historical pairs differs under the two rules, and a path that is taken back adds nothing to what merges. `atelier-h6ij`'s implementer can make the change with one more test case.

## Finding 8: a partial answer does not say where the decision is written
Recommendation: defer

`atelier-7637` has the planner remove answered proposals "(recording their decisions)" without saying where. Step 4's recovery carries any proposal of the last Report "that no existing item already covers". A declined proposal has no item of its own, so if its decision is only in the commit message or the conversation, the next run brings it back to the human.

Say that the decision stays in `Garden proposals`, for example as one line under Decided, and that this counts as covered. The same step has the planner edit that item on "a backlog branch". Naming `backlog/garden-proposals` there too avoids the two-branch conflict round 2 raised (`docs/product.md`, "Two branches that change one record conflict").

## Finding 9: `git reset --keep` unstages what was staged
Recommendation: no action

`atelier-zc9b` says the recovery preserves uncommitted changes, and the files are preserved in all six states I ran. The index is not: a staged edit comes back unstaged and a staged new file comes back untracked. The printed steps could say so.
