# Administration and Recovery

Use this reference for tracker setup, authentication, integrity checks, repair,
and cleanup. Do not use repair or maintenance commands as a normal work path.

## Atelier

- Use `atelier init` only when installing Atelier in the repository.
- Use `atelier check` for tracked-state and workflow health; repair canonical
  records through reviewed edits, then rerun the check. There is no cache repair.
- Use `atelier setup claude` for the supported SessionStart/Stop hooks. Its
  default executable is `atelier`; use `--binary <path>` for a local rebuilt
  binary. Setup preserves unrelated settings; commit shared hook configuration
  deliberately and restart Claude Code.
- `atelier check` stops at the first structural load error. Repair the named
  error and rerun; there is no structural-error aggregation promise.
- Use `atelier prune` to inspect cleanup candidates and `atelier prune --apply`
  only when cleanup is intended. Commit canonical deletions yourself.
- Recover terminal records from Git history; start diagnosis with command help.

## GitHub Issues

- Verify the target with `gh repo view` and authentication with `gh auth status`.
- Confirm Issues are enabled and that the actor can read or mutate the selected
  repository before planning work around GitHub Issues.
- Repository setup may include issue templates, labels, issue types, and project
  fields. Install only the conventions the repository selects for representing
  Agent Factory work.
- Diagnose permissions, API availability, rate limits, and CLI-version gaps
  before treating a failed tracker operation as a product or planning failure.
- For raw `gh api` calls, set `GH_HOST` explicitly from the git remote, for
  example `GH_HOST=<git-origin-host> gh api ...`: `gh api` does not reliably
  infer a GitHub Enterprise host from the checkout when multiple hosts are
  configured. Prefer the repository-aware high-level commands for ordinary
  operations.
