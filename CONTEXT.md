# Atelier Domain Language

Use these terms for the verified repo-native tracker defined by
[ADR 0019](docs/adr/0019-verified-repo-native-tracker.md). Five words have
exactly one meaning across the CLI, its help, these docs and Agent Factory
([ADR 0028](docs/adr/0028-simpler-lifecycle.md) decision 8): **check**,
**review**, **land**, **done** and **close**. Each is defined below; use
another word for anything else.

| Term | Meaning |
| --- | --- |
| Record | Tracked Markdown with typed YAML front matter and a human-readable body under `.atelier/`. |
| Mission | An issue of type `mission`: a themed container whose Outcome says why its work belongs together, optionally with a strategy and success criteria; its execution scope uses `advances` links. |
| Strategy | Ordinary Markdown owning intent, any success criteria, boundaries, tradeoffs, and adaptation authority. Link by path. |
| Issue | One accountable slice; work items have `Goal` and `Done when` sections. |
| Line | One `- [ ]` item of a Done when, ticked (`- [x]`) once it holds. It is not called a check. |
| Parent | Any work item with direct children, one level deep; it owns their branch, and the merge that integrates it marks it and its open children done. Replaced the epic type (atelier-9vq3). |
| Validation item | An independent assessment of a mission's criteria before it closes, done on the base by its `done` transition with no branch or review of its own; it shares the loose end's workflow. |
| Loose end | A `loose_end` issue: work that changes no code and can only happen after a merge or a release, such as pushing a tag, a play-test on a device or an eval run. It is `blocked_by` what it waits on, or, for a release on the light path, filed on the release branch with no blocker so it reaches the base with the merge; it is ready once nothing it waits on is open, never part of a mission or any issue's children, and done on the base by its `done` transition with a one-line result in its Report; a bad result files a linked bug (ADR 0028). It replaces the After merge line of ADR 0027, which `atelier migrate after-merge` converts. |
| Decision | A `decision` record: a Question for the human, with Options, done by `decide` once its Decision is recorded; it needs no branch. |
| Blocker | A native dependency that prevents another item becoming executable or done. |
| Branch owner | A parent for its children, or a standalone work item; its recorded `work_branch` names its work branch, and the base is always the policy's `base_branch`. A branch is owned while an issue that is not done on the base records it; an owner done on the base no longer owns the name. |
| Light path | Reviewing and merging a branch no issue owns, such as a release, docs or a mockup, on an independent approval of its head, under the same merge checks, with no issue, start or Report (ADR 0028 decision 6): `review request` and `merge` with no ID, and review records that name the branch (`--branch`). Those records count only for the branch's current run, never for an issue, and an approval of a branch covers no done claim. A trivial exemption on the base covers one evident commit already there; the light path is for anything that deserves a review. |
| Status | Repository-defined lifecycle state; its category supports work queries. Agent Factory decides the process. |
| Transition | A permitted status change plus read-only validators; it writes records and has no Git or provider actions. |
| Gate | A rule `merge` and `land` evaluate, such as `review.current` or a policy validator; it is not a persisted record. Their verdict lines still count gates as `checks`, and some output still calls terminal work `closed`, until atelier-xz5c. |
| Check | One of three things: `atelier check`, which validates records and their graph; the merge checks, the policy's `merge_checks` commands run on the merged tree; and `atelier merge --check`, which evaluates the merge gates without merging. |
| Evidence | An optional proof receipt with an explicit result and full Git commit, attached to the claims it supports, for proof nothing else records; no gate reads it. |
| Result | `pass`, `fail`, `blocked`, or `not-applicable`; receipt presence alone never means pass. |
| Review | Independent review: someone who did not write the work examines it. An agent reviews a change before it merges (`atelier review request` prints the brief) or a plan before its work starts; the human approves a mission's plan (`brief`) and reads its outcome (`debrief`). |
| Review record | A file under `.atelier/reviews/` for an examination, approval, finding, or trivial exemption. A finding can be corrected or deleted until it reaches the base branch, while an approval or exemption stays as first committed; on the base, only a finding's disposition changes in place. |
| Approval | A review decision pinned to a commit, by an independent reviewer whose identity is recorded, not compared with the change's authors. An approval of a later commit that holds a `fix_now` finding's record, recorded after the finding, resolves it. |
| Finding | A recorded problem with a recommended `FIX NOW`, `DEFER`, or `NO ACTION`. A `fix_now` finding is resolved by the approval after its fix; a deferred or dismissed one needs a recorded disposition. |
| Disposition | What was decided about a finding: a field of the finding's record with the decision, who set it, the commit, and why. Setting it again replaces it; Git history keeps earlier ones. Needed for `defer` and `no_action`; a `fix_now` finding records none, and its fix's re-review, which may cover only the fix's diff and may be by the same reviewer, resolves it. |
| Trivial exemption | An explicit visible declaration that independent approval is unnecessary because correctness is evident from the diff. A branch has none: an approval is the light path's only cover. |
| Freshness | Whether later commits changed material relevant to the proof or approval. For an approval that is code: anything outside the tracker records. |
| Report | The `## Report` section saying what happened: the implementer report a work item's author writes before review, or the reason a transition that requires `close_reason` records (supersede, a mission's close, a decision's `decide`). Records written before atelier-jnrh call it Close Reason. |
| Done | The terminal status work gets when `atelier merge` integrates it; the status a decision, loose end or validation gets on the base by its own transition; and the status of a closed mission. Superseded is the other terminal status. Work has no close step. |
| Close | What a mission does, by its `close` transition, once all its work is merged and its validation, if it has one, is done. Nothing else closes; the `close_reason` policy key and `--reason` keep their names and write the Report. |
| Done claim | A terminal status in a record; for work, the one integration writes. An item an older Atelier marked done on its branch carries its own. |
| Integration | A gated merge commit onto the base branch whose tree passed `merge_checks` and which marks the owner's open work done; the only commit Atelier creates. |
| `atelier status` | Session orientation from records, Git state, proof, and review; `prime` is its deprecated alias. |
| Land | `atelier land`, run at every handoff: it judges committed records, what changed since the base and the commits about to be pushed, reports work in progress on a branch, and refuses only on the base or a detached HEAD, including unreviewed commits about to be pushed there. |
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
