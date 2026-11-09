# ADR 0031: A Forge Carries Branches and Reports CI

## Status

Accepted (2026-11-08, human-directed: Sumeet decided atelier-vmq2 in a design
discussion; mission atelier-1gb3, docs/strategies/forge-delivery.md).

Scopes [ADR 0019](0019-verified-repo-native-tracker.md) where it removed the
Forgejo provider: 0019 removed a review provider, and this ADR lets a forge
back in for transport and CI only. ADR 0019 Decision 6 stands. Also scopes the
Boundaries of [PRODUCT_INTENT.md](../../PRODUCT_INTENT.md), which this ADR
updates.

The decisions take effect as mission atelier-1gb3's items merge: the policy
keys and `ci.passed` with atelier-7tul, and `merge --push` and `--wait` with the
items after it. Until then the docs describe them ahead of the code, and this
repository keeps its hand-run routine and `scripts/merge-ci-passed.sh`.

## Context

Every change to this repository reaches `master` through a routine agents run
by hand: push the branch, open a pull request, wait for CI, `atelier merge`,
`atelier land`, push the base. The CI gate is `scripts/merge-ci-passed.sh`, a
`shell.check`. Atelier cannot see what a shell check proves, so the script
applies the merged-tree rule to CI: it refuses whenever the base gained code
since the branch left it, docs and strategies included, and asks for a sync
and a new run. On 2026-11-08 atelier-3sk9 needed three syncs and CI reruns
because other threads committed strategy docs and a skill change to `master`
while CI ran.

A pull request's CI run does not test the tree Atelier merges either way. For
`pull_request` events GitHub runs the workflow on `refs/pull/<n>/merge`, its
own merge of the branch head with the base as it was when the run started. By
the time Atelier merges, the base may have moved again.

ADR 0019 removed the Forgejo provider and its review rooms to shed complexity,
not because Atelier should never reach a forge. What it protects is that
reviews, findings and approvals are records in the repository and the guards
decide from Git and records.

## Decision

1. **A forge carries branches and reports CI; review stays in the
   repository.** When a repository's policy names a forge, Atelier may push
   work branches and the base, make sure a branch has a pull request, and read
   CI results for a commit. Reviews, findings and approvals stay Markdown
   records (ADR 0019 Decision 6). No forge state feeds a gate except a CI
   result for a commit. Pull request reviews, comments and forge issues are
   not read.

2. **GitHub first, through `gh`.** Atelier runs `gh` as it runs `git`, so
   authentication is whatever `gh auth` holds, and pushes go through `git`
   with git's own credentials. A missing or signed-out `gh` refuses with the
   command that fixes it. The provider interface is three operations: push a
   branch, ensure a pull request, read CI for a commit. Another forge is added
   only when a repository moves to one.

3. **Policy names the forge.** `.atelier/workflow.yaml` gains a `forge` block
   naming the provider, the remote and the CI workflow it trusts, reviewed like
   any policy change and read from the base branch's policy (ADR 0019
   Decision 5). A policy without a `forge` block gets none of these forge
   operations, and Atelier behaves as it did before this ADR: a `shell.check`
   that reaches the network, such as `scripts/merge-ci-passed.sh`, and explicit
   commands such as `land --push` keep their behaviour.

4. **CI is accepted for the branch head merged with an earlier base.** A typed
   merge check, `ci.passed`, passes when the trusted workflow succeeded on a
   pull request run for a commit whose code matches the branch head. That run
   tested the head merged with the base as it was then, and Atelier accepts it
   as the CI gate, so a base that moved since the run needs no new run. The
   check says what the run tested instead of claiming the merged tree. Local
   `shell.check`s still run on the tree Atelier merges. CI on the base after
   the push is the backstop for the combination nobody tested.

5. **A failing base run is shown, never a gate.** `merge` shows the trusted
   workflow's latest result on the base. Gating on it would block the merge
   that fixes it.

6. **Waiting is the agent's choice.** Without `--wait`, `merge` refuses at
   once with CI's state and the run's link; `--wait` polls until CI finishes
   or a timeout passes, for harnesses that cannot sleep.

The interface (policy keys, `ci.passed`'s messages, `merge --push` and
`--wait`) is in [workflow
configuration](../product/workflow-configuration.md#forge) and [reviews and
merge](../product/reviews-and-merge.md#ci-on-a-forge).

## Consequences

- `atelier merge <id> --push` takes an approved branch to the published base in
  one command, and a base that moved by docs or records alone no longer forces
  a sync and a CI rerun.
- The merged-tree rule now has a stated scope: it covers local `shell.check`s.
  A merge can publish a combination of head and base that no CI run tested,
  when the base gained code after the run started. Base CI catches it after
  the push, and the next change fixes it.
- `ci.passed` trusts the workflow file the base policy names, but a pull
  request run uses the branch's copy of that file. A branch that edits it is a
  code change, reviewed like any other.
- Atelier gains a network dependency in repositories that opt in, and depends
  on `gh`'s output, which changes between versions.
- `scripts/merge-ci-passed.sh` and the hand-run routine leave this
  repository's policy, AGENTS.md and the Agent Factory skill once the flow
  runs here.

## Alternatives Considered

- **Hooks.** Keep Atelier forge-free and let each repository plug scripts into
  merge. Rejected: Atelier cannot tell what a script proves, so it cannot
  explain refusals or decide when a base change needs a new CI run, and each
  repository copies and drifts its own script. ADRs 0012 and 0013 already
  rejected generic hooks.
- **No change.** Keep the routine and the script. Rejected for the cost
  atelier-3sk9 showed.
- **Require CI on the merged tree.** Run CI again on Atelier's merge commit
  before pushing. Rejected: it serializes every merge behind a full CI run,
  and the base run after the push gives the same signal one step later.
- **Gate on the base's CI.** Rejected: a red base would block the merge that
  makes it green.
