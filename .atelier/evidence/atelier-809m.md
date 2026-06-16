---
created_at: "2026-06-16T17:51:17.922919744+00:00"
id: "atelier-809m"
evidence_type: "validation"
captured_at: "2026-06-16T17:51:17.543167338+00:00"
command: "bash -lc 'rg -n \"Audited low-level surfaces|Command family|rebuild|workflow check|diagnostics slow|import-beads|maintenance delete|branch|worktree\" docs/product/command-audit/category-review.md && target/debug/atelier rebuild --help >/tmp/atelier-m1r7-rebuild-help.txt && target/debug/atelier workflow --help >/tmp/atelier-m1r7-workflow-help.txt && target/debug/atelier diagnostics --help >/tmp/atelier-m1r7-diagnostics-help.txt'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-m1r7"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 3201
    summary: "9:| Normal workflow | `status`, `start`, `issue show`, `issue transition --options`, `issue close`, `mission status`, `evidence record`, `lint`, `doctor`, `worktree for-mission` | `export`, `rebuild`, `workflow check`, `diagnostics slow`, `import-beads`, destructive `maintenance delete` | Normal commands answer operator questions in domain terms and may be cited for ordinary handoff, validation, health, and closeout. |\n10:| Admin maintenance | `init`, `doctor --fix`, `maintenance delete ... --force`, `branch status`, `branch merge`, `worktree repair` | `mission status`, `issue transition --options`, hidden `workflow check`, hidden `diagnostics slow` | Admin commands configure or repair Atelier itself, recover owner branches/worktrees manually, or perform explicit destructive record surgery. |\n11:| Hidden debug diagnostics | hidden `workflow check`, hidden `diagnostics slow`, hidden/advanced `export --check`, hidden/advanced `rebuild` used as a projection probe | `lint`, `doctor`, `mission status`, `status` | Debug diagnostics may expose raw policy, telemetry, projection, or deterministic-renderer mechanics. They must not be normal next actions or automation contracts for selecting work. |\n12:| Temporary migration | `init --import-beads`, hidden/manual `import-beads`, hidden/admin `export` for deterministic renderer testing during migration | backup `import`, `export --format json|markdown`, routine `export --check` handoff checks | Migration commands bridge inherited state or test deterministic renderers while the Markdown-first store stabilizes. They need a cleanup owner instead of compatibility promises. |\n14:Audited low-level surfaces:\n16:| Command family | Classification | Replacement or boundary | Follow-up |\n18:| `rebuild` | Hide/admin-frame | Keep as an advanced projection diagnostic. Normal local repair is `doctor --fix`; normal health proof is `lint` plus `doctor`. | Covered by `atelier-a7gd`; no new issue. |\n19:| `workflow check` | Hide/admin-frame | Keep as raw workflow-policy debugging. Normal readiness uses `issue transition --options`, `mission status`, and `lint`. | No new issue. |\n20:| `diagnostics slow` | Hide/admin-frame | Keep as local-only telemetry. It must not become workflow state or a normal automation contract. | No new issue. |\n21:| `import-beads` | Hide/migration-only | Keep as explicit predecessor import escape hatch. Normal setup uses `init`, with `init --import-beads` only for intentional migration. | No new issue. |\n22:| `maintenance delete` | Keep visible admin | Keep under `maintenance` as explicit destructive record surgery with force/confirmation framing. Never list as routine next action. | No new issue. |\n23:| `branch` | Keep visible advanced recovery | Keep for orchestrator recovery of epic owner branches. Routine branch setup and merge stay owned by `start` and close lifecycle integration. | Cross-reference: branch lifecycle epic `atelier-5sjm`. |\n24:| `worktree` | Keep visible advanced work management | Keep as real workspace management. `worktree for-mission` is the default manager/orchestrator setup path; issue worktrees are exceptional isolation; `worktree repair` is admin recovery. | No new issue. |\n"
    truncated: false
  stderr:
    bytes: 0
    summary: ""
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-m1r7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "adjacent command classification doc lists each audited family"
updated_at: "2026-06-16T17:51:21.476415340+00:00"
---

adjacent command classification doc lists each audited family

Command: bash -lc 'rg -n "Audited low-level surfaces|Command family|rebuild|workflow check|diagnostics slow|import-beads|maintenance delete|branch|worktree" docs/product/command-audit/category-review.md && target/debug/atelier rebuild --help >/tmp/atelier-m1r7-rebuild-help.txt && target/debug/atelier workflow --help >/tmp/atelier-m1r7-workflow-help.txt && target/debug/atelier diagnostics --help >/tmp/atelier-m1r7-diagnostics-help.txt'
Exit status: 0

Stdout summary:
9:| Normal workflow | `status`, `start`, `issue show`, `issue transition --options`, `issue close`, `mission status`, `evidence record`, `lint`, `doctor`, `worktree for-mission` | `export`, `rebuild`, `workflow check`, `diagnostics slow`, `import-beads`, destructive `maintenance delete` | Normal commands answer operator questions in domain terms and may be cited for ordinary handoff, validation, health, and closeout. |
10:| Admin maintenance | `init`, `doctor --fix`, `maintenance delete ... --force`, `branch status`, `branch merge`, `worktree repair` | `mission status`, `issue transition --options`, hidden `workflow check`, hidden `diagnostics slow` | Admin commands configure or repair Atelier itself, recover owner branches/worktrees manually, or perform explicit destructive record surgery. |
11:| Hidden debug diagnostics | hidden `workflow check`, hidden `diagnostics slow`, hidden/advanced `export --check`, hidden/advanced `rebuild` used as a projection probe | `lint`, `doctor`, `mission status`, `status` | Debug diagnostics may expose raw policy, telemetry, projection, or deterministic-renderer mechanics. They must not be normal next actions or automation contracts for selecting work. |
12:| Temporary migration | `init --import-beads`, hidden/manual `import-beads`, hidden/admin `export` for deterministic renderer testing during migration | backup `import`, `export --format json|markdown`, routine `export --check` handoff checks | Migration commands bridge inherited state or test deterministic renderers while the Markdown-first store stabilizes. They need a cleanup owner instead of compatibility promises. |
14:Audited low-level surfaces:
16:| Command family | Classification | Replacement or boundary | Follow-up |
18:| `rebuild` | Hide/admin-frame | Keep as an advanced projection diagnostic. Normal local repair is `doctor --fix`; normal health proof is `lint` plus `doctor`. | Covered by `atelier-a7gd`; no new issue. |
19:| `workflow check` | Hide/admin-frame | Keep as raw workflow-policy debugging. Normal readiness uses `issue transition --options`, `mission status`, and `lint`. | No new issue. |
20:| `diagnostics slow` | Hide/admin-frame | Keep as local-only telemetry. It must not become workflow state or a normal automation contract. | No new issue. |
21:| `import-beads` | Hide/migration-only | Keep as explicit predecessor import escape hatch. Normal setup uses `init`, with `init --import-beads` only for intentional migration. | No new issue. |
22:| `maintenance delete` | Keep visible admin | Keep under `maintenance` as explicit destructive record surgery with force/confirmation framing. Never list as routine next action. | No new issue. |
23:| `branch` | Keep visible advanced recovery | Keep for orchestrator recovery of epic owner branches. Routine branch setup and merge stay owned by `start` and close lifecycle integration. | Cross-reference: branch lifecycle epic `atelier-5sjm`. |
24:| `worktree` | Keep visible advanced work management | Keep as real workspace management. `worktree for-mission` is the default manager/orchestrator setup path; issue worktrees are exceptional isolation; `worktree repair` is admin recovery. | No new issue. |

Stderr summary:
(none)

