---
schema: atelier.item
schema_version: 3
id: atelier-7637
title: Decouple garden upkeep merges from human proposal sessions
status: changes_requested
mission: atelier-ww7w
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Aa7ab9ade-d27f-4fdc-9a10-37446bba3ca2%3Adelegate-task%3A7b72d090-3274-4e2a-b4b8-49eea01431b4
ready:
  by: Sumeet, in conversation on 2026-11-22
  at: 2026-11-22T05:33:31.467134267Z
trivial: null
caused_by: null
created_at: 2026-11-22T05:30:53.799635070Z
---

## Goal

Stop approved garden runs from stalling unmerged on human sessions, while cleanly reassigning every job the old garden session handled.

Today `skills/agent-factory/procedures/garden.md` requires an approved garden branch to wait for a synchronous human session before `atelier merge`. In practice, both `atelier-0019` (Nov 11) and `atelier-qb4f` (Nov 13) have sat unmerged (approved locally, still `in_review` on `origin`) for over a week, leaving their reviewed upkeep fixes and deferred-finding settlements off `master` and causing newer runs to collide on the same deferred findings.

Decouple the two halves: merge reviewed upkeep and deferred-finding settlements immediately on approval, and hand proposals and newly found bugs to the human asynchronously via `needs-human` items on a backlog branch. Also moots `atelier-y0ba`.

## Done when

- [x] `skills/agent-factory/procedures/garden.md` (opening paragraph and Steps 1, 4, 5, 6, and 7) is updated so that:
  - Step 7 merges the run branch (`atelier merge`) immediately once independent review approves it, landing its upkeep fixes and deferred-finding settlements.
  - Immediately after `atelier merge` (using today's action table where `atelier new` belongs on a backlog branch), the gardener uses one **stable** backlog slug `garden-proposals` (`backlog/garden-proposals`, never a dated slug and never picking a second slug):
    - If `backlog/garden-proposals` already exists locally or on the remote (**whether merged or unmerged**, without assuming `atelier-gqtw` deleted it): work in the checkout that already holds it if one does (since `git switch` refuses a branch checked out in another worktree), otherwise switch to it (tracking the remote branch if it only exists remotely), bring `base` into it (fast-forward if merged, or merge `base` if unmerged), and update the item there.
    - Call `atelier start --backlog garden-proposals` **only** when neither local nor remote has `backlog/garden-proposals`.
    - On that branch, file — or update if an open rolling proposals item already exists (identified by labels `needs-human,garden` **and** its fixed title `"Garden proposals"`, so the older one-off digest `atelier-j86z` is not mistaken for it) — the single proposals item referencing the merged run's Report, commit, submit, and merge it.
  - **No silent dropping of unanswered proposals:** Replace the old "brought once more, marked carried, and then left out" rule for unanswered proposals. Updating the open `"Garden proposals"` item keeps **every** unanswered proposal (noting the run/date it came from) alongside up to 3 new proposals from the current run; only the human's answer removes a proposal.
  - **Recovery if a run stops after merging its run branch:** The run's `## Report` lists both its `Proposals` and any `Bugs to file` found during the run; Step 4 of the next run checks open/recently closed `"Garden proposals"` items (both their open proposals and their `### Decided` section), open/recently closed `bug` items, AND the last merged garden run's `## Report`, carrying/filing any proposal or bug listed in that Report that no existing item already covers.
  - Bugs found during the run (Step 4/5) are filed on that same `backlog/garden-proposals` branch labelled `bug,needs-human` (with `--caused-by`), instead of waiting as drafts for a session.
  - Applying answers: once the human answers some or all proposals in `"Garden proposals"`, a **planner** works on `backlog/garden-proposals` (using the same reuse rule so `"Garden proposals"` is never edited on two competing backlog branches) and files agreed proposals as items there. If **every** proposal in `"Garden proposals"` is answered, the planner closes it with `atelier close <id> "<decisions>"`; if only **some** proposals are answered, the planner moves only the answered proposals out of the open list into a `### Decided` section in `"Garden proposals"` (recording each decision — filed item ID or declined reason — so Step 4 recovery treats them as already covered and never re-files them) and leaves the unanswered proposals open in the item.
  - Step 1's Done when checklist and Step 6's Report template match (removing `### Session` and the hold-before-merge requirement, and adding `Bugs to file` so crash recovery sees them).
- [x] `skills/agent-factory/SKILL.md` ("Who Works With the Human"), `docs/product.md`, and `docs/design.md` match the decoupled flow.
- [x] `atelier check` and `evals/agent-factory/test.sh` pass.

## Scope

In: `skills/agent-factory/procedures/garden.md`, `skills/agent-factory/SKILL.md`, `docs/product.md`, and `docs/design.md`.

Out: merging or settling the existing `atelier-0019` and `atelier-qb4f` branches themselves, changing the gardener's 8 focus areas or metrics calculations, and any Rust code changes.

## Report

An approved garden run now merges at once, so its upkeep and settled findings no longer wait on the human. Its proposals and bugs follow on `backlog/garden-proposals`.

What changed, in documents only:

- `garden.md`: Step 1's checklist drops the hold. Step 4 lists bugs and carries what the last merged Report left unfiled. Step 6's template has `Bugs to file` and no `Session`. Step 7 merges, hands over, gives the one rule for reusing the branch, and says how a planner applies answers.
- `SKILL.md`, the Gardener row of `docs/product.md` and a Decisions row in `docs/design.md` say the same.

Four choices the item left open:

- The run touches the proposals branch only when it has something to file or label, or that branch holds something unmerged. A run with nothing to say makes no branch, and a hand-over stranded unmerged still gets merged.
- Carried proposals and bugs go on the new run's page, outside the limit of three, so a second stop after a merge does not lose them. An item a run left waiting for labels is recovered the same way.
- With no forge the branch takes in the local base, since `merge` moves only that there.
- The planner takes `needs-human` off a bug before readying it, or drops it.

### Validation

- `atelier check`: exit 0 at e018cf7b8.
- `evals/agent-factory/test.sh "$(command -v atelier)"`, with the installed 0.7.0, not a build of this tree (no Rust changed): exit 0 at e018cf7b8, "All eval fixtures and harness tests pass."
- `git diff --check`: exit 0.
- A dry run of the branch rule and the planner's steps, with installed 0.7.0 in scratch clones with a bare remote and no forge: the branch was made when absent; reused and fast-forwarded when merged (`start --backlog` refuses then); refused to `git switch` when another worktree held it; and, in a second clone, tracked from the remote alone and merged with a moved base while unmerged. A partial answer left the rest under `### Open`, readying a bug took `needs-human` off it, and `close` ended the item once all were answered.
- Not run: the same cycle through GitHub. Read instead: `merge::pull_request` looks up only open pull requests, so a reused branch gets a new one.

### Left for the lead

- `docs/product/desk.md` ("Sessions, not queues", and its step 4) and draft atelier-bbia still describe a garden session that holds the merge or the proposals. Both are outside this item's Scope.
- `atelier start --backlog` on an existing branch suggests "Pick another slug", which this procedure must never do, so the procedure says to ignore that. Changing the message is Rust, which is out of Scope.

### Responses
