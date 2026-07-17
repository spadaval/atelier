---
created_at: "2026-07-09T16:58:41.752441923+00:00"
id: "atelier-0ovx"
evidence_type: "test"
captured_at: "2026-07-09T16:58:41.133386605+00:00"
command: "bash -lc 'set -euo pipefail; rg -n \"mission-review|Exact graph revision:|Assigned reviewer:\" .agents/skills/agent-factory/SKILL.md .agents/skills/agent-factory/procedures/mission-review.md .agents/skills/agent-factory/fixtures/mission-review-dogfood.md; git diff --check; atelier check atelier-l5mw'"
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
title: "Clean focused check: mission-review is routed independently, requires exact graph-revision and reviewer inputs, and includes complete plus negative independent-review packets."
updated_at: "2026-07-09T16:58:45.774568427+00:00"
---

## Summary

Clean focused check: mission-review is routed independently, requires exact graph-revision and reviewer inputs, and includes complete plus negative independent-review packets.

## Command

```console
bash -lc 'set -euo pipefail; rg -n "mission-review|Exact graph revision:|Assigned reviewer:" .agents/skills/agent-factory/SKILL.md .agents/skills/agent-factory/procedures/mission-review.md .agents/skills/agent-factory/fixtures/mission-review-dogfood.md; git diff --check; atelier check atelier-l5mw'
```

Exit status: 0

## Stdout

Bytes: 1196
Truncated: no

```text
.agents/skills/agent-factory/fixtures/mission-review-dogfood.md:3:These packets are input fixtures for a separately assigned `mission-review`
.agents/skills/agent-factory/fixtures/mission-review-dogfood.md:19:Exact graph revision: mission-plan-graph-v1:sha256:002869c1092e45a5e0bd40a5e2e56895094552f122231035d6449f98a829ccbc
.agents/skills/agent-factory/fixtures/mission-review-dogfood.md:22:Assigned reviewer: fixture-independent-reviewer
.agents/skills/agent-factory/fixtures/mission-review-dogfood.md:66:Exact graph revision: mission-plan-graph-v1:sha256:613018d218cebe89c42cf184e4af4706fae212cf4667a163526efd25049a9fce
.agents/skills/agent-factory/fixtures/mission-review-dogfood.md:69:Assigned reviewer: fixture-author
.agents/skills/agent-factory/SKILL.md:57:| `mission-review` | Independently falsify the readiness of an exact mission draft and its reachable issue graph | [procedures/mission-review.md](procedures/mission-review.md) |
.agents/skills/agent-factory/SKILL.md:77:   `mission-review`. This is neither code `review` nor scenario `validate`.
.agents/skills/agent-factory/procedures/mission-review.md:16:Exact graph revision: <revision algorithm version and digest>
Lint passed.
```

## Stderr

Bytes: 0
Truncated: no

```text
```
