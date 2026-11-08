# Workflow Configuration

`.atelier/workflow.yaml` is reviewed repository policy. It declares issue types,
statuses and their categories, permitted transitions and their validators, the
base branch, and the merge checks. Agent Factory owns process; this file must not become an agent
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

`atelier init` runs in a Git repository, which may have no commits yet;
outside one it refuses with `init_no_git` and says to run `git init` first,
writing nothing. A new policy names the base branch init detects: origin's
HEAD branch, else whichever one of `main` and `master` exists, else the
current branch, born or not. `init` prints its choice (`Base branch: master
(from the current branch)`); a detached HEAD that no other rule settles is
refused with `init_base_unknown` before anything is written. The starter
policy then merges as written (atelier-l457).

`init` ends by listing what it wrote and the next steps: read the generated
policy, commit the install on the base with `git add .atelier .gitignore &&
git commit -m "Install Atelier"` (it predates the gates, so it needs no
review), then create the first issue. Until that commit, `status`, `land`,
`doctor` and the Stop hook say there are no commits yet and name it; `check`
passes.

The commit that runs `atelier init` and imports the first records comes before
any policy on the base branch, so `atelier merge` has nothing to gate it with.
Commit it in one of two ways:

- **On the base branch.** Run `atelier init` there, read the generated
  policy, and commit, as `init` prints. `land` treats the first commit that
  adds the policy as the install and gates every commit after it.
- **On a branch, through the repository's own review.** Run `atelier init`
  and any bundle import on a branch such as `chore/atelier-tracker`. While no
  copy of the base has a workflow policy, `atelier land --install` on that
  branch passes with a note that names this path; plain `land` refuses with
  `land_install_unconfirmed` and points to the flag, because an install is
  stated, not inferred from the branch's own policy (atelier-bibd). Merge it with
  the repository's usual review as one merge commit (merge the pull request on
  the host and pull, or `git merge --no-ff`), and push. On the base, the merge
  that brings the policy in counts as the install, together with the branch
  commits it merges. A fast-forward merge leaves no such commit, so the
  branch's later commits are judged one by one and refused.

Once the pushed base has the policy, the gates apply: start work with
`atelier start`, integrate with `atelier merge`, and commit only records and
mission strategies on the base directly.

## Status and transitions

Each issue type selects a workflow with an initial status, terminal statuses,
and named transitions. A transition declares its accepted source statuses,
target status, optional description, required fields, and read-only validators.
The one required field is `close_reason`, a non-empty Report, which only a
done-category transition may require; an older policy's report parts
(`report.deviations`, `report.decisions`, `report.surprises`,
`report.open_items`) are read as `close_reason` (see the [work
model](work-model.md)). Categories support queries; the exact status remains
the stored lifecycle state.

The starter policy has draft objectives, someday `backlog` work, todo
executable work, active `in_progress`, and terminal `done`, `superseded` or
`declined`.
`backlog` is for work that is recorded but not near-term: it shares the todo
category but has no `start` transition, so `work ready` leaves it out. `defer`
parks todo work in `backlog`, `plan` brings it back to `todo`, `supersede`
retires it once it is obsolete or replaced, and `decline`, from `backlog`,
`todo` or `in_progress` with a Report, records that real work will not be
done (atelier-stnv). The `work` workflow has no `close`: `atelier merge` marks work
done in the merge commit that integrates it, which is how work reaches its
first done status ([ADR 0028](../adr/0028-simpler-lifecycle.md), decision
2). The objective workflow's `close` describes when a mission closes: once
all its work is merged and its validation, if it has one, is done. The
starter has no `blocked` or `review` status: a blocker is a `blocked_by`
link, which `work blocked` and `start` already read, and review is recorded
in review records, so work goes from `in_progress` straight to `done` when it
merges (atelier-bqbz). Policies from earlier
starters that declare `blocked` and `review`, with their `block`,
`request_review` and `revise` transitions, keep working unchanged; [moving
an existing policy](#moving-an-existing-policy) says how a repository adopts
the smaller set. Its `decision`
workflow has no start: `decide` marks a decision done once the
`decision.recorded` validator finds its Decision section. Repositories initialized
before the decision type existed can copy its `issue_types` entry and
`decision` workflow from a fresh `atelier init`. Loose ends and validations
share the `on_base` workflow, which has no start either: `done` takes one from
`todo` to `done` once nothing blocks it, and requires a Report, a one-line
result; `defer` parks one in `backlog` and `plan` brings it back, as for work;
`supersede` retires it and `decline`, from `backlog` or `todo`, declines it. Any item whose workflow has no `start`
transition takes no branch, review or merge: `atelier start` and `atelier
merge` refuse it and name its done transition, and `land` accepts it marked
done on the base, as it does a mission closed there (atelier-8bh4). This is
judged by workflow, so a policy that keeps `validation` in `work` starts
validations as before. A repository without `loose_end` adds it as [moving an
existing policy](#moving-an-existing-policy) says.
`atelier issue create --status backlog` files new work there directly. Create
accepts the workflow's initial status, or a todo-category status one
transition without validators away from it; a status behind validators or required fields,
such as an objective's `todo` behind `ready`, needs its transition. Bundle
`create` entries follow the same rule. Inspect actual policy and `atelier issue transition <id>` before executing a
transition; a todo category alone is not permission to start.

Transitions write the issue record after validators pass. They do not create
branches or commits, push, open review artifacts, or merge. `start` and `merge`
are explicit Git boundaries. A required Report is written in the record or
given with `--reason`.

A policy from before atelier-jnrh may still give work a `close`. It keeps
working: the transition still runs, and the done marks merge writes answer to
its required fields and validators.

## Moving an existing policy

A policy an earlier Atelier wrote keeps working, but it can strand items if
it changes in the wrong order: an item whose status its workflow no longer
has makes `atelier check` and every command that loads the records report
`workflow_issue_status_invalid`, including the transitions that would move
it. Move a policy to the starter's shape in this order:

1. **Move items out of `blocked` and `review`**, while the old policy is
   still in place: each goes to `todo`, or is superseded. The old policy's
   `defer` then `plan` take a `blocked` item to `todo`; no transition takes
   a `review` item there, so edit its record's `status`. `atelier check`
   judges either. When validations move to `on_base`, which has no
   `in_progress`, a validation still `in_progress` under the old `work`
   workflow moves to `todo` too.
2. **Switch the policy to the starter's shape**, copying each piece from the
   policy a fresh `atelier init` writes: no `close` on the `work` workflow,
   since `atelier merge` marks work done; no `blocked` or `review` status,
   nor the transitions that use them; and the `loose_end` issue type with
   the `on_base` workflow, applied to `loose_end` and `validation`, with
   `validation` taken out of `work`'s `applies_to`, since an issue type
   belongs to one workflow. `on_base` keeps `backlog`, so parked validations
   stay parked. A repository that keeps validations in `work` applies
   `on_base` to `loose_end` alone. An old `branch_policy.merge_strategy` can
   stay or go: a fresh init writes none, and every value, `fast_forward_only`
   and `squash` included, is read as a merge commit.
3. **Run `atelier migrate after-merge`**, which needs the `loose_end` type:
   its dry run shows the loose end each open After merge line becomes, and
   `--apply` writes them (see
   [validation](validation.md#work-after-the-merge)).

Then `atelier check` and `atelier doctor` should pass. `atelier migrate
dispositions` is optional: Atelier reads disposition events in the form
before [ADR 0025](../adr/0025-findings-carry-their-disposition.md) folded
into their findings, and the migration only rewrites them that way.

## Validators and merge checks

Every command validates the record files it loads, and refuses to write a
record that would fail a rule `atelier check` applies. `atelier check`
reports record and graph problems. Transition validators include parseable
intent, lint with no blocking findings (`lint.none_blocking`), and open
direct or transitive blockers where policy selects them.

Merge gates protect the scope merge marks done: a current independent
approval or a visible trivial exemption, a decision on each deferred or
dismissed finding and an approval since each `fix_now` finding, and the
absence of open blockers. No gate reads evidence receipts
([ADR 0028](../adr/0028-simpler-lifecycle.md) decision 5): an older policy
naming the `evidence.attached` validator, with any parameters, still parses,
and the name is ignored. Policy can add merge checks, read-only shell commands
run at merge, using this top-level YAML shape:

```yaml
branch_policy:
  base_branch: master
merge_checks:
  - shell.check:
      command: "git diff --check HEAD^ HEAD"
```

This is a fragment of the full workflow schema, not a complete policy.
`shell.check` is accepted only in `merge_checks`, never in transition validators.
It runs in a temporary checkout of the merge commit, the merged tree with the
done marks; a zero exit status passes and output explains the
result. Merge checks must preserve the pinned checkout and records. Git hooks
are disabled during integration and cleanup, so put mandatory merge checks in
this reviewed policy surface.

## Base authority

Merge integrates into the `branch_policy.base_branch` of the work branch's
committed policy, which the base branch's own committed policy must name too;
no recorded `branch_base` is read. Merge evaluates the base branch's committed
policy. Commit a valid policy on
the base before beginning governed work; without any valid base policy, merge
refuses. Editing the work branch's policy cannot loosen that branch's gate.
Invalid base policy emits visible diagnostics and falls back to its last valid
policy in first-parent history. The work branch's candidate policy must still
be valid; fallback does not excuse an invalid candidate. Ordinary commands can
read working policy and visibly fall back to committed history when it is
malformed. Fix policy through reviewed work.

`atelier merge` always integrates with a merge commit, the only commit
Atelier creates: a branch that merges cleanly into the current base merges as
one, after `merge_checks` pass on the merged tree in a temporary checkout that
reuses no build cache, and that commit marks the work done; see [reviews and
merge](reviews-and-merge.md). A base that moved on needs no catch-up merge
first. `branch_policy.merge_strategy` is optional: `merge_commit`, and an
older policy's `fast_forward_only` or `squash`, are all read as a merge commit,
and any other value is refused. A policy needs no `shell.check` to merge, but
without one, or CI on the merged tree, it merges a tree nobody ran.
Always merging gives up a linear history and the `merge_diverged` refusal
that `fast_forward_only` repositories had; in exchange no branch catches up
before it merges, which `fast_forward_only` forced on every open branch after
each merge (astromech hit `merge_diverged` about 56 times from 2026-10-15
to 2026-11-07; ADR 0028, decision 2). Rebase only before any review is
recorded: rebasing rewrites the commits review records pin.

Consult the workflow parser and command help for exact schema fields. See
[reviews and merge](reviews-and-merge.md) and
[record storage](../architecture/markdown-first-record-store.md) for the other
sides of this contract.
