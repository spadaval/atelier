---
created_at: "2026-06-16T17:43:40.203313764+00:00"
id: "atelier-tixz"
evidence_type: "validation"
captured_at: "2026-06-16T17:43:39.833962589+00:00"
command: "bash -lc 'rg -n \"export --check|atelier export|lint/export|export check\" AGENTFACTORY.md docs/architecture/quality/validation.md docs/product/command-audit /root/.agents/skills/agent-factory'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-jezn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jezn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "remaining export guidance references are classified as storage-rendering or migration debug"
updated_at: "2026-06-16T17:43:46.152498031+00:00"
---

## Summary

remaining export guidance references are classified as storage-rendering or migration debug

## Command

```console
bash -lc 'rg -n "export --check|atelier export|lint/export|export check" AGENTFACTORY.md docs/architecture/quality/validation.md docs/product/command-audit /root/.agents/skills/agent-factory'
```

Exit status: 0

## Stdout

Bytes: 3513
Truncated: no

```text
/root/.agents/skills/agent-factory/standards/tracker.md:94:storage-rendering diagnostics such as `atelier export --check` only when the
docs/architecture/quality/validation.md:310:| Deterministic export/projection diagnostic | `atelier export --check`, only for storage-rendering, migration, or debug claims |
/root/.agents/skills/agent-factory/procedures/install.md:119:  - `atelier export`
/root/.agents/skills/agent-factory/procedures/install.md:120:  - `atelier export --check`
docs/product/command-audit/export.md:1:# `atelier export`
docs/product/command-audit/export.md:28:| hidden/admin `atelier export` | Admin/migration | Materialize deterministic renderer output during migration or debugging. | Temporary migration or test-only. |
docs/product/command-audit/export.md:29:| hidden/admin `atelier export --check` | Admin/debug | Check deterministic renderer/projection freshness during migration or targeted diagnostics. | Not normal health; route normal operators to `lint` and `doctor`. |
docs/product/command-audit/index.md:45:- [export check reference classification](export-check-reference-classification.md)
docs/product/command-audit/export-check-reference-classification.md:1:# `export --check` Reference Classification
docs/product/command-audit/export-check-reference-classification.md:3:This audit classifies remaining `export --check`, `atelier export`, and
docs/product/command-audit/export-check-reference-classification.md:11:rg -n 'export --check|atelier export|export/rebuild|lint/export|export check' \
docs/product/command-audit/export-check-reference-classification.md:22:| `docs/product/command-audit/category-review.md` | Migration/debug | The category table explicitly keeps export/rebuild outside normal workflow and classifies `export --check` as hidden/advanced. |
docs/product/command-audit/export-check-reference-classification.md:23:| `docs/architecture/quality/validation.md` deterministic export/projection diagnostic row | Storage-rendering-specific | The validation router names `export --check` only for storage-rendering, migration, or debug claims. |
docs/product/command-audit/export-check-reference-classification.md:25:| `/root/.agents/skills/agent-factory/standards/tracker.md` mission closeout caveat | Storage-rendering-specific | The tracker standard says to use `export --check` only when the mission changes deterministic export, projection freshness, or migration/debug surfaces. |
docs/product/command-audit/export-check-reference-classification.md:37:`export --check` proof by default. Use explicit export diagnostics only when an
docs/product/command-audit/category-review.md:11:| Hidden debug diagnostics | hidden `workflow check`, hidden `diagnostics slow`, hidden/advanced `export --check`, hidden/advanced `rebuild` used as a projection probe | `lint`, `doctor`, `mission status`, `status` | Debug diagnostics may expose raw policy, telemetry, projection, or deterministic-renderer mechanics. They must not be normal next actions or automation contracts for selecting work. |
docs/product/command-audit/category-review.md:12:| Temporary migration | `init --import-beads`, hidden/manual `import-beads`, hidden/admin `export` for deterministic renderer testing during migration | backup `import`, `export --format json|markdown`, routine `export --check` handoff checks | Migration commands bridge inherited state or test deterministic renderers while the Markdown-first store stabilizes. They need a cleanup owner instead of compatibility promises. |
```

## Stderr

Bytes: 0
Truncated: no

```text
```
