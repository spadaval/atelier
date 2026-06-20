# Command Category Review

This review artifact maps the target command categories to examples and
excluded non-examples. It is intentionally about product placement, not current
implementation visibility.

| Category | Belongs here | Excluded non-examples | Review note |
| --- | --- | --- | --- |
| Normal workflow | `status`, `start`, `issue show`, `issue transition --options`, `issue close`, `mission status`, `evidence record`, `lint`, `worktree for-mission` | `doctor`, `export`, `rebuild`, `workflow check`, `diagnostics slow`, `import-beads`, destructive `maintenance delete` | Normal commands answer operator questions in domain terms and may be cited for ordinary handoff, validation, committed-state health, and terminal readiness. |
| Admin maintenance | `init`, `doctor`, `doctor --fix`, `maintenance delete ... --force`, `branch status`, `branch merge`, `worktree repair` | `mission status`, `issue transition --options`, hidden `workflow check`, hidden `diagnostics slow` | Admin commands configure or repair Atelier itself, recover owner branches/worktrees manually, or perform explicit destructive record surgery. |
| Hidden debug diagnostics | hidden `workflow check`, hidden `diagnostics slow`, hidden/advanced `export --check`, hidden/advanced `rebuild` used as a projection probe | `lint`, `doctor`, `mission status`, `status` | Debug diagnostics may expose raw policy, telemetry, projection, or deterministic-renderer mechanics. They must not be normal next actions or automation contracts for selecting work. |
| Temporary migration | `init --import-beads`, hidden/manual `import-beads`, hidden/admin `export` for deterministic renderer testing during migration | backup `import`, `export --format json|markdown`, routine handoff checks | Migration commands bridge inherited state or test deterministic renderers while the Markdown-first store stabilizes. They need a cleanup owner instead of compatibility promises. |

Audited low-level surfaces:

| Command family | Classification | Replacement or boundary | Follow-up |
| --- | --- | --- | --- |
| `rebuild` | Hide/admin-frame | Keep as an advanced projection diagnostic. Admin local repair is `doctor --fix`; ordinary proof uses `lint` plus the domain command being retried. | Covered by `atelier-a7gd`; no new issue. |
| `workflow check` | Hide/admin-frame | Keep as raw workflow-policy debugging. Normal readiness uses `issue transition --options`, `mission status`, and `lint`. | No new issue. |
| `diagnostics slow` | Hide/admin-frame | Keep as local-only telemetry. It must not become workflow state or a normal automation contract. | No new issue. |
| `import-beads` | Hide/migration-only | Keep as explicit predecessor import escape hatch. Normal setup uses `init`, with `init --import-beads` only for intentional migration. | No new issue. |
| `maintenance delete` | Keep visible admin | Keep under `maintenance` as explicit destructive record surgery with force/confirmation framing. Never list as routine next action. | No new issue. |
| `branch` | Keep visible advanced recovery | Keep for orchestrator recovery of epic owner branches. Routine branch setup and merge stay owned by `start` and close branch-policy integration. | Cross-reference: branch policy epic `atelier-5sjm`. |
| `worktree` | Keep visible advanced work management | Keep as real workspace management. `worktree for-mission` is the default manager/orchestrator setup path; issue worktrees are exceptional isolation; `worktree repair` is admin recovery. | No new issue. |

Boundary decisions:

- `.atelier/` canonical Markdown is the durable source of truth. Runtime,
  projection, diagnostic, lock, and cache state is ignored checkout state.
- Normal commands refresh projections safely when possible and report stale
  derived state as an operator-facing health problem.
- `doctor` and `doctor --fix` own explicit ignored runtime/cache/projection
  inspection and repair and must not edit tracked canonical records.
- If retained, `export` is hidden/admin migration or deterministic-renderer
  testing. It is not a normal health, validation, handoff, or terminal-readiness command.
- The adjacent surfaces audited by `atelier-1xmi` now have concrete placement:
  keep hidden/admin for low-level diagnostics and migration, keep visible admin
  for destructive or workspace recovery commands, and defer no additional
  cleanup issues from this pass.
