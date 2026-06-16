---
created_at: "2026-06-16T17:42:05.764467350+00:00"
id: "atelier-bsjp"
evidence_type: "validation"
captured_at: "2026-06-16T17:42:05.576084597+00:00"
command: "bash -lc 'rg -n \"atelier export|export --check|Refresh canonical export|tracker export\" crates/atelier-cli/src crates/atelier-app/src docs/product docs/architecture/quality AGENTFACTORY.md .atelier/issues/atelier-vuqb.md'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-vuqb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vuqb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "remaining export references are issue text, historical notes, migration/debug docs, or hidden-command labels"
updated_at: "2026-06-16T17:42:09.185039219+00:00"
---

## Summary

remaining export references are issue text, historical notes, migration/debug docs, or hidden-command labels

## Command

```console
bash -lc 'rg -n "atelier export|export --check|Refresh canonical export|tracker export" crates/atelier-cli/src crates/atelier-app/src docs/product docs/architecture/quality AGENTFACTORY.md .atelier/issues/atelier-vuqb.md'
```

Exit status: 0

## Stdout

Bytes: 7381
Truncated: yes

```text
.atelier/issues/atelier-vuqb.md:26:Remove `atelier export` from the ordinary product surface. Today the command can materialize canonical Markdown from a local projection, which is useful only for migration/debug work and dangerous as a general repair habit.
.atelier/issues/atelier-vuqb.md:30:- `atelier export` is absent from normal root help, role guides, common-command lists, normal recovery hints, validation recipes, and issue Evidence examples.
.atelier/issues/atelier-vuqb.md:38:- Help transcript proves normal root help and role help do not list `atelier export` as a routine command.
docs/product/cli-surface.md:51:| Hidden debug diagnostics | Callable implementation probes for raw workflow-policy detail, local telemetry, deterministic rendering, or projection debugging. They stay out of root help and ordinary role loops. Targeted diagnostics, tests, or migration notes may name them. | hidden `workflow check`, hidden `diagnostics slow`, hidden/advanced `export --check`, hidden/advanced `rebuild` when used as a projection probe | `lint`, `doctor`, `mission status`, `issue transition --options` |
docs/product/cli-surface.md:576:commands or explicitly by `doctor --fix`. `atelier export` and `atelier
crates/atelier-cli/src/main.rs:1606:                "export --check"
docs/architecture/quality/validation.md:310:| Deterministic export/projection diagnostic | `atelier export --check`, only for storage-rendering, migration, or debug claims |
docs/product/command-audit/export-check-reference-classification.md:1:# `export --check` Reference Classification
docs/product/command-audit/export-check-reference-classification.md:3:This audit classifies remaining `export --check`, `atelier export`, and
docs/product/command-audit/export-check-reference-classification.md:11:rg -n 'export --check|atelier export|export/rebuild|lint/export|export check' \
docs/product/command-audit/export-check-reference-classification.md:22:| `docs/product/command-audit/category-review.md` | Migration/debug | The category table explicitly keeps export/rebuild outside normal workflow and classifies `export --check` as hidden/advanced. |
docs/product/command-audit/export-check-reference-classification.md:23:| `docs/architecture/quality/validation.md` deterministic export/projection diagnostic row | Storage-rendering-specific | The validation router names `export --check` only for storage-rendering, migration, or debug claims. |
docs/product/command-audit/export-check-reference-classification.md:25:| `/root/.agents/skills/agent-factory/standards/tracker.md` mission closeout caveat | Storage-rendering-specific | The tracker standard says to use `export --check` only when the mission changes deterministic export, projection freshness, or migration/debug surfaces. |
docs/product/command-audit/export-check-reference-classification.md:37:`export --check` proof by default. Use explicit export diagnostics only when an
docs/architecture/quality/stabilization-closeout-inventory-2026-06-13.md:103:target/debug/atelier export --check
docs/architecture/quality/stabilization-closeout-inventory-2026-06-13.md:113:| `export --check` | Pass. Canonical export is current. |
docs/product/command-audit/category-review.md:11:| Hidden debug diagnostics | hidden `workflow check`, hidden `diagnostics slow`, hidden/advanced `export --check`, hidden/advanced `rebuild` used as a projection probe | `lint`, `doctor`, `mission status`, `status` | Debug diagnostics may expose raw policy, telemetry, projection, or deterministic-renderer mechanics. They must not be normal next actions or automation contracts for selecting work. |
docs/product/command-audit/category-review.md:12:| Temporary migration | `init --import-beads`, hidden/manual `import-beads`, hidden/admin `export` for deterministic renderer testing during migration | backup `import`, `export --format json|markdown`, routine `export --check` handoff checks | Migration commands bridge inherited state or test deterministic renderers while the Markdown-first store stabilizes. They need a cleanup owner instead of compatibility promises.
```

## Stderr

Bytes: 0
Truncated: no

```text
```
