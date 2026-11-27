---
schema: atelier.review
schema_version: 3
round: 1
commit: 6a2c26e15cf9af28189be303b5b48914fa521720
reviewers:
- 01a201bf-aa32-78e3-b465-9cb674f27b70
model: grok-4.6
verdict: changes
created_at: 2026-11-22T05:34:51.396653302Z
---

## Summary

Checked the plan at 6a2c26e: `mission.md`, `design.md`, and the six ready items; the eight dropped items they supersede; and the code those items name. Confirmed the CI `changes` job still installs Node/`pnpm` and runs `scripts/t3-extension-release.test.sh` ([ci.yml:39](.github/workflows/ci.yml:39)), `scripts/ci-jobs.sh` still skips the 0.4 paths ([ci-jobs.sh:60](scripts/ci-jobs.sh:60)), `plan_brief` always reprints the full plan ([output.rs:1232](crates/atelier-cli/src/output.rs:1232)), `drop` of a started item requires the item branch ([plan.rs:259](crates/atelier-cli/src/plan.rs:259)), the five CLI paper cuts still stand in `start_reviewer` / `overview` / `push_refused` / `responses`, garden runs `atelier-0019` and `atelier-qb4f` are `approved` on their branches, and `/api/repos/{repo}/missions/{id}/history` is unrouted (Desk test expects 404).

Complexity: high. Six items, three of them sharing `output.rs`, plus a Desk contract `t3-extension/` must keep with zero edits.
Value: high. Record-only CI, split plan reviews, started-item drop, `submit`/`status`, garden merge, and Desk launch are paths in current use.
Risk of merging it: high. A wrong `EventKind` mapping on `/api/history` silently mis-counts review rounds in the extension's lanes. Drop-on-backlog and remote-branch delete run in every clone.

Could not check: how many `clippy` warnings `master` currently has; how long `atelier_core::log::history` takes on this repository; record-only CI wall time; the other reviewer's file.

## Finding 1: After two split plan rounds, `submit` still opens the next round
Recommendation: fix now

`atelier-h6ij` stops the 4–7 round split loop by changing `status` on the mission branch and `merge`'s unapproved refusal after two consecutive split rounds. The command that opens the next round is `atelier submit` on the mission branch ([commands.rs:606](crates/atelier-cli/src/commands.rs:606)), and two printed lines already tell the planner to run it:

- `plan_submitted` ([output.rs:915](crates/atelier-cli/src/output.rs:915)): "If either asks for changes, change the plan, commit, and run `atelier submit` again for the next round."
- `plan_unapproved` ([output.rs:1849](crates/atelier-cli/src/output.rs:1849)): "Run `atelier submit` here to open the next round of plan reviews..."

The split-round loop is: verdicts split, merge refuses, planner follows that line and submits. `plan_submitted` is printed at the start of the round, before the split is known, so a planner that follows it never reads the new `status`/`merge` text. `submit_records` / `planned_here` do not look at trailing splits.

Name `submit` in the Done when: after two trailing split rounds it refuses with the same human-ruling message (the human can pass that refusal to allow one more round after an amendment). Change `plan_unapproved` and do not leave `plan_submitted` as the path that reopens the round. `atelier-s8wm`'s Done when already asked that `submit` and `status` agree on what ends a split.

## Finding 2: js4a does not name the EventKind mapping `log::history` needs
Recommendation: fix now

`atelier-js4a` replaces `crates/atelier-desk/src/history.rs`'s subject parser with `atelier_core::log::history` while keeping `/api/history` and `/api/launch` compatible so `t3-extension/` needs zero edits. Translating `Logged` into `HistoryEvent` is not a 1:1 map, and the item never names the table.

`Happened` ([log.rs:60](crates/atelier-core/src/log.rs:60)): `Review` is "review file appeared" ([log.rs:948](crates/atelier-core/src/log.rs:948)); `Verdict` is the verdict ([log.rs:960](crates/atelier-core/src/log.rs:960)); `Passed { merge: true }` is a plan merge ([log.rs:114](crates/atelier-core/src/log.rs:114)); mission creation as `Draft` emits nothing ([log.rs:924](crates/atelier-core/src/log.rs:924)).

Desk `EventKind` ([api.rs:636](crates/atelier-desk/src/api.rs:636)): `review` is the verdict with `round`/`verdict` ("Review {id}, round {n}: {verdict}"); `plan_review` is the same for a plan; `planned` comes from "Plan mission {id}"; `merged` from "Merge {id}".

`t3-extension/src/mission/model.ts` `lane()` counts every `review` event as a round and treats a missing verdict as changes, not approved. `desk/src/lib/mission.ts` `logEntries` draws "Mission planned" from `planned` and "Plan merged" from `merged` on the mission id. Emitting `Happened::Review` as `EventKind::Review`, dropping `planned`, or mapping `Passed { merge: true }` to nothing would change lanes and the progress log with no extension edit.

Put the mapping in `atelier-js4a`: which `Happened` becomes which `EventKind` (only `Verdict` to `review`/`plan_review`, synthesize `planned`, map a plan's `Passed { merge: true }` to `merged`), that no new `EventKind` variant is added, and extra actions (`TakeOver`, `Settle`, review-file-appeared) are omitted from `/api/history`.

## Finding 3: Garden decoupling still files bugs and proposals through a session
Recommendation: fix now

`atelier-7637` merges the run on approval and files the ≤3 proposals as one `needs-human,garden` item. Two filing paths in `garden.md` still assume the session that this item removes, and the proposals item has no command that succeeds on the run branch.

- Step 4 ([garden.md:132](skills/agent-factory/procedures/garden.md:132)): "Something broken today you file as a draft on a backlog branch...; the session makes it ready or drops it on the human's word." After decoupling there is no session, so those drafts stay unready.
- Step 7 today files each agreed proposal on a backlog branch during the session ([garden.md:195](skills/agent-factory/procedures/garden.md:195)). The new proposals item is unspecified: `atelier new` without `--from` is refused on an item's branch ([product.md:448](docs/product.md:448), [design.md](docs/design.md) action table). Filing it on the run branch before merge fails; the gardener after `atelier merge` is still on that item branch.

Say in `garden.md` (and the item's Done when): file today's bugs as `needs-human` items on a `backlog/*` branch, with no session to ready them; after the run merges, `atelier start --backlog garden-proposals-<date>` and file the one proposals item there. `atelier-dh7a` may later allow `new` on an item branch; write the procedure so it works with today's table.

## Finding 4: js4a has no path if `log::history` misses Desk's 15s bound
Recommendation: defer

The design names this as the riskiest assumption and settles it by benchmarking inside the item, keeping the ref-keyed cache. `/api/launch` already calls history on every host view and fails soft ([views.rs:177](crates/atelier-desk/src/server/views.rs:177)); today's parser bounds `git log` at 15s and 20_000 commits ([history.rs:17](crates/atelier-desk/src/history.rs:17)). `log::history` walks every named branch tip with record diffs ([log.rs:216](crates/atelier-core/src/log.rs:216)). The lead may shrink an item only for a conflict with `atelier-dh7a` or the evals, so a slow walk still has to ship. If the timing note on this repository misses the current bound, keep the subject parser for `/api/launch` and `/api/history` and use `log::history` for M5 only.
