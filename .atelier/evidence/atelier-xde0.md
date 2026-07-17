---
created_at: "2026-07-16T18:31:52.901512154+00:00"
id: "atelier-xde0"
evidence_type: "test"
captured_at: "2026-07-16T18:31:52.736054995+00:00"
command: "bash -lc 'set -euo pipefail; rg -q \"does not authorize the planner to approve\" .agents/skills/agent-factory/procedures/plan.md; rg -q \"current independent approval and is reported ready\" .agents/skills/agent-factory/procedures/orchestrate.md; rg -q \"Complete graph scope:\" .agents/skills/agent-factory/procedures/orchestrate.md; rg -q \"not fully operable for Agent Factory mission planning\" .agents/skills/agent-factory/fixtures/mission-review-handoffs.md; rg -q \"Atelier reports: mission atelier-fixture-handoff has current independent\" .agents/skills/agent-factory/fixtures/mission-review-handoffs.md; git diff --check'"
exit_status: "1"
target:
  kind: "issue"
  id: "atelier-qi40"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qi40"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Focused Agent Factory inspection attempt failed because its assertion mismatched the implemented wording."
updated_at: "2026-07-16T18:31:57.379176323+00:00"
---

## Summary

Focused Agent Factory inspection attempt failed before evaluating the remaining
assertions: its first search used `does not authorize the planner to approve`,
while the implemented guidance correctly says `does not authorize the planner
to approve`. The corrected focused inspection is recorded separately.

## Command

```console
bash -lc 'set -euo pipefail; rg -q "does not authorize the planner to approve" .agents/skills/agent-factory/procedures/plan.md; rg -q "current independent approval and is reported ready" .agents/skills/agent-factory/procedures/orchestrate.md; rg -q "Complete graph scope:" .agents/skills/agent-factory/procedures/orchestrate.md; rg -q "not fully operable for Agent Factory mission planning" .agents/skills/agent-factory/fixtures/mission-review-handoffs.md; rg -q "Atelier reports: mission atelier-fixture-handoff has current independent" .agents/skills/agent-factory/fixtures/mission-review-handoffs.md; git diff --check'
```

Exit status: 1

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
