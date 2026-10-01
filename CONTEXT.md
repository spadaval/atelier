# Atelier Domain Language

Use these terms for the verified repo-native tracker defined by
[ADR 0019](docs/adr/0019-verified-repo-native-tracker.md).

| Term | Meaning |
| --- | --- |
| Record | Tracked Markdown with typed YAML front matter and a human-readable body under `.atelier/`. |
| Mission | An issue of type `mission` that links strategy and success criteria; its execution scope uses `advances` links. |
| Strategy | Ordinary Markdown owning intent, success criteria, boundaries, tradeoffs, and adaptation authority. Link by path. |
| Epic | An outcome-bearing issue, normally the branch owner for its direct child issues. |
| Draft epic | Planning context whose immediate work and proof route are not yet executable. |
| Issue | One accountable slice; ordinary work has `Goal` and `Done when` sections. |
| Validation item | Work dedicated to independently checking referenced acceptance claims. |
| Blocker | A native dependency that prevents another item becoming executable or done. |
| Branch owner | An epic for its children, or a standalone non-mission issue; recorded branch metadata identifies its work branch and base. |
| Status | Repository-defined lifecycle state; its category supports work queries. Agent Factory decides the process. |
| Transition | A permitted status change plus read-only checks; it writes records and has no Git or provider actions. |
| Gate | An invariant check applied at a consequential boundary, including merge. |
| Evidence | An immutable proof receipt with an explicit result and full Git commit, attached to the claims it supports. |
| Result | `pass`, `fail`, `blocked`, or `not-applicable`; receipt presence alone never means pass. |
| Review | An append-only in-repo account of examination, approval, findings, and dispositions. |
| Approval | A review decision pinned to a commit, from an identity independent of the change's authors. |
| Finding | A recorded problem that needs an explicit `FIX NOW`, `DEFER`, or `NO ACTION` disposition. |
| Trivial exemption | An explicit visible declaration that independent approval is unnecessary because correctness is evident from the diff. |
| Freshness | Whether later commits changed material relevant to the proof or approval. |
| Done claim | A terminal status committed on the work branch, before integration. |
| Integration | A gated fast-forward of the base branch; it creates no commit and writes no tracker state. |
| Prime | Session orientation from records, Git state, proof, and repository guidance. |
| Land | A session handoff check of committed records and tracker invariants. |
| Active tree | Current `.atelier/` records, distinct from records retained only in Git history. |

The graph stores hierarchy in `children`, dependencies in `blocks`, evidence in
`attachments`, and semantic links in `relates`. Missions are not hierarchy
parents. Epics have direct ordinary issue children; standalone issues remain
valid. Mission membership uses `advances`, not parent fields.

Different branches may contain different records and statuses. Cross-branch
summaries name their branch and never overlay them onto the current checkout.
Merge-time facts come from Git, not a second stored integration state.

Historical terms such as review room, provider terminal action, activity file,
SQLite domain cache, and Mission Control describe retired designs. They are
not aliases for current concepts.
