# Command Category Review

This review artifact maps the target command categories to examples and
excluded non-examples. It is intentionally about product placement, not current
implementation visibility.

| Category | Belongs here | Excluded non-examples | Review note |
| --- | --- | --- | --- |
| Normal workflow | `status`, `issue show`, `issue transition`, `work queue`, `work mission`, `evidence record`, `review show`, `check` | `export`, `rebuild`, `workflow check`, `diagnostics slow`, `import-beads`, destructive `maintenance delete`, provider setup commands | Normal commands answer operator questions in domain terms and may be cited for ordinary handoff, validation, committed-state health, and terminal readiness. |
| Admin maintenance | `init`, `check`, `check --fix`, `prune`, `prune --apply` | `issue show <objective-id>`, `issue transition`, hidden `workflow check`, hidden `diagnostics slow` | Admin commands configure or repair Atelier itself and clean explicitly supported artifacts. Hidden or destructive recovery commands are routed only when needed. |
| Hidden debug diagnostics | hidden `workflow check`, hidden `diagnostics slow`, hidden/advanced `export --check`, hidden/advanced `rebuild` used as a projection probe | `check`, `issue show <objective-id>`, `status` | Debug diagnostics may expose raw policy, telemetry, projection, or deterministic-renderer mechanics. They must not be normal next actions or automation contracts for selecting work. |
| Temporary migration | `init --import-beads`, hidden/manual `import-beads`, hidden advanced `export` for deterministic renderer testing during migration | backup `import`, `export --format json|markdown`, routine handoff checks | Migration commands bridge inherited state or test deterministic renderers while the Markdown-first store stabilizes. They need a cleanup owner instead of compatibility promises. |

Audited low-level surfaces:

| Command family | Classification | Replacement or boundary | Follow-up |
| --- | --- | --- | --- |
| `rebuild` | Hide/admin-frame | Keep as an advanced projection diagnostic. Admin local repair is `check --fix`; ordinary proof uses `check` plus the domain command being retried. | Covered by `atelier-a7gd`; no new issue. |
| `workflow check` | Hide/admin-frame | Keep as raw workflow-policy debugging. Normal readiness uses `issue transition`, `issue show <objective-id>`, and `check`. | No new issue. |
| `diagnostics slow` | Hide/admin-frame | Keep as local-only telemetry. It must not become workflow state or a normal automation contract. | No new issue. |
| `import-beads` | Hide/migration-only | Keep as explicit predecessor import escape hatch. Normal setup uses `init`, with `init --import-beads` only for intentional migration. | No new issue. |
| `maintenance delete` | Hide or remove | Destructive record surgery is over budget as a normal visible surface. Keep only as explicitly routed recovery if it remains necessary. | Needs budget verdict before being taught. |
| `prune` | Keep visible admin | Keep dry-run by default. `--apply` may remove only cleanup classes with implemented retention contracts from the retention policy. | No new issue. |
| `bundle` | Keep visible manager/orchestrator | Keep as the reviewed bulk record creation surface. It replaces shell loops over issue/mission/evidence mutation commands, not normal single-record editing. | Implementation ownership is now `commands::bundle`. |
| `review` | Keep visible workflow/review | Keep review artifact management visible, but static guides should not decide when a review artifact is required. Lifecycle/status output owns that route. | Refine manual `review open` fields. |
| `forgejo` | Hide/provider setup | Provider-specific setup belongs outside normal review loops. Keep only as admin/provider diagnostics or fold behind provider-neutral review/admin guidance. | Needs budget verdict before being taught. |
| `mission` | Removed visible namespace | Objective coordination uses mission-typed issue records. Create, list, show, status, note, link, block, and close behavior belongs to `issue` forms. | Removed by mission-collapse work. |
| `graph` | Removed visible namespace | Relationship and hierarchy questions are answered from `issue show`, `issue show <objective-id>`, and blocker/evidence views. `graph tree` was duplicative; `graph impact` moved into issue detail/status. | Removed by `atelier-39um`. |
| `branch` | Hide/advanced recovery | Routine branch setup and merge stay owned by issue start/close transitions and close branch-policy integration. Manual branch recovery is routed only from failed transitions. | Cross-reference: branch policy epic `atelier-5sjm`. |
| `worktree` | Remove/defer visible workspace management | Strip the root command pending redesign. Current work should be visible through issue and status views; workspace isolation can use plain Git outside Atelier until a clearer product contract exists. | Tracked by `atelier-7eio`. |

Boundary decisions:

- `.atelier/` canonical Markdown is the durable source of truth. Runtime,
  projection, diagnostic, lock, and cache state is ignored checkout state.
- Normal commands refresh projections safely when possible and report stale
  derived state as an operator-facing health problem.
- `check` and `check --fix` own explicit ignored runtime/cache/projection
  inspection and repair and must not edit tracked canonical records.
- If retained, `export` is hidden advanced migration or deterministic-renderer
  testing. It is not a normal health, validation, handoff, or terminal-readiness command.
- The adjacent surfaces audited by `atelier-1xmi` now have concrete placement:
  keep hidden advanced for low-level diagnostics and migration, keep visible admin
  for destructive or owner-branch recovery commands, and defer no additional
  cleanup issues from this pass.
