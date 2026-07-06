# ADR 0007: Mission Workspaces And Epic Review Branches

## Status

Accepted. Updated by
[ADR 0016](0016-canonical-work-branches-and-mission-integration.md) for
canonical work branch names, branch base terminology, and opt-in mission
integration branches, and by
[ADR 0013](0013-workflow-transition-actions-and-branching.md) for current
transition-owned branch actions. Superseded command examples are retained below
only as non-normative history.

## Context

Atelier inherited Braid-style worktree ergonomics and an Agent Factory habit of
isolating mutating work by issue. That default reduces accidental overlap, but
it becomes expensive for coordinated missions: every child issue can create a
checkout, branch, review expectation, and cleanup obligation even when the
child is only one local slice of a coherent changeset.

The mission model now needs three boundaries that were previously easy to
collapse:

- workspace isolation for background agents;
- branch and review scope for coherent changes; and
- issue accountability for implementation slices and proof.

Treating all three as "one issue gets one worktree and one review" creates too
much tracker ceremony, fragments PR-equivalent review, and leaves cleanup work
spread across many branches and worktrees.

## Decision

Atelier uses this hierarchy as the target operating model:

1. Mission equals shared worktree or background workspace.
   A mission normally owns one shared worktree or equivalent checkout for the
   coordinated work under that objective.

2. Epic equals branch and review boundary.
   An epic normally owns one reviewable branch or PR-equivalent changeset under
   the mission workspace. Epic closeout maps child issue proof to the parent
   outcome and records review or validation judgment for the branch.

3. Issue equals implementation slice.
   Ordinary child issues execute on the parent epic branch and close with the
   local proof named by their `Evidence` section. They do not require an
   independent review by default.

4. Branch lifecycle is owned by issue transitions and configured actions.
   Routine workers inspect `atelier issue transition <id>` and execute the
   named transition selected by repository workflow. Transition actions prepare
   or integrate the canonical work branch and its recorded branch base. Child
   issue transitions stay on the owning epic branch when policy assigns one;
   standalone issue, epic, and mission integration behavior exists only when
   declared by workflow validators and actions.

5. Squash merge is the default integration strategy.
   Repositories may configure merge commit or fast-forward-only alternatives,
   while work branch names use the canonical `<issue_type>/<issue_id>` form
   from ADR 0016. Close must be failure-atomic: a failed tracker commit, merge,
   or required push must not leave the item closed in the integration branch.

6. Per-issue worktrees and per-issue branches are exceptional isolation.
   Use them only when the assignment or risk justifies containment: dirty
   experiments, destructive migration trials, cross-epic conflicts, high-risk
   validation, or explicitly isolated review work.

7. Independent validation remains risk-based.
   Review and validation move to the epic by default, but issue-level
   independent validation is still required for process-policy changes, public
   command/API contracts, migrations, persistence/workflow changes,
   Agent Factory process changes, stale-test risk, and any issue whose
   `Evidence` section or assignment explicitly requires another reviewer.

## Historical Command Examples (Non-Normative)

The original accepted text taught `atelier start <id>` for branch preparation
and `atelier issue close <id>` for close-state integration. Those removed paths
are preserved here only to explain the decision's evolution. Current routing is
`atelier issue transition <id>` followed by the repository-defined transition;
workflow actions own branch preparation, commits, integration, and push policy.
Likewise, references to mission or issue worktrees in the rationale describe
Git workspace isolation, not a live `atelier worktree` command.

## Alternatives Considered

### Keep Per-Issue Worktrees And Reviews As The Default

This maximizes isolation and keeps old Agent Factory guidance simple. It was
rejected because it makes every implementation slice carry branch, worktree,
review, and cleanup costs even when a single epic branch is the real reviewable
unit.

### Use One Mission Branch For All Work

This minimizes branch management. It was rejected because a mission can contain
multiple coherent epics that need separate review, sequencing, or rollback.
One branch per mission makes PR-equivalent review too broad.

### Require Review Only At Mission Closeout

This minimizes mid-mission review gates. It was rejected because mission
closeout is too late to find defects in a large branch set. Epic review keeps
the review boundary close to the coherent changeset while still avoiding
review ceremony for every ordinary child issue.

### Let Every Assignment Choose Its Own Boundary

This is flexible but makes status, cleanup, and closeout unpredictable. It was
rejected as the default because future agents need durable rules for deciding
where work should happen and where proof should be reviewed.

## Consequences

- Product docs, validation guidance, command help, and Agent Factory bindings
  must stop teaching per-issue worktrees as the normal mutating-subagent
  default.
- Product docs, command help, and Agent Factory bindings must teach
  transition-owned branch preparation through `atelier issue transition <id>`
  rather than routine pre-work calls to explicit branch helpers.
- Workflow policy should allow ordinary implementation issues to close with
  local proof and move review gates to epics, validation issues, closeout
  issues, or explicitly risk-escalated issue types.
- Mission status and closeout need to show the mission workspace, epic branch
  state, child issue proof, and parent review or validation gaps separately.
- Epic closeout becomes the normal review artifact for grouped implementation
  work. It must map child issue proof to epic outcomes and record residual
  risks.
- Advanced recovery commands such as `atelier branch for-epic` remain internal,
  diagnostic, or advanced repair surfaces for inspecting or recovering owner
  branch state.
- Per-issue isolation remains available but must be explicit, justified, and
  visible in handoff notes or assignment context.

## Cleanup Implications

- Existing issue worktrees and `codex/atelier-*` branches need classification
  before removal or folding into mission/epic state.
- Clean per-issue worktrees that contain only merged or obsolete work can be
  removed after validation.
- Dirty or unmerged issue worktrees must be preserved, folded into the right
  mission worktree and epic branch, or assigned explicit cleanup owners.
- Stale docs, help text, workflow policy, tests, and Agent Factory guidance
  that imply "every issue gets a worktree" or "every ordinary implementation
  issue needs independent review" must be removed or classified as exceptional
  isolation.
