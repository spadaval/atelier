# Command Category Review

This review artifact maps the target command categories to examples and
excluded non-examples. It is intentionally about product placement, not current
implementation visibility.

| Category | Belongs here | Excluded non-examples | Review note |
| --- | --- | --- | --- |
| Normal workflow | `status`, `start`, `issue show`, `issue transition --options`, `issue close`, `mission status`, `evidence record`, `lint`, `doctor`, `worktree for-mission` | `export`, `rebuild`, `workflow check`, `diagnostics slow`, `import-beads`, destructive `maintenance delete` | Normal commands answer operator questions in domain terms and may be cited for ordinary handoff, validation, health, and closeout. |
| Admin maintenance | `init`, `doctor --fix`, `maintenance delete ... --force`, `branch status`, `branch merge`, `worktree repair` | `mission status`, `issue transition --options`, hidden `workflow check`, hidden `diagnostics slow` | Admin commands configure or repair Atelier itself, recover owner branches/worktrees manually, or perform explicit destructive record surgery. |
| Hidden debug diagnostics | hidden `workflow check`, hidden `diagnostics slow`, hidden/advanced `export --check`, hidden/advanced `rebuild` used as a projection probe | `lint`, `doctor`, `mission status`, `status` | Debug diagnostics may expose raw policy, telemetry, projection, or deterministic-renderer mechanics. They must not be normal next actions or automation contracts for selecting work. |
| Temporary migration | `init --import-beads`, hidden/manual `import-beads`, hidden/admin `export` for deterministic renderer testing during migration | backup `import`, `export --format json|markdown`, routine `export --check` handoff checks | Migration commands bridge inherited state or test deterministic renderers while the Markdown-first store stabilizes. They need a cleanup owner instead of compatibility promises. |

Boundary decisions:

- `.atelier/` canonical Markdown is the durable source of truth. Runtime,
  projection, diagnostic, lock, and cache state is ignored checkout state.
- Normal commands refresh projections safely when possible and report stale
  derived state as an operator-facing health problem.
- `doctor` owns local health reporting; `doctor --fix` owns explicit ignored
  runtime/cache/projection repair and must not edit tracked canonical records.
- If retained, `export` is hidden/admin migration or deterministic-renderer
  testing. It is not a normal health, validation, handoff, or closeout command.
- `rebuild`, `workflow check`, `diagnostics slow`, `import-beads`,
  `maintenance`, `branch`, and `worktree` remain the adjacent ambiguous surfaces
  that follow-up implementation and validation should confirm against this
  category contract.
