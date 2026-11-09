---
status: draft
last_changed: "2026-11-08: correct what PR CI tests and the merged-state fallback, per planning review"
---

# Why

Every change to this repository reaches master through a routine agents run by
hand: push the branch, open a pull request, wait for CI, `atelier merge`,
`atelier land`, push the base. The CI gate is `scripts/merge-ci-passed.sh`, a
`shell.check` that Atelier cannot see into, so it applies the merged-tree rule
to CI too and refuses whenever the base gained code, docs included. On
2026-11-08 atelier-3sk9 needed three syncs and CI reruns because other threads
committed strategy docs and a skill change to master while CI ran. Decision
atelier-vmq2 lets Atelier use a forge for transport and CI, so it can run the
routine itself and judge CI for what it proves.

# Outcome

In a repository whose policy names a forge, `atelier merge <id> --push` takes an
approved branch to the published base in one command: it pushes the branch,
makes sure it has a pull request, checks that CI passed on the branch head,
merges and pushes the base, and the pull request shows as merged. A base
that moved by docs or records alone needs no new CI run. `merge` says exactly
what it is waiting for when CI is pending or failed. This repository runs that
way, with `scripts/merge-ci-passed.sh` and the hand-run routine gone from its
policy, AGENTS.md and the skill. A repository whose policy names no forge
behaves as today.

# Success Criteria

- one-command-merge: in this repository, an approved item merges and publishes
  with `atelier merge <id> --push` alone, with no `gh` or `git push` by hand,
  while another thread commits a strategy doc to master after its CI run; shown
  by the item's transcript and the pull request's merged state.

# Shape

- **The forge carries branches and reports CI; review stays in the
  repository.** ADR 0019 Decision 6 stands: reviews, findings and approvals are
  records, and no forge state feeds a gate except CI results for a commit.
  ADR 0031 (atelier-vmq2) records this.
- **GitHub first, through `gh`.** Atelier shells out to `gh` as it does to
  `git`, so authentication is `gh auth`'s. Missing or signed-out `gh` refuses
  with the command that fixes it. The provider interface stays small: push a
  branch, ensure a pull request, read CI for a commit. Another forge comes only
  when a repository moves to one.
- **Policy names the forge.** `.atelier/workflow.yaml` gains the forge and the
  CI workflow it trusts, reviewed like any policy change. No forge, no network:
  a repository without one keeps today's behaviour.
- **CI is accepted for the branch head, not the merged tree.** A typed merge
  check, `ci.passed`, passes when the trusted workflow succeeded on a pull
  request run for a commit whose code matches the branch head. That run tested
  GitHub's merge of the head with the base as it was then
  (`refs/pull/<n>/merge`), not the tree Atelier merges, and the check says so
  instead of claiming the merged tree; a moved base needs no rerun. Local
  `shell.check`s still run on the merged tree, and CI on the base after the push
  is the backstop for the untested combination. A failing base CI run is shown, never a gate:
  gating on it would block the merge that fixes it.
- **Docs first.** The ADR and the product docs (`reviews-and-merge.md`,
  `workflow-configuration.md`, the merge section of the CLI help) describe the
  policy keys, `ci.passed` and the merge flow before the code exists, and are
  reviewed as the interface.
- **Waiting is the agent's choice.** `merge` refuses at once with CI's state and
  the run's link; `--wait` polls until CI finishes or a timeout passes, for
  harnesses that cannot sleep.
- When qualities conflict, an honest refusal that names its next step beats
  doing more automatically.

# Scope

In: the GitHub provider, `ci.passed`, branch push and pull requests from
`merge`, `merge --push` (absorbing atelier-fad5), `--wait`, this repository's
policy, AGENTS.md's Merge Policy and the skill's Atelier reference.

Out: reviews or comments on the forge, forge issues, Forgejo or GitLab,
repositories without CI (atelier-bp3s), and dropping the two local merge checks
CI repeats (garden digest atelier-j86z R9 decides that). Moving astromech's
policy is a backlog item once this repository runs the new flow.

Target environment: this host's agents (Claude Code, Codex, T3) with `gh`
signed in, and GitHub Actions CI on Linux and macOS.

# Authority

The orchestrator may reorder and split items, choose names for policy keys and
flags, and plan later increments within this shape. Changing what feeds a gate,
adding a second forge, or putting review on the forge needs Sumeet.

# Risks and Unknowns

- `gh`'s output and exit codes are its own and change between versions. If
  that bites, read the REST API through `gh api` with fixed fields.
- Tests need a fake `gh`. If one on `PATH` cannot cover the cases, the provider
  takes an injected command, as the test hooks already do for git failures.
- GitHub documents marking a pull request merged when its head commits become
  reachable from the base (an indirect merge), which Atelier's pushed merge
  commit does. `merge --push` checks the pull request reports merged after the
  push and, if it does not, says the base was published and the pull request is
  still open, never closing it: closed is not merged.
- Pushing over HTTPS needs credentials this host's `origin` lacks (agents push
  over SSH). If `gh auth setup-git` does not fix it, `merge` pushes to a
  configured remote URL.
