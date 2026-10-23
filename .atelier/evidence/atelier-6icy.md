---
created_at: "2026-10-23T00:07:20.081392327+00:00"
id: "atelier-6icy"
evidence_type: "test"
commit: "03690da197713eb3933e75b1415f55e91bc56b3e"
result: "pass"
captured_at: "2026-10-23T00:07:19.988033944+00:00"
command: "sh -c '! grep -rn -i '\"'\"'is not code\\|non-trivial code change\\|non-trivial code has\\|non-trivial code goes\\|review of all non-trivial code\\|reviewer for non-trivial code'\"'\"' .agents/skills/agent-factory'"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-aufe"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-aufe"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "No statement in the vendored skill limits review to code outside code-specific guidance"
updated_at: "2026-10-23T00:07:20.081679792+00:00"
---

## Summary

No statement in the vendored skill limits review to code outside code-specific guidance

## Command

```console
sh -c '! grep -rn -i '"'"'is not code\|non-trivial code change\|non-trivial code has\|non-trivial code goes\|review of all non-trivial code\|reviewer for non-trivial code'"'"' .agents/skills/agent-factory'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 0
Truncated: no

```text
```

