---
created_at: "2026-11-07T03:34:44.058758137+00:00"
id: "atelier-ez91"
evidence_type: "test"
commit: "944e22c35dc76a8cd00d1b8108999357e3e7b3bb"
result: "pass"
captured_at: "2026-11-07T03:34:41.753015056+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test proof_timing"
exit_status: "0"
agent_identity: "independent mission validator (claude-opus-5-5)"
target:
  kind: "issue"
  id: "atelier-n4mj"
  role: "validates"
criteria:
- slug: "proof-timing"
  result: "pass"
criteria_mission: "atelier-n4mj"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-n4mj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "proof-timing: scenario test passes (brief lists check with reason/stand-in and 'reason missing'; Awaiting Validation in status, work all, work mission; tick on base clears and land accepts; mission close warns and names, carried check exempt). Caveat: the scenario item has no review findings; with a resolved fix_now finding, ticking the check on the base makes land refuse (findings.disposed), pre-existing gate, see validation report."
updated_at: "2026-11-07T03:34:44.059054940+00:00"
---

## Summary

proof-timing: scenario test passes (brief lists check with reason/stand-in and 'reason missing'; Awaiting Validation in status, work all, work mission; tick on base clears and land accepts; mission close warns and names, carried check exempt). Caveat: the scenario item has no review findings; with a resolved fix_now finding, ticking the check on the base makes land refuse (findings.disposed), pre-existing gate, see validation report.

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test proof_timing
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 681
SHA-256: 8a04a8488e7c8a441ed18e4a9f4fad9860aa813fd04445a8cea3839ca16566d0
Test summary: Summary [   0.730s] 2 tests run: 2 passed, 0 skipped
