---
schema: atelier.review
schema_version: 3
round: 2
commit: dcd2af8f5ce2303f1728c27be3eae2f36f5cb6a0
reviewers:
- 01a1f52e-ac07-7f33-b96d-255296baea09
model: grok-4.6
verdict: approve
created_at: 2026-11-19T19:01:28.930068186Z
---

## Summary

Checked the plan at dcd2af8f against round 1's four carried findings and the
diff from 804cde4c (1d8c933d1 and dcd2af8f5). Read the mission, all four
items, `MissionStatus` and `MissionStatus::status()` in
`crates/atelier-core/src/model/mission.rs`, `Repo::locate` in `repo.rs`,
`misplaced` in `plan.rs`, `stale` in `actions.rs`, `plans_changed` and
item-merge plan refusal in `crates/atelier-cli/src/merge.rs`, the shared
review brief in `output.rs`, Desk `ProjectHome.tsx` / `group.ts` /
`words.ts`, and the extension `home/model.ts` and `mission/model.ts`.
Counted draft missions in the five other repositories the binary serves:
nine (`life-on-rails` 1, `cosplay-studio` 6, `demiurge` 1, `worldcrafter` 1).

Did not run `list`/`check` with a new binary, the house eval, or Desk e2e.
Did not read another reviewer's round-2 file.

Complexity: high. Item status, old-record reading, the action table's
mission encoding, merge of extra item records, two clients, evals, and six
repositories.
Value: high. The gate is ceremony, filing on a second branch is why loose
ends are lost, and the human asked for all three legs.
Risk of merging it: medium. Failures in `start`, `list` and `check` show; a
wrong old-record mapping is now a named Done when. A skill line that tells
a plan reviewer to file on the checkout they hold can still ride a loose
item in with the plan (Finding 5).

The four carried findings are fixed. The outcome is worth the work. One
leftover in atelier-8phx's skill sentence should be named before that item
is written; it does not block the rest.

## Finding 1: Item draft/ready as open must not agree old mission drafts
Recommendation: fix now
Was: 1-lk71.1
Now: fixed

The Outcome now bounds the mapping to items (`mission.md` 71–78) and keeps
an old mission `draft` proposed, unable to be led or to have its items
started. `old-records-read` tests that (`mission.md` 100–103). atelier-v06d
names `MissionStatus::status()` and requires a test that a stored mission
`draft` still cannot be led and that `start` refuses its items
(`.atelier/items/atelier-v06d/item.md` 41–45). The pin still maps
`MissionStatus::Draft` to `Status::Draft` and `Open` to `Status::Ready`
(`crates/atelier-core/src/model/mission.rs` 64–70). The five other
repositories still hold nine draft missions, matching the Outcome.

## Finding 2: Preserve publication between filing an item and starting it
Recommendation: fix now
Was: 1-vgj6.1
Now: fixed

`no-gate` now requires `new`, merge of the filing branch, then `start` with
no readying, and keeps an unmerged item unstartable (`mission.md` 87–91).
atelier-v06d's first Done when matches, with a test for the unmerged case
(`.atelier/items/atelier-v06d/item.md` 30–33). The Outcome's `start` line
now says the item must be on the base (`mission.md` 56–57). `Repo::locate`
still reads only the base and the item's own branch
(`crates/atelier-core/src/repo.rs` 244–250).

## Finding 3: Reconcile reviewer filing with the brief that forbids other writes
Recommendation: fix now
Was: 1-vgj6.2
Now: fixed

atelier-u6oy now owns both briefs: an item reviewer files in the
implementer's checkout, the printed brief replaces "change no other file",
and a test follows that brief through commit and merge
(`.atelier/items/atelier-u6oy/item.md` 38–42). A plan reviewer is told to
publish the verdict first and then file on a backlog branch (`item.md`
48–50). `approval-holds` is a mission criterion (`mission.md` 110–112) and
a u6oy Done when (`item.md` 43–47). The shared brief at
`crates/atelier-cli/src/output.rs` 1160 still says "change no other file";
that is the text u6oy replaces. A leftover in atelier-8phx's skill sentence
is Finding 5.

## Finding 4: Require working status consumers when the binary changes to open
Recommendation: fix now
Was: 1-vgj6.3
Now: fixed

The Outcome now requires both clients to keep showing and counting every
unstarted item from the merge that changes the binary on (`mission.md`
79–81). atelier-v06d requires a loose open item and an open mission item to
appear on the Desk project and mission pages and on the extension home and
mission pages, with a test in each, and leaves layout to atelier-rme3
(`.atelier/items/atelier-v06d/item.md` 64–69, Scope 75–76). The pin still
drops `open`: `desk/src/pages/ProjectHome.tsx` 233–234 admits loose items
only as `ready` or `draft`; `desk/src/lib/group.ts` 9 groups only
`LIFECYCLE` in `words.ts` 11, which has no `open`;
`t3-extension/src/home/model.ts` 42–43 and `t3-extension/src/mission/model.ts`
210 do the same.

## Finding 5: Skill line still files a plan reviewer onto the plan checkout
Recommendation: defer

atelier-8phx's Done when still says every role files with `atelier new` on
the branch it is on (`.atelier/items/atelier-8phx/item.md` 32–36). A plan
reviewer's checkout is detached at the plan commit. `misplaced` allows
`new` there: a detached HEAD or a branch not named for an item returns
`Ok(None)` (`crates/atelier-core/src/plan.rs` 241–243, 261–263). A loose
item (no `mission:`) is invisible to `plans_changed`, which only follows a
new item's `mission` field (`crates/atelier-cli/src/merge.rs` 792–799), so
it can merge with the plan after the reviews without staling them.

u6oy's brief is the only prevention (`item.md` 48–50). Round 1 said that
brief would suffice for the plan-reviewer case, so this does not block
start. When 8phx writes the skill line, it should match that brief: file on
a start-made branch you hold; if you hold none, after the work in hand is
published, on a backlog branch. The 8phx eval is implementer-only and will
not catch this.
