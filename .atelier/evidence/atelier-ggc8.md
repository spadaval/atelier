---
created_at: "2026-07-09T16:58:21.280657794+00:00"
id: "atelier-ggc8"
evidence_type: "test"
captured_at: "2026-07-09T16:58:20.699369600+00:00"
command: "bash -lc 'set -euo pipefail; rg -n -F -e \"| `mission-review` |\" -e \"If the work starts from an exact mission draft\" -e \"Exact graph revision:\" -e \"Assigned reviewer:\" .agents/skills/agent-factory/SKILL.md .agents/skills/agent-factory/procedures/mission-review.md .agents/skills/agent-factory/fixtures/mission-review-dogfood.md; git diff --check; atelier check atelier-l5mw'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-l5mw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-l5mw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Mission-review routing, procedure inputs, and independent-review dogfood packets are present; no dogfood approval was self-issued."
updated_at: "2026-07-09T16:58:25.559806983+00:00"
---

## Summary

Mission-review routing, procedure inputs, and independent-review dogfood packets are present; no dogfood approval was self-issued.

## Command

```console
bash -lc 'set -euo pipefail; rg -n -F -e "| `mission-review` |" -e "If the work starts from an exact mission draft" -e "Exact graph revision:" -e "Assigned reviewer:" .agents/skills/agent-factory/SKILL.md .agents/skills/agent-factory/procedures/mission-review.md .agents/skills/agent-factory/fixtures/mission-review-dogfood.md; git diff --check; atelier check atelier-l5mw'
```

Exit status: 0

## Stdout

Bytes: 836
Truncated: no

```text
.agents/skills/agent-factory/fixtures/mission-review-dogfood.md:19:Exact graph revision: mission-plan-graph-v1:sha256:002869c1092e45a5e0bd40a5e2e56895094552f122231035d6449f98a829ccbc
.agents/skills/agent-factory/fixtures/mission-review-dogfood.md:22:Assigned reviewer: fixture-independent-reviewer
.agents/skills/agent-factory/fixtures/mission-review-dogfood.md:66:Exact graph revision: mission-plan-graph-v1:sha256:613018d218cebe89c42cf184e4af4706fae212cf4667a163526efd25049a9fce
.agents/skills/agent-factory/fixtures/mission-review-dogfood.md:69:Assigned reviewer: fixture-author
.agents/skills/agent-factory/SKILL.md:75:5. If the work starts from an exact mission draft and its complete reachable
.agents/skills/agent-factory/procedures/mission-review.md:16:Exact graph revision: <revision algorithm version and digest>
Lint passed.
```

## Stderr

Bytes: 48
Truncated: no

```text
bash: line 1: mission-review: command not found
```
