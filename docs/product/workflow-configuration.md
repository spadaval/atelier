# Workflow Configuration

`.atelier/workflow.yaml` is reviewed repository policy. It declares issue types,
statuses and their categories, permitted transitions, branch base, and invariant
checks. Agent Factory owns process; this file must not become an agent
choreography or a way to evade review.

`.atelier/config.toml` contains project identity, the canonical state root,
prune defaults, and supported project settings. `project_slug` is the prefix of
every new record ID. `atelier init` derives it from the repository's name:
the origin remote's repository name, else the main worktree's directory (so a
linked worktree such as `project-feature` still gets `project`), else a bare
repository's name (`proj.git`, or `proj` for `proj/.bare`), else the
current directory. The Git sources count only when `init` runs at a
repository's top level, so a new project inside a tracked parent keeps its own
directory name. The name is lowercased, with other characters replaced by
single hyphens. `init` prints the slug it chose and where it came from; pass
`atelier init --slug <slug>` to choose another. Changing the slug later affects
only new IDs: existing records keep their IDs and stay valid. There is no review-provider
mode or global provider credential configuration.

## Installing Atelier

The commit that runs `atelier init` and imports the first records comes before
any policy on the base branch, so `atelier merge` has nothing to gate it with.
Land it in one of two supported ways:

- **On the base branch.** Run `atelier init` there, review the generated
  policy, and commit. `land` treats the first commit that adds the policy as
  the install and gates every commit after it.
- **On a branch, through the repository's own review.** Run `atelier init`
  and any bundle import on a branch such as `chore/atelier-tracker`. While no
  copy of the base has a workflow policy, `land` on that branch passes with a
  note that names this path instead of `land_work_untraceable`. Merge it with
  the repository's usual review as one merge commit (merge the pull request on
  the host and pull, or `git merge --no-ff`), and push. On the base, the merge
  that brings the policy in counts as the install, together with the branch
  commits it merges. A fast-forward merge leaves no such commit, so the
  branch's later commits are checked one by one and refused.

Once the pushed base has the policy, the gates apply: start work with
`atelier start`, integrate with `atelier merge`, and commit only records and
mission strategies on the base directly.

## Status and transitions

Each issue type selects a workflow with an initial status, terminal statuses,
and named transitions. A transition declares its accepted source statuses,
target status, optional description, required fields such as `close_reason`,
and read-only checks. Categories support queries; the exact status remains the
stored lifecycle state.

The starter policy has draft objectives, someday `backlog` work, todo
executable work, active `in_progress` and `review`, blocked work, and terminal
`done` or `superseded`. `backlog` is for work that is recorded but not
near-term: it shares the todo category but has no `start` transition, so
`work ready` leaves it out. `defer` parks todo or blocked work in `backlog`,
`plan` brings it back to `todo`, and `supersede` retires it. Its `decision`
workflow has no start: `decide` closes a decision once the
`decision.recorded` check finds its Decision section. Repositories initialized
before the decision type existed can copy its `issue_types` entry and
`decision` workflow from a fresh `atelier init`.
`atelier issue create --status backlog` files new work there directly. Create
accepts the workflow's initial status, or a todo-category status one
unchecked transition away from it; a status behind checks or required fields,
such as an objective's `todo` behind `ready`, needs its transition. Inspect actual policy and `atelier issue transition <id>` before executing a
transition; a todo category alone is not permission to start.

Transitions write the issue record after checks pass. They do not create
branches or commits, push, open review artifacts, or merge. `start` and `merge`
are explicit Git boundaries. Required close reasons are recorded with terminal
claims on the work branch.

## Invariant checks

Record-file validity is checked whenever a command loads the repository.
`atelier check` exposes record and graph problems. Transition checks include
parseable intent, lint with no blocking findings (`lint.none_blocking`), and
open direct or transitive blockers where policy selects them.

Merge gates protect terminal committed scope, fresh passing evidence,
independent current approval or a visible trivial exemption, finding
dispositions, and absence of open blockers. Policy can add read-only shell
checks at merge, using this top-level YAML shape:

```yaml
branch_policy:
  base_branch: master
  merge_strategy: fast_forward_only
merge_checks:
  - shell.check:
      command: "git diff --check HEAD^ HEAD"
```

This is a fragment of the full workflow schema, not a complete policy.
`shell.check` is accepted only in `merge_checks`, never in transition validators.
It runs in the work checkout; a zero exit status passes and output explains the
result. Checks must preserve the pinned checkout and records. Git hooks are
disabled during integration and cleanup, so put mandatory checks in this
reviewed policy surface.

## Base authority

Merge evaluates the base branch's committed policy. Commit a valid policy on
the base before beginning governed work; without any valid base policy, merge
refuses. Editing the work branch's policy cannot loosen that branch's gate.
Invalid base policy emits visible diagnostics and falls back to its last valid
policy in first-parent history. The work branch's candidate policy must still
be valid; fallback does not excuse an invalid candidate. Ordinary commands can
read working policy and visibly fall back to committed history when it is
malformed. Fix policy through reviewed work.

The supported gated integration strategy is `fast_forward_only`. Atelier never creates
commits. If the base has moved on, merge it into the work branch with ordinary
Git and renew any proof or approval made stale by the merge. Rebase only before
any review is recorded: rebasing rewrites the commits review records pin.

Consult the workflow parser and command help for exact schema fields. See
[reviews and merge](reviews-and-merge.md) and
[record storage](../architecture/markdown-first-record-store.md) for the other
sides of this contract.
