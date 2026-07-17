---
created_at: "2026-07-16T18:48:30.091502807+00:00"
id: "atelier-av92"
evidence_type: "validation"
captured_at: "2026-07-16T18:48:29.463227034+00:00"
command: "bash -lc 'set -euo pipefail\nfixture=.agents/skills/agent-factory/fixtures/mission-review-handoffs.md\ndigest=mission-plan-graph-v1:sha256:5f2a625447f52a44e6b17ca868c54128a6d98db78ded4a0d245268a355cab40b\nrg -Fq \"State reported by Atelier before a review request: draft\" \"$fixture\"\nrg -Fq \"Atelier transition: draft -> plan_review\" \"$fixture\"\nrg -Fq \"State reported by Atelier after the request: plan_review; review requested\" \"$fixture\"\ntest \"$(rg -o \"$digest\" \"$fixture\" | wc -l)\" -eq 4\ntest \"$(rg -o \"mission-plan-graph-v1:sha256:[0-9a-f]+\" \"$fixture\" | sort -u | wc -l)\" -eq 1\nrg -q \"Live lifecycle behavior remains deferred to .atelier-72k4.\" \"$fixture\"\n! rg -Fq \"2d9c6a\" \"$fixture\"\ngit diff --check\natelier check atelier-qi40'"
exit_status: "0"
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
title: "Static fixture inspection passed: the documented handoff distinguishes draft before the repository review request from plan_review after it, binds all handoff stages to one full digest, and classifies live lifecycle behavior as deferred to atelier-72k4; it does not prove live lifecycle behavior."
updated_at: "2026-07-16T18:48:34.372831441+00:00"
---

## Summary

Static fixture inspection passed: the documented handoff distinguishes draft before the repository review request from plan_review after it, binds all handoff stages to one full digest, and classifies live lifecycle behavior as deferred to atelier-72k4; it does not prove live lifecycle behavior.

## Command

```console
bash -lc 'set -euo pipefail
fixture=.agents/skills/agent-factory/fixtures/mission-review-handoffs.md
digest=mission-plan-graph-v1:sha256:5f2a625447f52a44e6b17ca868c54128a6d98db78ded4a0d245268a355cab40b
rg -Fq "State reported by Atelier before a review request: draft" "$fixture"
rg -Fq "Atelier transition: draft -> plan_review" "$fixture"
rg -Fq "State reported by Atelier after the request: plan_review; review requested" "$fixture"
test "$(rg -o "$digest" "$fixture" | wc -l)" -eq 4
test "$(rg -o "mission-plan-graph-v1:sha256:[0-9a-f]+" "$fixture" | sort -u | wc -l)" -eq 1
rg -q "Live lifecycle behavior remains deferred to .atelier-72k4." "$fixture"
! rg -Fq "2d9c6a" "$fixture"
git diff --check
atelier check atelier-qi40'
```

Exit status: 0

## Stdout

Bytes: 13
Truncated: no

```text
Lint passed.
```

## Stderr

Bytes: 0
Truncated: no

```text
```
