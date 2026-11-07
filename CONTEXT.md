# Atelier Domain Language

Use these terms for the verified repo-native tracker defined by
[ADR 0019](docs/adr/0019-verified-repo-native-tracker.md).

| Term | Meaning |
| --- | --- |
| Record | Tracked Markdown with typed YAML front matter and a human-readable body under `.atelier/`. |
| Mission | An issue of type `mission`: a themed container whose Outcome says why its work belongs together, optionally with a strategy and success criteria; its execution scope uses `advances` links. |
| Strategy | Ordinary Markdown owning intent, any success criteria, boundaries, tradeoffs, and adaptation authority. Link by path. |
| Issue | One accountable slice; work items have `Goal` and `Done when` sections. |
| Parent | Any work item with direct children, one level deep; it owns their branch, and the merge that integrates it marks it and its open children done. Replaced the epic type (atelier-9vq3). |
| Validation item | Work dedicated to independently checking referenced acceptance claims. |
| After-merge check | A Done when line marked `After merge (<reason>):` with its stand-in: proof only the merged or deployed system, an unreachable device or place, the human's judgment, or time and real use can give. Every other line is checked before merge (ADR 0027). |
| Awaiting validation | Done work with an after-merge check neither ticked (`[x]`, validated on the base) nor carried to another existing item (a failed check to its follow-up bug, or one needing time and real use beyond the mission). |
| Decision | A `decision` record: a Question for the human, with Options, closed by `decide` once its Decision is recorded; it needs no branch. |
| Blocker | A native dependency that prevents another item becoming executable or done. |
| Branch owner | A parent for its children, or a standalone work item; recorded branch metadata identifies its work branch and base. |
| Status | Repository-defined lifecycle state; its category supports work queries. Agent Factory decides the process. |
| Transition | A permitted status change plus read-only checks; it writes records and has no Git or provider actions. |
| Gate | A derived `workflow_validator` query node for an invariant check; it is not a persisted record. |
| Evidence | An immutable proof receipt with an explicit result and full Git commit, attached to the claims it supports. |
| Result | `pass`, `fail`, `blocked`, or `not-applicable`; receipt presence alone never means pass. |
| Review | An immutable in-repo event for examination, approval, a finding, or a trivial exemption; additions get new files. Only a finding's disposition changes in place. |
| Approval | A review decision pinned to a commit, from an identity independent of the change's authors. |
| Finding | A recorded problem that needs an explicit `FIX NOW`, `DEFER`, or `NO ACTION` disposition. |
| Disposition | What was decided about a finding: a field of the finding's record with the decision, who set it, the commit, and why. Setting it again replaces it; Git history keeps earlier ones. A `fix_now` is followed by a re-review, which may cover only the fix's diff and may be by the same reviewer. |
| Trivial exemption | An explicit visible declaration that independent approval is unnecessary because correctness is evident from the diff. |
| Freshness | Whether later commits changed material relevant to the proof or approval. For an approval that is code: anything outside the tracker records. |
| Report | The `## Report` section saying what happened: the implementer report a work item's author writes before review, or the reason a transition that requires `close_reason` records (supersede, a mission's close, a decision's `decide`). Records written before atelier-jnrh call it Close Reason. |
| Done | A work item is done when its change is merged: integration marks it done. A mission is closed when all its work is merged and its validation, if it has one, is done. |
| Done claim | A terminal status in a record; for work, the one integration writes. An item an older Atelier closed on its branch carries its own. |
| Integration | A gated merge commit onto the base branch whose tree passed `merge_checks` and which marks the owner's open work done; the only commit Atelier creates. |
| Status | Session orientation from records, Git state, proof, and review; `prime` is its deprecated alias. |
| Land | A session handoff check of committed records and tracker invariants. |
| Active tree | Current `.atelier/` records, distinct from records retained only in Git history. |

The graph stores hierarchy in `children`, dependencies in `blocks` (on the
blocker) or as `blocked_by` in `relates` (on the blocked issue), evidence in
`attachments`, and semantic links in `relates`. Each issue link is stored only
on the record that declares it; reverse views are derived when read. Missions are not hierarchy
parents. Any work item may have direct children, one level deep; standalone
issues remain valid. Mission membership uses `advances`, not parent fields.

Different branches may contain different records and statuses. Cross-branch
summaries name their branch and never overlay them onto the current checkout.
Merge-time facts come from Git, not a second stored integration state.

Historical terms such as review room, provider terminal action, activity file,
SQLite domain cache, and Mission Control describe retired designs. They are
not aliases for current concepts.
