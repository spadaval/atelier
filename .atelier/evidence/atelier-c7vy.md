---
created_at: "2026-11-07T04:10:16.469358862+00:00"
id: "atelier-c7vy"
evidence_type: "test"
commit: "74aec6f61766987840cd50c5e757869fb7d5687b"
result: "pass"
captured_at: "2026-11-07T04:10:14.213034876+00:00"
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
title: "proof-timing at 74aec6f6 (after atelier-secy): scenario now has a fixed fix_now finding; tick and carried-to-issue on the base land, other Done when/Goal edits still refuse. Hand matrix (finding 0/1 x evidence.attached gate 0/1 x tick/carry/carry-prose/goal edit, pushed upstream): tick and carry land and clear Awaiting Validation, receipt stays fresh; carry to prose and goal edits refuse as before (findings.disposed, evidence.attached)."
updated_at: "2026-11-07T04:10:16.469859025+00:00"
---

## Summary

proof-timing at 74aec6f6 (after atelier-secy): scenario now has a fixed fix_now finding; tick and carried-to-issue on the base land, other Done when/Goal edits still refuse. Hand matrix (finding 0/1 x evidence.attached gate 0/1 x tick/carry/carry-prose/goal edit, pushed upstream): tick and carry land and clear Awaiting Validation, receipt stays fresh; carry to prose and goal edits refuse as before (findings.disposed, evidence.attached).

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test proof_timing
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 937
SHA-256: e87691872d4c96f5afa06fda6b21c2d13bc403f4ad674d165e7a1d46f0c341aa
Test summary: Summary [   0.764s] 4 tests run: 4 passed, 0 skipped
