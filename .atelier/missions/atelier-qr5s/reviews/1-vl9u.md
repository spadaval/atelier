---
schema: atelier.review
schema_version: 3
round: 1
commit: 701aafa228bfc2bd406ced5e337e84f3ab71e804
reviewers:
- thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3A7595c84d-0bac-4fed-8e7f-7fdd5258302f
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-11T15:26:06.789503266Z
---

## Summary

Complexity: high. The whole mission replaces the shared binary, record
layout and skill across five repositories; its remaining work spans CI,
command guidance, eval fixtures and the product and architecture docs.

Value: high. The installed tool needs a real first-use proof and documentation
that describes the commands agents can actually run.

Risk of merging it: medium. Wrong instructions reach agents in five
repositories, but the failures are visible and the changes can be reverted.

The value justifies the work. I found no substantially smaller change that
would achieve the mission's remaining outcomes.

I reviewed the plan at 701aafa: mission.md, design.md, notes.md and all four
member items. I compared the three remaining items with the product and
architecture specifications, the recorded skill matrix, the current skill,
the workflow and job-selection script, the next-* fixture dependencies, and
the retained cutover runbook and adoption implementation. I also read the
planned worldcrafter-s9lp item in demiurge's local checkout and searched the
four product repositories' available AGENTS.md and CLAUDE.md files for old
Atelier command instructions. atelier check passed for 133 items, seven
missions and config.toml, and reported only this uncommitted review.

This was a plan review, not a new execution of the cutover, eval matrix or
first-use items. I ran no build and made no forge request. I checked the
ordering problem below against the implementation of submit as well as its
specification; I did not exercise a merge on GitHub.

## Finding 1: Move the first-use item's post-submit and post-merge proof out of its submission checklist
Recommendation: fix now

In .atelier/items/atelier-2mlh/item.md:26 and :28, a Done when line requires
CI to have passed on this item's pull request, and another requires this
item already to have reached done, with the Report recording every command
and the CI run merge waited for. These are prerequisites for submit:
crates/atelier-app/src/acts.rs:912 checks every Done when line, and the
product specification at docs/strategies/redesign-product.md:215 says submit
only then pushes the branch and opens the pull request. This item's line 28
also forbids creating that pull request or pushing the branch by hand.

The first submission therefore cannot truthfully tick the CI line, and no
submission can truthfully tick the line saying its own eventual merge has
already happened. Appending the missing proof to the item after merge is
not the stated escape: docs/strategies/redesign-architecture.md:283 allows
only finding settlements as writes to done items, and the mission forbids
direct pushes outside the runbook.

Keep the item's submission criteria satisfiable before submit. Explicitly
assign the lead the later checks of the pull request, CI, merge and push,
and the complete command transcript in the mission's Result after the item
merges. That preserves the first-use proof without a premature tick or a
lifecycle bypass.

## Finding 2: Give the first-use item the limited scope exception its blocking-fix requirement needs
Recommendation: fix now

.atelier/items/atelier-2mlh/item.md:29 requires a small blocking defect in
the binary, skill or AGENTS.md to be fixed here with a test, when it is
inside the specifications. Its Scope at :34 permits only the workflow,
scripts serving the fallback, and the item's record. Rust, the skill and
AGENTS.md are consequently outside that scope. The product specification's
Review rules require escalation when a fix leaves the item's scope.

The first real forge operation is exactly where this item expects to find
a blocker. As written, following its blocking-fix requirement would also
break its scope, while escalating instead would leave that requirement
unfulfilled. Add the narrow exception for small blocking first-use fixes,
keeping status changes assigned to atelier-ivbq, or consistently require
such defects to be escalated and fixed in another item before resuming.

## Finding 3: Allow the docs move to update the retained adoption runbook's reference
Recommendation: fix now

.atelier/items/atelier-6w8d/item.md:27 deletes docs/strategies/, but its
Scope at :36 prohibits every change under scripts/cutover/. The retained
scripts/cutover/README.md:5-6 tells the reader that its protocol is the
Transition section of docs/strategies/redesign-architecture.md, which this
item removes. This is an active reader reference: the item's own :29 sends
0.4 repositories to that README, and the mission still owes Sumeet a usable
work-host adoption runbook.

Allow the documentation-only reference update in scripts/cutover/README.md
and include that page in the move's reference checks. Point it to the new
design record and its retained adoption section. This does not require
changing the converter, rollback or adoption behaviour.
