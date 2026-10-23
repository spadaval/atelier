---
created_at: "2026-10-23T00:11:59.825020829+00:00"
id: "atelier-864c"
evidence_type: "test"
commit: "2f43f5235f17814eab0550cd34f925a8a6dc6e68"
result: "fail"
captured_at: "2026-10-23T00:11:59.637980160+00:00"
command: "sh -c '! grep -rn -i '\"'\"'is not code\\|non-trivial code change\\|non-trivial code has\\|non-trivial code goes\\|review of all non-trivial code\\|reviewer for non-trivial code\\|own code\\|code you write\\|code you wrote\\|code the orchestrator\\|unreviewed code\\|wrote code\\|wrote the code\\|code fact\\|write this code\\|against the code\\|count as code for review'\"'\"' .agents/skills/agent-factory'"
exit_status: "1"
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
title: "No code-only review, self-review, dispute or rebuttal statement remains outside code-specific guidance"
updated_at: "2026-10-23T00:11:59.825675198+00:00"
---

## Summary

No code-only review, self-review, dispute or rebuttal statement remains outside code-specific guidance

## Command

```console
sh -c '! grep -rn -i '"'"'is not code\|non-trivial code change\|non-trivial code has\|non-trivial code goes\|review of all non-trivial code\|reviewer for non-trivial code\|own code\|code you write\|code you wrote\|code the orchestrator\|unreviewed code\|wrote code\|wrote the code\|code fact\|write this code\|against the code\|count as code for review'"'"' .agents/skills/agent-factory'
```

Exit status: 1

## Stdout

Bytes: 207
Truncated: no

```text
.agents/skills/agent-factory/maintainers/skill-evaluation.md:91:| Context gap | A disposition depends on a code fact the orchestrator lacks | Ask one focused question rather than guessing or re-reviewing. |
```

## Stderr

Bytes: 0
Truncated: no

```text
```

