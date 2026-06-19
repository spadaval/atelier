---
created_at: "2026-06-19T20:51:18.803835394+00:00"
id: "atelier-fonb"
evidence_type: "transcript"
captured_at: "2026-06-19T20:51:18.795363777+00:00"
command: "rg -n 'Review commands must not|not by treating `atelier review` as workflow authority|Review merge is not workflow transition authority|Let Review Commands Move Issue Workflow|review commands transition' SPEC.md CONTEXT.md docs/product docs/adr"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-8cjb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-8cjb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "rg -n 'Review commands must not|not by treating `atelier review` as workflow authority|Review merge is not workflow transition authority|Let Review Commands Move Issue Workflow|review commands transition' SPEC.md CONTEXT.md docs/product docs/adr"
updated_at: "2026-06-19T20:51:21.636749246+00:00"
---

## Summary

rg -n 'Review commands must not|not by treating `atelier review` as workflow authority|Review merge is not workflow transition authority|Let Review Commands Move Issue Workflow|review commands transition' SPEC.md CONTEXT.md docs/product docs/adr

## Command

```console
rg -n 'Review commands must not|not by treating `atelier review` as workflow authority|Review merge is not workflow transition authority|Let Review Commands Move Issue Workflow|review commands transition' SPEC.md CONTEXT.md docs/product docs/adr
```

Exit status: 0

## Stdout

Bytes: 597
Truncated: no

```text
docs/product/cli-surface.md:520:routing. Review commands must not start, close, or otherwise transition Atelier
docs/product/cli-surface.md:525:not by treating `atelier review` as workflow authority. Review commands remain
docs/adr/0012-transition-effects-and-review-artifact-boundary.md:14:Without a durable boundary, implementation could make review commands transition
docs/adr/0012-transition-effects-and-review-artifact-boundary.md:77:### Let Review Commands Move Issue Workflow
docs/adr/0011-native-review-modes-and-room-authority.md:44:5. Review merge is not workflow transition authority.
```

## Stderr

Bytes: 0
Truncated: no

```text
```

