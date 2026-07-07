---
created_at: "2026-07-06T22:57:54.610979745+00:00"
id: "atelier-riu1"
evidence_type: "test"
captured_at: "2026-07-06T22:57:43.651904308+00:00"
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
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "PASS implementation proof responding to atelier-igno at 12c65606: all 14 exact production-pipeline cases now pass (six common action/table rejects, four for-the data/type/transition/role/label allows, two realistic console-prompt rejects, and labeled/untyped YAML scalar-list allows), while the prior nine-case batch remains 0 failures. Classification stays bounded to span-adjacent action/data grammar, recognized prompt prefixes, and explicit data-fence/scalar shapes; shell prompts and command-shaped subcommand/option lines still reject. Guard and wrapper self-test/inventory/live modes pass (294 cases; 46 removed roots; 88 indexed docs); focused nextest passes 9/9; root/work/issue/check help, fmt, diff, mission/c0mp ancestry and byte identity, and atelier check pass. This is implementation proof, not independent validation or review approval."
updated_at: "2026-07-06T22:58:15.707937368+00:00"
---

## Summary

PASS implementation proof responding to atelier-igno at 12c65606: all 14 exact production-pipeline cases now pass (six common action/table rejects, four for-the data/type/transition/role/label allows, two realistic console-prompt rejects, and labeled/untyped YAML scalar-list allows), while the prior nine-case batch remains 0 failures. Classification stays bounded to span-adjacent action/data grammar, recognized prompt prefixes, and explicit data-fence/scalar shapes; shell prompts and command-shaped subcommand/option lines still reject. Guard and wrapper self-test/inventory/live modes pass (294 cases; 46 removed roots; 88 indexed docs); focused nextest passes 9/9; root/work/issue/check help, fmt, diff, mission/c0mp ancestry and byte identity, and atelier check pass. This is implementation proof, not independent validation or review approval.

## Command

```console
scripts/check_active_command_guidance.sh --self-test
```

Exit status: 0

## Stdout

Bytes: 163
Truncated: no

```text
active command guidance self-test passed: 294 prohibited/context-restricted example(s), including 38 adversarial occurrence fixture(s) and all prior quality cases
```

## Stderr

Bytes: 0
Truncated: no

```text
```
