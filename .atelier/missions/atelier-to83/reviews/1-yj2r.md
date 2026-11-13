---
schema: atelier.review
schema_version: 3
round: 1
commit: 7c28a3dbd330faa452b65d89d06988b0bb002fa0
reviewers:
- 01a1d38d-0ff1-7683-a6a7-dc6fea3fed7d
model: grok-4.6
verdict: changes
created_at: 2026-11-13T06:16:52.025742407Z
---

## Summary

Checked the plan at 7c28a3d: `mission.md`, `design.md`, and all eight items; the blocker graph; current `docs/product.md` lifecycle and records rules, `docs/design.md` act table, merge, hooks, forge, and "Tried and removed"; `crates/atelier-app/src/acts.rs` (`Submit` / `Merge` / `CloseMission`); `crates/atelier-core/src/mission.rs` (`MissionStatus`); `crates/atelier-cli/src/forge.rs` (no merge-PR call today); `roles/planner.md`; and the cited neighbours atelier-lkf5, atelier-qxmr (already dropped into v958), atelier-g961 (round-3 approvals, still `draft`), atelier-7w9s, and atelier-w0su. Did not run commands against a protected remote, time a record-only CI path in the four product repositories, or read another reviewer's file.

- Complexity: high. Seven commands move off the base, `merge` and the pre-push rule change, the mission record format changes, the reader and the Desk grow a multi-ref view, and one binary then converts five repositories.
- Value: high. Direct pushes are why the tool cannot sit on a protected base, why draft plans live on `master`, and why the Desk is stale; atelier-g961 has two approving reviews and is still `draft` with `ready: null`.
- Risk of merging it: high. A wrong `merge` or hook stops work in five repositories at once. The value is worth that if the holes below are closed; they are not.

Could not check: how GitHub records `done` on a merge commit it creates (left to atelier-ogpg); whether a record-only pull request is under two minutes in astromech, demiurge, life-on-rails, or cosplay-studio; whether listing 200 named remote branches stays under two seconds without a cache.

## Finding 1: Record-only merge never refuses a branch that also changes code
Recommendation: fix now
Independent review is not negotiable (`docs/product.md` principle 5). The Outcome and `design.md` Shape say a branch that holds code needs an independent review, and records alone need `atelier check`. atelier-ob9u's Done when (item.md:35-36) is: `submit` on a branch that changes only records opens a pull request, and `merge` merges it once `atelier check` passes, with no review file asked for. The only extra refuse it names is a branch that also changes a mission, left to atelier-2vdq.

Nothing says `merge` refuses when that branch also changes files those record commands do not write (`crates/`, `desk/`, `docs/`, the skill). `atelier check` validates records, not that the diff is records. A `backlog/<slug>` branch that carries a crate change would merge with no review. Name the path set that counts as records (at least `.atelier/`) and make the record-only merge refuse any other path, in atelier-ob9u, before that path exists.

## Finding 2: No act names who may submit or merge a mission or backlog branch
Recommendation: fix now
`merge` today is an implementer act on an approved item (`crates/atelier-app/src/acts.rs` `Act::Merge`, from `approved` to `done`; `docs/design.md` Merge: it runs in the checkout on the item's branch). `submit` is the same role, from `building` / `changes_requested` / `approved`. A mission has no `implementers` and, after this plan, no `draft` / `ready` / `approved`. A `backlog/` record change has no item status at all.

atelier-ob9u:35-36 and atelier-2vdq:31-34 say `submit` and `merge` run on those branches and what they require of reviews, and they never name the role, the from-status, or which checkout the command must be in. Authority does not give the lead this choice. Without new act-table rows, `merge` on a plan either reuses the item act and refuses, or becomes a command anyone can run on any branch. Spell the rows: who binds, from what, and that `merge` of a mission or backlog branch is not the item implementer act.

## Finding 3: Two plan reviewers sharing the mission branch cannot stay blind or both push
Recommendation: fix now
The product rule is two blind plan reviews (`docs/product.md` Plan reviews; `roles/planner.md`: each reviewer in its own checkout, tell neither about the other). `docs/design.md` 767-770 puts each reviewer in a detached worktree and gives review files distinct names so the planner can cherry-pick both without a conflict.

atelier-2vdq:31-32 replaces that with: `review` and `verdict` write on the mission's branch, in the reviewer's own checkout of it, and push; no cherry-pick; two reviewers in one round do not conflict. Both checkouts track the same ref. The second `git push` is a non-fast-forward even when the files do not overlap, unless the command fetches and rebases; after that fetch the second reviewer has the first's findings. Distinct file names solved cherry-pick, not this. Specify the refs (for example one review ref per thread that `merge` reads), that `verdict` is the thing that publishes, and that neither reviewer fetches the other's file before its verdict.

## Finding 4: The plan-review gate's surface disagrees with the criterion and with lead
Recommendation: fix now
Success criterion `reviews-are-the-gate` (`mission.md`:77-80): `merge` refuses a branch that adds or changes a mission while it has fewer than two approving plan reviews of its head. `design.md` Shape:28 is the same: "a new or changed mission: two approving plan reviews."

atelier-2vdq:33-34 gates only a branch that adds a mission, or that changes Why, Outcome, Scope or Authority of one already on the base. `design.md` Decisions keep `lead`, a lead's notes, and mid-mission items on the record-only path, which do change the mission record and would fail the criterion as written (zero plan reviews, should merge). Success criteria is in neither the gated list nor the record-only list, so a record-only branch can rewrite the mission's tests.

Pick one surface and write it in the criterion, in `design.md`, and in atelier-2vdq. The gated sections have to include Success criteria if that text is what reviews-are-the-gate is meant to protect; `lead`, notes, and `new --in` after the mission is on the base stay off that list.

## Finding 5: Merge cannot tell a planner from a reviewer on a mission
Recommendation: fix now
atelier-2vdq:34 requires the two approving reviews to come from threads that are not the planner's. Item reviews use `implementers` (`acts.rs` `FormerImplementer`; `review` refuses a caller on that list). A mission has no `implementers` (`crates/atelier-core/src/mission.rs`). `lead` is attribution written through a record-only branch once the plan has merged (`design.md` Decisions; atelier-2vdq:37), and this mission's own record has `lead: null`. Git authorship is not a thread id.

The gate as specified cannot be implemented. Record the planner when `new --mission` (or the first commit on the mission branch) binds, the same way `start` binds an implementer, and have `merge` read that list.

## Finding 6: An agreed open mission has no status for close, lead, show or the Desk
Recommendation: fix now
A mission's stored statuses today are `draft`, `ready`, `done`, `dropped` (`MissionStatus` in `crates/atelier-core/src/mission.rs`; `docs/product.md` Mission table). `close` on a mission runs from `ready` (`Act::CloseMission`). `lead` runs on a ready mission. The Desk and `show` print a status.

The plan removes `draft` and the `ready` field (`design.md` Decisions; atelier-2vdq:35) and does not name the replacement for an agreed mission that is still open: a new value, a missing status, or display derived only from "on the base and not done". atelier-2vdq:38 maps old `status: draft` on the base to proposed and says `ready` is ignored; it does not map old `status: ready`, which is what every in-progress mission on the five repositories actually has, even though the Done when asks for a test that reads those records. atelier-ogpg is to copy this mission's format into `docs/design.md`, not to invent the enum. Name the open-agreed state, the from-status of `close` and `lead`, and the old-format mapping for `status: ready`.

## Finding 7: t5am can rewrite merge in parallel with the plan-review gate
Recommendation: defer
atelier-t5am and atelier-2vdq both change `crates/atelier-cli` `merge.rs`. t5am is blocked only by atelier-ob9u; 2vdq also waits on atelier-dciw, so t5am can land first. t5am's Done when says it does "its checks as now" and then asks the forge to merge. The plan-review gate is a new check. The lead may order items, so this need not block starting, but t5am should wait on 2vdq (or 2vdq on t5am) so the forge finish runs the gate rather than a stale check list.

## Finding 8: AGENTS.md still sends planning commits to the base until j5lr
Recommendation: defer
After atelier-ob9u the commands commit on branches. The pre-push rule still allows planning commits on the base until atelier-t5am, and `AGENTS.md` Work / Merge Policy still say planning records may be committed on `master` until atelier-j5lr, which waits on both 2vdq and t5am. Agents that follow the map will `git commit` on the base for most of this mission. Moving the one AGENTS.md sentence with ob9u is enough; the rest of j5lr can wait for the new flow.
