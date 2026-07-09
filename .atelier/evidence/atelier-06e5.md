---
created_at: "2026-07-06T18:13:58.746335404+00:00"
id: "atelier-06e5"
evidence_type: "validation"
captured_at: "2026-07-06T18:13:58.715101597+00:00"
command: "sh -c '! rg -n \"lint.*/.*doctor.*health|normal operators use .*lint|normal operators use .*doctor|repair uses .*doctor|predecessor imports use .*atelier import-beads|normal health uses lint|normal operator checks use lint|atelier branch (for-epic|status|merge)\" PRODUCT_INTENT.md AGENTS.md docs/spec/storage/export/rebuild/canonical-layout.md docs/product/command-audit/export-check-reference-classification.md docs/product/command-audit/export.md docs/product/cli-surface.md docs/product/work-model.md docs/architecture/quality/validation.md crates/atelier-cli/src/main.rs crates/atelier-cli/src/commands/man.rs crates/atelier-cli/src/commands/work.rs'"
exit_status: "1"
agent_identity: "independent-validator"
target:
  kind: "issue"
  id: "atelier-eqq6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-eqq6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "sh -c '! rg -n \"lint.*/.*doctor.*health|normal operators use .*lint|normal operators use .*doctor|repair uses .*doctor|predecessor imports use .*atelier import-beads|normal health uses lint|normal operator checks use lint|atelier branch (for-epic|status|merge)\" PRODUCT_INTENT.md AGENTS.md docs/spec/storage/export/rebuild/canonical-layout.md docs/product/command-audit/export-check-reference-classification.md docs/product/command-audit/export.md docs/product/cli-surface.md docs/product/work-model.md docs/architecture/quality/validation.md crates/atelier-cli/src/main.rs crates/atelier-cli/src/commands/man.rs crates/atelier-cli/src/commands/work.rs'"
updated_at: "2026-07-06T18:14:03.155889298+00:00"
---

## Summary

sh -c '! rg -n "lint.*/.*doctor.*health|normal operators use .*lint|normal operators use .*doctor|repair uses .*doctor|predecessor imports use .*atelier import-beads|normal health uses lint|normal operator checks use lint|atelier branch (for-epic|status|merge)" PRODUCT_INTENT.md AGENTS.md docs/spec/storage/export/rebuild/canonical-layout.md docs/product/command-audit/export-check-reference-classification.md docs/product/command-audit/export.md docs/product/cli-surface.md docs/product/work-model.md docs/architecture/quality/validation.md crates/atelier-cli/src/main.rs crates/atelier-cli/src/commands/man.rs crates/atelier-cli/src/commands/work.rs'

## Command

```console
sh -c '! rg -n "lint.*/.*doctor.*health|normal operators use .*lint|normal operators use .*doctor|repair uses .*doctor|predecessor imports use .*atelier import-beads|normal health uses lint|normal operator checks use lint|atelier branch (for-epic|status|merge)" PRODUCT_INTENT.md AGENTS.md docs/spec/storage/export/rebuild/canonical-layout.md docs/product/command-audit/export-check-reference-classification.md docs/product/command-audit/export.md docs/product/cli-surface.md docs/product/work-model.md docs/architecture/quality/validation.md crates/atelier-cli/src/main.rs crates/atelier-cli/src/commands/man.rs crates/atelier-cli/src/commands/work.rs'
```

Exit status: 1

## Stdout

Bytes: 95
Truncated: no

```text
PRODUCT_INTENT.md:679:`atelier check --fix`. Branch commands such as `atelier branch for-epic`
```

## Stderr

Bytes: 0
Truncated: no

```text
```
