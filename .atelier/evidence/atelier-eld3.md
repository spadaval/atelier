---
created_at: "2026-07-06T22:41:09.480084997+00:00"
id: "atelier-eld3"
evidence_type: "test"
captured_at: "2026-07-06T22:40:59.692246974+00:00"
command: "scripts/check_active_command_guidance.sh --self-test"
exit_status: "0"
agent_identity: "implementation-agent"
target:
  kind: "issue"
  id: "atelier-durs"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-durs"
    role: "validates"
  - kind: "issue"
    id: "atelier-p0am"
    role: "validates"
  - kind: "issue"
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "PASS implementation proof for corrected failure atelier-pgwg/transcript atelier-ztnc at d75bd7f8: the production scanner rejects all four exact actionable inline variants and allows all five exact data/schema/YAML variants. Action context is bounded around each inline span; explicit adjacent as-type/transition/role/value/label/data syntax is non-command data; untyped fenced YAML/structured literals are distinct from shell prompts and recognizable command forms. Guard and wrapper self-test/inventory/live modes pass (269 cases; 46 removed roots; 88 indexed docs); independent exact nine-case batch passes; focused nextest passes 9/9; root/work/issue/check help, fmt, diff, mission/c0mp ancestry and byte identity, and atelier check pass. This is implementation proof, not independent validation or review approval."
updated_at: "2026-07-06T22:41:50.484215889+00:00"
---

## Summary

PASS implementation proof for corrected failure atelier-pgwg/transcript atelier-ztnc at d75bd7f8: the production scanner rejects all four exact actionable inline variants and allows all five exact data/schema/YAML variants. Action context is bounded around each inline span; explicit adjacent as-type/transition/role/value/label/data syntax is non-command data; untyped fenced YAML/structured literals are distinct from shell prompts and recognizable command forms. Guard and wrapper self-test/inventory/live modes pass (269 cases; 46 removed roots; 88 indexed docs); independent exact nine-case batch passes; focused nextest passes 9/9; root/work/issue/check help, fmt, diff, mission/c0mp ancestry and byte identity, and atelier check pass. This is implementation proof, not independent validation or review approval.

## Command

```console
scripts/check_active_command_guidance.sh --self-test
```

Exit status: 0

## Stdout

Bytes: 163
Truncated: no

```text
active command guidance self-test passed: 269 prohibited/context-restricted example(s), including 38 adversarial occurrence fixture(s) and all prior quality cases
```

## Stderr

Bytes: 0
Truncated: no

```text
```
