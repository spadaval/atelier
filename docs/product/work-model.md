# Work Model

Atelier parses tracked Markdown records on every command. Records are the source
of truth for current work; Git is the source of truth for branch ancestry,
integration, and history. See [storage](../architecture/markdown-first-record-store.md)
for the on-disk contract and [Agent Factory record templates](../../.agents/skills/agent-factory/references/tracker-records.md)
for authoring guidance.

## Records and relationships

Missions and epics are issue records with distinct `issue_type` values. A mission
links a strategy file and outcome or success criteria. An epic states an Outcome
and, before it becomes ready, a Proof route. Ordinary issues state Goal and
Done when, with optional Scope and Context. Notes and Close Reason carry dated
handoffs and terminal reasons.

Missions have no parent or children. An epic has no parent and can own direct
ordinary issue children. Ordinary issues may be standalone or children of an
epic; they cannot own children. Execution work links to missions through
`advances`. Native blockers represent sequencing, not membership or proof.
Evidence attachments represent proof; semantic links remain separate.

A draft epic is planning context. Refine its outcome, immediate children,
dependencies, and proof before making it ready. A todo-category status alone
does not establish readiness: ready work must expose a direct transition into
active work and have no open blockers.

## Ownership and transitions

A standalone non-mission issue owns its branch. An epic owns its children's
branch. The default branch name is `<issue_type>/<id>`; typed `workflow_branch`
metadata records an explicit `work_branch` and `branch_base` on the owner.
Missions coordinate work rather than own a work branch.

`atelier start <id>` resolves the owner, checks the target branch's records,
checks out or creates its work branch, records branch metadata, and applies the
available `start` transition. Commit those record changes with the work.

Statuses and permitted transitions come from `.atelier/workflow.yaml`.
`atelier issue transition <id>` shows options and blockers. Supplying a
transition changes the record after read-only checks. Transitions do not branch,
commit, push, open reviews, or merge.

## Completion

A done claim is a terminal status written and committed on the work branch.
Open dependencies prevent completion. Evidence must state what was checked,
against which commit, and whether it passed. Independent review examines the
implementation and the done claim together.

`atelier merge <id>` checks the base branch's policy and review gates, then
fast-forwards the base to the committed work. It creates no commit and changes
no records. The base therefore shows the item done together with its code.
See [reviews and merge](reviews-and-merge.md) for approval freshness and findings.

The done claim comes before review on purpose (atelier-9k7o, decided by
Sumeet 2026-10-15): the reviewer sees the claim together with the code. A
review that finds problems leaves the claim in place: the author fixes the
findings on the work branch, and the merge waits for fresh approval. Until the
merge, though, that branch says done for work the base does not have. So
`prime`, `status`, and the other-branch rows in `status` show a done owner as
`done on branch <work-branch>, not merged into <base>`, or as
`done, merged into <base>` once the base branch's own copy of the record is
done.

Branch-local state can differ until integration. Use labeled cross-branch
status for orientation; it does not rewrite the current checkout's graph.
Terminal records eventually leave the active tree through
[prune](retention-and-prune.md), with their history retained in Git.

## Decided changes

These decisions are accepted but not all built yet. The sections above
describe current behaviour until the named follow-up merges.

- **Parent issues replace epics** (atelier-9vq3, decided by Sumeet
  2026-10-15). Elsewhere "epic" usually means a theme, so adopters modelled
  themes as epics and got branches that never merged. The `epic` type goes
  away: any non-mission issue may have children, one level deep. The parent
  owns its children's branch and cannot close until they are terminal.
  Unknowns and Design become optional sections on any issue. Themes are
  labels, and missions keep the why. Existing epics are retyped and keep
  their links. Implementation: atelier-o6h9.
