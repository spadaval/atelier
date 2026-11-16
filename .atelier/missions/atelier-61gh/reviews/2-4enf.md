---
schema: atelier.review
schema_version: 3
round: 2
commit: d2ee2a4200204831d5058252b7cf47b9d7503ba6
reviewers:
- thread:delegated-task:command%3Amcp%3Abedbfab3-88f2-4387-b923-ba6bd0a0f9b1%3Adelegate-task%3Ab1848b22-aa8b-4878-a019-00b575b69b09
model: claude-opus-5-5
verdict: approve
created_at: 2026-11-16T02:38:17.644411002Z
---

## Summary

I read the mission and its four items at d2ee2a4 and the two round-1
reviews, and checked each claim the revised plan makes against the code.

- `holds` and `named_in_item`/`named_in_mission` in
  `crates/atelier-desk/src/read.rs:409-470` do collect every implementer and
  every round's reviewers, as prx5's Context now says. `Entry::holder` and
  `Entry::reviewer` exist as fields of `Entry` in
  `crates/atelier-core/src/view.rs:217`, and `Threads::holder` gives the
  last holder (`crates/atelier-core/src/model/mod.rs:75`), so prx5 has what
  it needs to compute `current`.
- `Agent.identity` (`crates/atelier-desk/src/api.rs:390`) and `LiveThread.id`
  (`api.rs:443`) exist, so fnm5's source for a thread link's identity is
  real, and it allows the server to add the identity where a page lacks it.
- `t3code://app` is T3 Code's packaged origin: the scheme is registered
  `standard` and `secure` (`apps/desktop/src/electron/ElectronProtocol.ts:112`
  in a local T3 Code checkout), so a framed page sees `t3code://app` as the
  parent's origin. The extension (a review checkout of
  `t3-code-atelier`, `custom.js:96`) frames the Desk in a plain `iframe` in
  T3's renderer, so that is the parent the Desk will see.
- The pages vg1l lists match the routes in `desk/src/App.tsx`.
- The blocking order (vg1l, then fnm5; prx5 and fnm5, then knx8) lets each
  item build on what it needs, and the mission's criteria map onto the items'
  Done when lines.

I did not run anything; this is a plan review. I could not check that an
`http` Desk on the tailnet loads inside the secure `t3code://app` page on
Windows without a mixed-content block, but the extension already shows it
there today, so this mission does not change that.

## Finding 1: The thread read gives knx8 too little to choose the page, and knx8 cannot change it
Recommendation: fix now
Was: 1-0pno.1
Now: fixed
prx5 now gives each tie its record's status and `current`, and knx8 chooses from current ties only, in a stated order.
<!-- Set Now: to fixed, withdrawn or kept. To keep it for later work instead, set Recommendation: to defer or no action and add a line that says why. -->

## Finding 2: The contract does not say which origins it posts to or accepts, and the Desk is framable by any page
Recommendation: fix now
Was: 1-0pno.2
Now: fixed
The mission and fnm5 now keep a configured list of origins, empty by default, post with the listed origin as target, read only from the parent with a listed origin, and send and read nothing otherwise; the Authority section asks before going beyond the list, and fnm5 names `t3code://app`.
<!-- Set Now: to fixed, withdrawn or kept. To keep it for later work instead, set Recommendation: to defer or no action and add a line that says why. -->

## Finding 3: Thread links today are web links that open a new tab, not `t3code:` links, so asks-the-host tests the wrong thing
Recommendation: fix now
Was: 1-0pno.3
Now: fixed
The Why now describes the web link in a new tab, the criterion and fnm5's test check that no page or popup opens and the frame's URL does not change, and fnm5 names `Agent.identity` and `LiveThread.id` as the identity's source.
<!-- Set Now: to fixed, withdrawn or kept. To keep it for later work instead, set Recommendation: to defer or no action and add a line that says why. -->

## Finding 4: Define current work in the thread lookup before using it to choose a page
Recommendation: fix now
Was: 1-92k2.1
Now: fixed
prx5 defines `current` for each role, keeps `/api/threads` as on the base, and tests a done-then-reviewing thread and a replaced implementer; knx8 sends a thread with only past ties to the inbox and tests it.
<!-- Set Now: to fixed, withdrawn or kept. To keep it for later work instead, set Recommendation: to defer or no action and add a line that says why. -->

## Finding 5: prx5's `current` rule does not name changes-requested items or deciders
Recommendation: no action

`.atelier/items/atelier-prx5/item.md` makes an implementer current for an
item "that is started, in review or approved". Item statuses are `building`,
`in_review`, `changes_requested` and `approved`
(`crates/atelier-core/src/model/item.rs:39-48`). `docs/product.md:175` uses
"started status" for that whole set, so "started" covers `changes_requested`
and the list is only redundant. An implementer who reads "started" as
`building` would mark the thread fixing a review's findings as not current,
the common case for knx8. The rule also does not classify `Decider` ties
(`api.rs:417`), but knx8 never chooses a page by them. Both are clear enough
from the docs for an implementer to get right; noting them for the
implementer's attention, nothing to change in the plan.
