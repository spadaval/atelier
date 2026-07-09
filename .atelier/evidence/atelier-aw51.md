---
created_at: "2026-07-09T16:25:47.660626032+00:00"
id: "atelier-aw51"
evidence_type: "test"
captured_at: "2026-07-09T16:25:46.993895034+00:00"
command: "bash -lc 'git diff --check -- '\"'\"'*.md'\"'\"' && atelier check atelier-wlk4 && rg -n '\"'\"'Mission And Issue-Set Authoring Standard|mission-authoring\\.md'\"'\"' docs .agents --glob '\"'\"'*.md'\"'\"''"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-wlk4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-wlk4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'git diff --check -- '\"'\"'*.md'\"'\"' && atelier check atelier-wlk4 && rg -n '\"'\"'Mission And Issue-Set Authoring Standard|mission-authoring\\.md'\"'\"' docs .agents --glob '\"'\"'*.md'\"'\"''"
updated_at: "2026-07-09T16:25:51.629259319+00:00"
---

## Summary

bash -lc 'git diff --check -- '"'"'*.md'"'"' && atelier check atelier-wlk4 && rg -n '"'"'Mission And Issue-Set Authoring Standard|mission-authoring\.md'"'"' docs .agents --glob '"'"'*.md'"'"''

## Command

```console
bash -lc 'git diff --check -- '"'"'*.md'"'"' && atelier check atelier-wlk4 && rg -n '"'"'Mission And Issue-Set Authoring Standard|mission-authoring\.md'"'"' docs .agents --glob '"'"'*.md'"'"''
```

Exit status: 0

## Stdout

Bytes: 746
Truncated: no

```text
Lint passed.
docs/product/mission-authoring.md:1:# Mission And Issue-Set Authoring Standard
docs/product/work-model.md:120:Standard](mission-authoring.md). The reviewer approves one exact graph revision.
docs/product/work-model.md:215:are in [Mission And Issue-Set Authoring Standard](mission-authoring.md). This
.agents/skills/agent-factory/procedures/plan.md:55:Standard](../../../../docs/product/mission-authoring.md) is the single normative
docs/product/index.md:22:- [Mission And Issue-Set Authoring Standard](mission-authoring.md): the
docs/index.md:66:- [docs/product/mission-authoring.md](product/mission-authoring.md):
docs/architecture/quality/validation.md:35:Standard](../../product/mission-authoring.md) to the exact current mission
```

## Stderr

Bytes: 0
Truncated: no

```text
```
