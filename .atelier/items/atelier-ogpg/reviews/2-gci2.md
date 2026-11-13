---
schema: atelier.review
schema_version: 3
round: 2
commit: 5e535c93ff9e3431ffe5352e989297d0f823ecfc
reviewers:
- thread:delegated-task:command%3Amcp%3A9f96e1ea-9eb7-432f-85d1-e61ac8cede94%3Adelegate-task%3Ac0d9d222-deeb-4c8f-ae55-43da5864ebda
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-13T08:54:47.618242797Z
---

## Summary

Complexity: high. These two documents define a cross-branch lifecycle and
merge contract that several later items must implement consistently.

Value: high. Reviewed text is the prerequisite for every implementation item
in atelier-to83.

Risk of merging it: medium. Contradictory instructions can propagate into the
binary shared by five repositories, although the documentation is reversible
and implementation remains separately reviewed.

The value warrants the scope. I read both documents, compared them with the
mission's Outcome, Authority and design decisions, checked every Done when
line, and examined both the full diff and the changes since round 1. All five
earlier findings are fixed. I walked the new-mission, agreed-plan amendment,
record-only, started-item drop, legacy-format and forge/no-forge merge paths
through the act table and the detailed procedures. The two remaining findings
below concern the legacy-format and amendment paths.

`git diff --check master...5e535c93ff9e3431ffe5352e989297d0f823ecfc --
docs/product.md docs/design.md` passes. The unrestricted diff check reports
an extra blank line at the end of the item's Report, outside the document
change. GitHub reports PR #153 at the reviewed head with no status checks.
This is documentation ahead of implementation, as both documents explicitly
state; I did not run runtime tests or treat the proposed behaviour as proven
by the current binary.

## Finding 1: Apply the content-based review gates on every kind of branch
Recommendation: fix now
Was: 1-ep2v.1
Response: fixed in c328a9380: `merge` step 5 and both documents now apply every gate a branch's contents need, whatever its kind. Code is allowed only on an item's branch (refused elsewhere, not skippable); a plan change needs two plan approvals on a mission's or backlog branch and is refused on an item's branch.
Now: fixed
<!-- Set Now: to fixed, withdrawn or kept. -->

## Finding 2: Define a usable review and merge path for amending an agreed plan
Recommendation: fix now
Was: 1-ep2v.2
Response: fixed in c328a9380: an amendment of an agreed plan goes on a backlog branch. `submit` there adds the submitter to the mission's `planners` and opens the next plan round; `review` and both plan verdicts run from `open` with the plan changed on that branch; `merge` (backlog) applies the plan gate for each mission it changes.
Now: fixed
<!-- Set Now: to fixed, withdrawn or kept. -->

## Finding 3: Exclude config.toml changes from the unreviewed record-only path
Recommendation: fix now
Was: 1-ep2v.3
Response: fixed in c328a9380: `config.toml` counts as code in the definition of a record-only change, the Branches table, `merge` step 5 and the backlog merge row.
Now: fixed
<!-- Set Now: to fixed, withdrawn or kept. -->

## Finding 4: Make the product's push rule publish the head the forge must merge
Recommendation: fix now
Was: 1-ep2v.4
Response: fixed in c328a9380: the product's Merge section now pushes whenever the forge's copy is not the head, and explains the extra CI run under a protection that requires CI on the head.
Now: fixed
<!-- Set Now: to fixed, withdrawn or kept. -->

## Finding 5: Remove the old claim that every Atelier merge writes done
Recommendation: fix now
Was: 1-ep2v.5
Response: fixed in c328a9380: The walk's `--full-history` bullet now explains why the flag is needed when a merge writes nothing of its own.
Now: fixed
<!-- Set Now: to fixed, withdrawn or kept. -->

## Finding 6: Preserve proposed state when rewriting a legacy draft mission
Recommendation: fix now

`docs/design.md:198-208` says an old `draft` mission remains proposed even on
the base, until it is moved to its branch or its plan is reviewed and merged.
It also says that the next command to write an old record writes the new
form, which has only `open`, `done` and `dropped`, but specifies a conversion
only for old `ready`. There is no safe write rule for old `draft`.

For example, `set <mission> --labels ...` on a backlog branch can encounter
an old draft mission already on the base. Labels are not plan content, so
this branch needs only the record checks. Rewriting the draft as new-form
`open` makes it agreed when that record-only change merges, although its
plan never received the two approvals. Keeping `draft` instead contradicts
the promise to write the new form. This also leaves the subsequent item
implementing compatibility to choose the policy itself. Specify what a
non-drop write to an old draft does before migration, and make any conversion
that would establish an agreed mission take the plan-review path. Refusing
such writes until migration is another explicit way to preserve the stated
invariant.

## Finding 7: Push amended-plan reviews to the backlog branch being reviewed
Recommendation: fix now

`docs/design.md:873-889` now explicitly lets a reviewer examine an agreed
mission's amendment on a backlog branch, but its publishing instruction at
lines 886-888 still says to push the review file to the mission's branch.
The same unconditional target remains in `docs/product.md:317`. This
contradicts the corrected branch table at `docs/design.md:410`.

Following the detailed instruction publishes the amendment's approval to the
old mission branch instead of the backlog branch whose head `merge` checks.
That backlog branch consequently lacks the two approving reviews, and
cannot merge. The original mission branch may also have been deleted after
its merge. Make the publishing instruction and the printed commands target
the submitted branch in both cases: the mission branch for a new plan and
the backlog branch for an amendment. The round numbering and review-file
location can remain as specified.
