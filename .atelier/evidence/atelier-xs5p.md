---
created_at: "2026-07-06T18:33:15.767655169+00:00"
id: "atelier-xs5p"
evidence_type: "validation"
captured_at: "2026-07-06T18:33:15.578585987+00:00"
command: "bash -lc 'set -euo pipefail; rg -n \"atelier review open/show/submit/resolve/merge\" docs/product/cli-surface.md docs/adr/0011-native-review-modes-and-room-authority.md; rg -n \"Routine creation derives title, body, source branch, target branch, issue owner, role context\" docs/product/cli-surface.md; rg -n \"review open --existing\" docs/adr/0011-native-review-modes-and-room-authority.md docs/product/workflow-configuration.md docs/product/command-audit/review.md; rg -n \"Fold .*review (link|status|comments|comment|approve|request-changes)\" docs/product/command-audit/review.md; rg -n \"Review command role attribution is status-derived by default\" CONTEXT.md; if rg -n \"atelier review (status|comments|link|comment|approve|request-changes)\" PRODUCT_INTENT.md CONTEXT.md docs AGENTS.md .agents --glob \"*.md\" --glob \"*.yaml\" --glob \"!docs/product/command-audit/**\" --glob \"!docs/adr/0011-native-review-modes-and-room-authority.md\"; then echo \"active stale review command guidance found\"; exit 1; fi; echo \"no active stale review command guidance\"'"
exit_status: "1"
target:
  kind: "issue"
  id: "atelier-ye11"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ye11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent docs/help contract audit: product CLI map, complexity decision, ADR 0011, workflow policy, context, and intent agree on derived open, collapsed show/submit, distinct resolve/merge, and no active stale command guidance."
updated_at: "2026-07-06T18:33:19.327710863+00:00"
---

## Summary

Independent docs/help contract audit: product CLI map, complexity decision, ADR 0011, workflow policy, context, and intent agree on derived open, collapsed show/submit, distinct resolve/merge, and no active stale command guidance.

## Command

```console
bash -lc 'set -euo pipefail; rg -n "atelier review open/show/submit/resolve/merge" docs/product/cli-surface.md docs/adr/0011-native-review-modes-and-room-authority.md; rg -n "Routine creation derives title, body, source branch, target branch, issue owner, role context" docs/product/cli-surface.md; rg -n "review open --existing" docs/adr/0011-native-review-modes-and-room-authority.md docs/product/workflow-configuration.md docs/product/command-audit/review.md; rg -n "Fold .*review (link|status|comments|comment|approve|request-changes)" docs/product/command-audit/review.md; rg -n "Review command role attribution is status-derived by default" CONTEXT.md; if rg -n "atelier review (status|comments|link|comment|approve|request-changes)" PRODUCT_INTENT.md CONTEXT.md docs AGENTS.md .agents --glob "*.md" --glob "*.yaml" --glob "!docs/product/command-audit/**" --glob "!docs/adr/0011-native-review-modes-and-room-authority.md"; then echo "active stale review command guidance found"; exit 1; fi; echo "no active stale review command guidance"'
```

Exit status: 1

## Stdout

Bytes: 191
Truncated: no

```text
docs/adr/0011-native-review-modes-and-room-authority.md:27:   `atelier review open/show/submit/resolve/merge`
docs/product/cli-surface.md:47:- `atelier review open/show/submit/resolve/merge`
```

## Stderr

Bytes: 0
Truncated: no

```text
```
