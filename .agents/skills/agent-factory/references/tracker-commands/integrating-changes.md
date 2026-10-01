# Integrating Changes

Use this reference to connect tracked work with workspaces, branches, commits,
reviews, and integration state. Repository workflow policy remains authoritative.

## Atelier

- Use `atelier start <id>` to prepare the branch owner and start work.
- Commit records with the implementation; Atelier never creates commits.
- Record approvals, findings, dispositions, and explicit trivial exemptions
  through `atelier review record|approve|finding|trivial`, with an explicit
  `--identity` and summary. Append `review disposition` for each finding;
  FIX NOW needs a resolution commit and renewed independent approval, DEFER a
  follow-up issue, and DEFER/NO ACTION independent acceptance. Reviews are
  immutable in-repo Markdown events.
- Commit the done claim on the work branch. `atelier merge <id>` checks the
  base branch's policy, fresh evidence, independent approval or exemption,
  finding dispositions, and blockers, then fast-forwards without tracker writes.
- Code, policy, acceptance, and other material changes stale approval. Allowed
  additions of evidence/review records and in-scope status-only edits can follow
  approval; inspect exact freshness rules in repository docs.
- Use `atelier merge <id> --check` to inspect gates without integrating. Source
  and base inputs must be clean committed snapshots. Mandatory read-only checks
  belong in base policy `merge_checks`; merge disables Git hooks and deletes
  the integrated local branch. A separate source worktree is left detached for
  explicit cleanup.
- Use `atelier land [id]` before handoff; active work may remain open with an
  honest handoff, while done claims satisfy their applicable gates.
- Use `atelier prune` for safe cleanup and
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
