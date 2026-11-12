---
schema: atelier.review
schema_version: 3
round: 1
commit: bafc413fc011abbe76e20dbd2f8babb2d292eb25
reviewers:
- thread:delegated-task:command%3Amcp%3Aa0526aa8-7650-4e5d-80ff-0b0535866187%3Adelegate-task%3Af114b306-53f1-42fb-bc24-feda393a49db
model: gpt-6-astra
verdict: changes
created_at: 2026-11-12T01:04:41.803249747Z
---

## Summary

Reviewed the mission and all three draft items at bafc413 against the current
skill, planner role, garden, readiness and groom procedures, and the referenced
Desk/product documentation. Cross-checked the reported first-run gaps against
atelier-h201 and atelier-613a, and the old backlog digest atelier-8i86. Checked
scope ownership, dependencies, acceptance criteria and the authority being
added. This is a plan review: I did not run the proposed trial, inspect the
other repositories' transcripts, or independently recount the 104 drops.

- **Complexity: medium.** Two shared procedures change and a separate gardener
  trial must validate the result; no new command or subsystem is needed.
- **Value: high.** The first-run report supports the need for better evidence
  and the digest shows the cost of routing every backlog call to Sumeet.
- **Risk of merging it: medium.** The skill changes agents' authority across
  repositories; mistaken drops can hide real work, although Git records them.

The scope is worth doing and is already limited to the procedures, their
product descriptions and one trial. A smaller wording-only change would not
establish the new planner authority or test the gardener's behavior. Clarify
the proof required for an autonomous drop before starting.

## Finding 1: A missing code reference does not prove the work is stale
Recommendation: fix now

`.atelier/items/atelier-s47q/item.md:31` defines proof for an autonomous drop
as alternatives, including "a search of the code that finds nothing the item
names". That shows a reference is obsolete, not that the item's outcome is
obsolete. A rename or replacement can leave the same fault or unmet need in
new code. The same line explicitly allows rewriting kept items that name
removed code, but gives no test for choosing that route instead of dropping
them. Likewise, naming a replacement item alone does not show that it covers
all the old item's unfinished work.

This distinction matters now that the planner will act without a human
checking each drop. The old digest's G13, for example, established that both
the named test and the rule it covered were removed; G21 retained the
unfinished half of an item after its documentation half became obsolete.
Those are stronger checks than the proposed list requires.

Require proof against the item's Goal and unfinished Done when lines: the
need has gone, the behavior is already satisfied, or a named replacement
explicitly covers the remaining work. Treat missing names as a reason to
trace replacements and check the outcome, not sufficient proof by itself.
Keep or rewrite partially stale items, and bring unresolved cases to Sumeet.
Record that outcome-level proof in the drop reason. This preserves autonomous
drops while making the boundary with the permitted rewrites reviewable.
