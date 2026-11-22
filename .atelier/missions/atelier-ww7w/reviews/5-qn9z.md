---
schema: atelier.review
schema_version: 3
round: 5
commit: 250b429f2e2b31698e5f7431f9b9dff00849a7da
reviewers:
- thread:delegated-task:command%3Amcp%3A42ca7c4b-9f7f-469d-9389-3cf3e5d281ff%3Adelegate-task%3Aww7w-plan-review-r5-claude
model: claude-opus-5-5
verdict: approve
created_at: 2026-11-22T06:53:59.615836969Z
---

## Summary

Approve. The carried fix-now finding is fixed, and so are round 4's deferred 4-a5oe.7 and 4-a5oe.8. I found no new blocking problem. The two notes below need no change to the plan before it starts.

- **Complexity: high** for the mission, as in round 4. This round's delta is low: seven lines in four files, and no new mechanism.
- **Value: high.** A garden run would otherwise ask the human again about a proposal they had answered, and an amendment that took back an edit would read as "plan unchanged".
- **Risk of merging it: low** for this delta. Both rules are text for items not yet built, and a revert undoes either.

What I checked:

- `96227cc29..250b429f2`, less the review files: the mission's criterion, the design, `atelier-7637` and `atelier-h6ij`. The four agree, and no copy of the replaced "removes only the answered" wording remains.
- 4-fr4u.7 and 4-a5oe.8, with atelier 0.7.0 in a scratch repository with a bare origin: file A, B and C and a bug; answer A and drop the bug; merge and reuse `backlog/garden-proposals`; answer B and C and close. After each merge `atelier show` on the base still holds A's decision, `list --label garden` (with `--done` once closed) finds the item, `list --done --label bug` finds the dropped bug, and `atelier check` passes.
- `### Decided` is accepted both as a subsection of Goal and under `## Decisions`, whose entries the parser reads, so wherever the planner puts it the record stays valid.
- 4-a5oe.7, as Git commands in a scratch repository: an amendment edits an item and adds one, the base changes a third item, then the branch merges the base and takes both edits back. The old rule drops both reversions. The new union lists both, still leaves out the base's change, and is empty for a round that only merges the base.
- `atelier check` passes at 250b429, and the branch merges into `origin/master` (ac44653e3) without conflict.

What I could not check or did not repeat: the conversation that gave the Authority, code that is not built yet, GitHub, the Rust, Desk, extension and eval suites (the delta changes no code), and the unchanged plan beyond reading it for a serious miss. I did not read the other round 5 review.

## Finding 1: Step 4 recovery puts answered proposals back in the Inbox
Recommendation: fix now
Was: 4-fr4u.7
Now: fixed

`atelier-7637` now moves an answered proposal into `### Decided` with its item ID or the reason it was declined, and Step 4 reads that section as covered. The design and the criterion say the same. In the finding's own case, A is still on the base under Decided when the next run reads it, so recovery has nothing to file again. After the last answer the closed item keeps Decided, with the final decisions in its Report. The planner also works on `backlog/garden-proposals`, so the edit does not compete with the gardener's.

## Finding 2: the amendment test list has no case for a taken-back edit
Recommendation: defer

`atelier-h6ij` states the union rule, but its test (b) covers only an edit still present at `head` ([item.md:38](.atelier/items/atelier-h6ij/item.md)). An implementation that kept round 4's `<base>...<head>` rule would pass every listed test. The implementer should add the case 4-a5oe.7 gave: edit one item and add another by `prev`, take both back by `head`, and expect both paths in the delta. The rule is already a Done when line, so this needs no plan edit.

## Finding 3: the first recovery will read a Report from the old session flow
Recommendation: no action

`atelier-0019` and `atelier-qb4f` record answers under `### Session`, and are settled outside this mission. If one is the last merged run when the new Step 4 first runs, its answers are in that Report and not in a `"Garden proposals"` item. The first gardener should count an answer under that Session as covered. Both sections are empty today, and it can happen only once.
