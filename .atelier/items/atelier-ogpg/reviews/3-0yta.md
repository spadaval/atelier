---
schema: atelier.review
schema_version: 3
round: 3
commit: fa983915a39716c78b4b86ec2cae9dd16667fe94
reviewers:
- thread:delegated-task:command%3Amcp%3A9f96e1ea-9eb7-432f-85d1-e61ac8cede94%3Adelegate-task%3A7b6372e3-9c95-470b-80ce-5db49a91a2ba
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-13T08:58:42.843464492Z
---

## Summary

Complexity: high. The documents define a branch, record and merge contract
that several later implementation items must follow together.

Value: high. Reviewed text is the prerequisite for the branch-model work
in atelier-to83.

Risk of merging it: medium. An incorrect contract could spread into the
binary shared by five repositories, although the documentation is reversible
and the implementation receives separate reviews.

The value warrants the scope. I read both documents, compared their
lifecycle, act table, branch ownership, record lookup, merge procedures and
compatibility rules with the mission's Outcome, Authority and design
decisions, and checked each Done when line. I examined the document changes
since round 2 and verified both carried fixes at the reviewed commit
fa983915a39716c78b4b86ec2cae9dd16667fe94. I found no additional problems
requiring changes.

`git diff --check master...fa983915a39716c78b4b86ec2cae9dd16667fe94 --
docs/product.md docs/design.md` passes. GitHub reports PR #153 open at that
head with no status checks. This is documentation ahead of implementation,
as both documents state; I did not run runtime tests or treat the current
binary as proof of the proposed behaviour.

## Finding 1: Preserve proposed state when rewriting a legacy draft mission
Recommendation: fix now
Was: 2-gci2.6
Response: fixed in 89aef9cb0: every command but `drop` refuses to write an old `draft` mission, naming the rollout, which moves it onto its own branch to go through plan reviews; `drop` writes it as `dropped`.
Now: fixed

`docs/design.md:206-214` now separates rewriting old `ready` records from
old `draft` records. Every non-drop write to an old draft is refused until
rollout, while rollout puts the plan on its branch for review. A drop writes
`dropped`, which cannot establish an agreed plan. This closes the unreviewed
conversion path the finding identified.

## Finding 2: Push amended-plan reviews to the backlog branch being reviewed
Recommendation: fix now
Was: 2-gci2.7
Response: fixed in 89aef9cb0: a plan reviewer pushes to the branch it reviewed (the mission's branch, or the backlog branch for an amendment), in both documents and the Decisions row.
Now: fixed

`docs/design.md:879-900`, its act-table row at line 416 and its Decisions
row at line 1675 agree that the reviewer pushes to the branch being
reviewed. `docs/product.md:317` and lines 501-510 explicitly name the backlog
branch for an amendment. The detailed instructions now publish approvals
where that branch's merge gate can read them.
