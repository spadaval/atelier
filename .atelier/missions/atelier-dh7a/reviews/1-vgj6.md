---
schema: atelier.review
schema_version: 3
round: 1
commit: 804cde4c05e59a0428f5ccb4c03a9843e837ddd0
reviewers:
- thread:delegated-task:command%3Amcp%3Ad8a778d7-1203-4ffa-a7bf-a4675dc4d51b%3Adelegate-task%3A5eef01c4-1887-49f5-b28c-42aa54ebf619
model: gpt-6-astra
verdict: changes
created_at: 2026-11-19T15:26:56.246725067Z
---

## Summary

Complexity: high. One status contract changes across the CLI, record reader,
history, Desk API, two clients, skills and eval fixtures, with compatibility
required in six repositories.

Value: high. Removing the readiness ceremony and letting an implementer file
follow-ups on its branch directly address the stated human request.

Risk of merging it: medium. Existing records are preserved and changes are
reversible, but the new binary changes the behavior of other repositories and
clients immediately.

The value warrants the change. I reviewed the mission and all four items at
804cde4c05e59a0428f5ccb4c03a9843e837ddd0, then traced the existing start lookup,
planning rules, review brief, Desk status consumers and CI jobs. In a disposable
fixture, the installed binary refused to start a newly filed item both before
and after readying and committing it on a backlog branch, because its record
was absent from the base. This confirms the publication requirement is separate
from readiness. I did not run the proposed implementation's tests or eval arms,
which do not exist yet, or validate a new binary against the five other
repositories.

## Finding 1: Preserve publication between filing an item and starting it
Recommendation: fix now

The mission's `no-gate` criterion at
`.atelier/missions/atelier-dh7a/mission.md:79` and atelier-v06d's first Done when
at `.atelier/items/atelier-v06d/item.md:30` require `new` followed by `start`
with no intervening command. That is not just removing `ready`: `new` writes
an uncommitted record, while `start` uses `Repo::locate`
(`crates/atelier-cli/src/commands.rs:182`), which reads only the base and the
item's own branch (`crates/atelier-core/src/repo.rs:244`). A new record on a
backlog or another item's branch is not startable until that branch merges.
The fixture returned "No item ... is on the base branch main or on a branch
named ..." even after the ready record was committed.

The plan elsewhere preserves proposed records until merge and explicitly
excludes `new` committing or merging itself (mission lines 60–64 and 121–122).
Reword both criteria to require no readiness command or approval after the
record's normal publication. Test `new`, commit and merge the filing branch,
then `start`. Keep a test that an unmerged proposed item still cannot start.
Otherwise an implementer must break the branch model to satisfy the literal
Done when.

## Finding 2: Reconcile reviewer filing with the brief that forbids other writes
Recommendation: fix now

Atelier-u6oy requires a reviewer to file on the implementer's branch
(`.atelier/items/atelier-u6oy/item.md:39`), and atelier-8phx tells every role to
file outside-scope problems and carry on
(`.atelier/items/atelier-8phx/item.md:32`). But the shared item/plan review brief
in `crates/atelier-cli/src/output.rs:1160` explicitly says "Write your review
there, and change no other file." Allowing `new` through the hook and adding a
skill sentence leaves the reviewer with opposing instructions. The implementer
case in atelier-8phx cannot detect this conflict.

Assign the brief change to atelier-u6oy and make its reviewer test exercise the
actual printed workflow, including publication of the filed record. State how
a plan reviewer in a detached checkout files an unrelated problem without
editing the reviewed plan or losing its verdict. A short instruction to file
after publishing the review on a backlog branch would suffice for that case;
no new mechanism is needed.

## Finding 3: Require working status consumers when the binary changes to open
Recommendation: fix now

Atelier-v06d changes the Desk server's statuses but restricts client changes to
what keeps CI passing and assigns what an open item displays to the later
atelier-rme3 (`.atelier/items/atelier-v06d/item.md:56` and `:62`). Atelier-rme3
is blocked by v06d, while the mission requires reinstalling the binary after
v06d merges (mission lines 125–127). The intermediate behavior is unspecified.

The current clients discard open items: `desk/src/pages/ProjectHome.tsx:233`
and `t3-extension/src/home/model.ts:42` admit loose unstarted items only when
they are ready or draft; `desk/src/lib/group.ts:9` groups only statuses in the
old lifecycle list. The extension's mission counter similarly omits them at
`t3-extension/src/mission/model.ts:210`. Updating types or retaining fixture-based
CI alone is not a criterion that these views show the new server's items.

Give v06d the minimum functional client changes and a check that a loose and a
mission item returned as open remain visible and correctly counted in both
clients at that merge. Leave layout and wording polish to rme3, or combine the
status-consuming UI work with v06d. The plan should not require a later item to
restore visibility after the server starts returning open.
