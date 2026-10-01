# Integrating Changes

Use this reference to connect tracked work with workspaces, branches, commits,
reviews, and integration state. Repository workflow policy remains authoritative.

## Atelier

- Use `atelier start <id>` to prepare the branch owner and start work.
- Commit records with the implementation; Atelier never creates commits.
- Record approvals, findings, dispositions, and explicit trivial exemptions
  through `atelier review`. Reviews are append-only in-repo Markdown.
- Commit the done claim on the work branch. `atelier merge <id>` checks the
  base branch's policy, fresh evidence, independent approval or exemption,
  finding dispositions, and blockers, then fast-forwards without tracker writes.
- Code, policy, acceptance, and other material changes stale approval. Allowed
  additions of evidence/review records and in-scope status-only edits can follow
  approval; inspect exact freshness rules in repository docs.
- Use `atelier land` before handoff. Use `atelier prune` for safe cleanup and
  ordinary Git for integration history. Pushing follows repository authority.

## GitHub Issues

- Use normal repository branch and worktree policy, and link pull requests to
  their accountable issues with GitHub closing or reference syntax as
  appropriate.
- Use `gh pr create`, `view`, `diff`, `checks`, `review`, and `merge` for pull
  request state. Do not merge merely because the command is available; follow
  the repository's review and integration policy.
- Record the branch, commit, pull request, checks, and residual dirty state in
  the issue handoff when they matter to the next agent.
