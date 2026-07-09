---
created_at: "2026-07-09T16:40:01.419935995+00:00"
id: "atelier-06di"
evidence_type: "validation"
captured_at: "2026-07-09T16:40:01.236309500+00:00"
command: "bash -lc 'test \"$(git rev-parse HEAD)\" = efddfa3f97074ec49ac80dd5ef567178b8d8a261; test \"$(git diff --numstat efddfa3f^ efddfa3f -- PRODUCT_INTENT.md docs/product/workflow-configuration.md)\" = $'\"'\"'0\\t1\\tPRODUCT_INTENT.md\\n0\\t1\\tdocs/product/workflow-configuration.md'\"'\"'; ! sed -n '\"'\"'333,376p'\"'\"' PRODUCT_INTENT.md | rg '\"'\"'evidence\\.attached'\"'\"'; ! sed -n '\"'\"'151,205p'\"'\"' docs/product/workflow-configuration.md | rg '\"'\"'evidence\\.attached'\"'\"'; rg -n '\"'\"'Direct mission evidence|second direct mission evidence|accountable child'\"'\"' PRODUCT_INTENT.md docs/architecture/quality/validation.md'"
exit_status: "0"
agent_identity: "agent-factory.validate"
target:
  kind: "issue"
  id: "atelier-2uim"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2uim"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'test \"$(git rev-parse HEAD)\" = efddfa3f97074ec49ac80dd5ef567178b8d8a261; test \"$(git diff --numstat efddfa3f^ efddfa3f -- PRODUCT_INTENT.md docs/product/workflow-configuration.md)\" = $'\"'\"'0\\t1\\tPRODUCT_INTENT.md\\n0\\t1\\tdocs/product/workflow-configuration.md'\"'\"'; ! sed -n '\"'\"'333,376p'\"'\"' PRODUCT_INTENT.md | rg '\"'\"'evidence\\.attached'\"'\"'; ! sed -n '\"'\"'151,205p'\"'\"' docs/product/workflow-configuration.md | rg '\"'\"'evidence\\.attached'\"'\"'; rg -n '\"'\"'Direct mission evidence|second direct mission evidence|accountable child'\"'\"' PRODUCT_INTENT.md docs/architecture/quality/validation.md'"
updated_at: "2026-07-09T16:40:06.154547137+00:00"
---

## Summary

bash -lc 'test "$(git rev-parse HEAD)" = efddfa3f97074ec49ac80dd5ef567178b8d8a261; test "$(git diff --numstat efddfa3f^ efddfa3f -- PRODUCT_INTENT.md docs/product/workflow-configuration.md)" = $'"'"'0\t1\tPRODUCT_INTENT.md\n0\t1\tdocs/product/workflow-configuration.md'"'"'; ! sed -n '"'"'333,376p'"'"' PRODUCT_INTENT.md | rg '"'"'evidence\.attached'"'"'; ! sed -n '"'"'151,205p'"'"' docs/product/workflow-configuration.md | rg '"'"'evidence\.attached'"'"'; rg -n '"'"'Direct mission evidence|second direct mission evidence|accountable child'"'"' PRODUCT_INTENT.md docs/architecture/quality/validation.md'

## Command

```console
bash -lc 'test "$(git rev-parse HEAD)" = efddfa3f97074ec49ac80dd5ef567178b8d8a261; test "$(git diff --numstat efddfa3f^ efddfa3f -- PRODUCT_INTENT.md docs/product/workflow-configuration.md)" = $'"'"'0\t1\tPRODUCT_INTENT.md\n0\t1\tdocs/product/workflow-configuration.md'"'"'; ! sed -n '"'"'333,376p'"'"' PRODUCT_INTENT.md | rg '"'"'evidence\.attached'"'"'; ! sed -n '"'"'151,205p'"'"' docs/product/workflow-configuration.md | rg '"'"'evidence\.attached'"'"'; rg -n '"'"'Direct mission evidence|second direct mission evidence|accountable child'"'"' PRODUCT_INTENT.md docs/architecture/quality/validation.md'
```

Exit status: 0

## Stdout

Bytes: 711
Truncated: no

```text
docs/architecture/quality/validation.md:27:are proven by evidence on accountable child work plus validation work when
docs/architecture/quality/validation.md:252:`deferred`, or `not-applicable`, then cite the accountable child issue IDs and
docs/architecture/quality/validation.md:370:mission blockers are clear, required evidence is attached to accountable child
docs/architecture/quality/validation.md:374:not require a second direct mission evidence record when those child and health
docs/architecture/quality/validation.md:394:  accountable child work, configured transition gates passing, and clean Git
PRODUCT_INTENT.md:242:Direct mission evidence links are retained only for legacy imports or migration
```

## Stderr

Bytes: 0
Truncated: no

```text
```
