---
created_at: "2026-07-06T18:33:33.957624704+00:00"
id: "atelier-a5kn"
evidence_type: "validation"
captured_at: "2026-07-06T18:33:33.731144241+00:00"
command: "bash -lc 'set -euo pipefail; rg -n \"atelier review open/show/submit/resolve/merge\" docs/product/cli-surface.md docs/adr/0011-native-review-modes-and-room-authority.md; rg -n \"Routine creation\" docs/product/cli-surface.md; rg -n \"manual provider plumbing is not a public fallback\" docs/product/cli-surface.md; rg -n \"review open --existing\" docs/adr/0011-native-review-modes-and-room-authority.md docs/product/workflow-configuration.md docs/product/command-audit/review.md; rg -n \"Fold .*review (link|status|comments|comment|approve|request-changes)\" docs/product/command-audit/review.md; rg -n \"Review command role attribution is status-derived by default\" CONTEXT.md; if rg -n \"atelier review (status|comments|link|comment|approve|request-changes)\" PRODUCT_INTENT.md CONTEXT.md docs AGENTS.md .agents --glob \"*.md\" --glob \"*.yaml\" --glob \"!docs/product/command-audit/**\" --glob \"!docs/adr/0011-native-review-modes-and-room-authority.md\"; then echo \"active stale review command guidance found\"; exit 1; fi; echo \"no active stale review command guidance\"'"
exit_status: "0"
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
title: "Corrected independent docs/help contract audit using line-stable assertions after the prior probe stopped on Markdown wrapping."
updated_at: "2026-07-06T18:33:37.938453542+00:00"
---

## Summary

Corrected independent docs/help contract audit using line-stable assertions after the prior probe stopped on Markdown wrapping.

## Command

```console
bash -lc 'set -euo pipefail; rg -n "atelier review open/show/submit/resolve/merge" docs/product/cli-surface.md docs/adr/0011-native-review-modes-and-room-authority.md; rg -n "Routine creation" docs/product/cli-surface.md; rg -n "manual provider plumbing is not a public fallback" docs/product/cli-surface.md; rg -n "review open --existing" docs/adr/0011-native-review-modes-and-room-authority.md docs/product/workflow-configuration.md docs/product/command-audit/review.md; rg -n "Fold .*review (link|status|comments|comment|approve|request-changes)" docs/product/command-audit/review.md; rg -n "Review command role attribution is status-derived by default" CONTEXT.md; if rg -n "atelier review (status|comments|link|comment|approve|request-changes)" PRODUCT_INTENT.md CONTEXT.md docs AGENTS.md .agents --glob "*.md" --glob "*.yaml" --glob "!docs/product/command-audit/**" --glob "!docs/adr/0011-native-review-modes-and-room-authority.md"; then echo "active stale review command guidance found"; exit 1; fi; echo "no active stale review command guidance"'
```

Exit status: 0

## Stdout

Bytes: 1047
Truncated: no

```text
docs/adr/0011-native-review-modes-and-room-authority.md:27:   `atelier review open/show/submit/resolve/merge`
docs/product/cli-surface.md:47:- `atelier review open/show/submit/resolve/merge`
513:resolve review artifacts owned by the configured review mode. Routine creation
515:and provider/mode context; manual provider plumbing is not a public fallback.
docs/product/command-audit/review.md:45:- Fold `review link` into `review open --existing`.
docs/product/workflow-configuration.md:631:Provider-mode `review open --existing` inputs may accept a review number or a
docs/adr/0011-native-review-modes-and-room-authority.md:28:   operate in the configured mode. `review open --existing <url-or-number>` is
45:- Fold `review link` into `review open --existing`.
46:- Fold `review status` and `review comments` into `review show`; comments and
48:- Fold `review comment`, `review approve`, and `review request-changes` into
228:- Review command role attribution is status-derived by default. Mutating review
no active stale review command guidance
```

## Stderr

Bytes: 0
Truncated: no

```text
```
